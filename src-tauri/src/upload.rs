// Copyright 2025 Miku Push! Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::events::*;
use mikupush_common::{Progress, UploadRequest};
use mikupush_client::{Client, ClientError, FileStatus, FileUploadError, FILE_INFO_ERROR_NOT_EXISTS};
use crate::state::{SelectedServerState, UploadsState};
use log::{debug, info, warn};
use rust_i18n::t;
use tauri::{AppHandle, Emitter, Manager, State, Window};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_notification::NotificationExt;
use uuid::Uuid;
use crate::MAIN_WINDOW;

#[tauri::command]
pub async fn select_files_to_upload(
    window: Window,
    app_state: State<'_, UploadsState>,
    server_state: State<'_, SelectedServerState>,
    app_handle: AppHandle,
) -> Result<Vec<UploadRequest>, String> {
    let files = app_handle
        .dialog()
        .file()
        .blocking_pick_files()
        .unwrap_or_default()
        .iter()
        .map(|file| file.to_string())
        .collect();

    debug!("attempting to upload files {:?}", files);
    let in_progress_uploads =
        enqueue_many_uploads(window, app_handle.clone(), app_state, server_state, files).await?;

    debug!(
        "returning in progress equeued uploads: {:?}",
        in_progress_uploads
    );
    Ok(in_progress_uploads)
}

#[tauri::command]
pub async fn enqueue_upload(
    window: Window,
    app_handle: AppHandle,
    app_state: State<'_, UploadsState>,
    server_state: State<'_, SelectedServerState>,
    file_path: String,
) -> Result<Vec<UploadRequest>, String> {
    let server = server_state.current_server();
    let request = UploadRequest::from_file_path(file_path, server)?;
    let upload_id = request.upload.id.clone().to_string();
    let in_progress_uploads = app_state.add_request(request.clone());
    let app_handle_clone = app_handle.clone();
    let client = server_state.clone().client();

    show_notification(
        app_handle.clone(),
        t!("notifications.upload.enqueued.title", name = request.upload.name).to_string(),
        t!("notifications.upload.enqueued.body", name = request.upload.name).to_string()
    );

    tauri::async_runtime::spawn(async move {
        if let Err(error) = client.create(&request.clone().upload).await {
            warn!("error registering file {:?}", error);
            handle_upload_failed(window.clone(), app_handle, error.into(), upload_id);
            return;
        }

        match upload_file(window.clone(), app_handle, client, request.clone()).await {
            Ok(_) => handle_upload_finish(window, app_handle_clone, upload_id),
            Err(error) => handle_upload_failed(window, app_handle_clone, error, upload_id),
        }
    });

    Ok(in_progress_uploads)
}

#[tauri::command]
pub async fn enqueue_many_uploads(
    window: Window,
    app_handle: AppHandle,
    app_state: State<'_, UploadsState>,
    server_state: State<'_, SelectedServerState>,
    paths: Vec<String>,
) -> Result<Vec<UploadRequest>, String> {
    debug!("enqueue many files to uploads: {}", paths.join(";"));
    let mut in_progress_uploads: Vec<UploadRequest> = vec![];

    for path in paths {
        in_progress_uploads = enqueue_upload(
            window.clone(),
            app_handle.clone(),
            app_state.clone(),
            server_state.clone(),
            path,
        )
        .await?;
    }

    Ok(in_progress_uploads)
}

#[tauri::command]
pub async fn retry_upload(
    window: Window,
    app_handle: AppHandle,
    app_state: State<'_, UploadsState>,
    server_state: State<'_, SelectedServerState>,
    upload_id: String,
) -> Result<(), String> {
    debug!("retrying upload with id {}", upload_id);

    let client = server_state.client();
    let upload_request = app_state.get_request(upload_id.clone());
    if let None = upload_request {
        warn!("can't retry upload request with id {}: not found", upload_id);
        return Ok(());
    }

    let upload_request = upload_request.unwrap();

    tauri::async_runtime::spawn(async move {
        let info = client.info(upload_request.upload.id).await;
        if let Err(error) = info.clone() {
            if error.code() != FILE_INFO_ERROR_NOT_EXISTS {
                debug!("error retrieving file info during upload retry: {}", error);
                return;
            }

            debug!("upload with id {} is not registered, registering again", upload_request.upload.id);
            if let Err(error) = client.create(&upload_request.clone().upload).await {
                warn!("error registering file {:?}", error);
                handle_upload_failed(window.clone(), app_handle, error.into(), upload_id);
                return;
            }
        }

        if let Ok(info) = info && info.status == FileStatus::Uploaded {
            debug!("upload with id {} is already uploaded, aborting retry", upload_request.upload.id);
            return;
        }

        match upload_file(window.clone(), app_handle.clone(), client, upload_request.clone()).await {
            Ok(_) => handle_upload_finish(window, app_handle, upload_id),
            Err(error) => handle_upload_failed(window, app_handle, error, upload_id),
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
pub fn get_all_in_progress_uploads(
    uploads_state: State<'_, UploadsState>,
) -> Vec<UploadRequest> {
    debug!("get uploads");
    uploads_state.get_all_in_progress()
}

async fn upload_file(
    window: Window,
    app_handle: AppHandle,
    client: Client,
    request: UploadRequest,
) -> Result<(), FileUploadError> {
    let state = app_handle.state::<UploadsState>();
    let upload_id = request.upload.id.clone().to_string();
    debug!("launching file upload for upload with id {}", upload_id);
    let task = client.upload(&request).await?;

    state.add_cancellation_token(upload_id.clone(), task.cancellation_token.clone());

    let mut progress_receiver = task.progress_receiver.clone();
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
            emit_uploads_changed(&window, state.get_all_in_progress())
        }
    });

    let result = handle.await.map_err(|err| {
        FileUploadError::ClientError { message: format!("upload task join error: {}", err.to_string()) }
    })?;
    state.remove_cancellation_token(upload_id.clone());

    debug!("upload task finished for upload with id {}", upload_id);
    result
}

fn handle_upload_finish(window: Window, app_handle: AppHandle, upload_id: String) {
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
    emit_uploads_changed(&window, state.get_all_in_progress());
    show_notification(
        app_handle,
        t!("notifications.upload.success.title", name = request.upload.name).to_string(),
        t!("notifications.upload.success.body", name = request.upload.name).to_string()
    );
}

fn handle_upload_failed(
    window: Window,
    app_handle: AppHandle,
    error: FileUploadError,
    upload_id: String,
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
    emit_uploads_changed(&window, state.get_all_in_progress());
    show_notification(
        app_handle,
        t!("notifications.upload.error.title", name = request.upload.name).to_string(),
        t!("notifications.upload.error.body", name = request.upload.name).to_string()
    );
}

fn emit_uploads_changed(window: &Window, requests: Vec<UploadRequest>) {
    match window.emit(UPLOADS_CHANGED_EVENT, requests) {
        Ok(_) => debug!("event {} emited", UPLOADS_CHANGED_EVENT),
        Err(error) => warn!("event {} failed emited: {}", UPLOADS_CHANGED_EVENT, error),
    }
}

fn show_notification(app_handle: AppHandle, title: String, body: String) {
    debug!("showing notification: {} - {}", title, body);

    let mut is_visible = false;
    let window = app_handle.get_webview_window(MAIN_WINDOW);

    if let Some(window) = window {
        is_visible = window.is_visible().unwrap_or(false);
    }

    if is_visible {
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
}