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

use std::borrow::Cow;
use std::fs::File;
use crate::events::*;
use mikupush_common::{ConfigKey, Progress, UploadRequest, CONFIG_CHUNK_SIZE_DEFAULT, CONFIG_TRUE_VALUE};
use mikupush_client::{Client, ClientError, FileStatus, FileUploadError, FILE_INFO_ERROR_NOT_EXISTS};
use crate::state::{SelectedServerState, UploadsState};
use log::{debug, error, info, warn};
use rust_i18n::t;
use tauri::{AppHandle, Emitter, Manager, State, Window};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_notification::NotificationExt;
use uuid::Uuid;
use crate::config::Configuration;
use crate::MAIN_WINDOW;

#[tauri::command]
pub async fn select_files_to_upload(app_handle: AppHandle) -> Result<Vec<UploadRequest>, String> {
    let files = app_handle
        .dialog()
        .file()
        .blocking_pick_files()
        .unwrap_or_default()
        .iter()
        .map(|file| file.to_string())
        .collect();

    debug!("attempting to upload files {:?}", files);
    let in_progress_uploads = enqueue_many_uploads(app_handle, files).await?;

    debug!(
        "returning in progress equeued uploads: {:?}",
        in_progress_uploads
    );
    Ok(in_progress_uploads)
}

pub fn start_upload_for_collection(
    app_handle: &AppHandle,
    file_paths: Vec<String>,
    always_notify: bool
) -> Result<Vec<UploadRequest>, String> {
    debug!("enqueue many files to uploads: {}", file_paths.join(";"));
    let mut in_progress_uploads: Vec<UploadRequest> = vec![];

    for path in file_paths {
        in_progress_uploads = start_upload(&app_handle, path, always_notify)?;
    }

    Ok(in_progress_uploads)
}

pub fn start_upload(
    app_handle: &AppHandle,
    file_path: String,
    always_notify: bool
) -> Result<Vec<UploadRequest>, String> {
    debug!("starting upload for path: {}", file_path);
    let server_state = app_handle.state::<SelectedServerState>();
    let upload_state = app_handle.state::<UploadsState>();

    let configuration_repository = Configuration::from_app_handle(app_handle)?;
    let chunked_mode = configuration_repository.get(ConfigKey::UploadInChunks) == CONFIG_TRUE_VALUE;
    let chunk_size = match configuration_repository.get(ConfigKey::UploadChunkSize).parse::<u64>() {
        Ok(size) => size * 1024 * 1024, // MB to bytes
        Err(err) => {
            warn!("failed to parse configured upload chunk size: {}; using default", err);
            CONFIG_CHUNK_SIZE_DEFAULT
        }
    };

    let server = server_state.current_server();
    let mut request = UploadRequest::from_file_path(file_path, server)?;
    if chunked_mode {
        request = request.upload_by_chunks(chunk_size);
    }

    let upload_id = request.upload.id.clone().to_string();
    let in_progress_uploads = upload_state.add_request(request.clone());
    let client = server_state.clone().client();

    show_notification(
        &app_handle,
        t!("notifications.upload.enqueued.title", name = request.upload.name).to_string(),
        t!("notifications.upload.enqueued.body", name = request.upload.name).to_string(),
        always_notify
    );

    let app_handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(error) = client.create(&request.clone().upload).await {
            warn!("error registering file {:?}", error);
            handle_upload_failed(&app_handle, error.into(), upload_id, always_notify);
            return;
        }

        match upload_file(&app_handle, client, request.clone()).await {
            Ok(_) => handle_upload_finish(&app_handle, upload_id, always_notify),
            Err(error) => handle_upload_failed(&app_handle, error, upload_id, always_notify),
        }
    });

    Ok(in_progress_uploads)
}

#[tauri::command]
pub async fn enqueue_many_uploads(app_handle: AppHandle, paths: Vec<String>) -> Result<Vec<UploadRequest>, String> {
    start_upload_for_collection(&app_handle, paths, false)
}

#[tauri::command]
pub async fn enqueue_upload(app_handle: AppHandle, file_path: String) -> Result<Vec<UploadRequest>, String> {
    start_upload(&app_handle, file_path, false)
}

#[tauri::command]
pub async fn retry_upload(
    app_handle: AppHandle,
    uploads_state: State<'_, UploadsState>,
    server_state: State<'_, SelectedServerState>,
    upload_id: String,
) -> Result<(), String> {
    debug!("retrying upload with id {}", upload_id);

    let client = server_state.client();
    let upload_request = uploads_state.get_request(upload_id.clone());
    if let None = upload_request {
        warn!("can't retry upload request with id {}: not found", upload_id);
        return Ok(());
    }

    let upload_request = upload_request.unwrap();
    update_upload_request_state(&app_handle, upload_request.reset_progress());

    let app_handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        let info = client.info(upload_request.upload.id).await;
        if let Err(error) = info.clone() {
            if error.code() != FILE_INFO_ERROR_NOT_EXISTS {
                debug!("error retrieving file info during upload retry: {}", error);
                update_upload_request_state(
                    &app_handle,
                    upload_request.finish_with_error(error.code(), error.to_string())
                );
                return;
            }

            debug!("upload with id {} is not registered, registering again", upload_request.upload.id);
            if let Err(error) = client.create(&upload_request.clone().upload).await {
                warn!("error registering file {:?}", error);
                handle_upload_failed(&app_handle, error, upload_id, false);
                return;
            }
        }

        if let Ok(info) = info && info.status == FileStatus::Uploaded {
            debug!("upload with id {} is already uploaded, aborting retry", upload_request.upload.id);
            return;
        }

        match upload_file(&app_handle, client, upload_request.clone()).await {
            Ok(_) => handle_upload_finish(&app_handle, upload_id, false),
            Err(error) => handle_upload_failed(&app_handle, error, upload_id, false),
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn delete_upload(
    server_state: State<'_, SelectedServerState>,
    uploads_state: State<'_, UploadsState>,
    upload_id: String,
) -> Result<Vec<UploadRequest>, String> {
    debug!("deleting upload with id {}", upload_id.clone());

    let id = Uuid::parse_str(upload_id.as_str()).map_err(|err| err.to_string())?;
    let client = server_state.client();
    client.delete(id).await.map_err(|err| err.to_string())?;
    let uploads = uploads_state.delete_request(upload_id.clone());

    debug!("deleted upload with id {}", upload_id.clone());
    Ok(uploads)
}

#[tauri::command]
pub fn cancel_upload(
    uploads_state: State<'_, UploadsState>,
    upload_id: String,
) -> Vec<UploadRequest> {
    debug!("canceling upload for: {}", upload_id);
    uploads_state.cancel_upload(upload_id.clone());
    uploads_state.delete_request(upload_id.clone())
}

#[tauri::command]
pub async fn copy_upload_link(
    app_handle: AppHandle,
    uploads_state: State<'_, UploadsState>,
    upload_id: String,
) -> Result<(), String> {
    let upload = uploads_state.get_request(upload_id.clone());
    if upload.is_none() {
        warn!("upload with id {} not found", upload_id);
        return Err(t!("errors.upload.not_found").to_string());
    }

    let upload = upload.unwrap();
    let upload = upload.upload;
    let result = app_handle.clipboard().write_text(upload.url);
    if let Err(error) = result {
        warn!(
            "failed to copy link to the clipboard for upload id {}: {}",
            upload_id,
            error.to_string()
        );
        return Err(t!("errors.upload.copy_link").to_string());
    }

    Ok(())
}

#[tauri::command]
pub fn get_all_in_progress_uploads(uploads_state: State<'_, UploadsState>) -> Vec<UploadRequest> {
    debug!("get uploads");
    uploads_state.get_all_in_progress()
}

async fn upload_file(
    app_handle: &AppHandle,
    client: Client,
    request: UploadRequest,
) -> Result<(), FileUploadError> {
    let state = app_handle.state::<UploadsState>();
    let upload_id = request.upload.id.clone().to_string();
    debug!("launching file upload for upload with id {}", upload_id);
    let task = client.upload(&request).await?;

    state.add_cancellation_token(upload_id.clone(), task.get_cancellation_token());

    let mut progress_receiver = task.get_progress_receiver();
    let app_handle_clone = app_handle.clone();
    let upload_id_clone = upload_id.clone();
    let handle = task.start();

    tauri::async_runtime::spawn(async move {
        let state = app_handle_clone.state::<UploadsState>();
        let request = state.get_request(upload_id_clone.clone());
        if let None = request {
            warn!(
                "upload request with id {} not found during progress listen",
                upload_id_clone
            );
            return;
        }

        let mut request = request.unwrap();
        while progress_receiver.changed().await.is_ok() {
            let current = *progress_receiver.borrow();
            let progress: Progress = current.clone().into();

            request = request.update_progress(progress);
            state.update_request(request.clone());

            emit_uploads_changed(&app_handle_clone, state.get_all_in_progress())
        }
    });

    let result = handle.await.map_err(|err| {
        FileUploadError::ClientError { message: format!("upload task join error: {}", err.to_string()) }
    })?;
    state.remove_cancellation_token(upload_id.clone());

    debug!("upload task finished for upload with id {}", upload_id);
    result
}

fn handle_upload_finish(app_handle: &AppHandle, upload_id: String, always_notify: bool) {
    info!("upload with id {} finished", upload_id);
    let state = app_handle.state::<UploadsState>();
    let request = state.get_request(upload_id.clone());
    if let None = request {
        warn!(
            "upload request with id {} not found during finish handle",
            upload_id
        );
        return;
    }

    let mut request = request.unwrap();
    request = request.finish();
    state.update_request(request.clone());

    emit_uploads_changed(&app_handle, state.get_all_in_progress());

    show_notification(
        &app_handle,
        t!("notifications.upload.success.title", name = request.upload.name).to_string(),
        t!("notifications.upload.success.body", name = request.upload.name).to_string(),
        always_notify
    );
}

fn handle_upload_failed(
    app_handle: &AppHandle,
    error: FileUploadError,
    upload_id: String,
    always_notify: bool,
) {
    info!("upload with id {} failed: {}", upload_id, error);
    let state = app_handle.state::<UploadsState>();
    let request = state.get_request(upload_id.clone());
    if let None = request {
        warn!(
            "upload request with id {} not found during failed handle",
            upload_id
        );
        return;
    }

    let mut request = request.unwrap();
    if error == FileUploadError::Canceled {
        request = request.canceled();
    } else {
        request = request.finish_with_error(error.code(), error.to_string());
    }

    state.update_request(request.clone());
    emit_uploads_changed(&app_handle, state.get_all_in_progress());

    show_notification(
        app_handle,
        t!("notifications.upload.error.title", name = request.upload.name).to_string(),
        t!("notifications.upload.error.body", name = request.upload.name).to_string(),
        always_notify
    );
}

fn emit_uploads_changed(app_handle: &AppHandle, requests: Vec<UploadRequest>) {
    if let Some(window) = app_handle.get_webview_window(MAIN_WINDOW) {
        match window.emit(UPLOADS_CHANGED_EVENT, requests) {
            Ok(_) => debug!("event {} emited", UPLOADS_CHANGED_EVENT),
            Err(error) => warn!("event {} failed emited: {}", UPLOADS_CHANGED_EVENT, error),
        }
    }
}

fn show_notification(app_handle: &AppHandle, title: String, body: String, always: bool) {
    let app_handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        debug!("showing notification: {} - {}", title, body);

        let is_main_window_visible = || -> bool {
            let window = app_handle.get_webview_window(MAIN_WINDOW);

            if let Some(window) = window {
                window.is_visible().unwrap_or(false)
            } else {
                false
            }
        };

        if !always && is_main_window_visible() {
            debug!("skipping notification because window is visible");
            return;
        }

        let result = app_handle.notification()
            .builder()
            .title(title)
            .body(body)
            .show();

        if let Err(error) = result {
            warn!("failed to show notification: {}", error.to_string());
        }
    });
}

fn update_upload_request_state(app_handle: &AppHandle, upload_request: UploadRequest) {
    let state = app_handle.state::<UploadsState>();
    state.update_request(upload_request.clone());
    emit_uploads_changed(&app_handle, state.get_all_in_progress());
}

pub fn handle_upload_deep_link(app_handle: &AppHandle, request_file: &str) {
    debug!("handling share deep-link: {}", request_file);

    #[cfg(target_os = "macos")]
    let directory = match app_handle.path().home_dir() {
        Ok(path) => {
            path
                .join("Library")
                .join("Group Containers")
                .join("group.io.mikupush.client")
        },
        Err(err) => {
            warn!("failed to get home directory: {}", err);
            return;
        },
    };

    #[cfg(not(target_os = "macos"))]
    let directory = match app_handle.path().temp_dir() {
        Ok(path) => path.join("io.mikupush.client"),
        Err(err) => {
            warn!("failed to get local data directory: {}", err);
            return;
        },
    };

    let request_file_path = directory.join(request_file);
    let file = match File::open(&request_file_path) {
        Ok(file) => file,
        Err(err) => {
            warn!("failed to open share request paths file: {}", err);
            return;
        }
    };

    let paths: Vec<String> = match serde_json::from_reader(file) {
        Ok(paths) => {
            debug!("deleting share request paths file");
            if let Err(err) = std::fs::remove_file(&request_file_path) {
                warn!("failed to delete share request paths file: {}", err);
            }

            paths
        },
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

        let result = start_upload(app_handle, path, true);

        if let Err(err) = result {
            warn!("failed handling share deep-link: {}", err)
        }
    }
}