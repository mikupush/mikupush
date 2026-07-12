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
use crate::upload::UploadRequest;
use log::{debug, warn};
use rust_i18n::t;
use std::borrow::Cow;
use std::fs::File;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub fn enqueue_upload_paths(
    app_handle: &AppHandle,
    file_paths: Vec<String>,
    always_notify: bool,
) -> Result<Vec<UploadRequest>, String> {
    debug!("enqueue many files to uploads: {}", file_paths.join(";"));
    let mut in_progress_uploads: Vec<UploadRequest> = vec![];

    for path in file_paths {
        in_progress_uploads = enqueue_upload_path(app_handle, path, always_notify)?;
    }

    Ok(in_progress_uploads)
}

pub(super) fn enqueue_upload_path(
    app_handle: &AppHandle,
    file_path: String,
    always_notify: bool,
) -> Result<Vec<UploadRequest>, String> {
    debug!("starting upload for path: {}", file_path);
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
    let mut request = UploadRequest::from_path(file_path, server.clone())?;
    if chunked_mode {
        request = request.upload_by_chunks(chunk_size);
    }

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
        server,
        always_notify,
        retry: false,
    })?;

    Ok(in_progress_uploads)
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

    debug!("launching enqueue uploads task");
    for path in paths {
        #[cfg(target_os = "macos")]
        let path = {
            let original = path.as_str();
            let decoded = urlencoding::decode(original).unwrap_or(Cow::from(original));
            String::from(decoded)
        };

        let result = enqueue_upload_path(app_handle, path, true);

        if let Err(err) = result {
            warn!("failed handling share deep-link: {}", err)
        }
    }
}
