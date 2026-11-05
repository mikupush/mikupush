// Miku Push! is a simple, lightweight, and open-source WeTransfer alternative for desktop.
// Copyright (C) 2025  Miku Push! Team
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#[cfg(target_os = "macos")]
use super::helpers::MACOS_APP_GROUP_ID;
use super::helpers::{
    emit_uploads_changed_event, show_upload_notification, update_upload_request_and_emit,
};
use super::queue::{UploadQueueJob, enqueue_upload_job};
use crate::config::Configuration;
use crate::config::{CONFIG_CHUNK_SIZE_DEFAULT, CONFIG_TRUE_VALUE, ConfigKey};
use crate::state::{SelectedServerState, UploadsState};
use crate::upload::{UploadPreferences, UploadRequest};
use log::{debug, warn};
use rust_i18n::t;
use std::borrow::Cow;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager};

static PENDING_UPLOAD_PATHS: Mutex<Vec<Vec<String>>> = Mutex::new(Vec::new());

pub fn take_pending_upload_paths() -> Vec<Vec<String>> {
    std::mem::take(&mut *PENDING_UPLOAD_PATHS.lock().unwrap())
}

pub fn enqueue_upload_paths(
    app_handle: &AppHandle,
    file_paths: Vec<String>,
    always_notify: bool,
) -> Result<Vec<UploadRequest>, String> {
    if file_paths.is_empty() {
        return Ok(app_handle.state::<UploadsState>().get_all_in_progress());
    }

    let config = Configuration::from_app_handle(app_handle)?;

    if config.get(ConfigKey::AskUploadPreferences) == CONFIG_TRUE_VALUE {
        PENDING_UPLOAD_PATHS.lock().unwrap().push(file_paths);
        crate::window::restore_main_window(app_handle, false);
        app_handle
            .emit("upload-preferences-requested", ())
            .map_err(|err| err.to_string())?;

        return Ok(app_handle.state::<UploadsState>().get_all_in_progress());
    }

    let preferences = serde_json::from_str(&config.get(ConfigKey::UploadPreferences))
        .map_err(|err| format!("Invalid upload preferences: {}", err))?;

    enqueue_upload_paths_with_preferences(app_handle, file_paths, always_notify, preferences)
}

pub fn enqueue_upload_paths_with_preferences(
    app_handle: &AppHandle,
    file_paths: Vec<String>,
    always_notify: bool,
    preferences: UploadPreferences,
) -> Result<Vec<UploadRequest>, String> {
    debug!("enqueue many files to uploads: {}", file_paths.join(";"));
    let server_state = app_handle.state::<SelectedServerState>();
    let upload_state = app_handle.state::<UploadsState>();

    let configuration_repository = Configuration::from_app_handle(app_handle)?;
    let chunked_mode = configuration_repository.get(ConfigKey::UploadInChunks) == CONFIG_TRUE_VALUE;
    let chunk_size = match configuration_repository
        .get(ConfigKey::UploadChunkSize)
        .parse::<u64>()
    {
        Ok(size) => size * 1024 * 1024, // MB to bytes
        Err(err) => {
            warn!(
                "failed to parse configured upload chunk size: {}; using default",
                err
            );
            CONFIG_CHUNK_SIZE_DEFAULT
        }
    };

    let server = server_state.current_server();
    // Validate the whole selection before starting any transfer. A missing path
    // must not leave a partly started batch that the modal could submit again.
    let requests = file_paths
        .into_iter()
        .map(|path| {
            let mut request = UploadRequest::from_path(path, server.clone(), preferences.clone())?;
            if chunked_mode {
                request = request.upload_by_chunks(chunk_size);
            }
            Ok(request)
        })
        .collect::<Result<Vec<_>, String>>()?;

    for request in requests {
        let in_progress_uploads = upload_state.add_request(request.clone());
        emit_uploads_changed_event(app_handle, in_progress_uploads.clone());

        show_upload_notification(
            app_handle,
            t!(
                "notifications.upload.enqueued.title",
                name = request.upload.name
            )
            .to_string(),
            t!(
                "notifications.upload.enqueued.body",
                name = request.upload.name
            )
            .to_string(),
            always_notify,
        );

        enqueue_upload_job(UploadQueueJob {
            request,
            server: server.clone(),
            always_notify,
            retry: false,
        })?;
    }
    Ok(upload_state.get_all_in_progress())
}

pub fn enqueue_upload_retry(
    app_handle: &AppHandle,
    uploads_state: &UploadsState,
    server_state: &SelectedServerState,
    upload_id: String,
) -> Result<(), String> {
    debug!("retrying upload with id {}", upload_id);

    let upload_request = uploads_state.get_request(upload_id.clone());
    if let None = upload_request {
        warn!(
            "can't retry upload request with id {}: not found",
            upload_id
        );
        return Ok(());
    }

    let upload_request = upload_request.unwrap();
    let upload_request = upload_request.reset_progress();
    update_upload_request_and_emit(app_handle, upload_request.clone());
    enqueue_upload_job(UploadQueueJob {
        request: upload_request,
        server: server_state.current_server(),
        always_notify: false,
        retry: true,
    })?;

    Ok(())
}

pub fn enqueue_uploads_from_deep_link(app_handle: &AppHandle, request_file: &str) {
    debug!("handling share deep-link: {}", request_file);

    #[cfg(target_os = "macos")]
    let directory: PathBuf = match crate::macos::get_group_container_path(MACOS_APP_GROUP_ID) {
        Some(path) => path.as_str().into(),
        None => {
            warn!("failed to get app group {} directory", MACOS_APP_GROUP_ID);
            return;
        }
    };

    #[cfg(not(target_os = "macos"))]
    let directory = match app_handle.path().temp_dir() {
        Ok(path) => path.join("io.mikupush.client"),
        Err(err) => {
            warn!("failed to get local data directory: {}", err);
            return;
        }
    };

    let request_file_path = directory.join(request_file);
    debug!("opening request file: {:?}", request_file_path);
    let file = match File::open(&request_file_path) {
        Ok(file) => file,
        Err(err) => {
            warn!("failed to open share request paths file: {}", err);
            return;
        }
    };

    let paths: Vec<String> = match serde_json::from_reader(file) {
        Ok(paths) => {
            debug!("requested paths for upload from deep-link: {:?}", paths);
            debug!("deleting share request paths file");
            if let Err(err) = std::fs::remove_file(&request_file_path) {
                warn!("failed to delete share request paths file: {}", err);
            }

            paths
        }
        Err(err) => {
            warn!("failed to parse share requests paths: {}", err);
            return;
        }
    };

    // A share action is one selection, even when it contains several paths.
    #[cfg(target_os = "macos")]
    let paths = paths
        .into_iter()
        .map(|path| {
            urlencoding::decode(&path)
                .unwrap_or(Cow::from(path.as_str()))
                .into_owned()
        })
        .collect();

    if let Err(err) = enqueue_upload_paths(app_handle, paths, true) {
        warn!("failed handling share deep-link: {}", err);
    }
}
