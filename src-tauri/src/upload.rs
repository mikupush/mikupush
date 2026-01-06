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
use mikupush_common::{Progress, UploadRequest};
use mikupush_client::{Client, ClientError, FileStatus, FileUploadError, FILE_INFO_ERROR_NOT_EXISTS};
use crate::state::{SelectedServerState, UploadsState};
use log::{debug, error, info, warn};
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
    let in_progress_uploads = enqueue_many_uploads(window, app_handle.clone(), files).await?;

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
    file_path: String,
) -> Result<Vec<UploadRequest>, String> {
    start_upload(Some(window), app_handle, file_path)
}

fn start_upload(
    window: Option<Window>,
    app_handle: AppHandle,
    file_path: String
) -> Result<Vec<UploadRequest>, String> {
    debug!("starting upload for path: {}", file_path);
    let server_state = app_handle.state::<SelectedServerState>();
    let upload_state = app_handle.state::<UploadsState>();

    let server = server_state.current_server();
    let request = UploadRequest::from_file_path(file_path, server)?;
    let upload_id = request.upload.id.clone().to_string();
    let in_progress_uploads = upload_state.add_request(request.clone());
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
            Ok(_) => handle_upload_finish(window.clone(), app_handle_clone, upload_id),
            Err(error) => handle_upload_failed(window.clone(), app_handle_clone, error, upload_id),
        }
    });

    Ok(in_progress_uploads)
}

#[tauri::command]
pub async fn enqueue_many_uploads(
    window: Window,
    app_handle: AppHandle,
    paths: Vec<String>,
) -> Result<Vec<UploadRequest>, String> {
    debug!("enqueue many files to uploads: {}", paths.join(";"));
    let mut in_progress_uploads: Vec<UploadRequest> = vec![];

    for path in paths {
        in_progress_uploads = enqueue_upload(
            window.clone(),
            app_handle.clone(),
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
    update_upload_request_state(
        &window,
        app_handle.clone(),
        upload_request.reset_progress()
    );

    tauri::async_runtime::spawn(async move {
        let info = client.info(upload_request.upload.id).await;
        if let Err(error) = info.clone() {
            if error.code() != FILE_INFO_ERROR_NOT_EXISTS {
                debug!("error retrieving file info during upload retry: {}", error);
                update_upload_request_state(
                    &window,
                    app_handle.clone(),
                    upload_request.finish_with_error(error.code(), error.to_string())
                );
                return;
            }

            debug!("upload with id {} is not registered, registering again", upload_request.upload.id);
            if let Err(error) = client.create(&upload_request.clone().upload).await {
                warn!("error registering file {:?}", error);
                handle_upload_failed(Some(window.clone()), app_handle, error, upload_id);
                return;
            }
        }

        if let Ok(info) = info && info.status == FileStatus::Uploaded {
            debug!("upload with id {} is already uploaded, aborting retry", upload_request.upload.id);
            return;
        }

        match upload_file(Some(window.clone()), app_handle.clone(), client, upload_request.clone()).await {
            Ok(_) => handle_upload_finish(Some(window), app_handle, upload_id),
            Err(error) => handle_upload_failed(Some(window), app_handle, error, upload_id),
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
    window: Option<Window>,
    app_handle: AppHandle,
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

            if let Some(window) = window.as_ref() {
                emit_uploads_changed(window, state.get_all_in_progress())
            }
        }
    });

    let result = handle.await.map_err(|err| {
        FileUploadError::ClientError { message: format!("upload task join error: {}", err.to_string()) }
    })?;
    state.remove_cancellation_token(upload_id.clone());

    debug!("upload task finished for upload with id {}", upload_id);
    result
}

fn handle_upload_finish(window: Option<Window>, app_handle: AppHandle, upload_id: String) {
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

    if let Some(window) = window {
        emit_uploads_changed(&window, state.get_all_in_progress());
    }

    show_notification(
        app_handle,
        t!("notifications.upload.success.title", name = request.upload.name).to_string(),
        t!("notifications.upload.success.body", name = request.upload.name).to_string()
    );
}

fn handle_upload_failed(
    window: Option<Window>,
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

    if let Some(window) = window {
        emit_uploads_changed(&window, state.get_all_in_progress());
    }

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

fn update_upload_request_state(
    window: &Window,
    app_handle: AppHandle,
    upload_request: UploadRequest
) {
    let state = app_handle.state::<UploadsState>();
    state.update_request(upload_request.clone());
    emit_uploads_changed(&window, state.get_all_in_progress());
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

        let window = app_handle.get_window(MAIN_WINDOW);
        let result = start_upload(
            window,
            app_handle.clone(),
            path
        );

        if let Err(err) = result {
            warn!("failed handling share deep-link: {}", err)
        }
    }
}