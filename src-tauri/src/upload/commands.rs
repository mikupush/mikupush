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

use super::enqueue;
use super::helpers::copy_upload_link_to_clipboard;
use super::queue::remove_upload_job;
use crate::AppContext;
use crate::client::{Client, FileDeleteError, FileInfoError};
use crate::events::ARCHIVED_UPLOADS_CHANGED_EVENT;
use crate::server::ServerRepository;
use crate::state::{SelectedServerState, UploadsState};
use crate::upload::UploadRepository;
use crate::upload::{Upload, UploadRequest};
use log::{debug, warn};
use rust_i18n::t;
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;
use tokio::task::spawn_blocking;
use uuid::Uuid;

#[tauri::command]
pub async fn select_files_to_upload(app_handle: AppHandle) -> Vec<String> {
    app_handle
        .dialog()
        .file()
        .blocking_pick_files()
        .unwrap_or_default()
        .iter()
        .map(|file| file.to_string())
        .collect()
}

#[tauri::command]
pub async fn select_folders_to_upload(app_handle: AppHandle) -> Vec<String> {
    app_handle
        .dialog()
        .file()
        .blocking_pick_folders()
        .unwrap_or_default()
        .iter()
        .map(|file| file.to_string())
        .collect()
}

#[tauri::command]
pub async fn enqueue_uploads(
    app_handle: AppHandle,
    paths: Vec<String>,
    preferences: Option<super::UploadPreferences>,
) -> Result<Vec<UploadRequest>, String> {
    match preferences {
        Some(preferences) => {
            enqueue::enqueue_upload_paths_with_preferences(&app_handle, paths, false, preferences)
        }
        None => enqueue::enqueue_upload_paths(&app_handle, paths, false),
    }
}

#[tauri::command]
pub async fn enqueue_upload(
    app_handle: AppHandle,
    file_path: String,
) -> Result<Vec<UploadRequest>, String> {
    enqueue::enqueue_upload_paths(&app_handle, vec![file_path], false)
}

#[tauri::command]
pub fn take_pending_upload_paths() -> Vec<Vec<String>> {
    enqueue::take_pending_upload_paths()
}

#[tauri::command]
pub async fn retry_upload(
    app_handle: AppHandle,
    uploads_state: State<'_, UploadsState>,
    server_state: State<'_, SelectedServerState>,
    upload_id: String,
) -> Result<(), String> {
    enqueue::enqueue_upload_retry(&app_handle, &uploads_state, &server_state, upload_id)
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
pub async fn delete_archived_upload(
    app_handle: AppHandle,
    app_context: State<'_, AppContext>,
    upload_id: String,
) -> Result<Vec<Upload>, String> {
    debug!("deleting archived upload with id {upload_id}");

    let id = Uuid::parse_str(&upload_id).map_err(|err| err.to_string())?;
    let pool = app_context
        .db_connection
        .get()
        .cloned()
        .ok_or_else(|| "database connection is not initialized".to_string())?;

    let read_pool = pool.clone();
    let server = spawn_blocking(move || {
        let upload = UploadRepository::new(read_pool.clone())
            .find_by_id(id)
            .map_err(|err| err.to_string())?
            .ok_or_else(|| t!("errors.upload.not_found").to_string())?;
        ServerRepository::new(read_pool)
            .find_by_id(upload.server_id)
            .map_err(|err| err.to_string())?
            .ok_or_else(|| format!("server {} not found", upload.server_id))
    })
    .await
    .map_err(|err| err.to_string())??;

    let server_id = server.id;
    let client = Client::new(server);

    match client.info(id).await {
        Ok(_) => match client.delete(id).await {
            Ok(()) | Err(FileDeleteError::NotExists { .. }) => {}
            Err(error) => return Err(error.to_string()),
        },
        Err(FileInfoError::NotExists { .. }) => {
            debug!("archived upload {id} is already absent from server {server_id}");
        }
        Err(error) => return Err(error.to_string()),
    }

    spawn_blocking(move || {
        let repository = UploadRepository::new(pool);
        repository.delete(id)?;
        if let Err(error) = app_handle.emit(ARCHIVED_UPLOADS_CHANGED_EVENT, &server_id) {
            warn!("could not notify the upload history about deletion: {error}");
        }
        debug!("deleted archived upload with id {upload_id}");
        repository.find_by_server_id(server_id.to_string())
    })
    .await
    .map_err(|err| err.to_string())?
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub fn cancel_upload(
    uploads_state: State<'_, UploadsState>,
    upload_id: String,
) -> Vec<UploadRequest> {
    debug!("canceling upload for: {}", upload_id);
    if let Err(error) = remove_upload_job(&upload_id) {
        warn!(
            "failed to remove upload {} from queue: {}",
            upload_id, error
        );
    }
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

    copy_upload_link_to_clipboard(&upload.unwrap().upload, &app_handle)
}

#[tauri::command]
pub async fn copy_archived_upload_link(
    app_handle: AppHandle,
    app_context: State<'_, AppContext>,
    upload_id: String,
) -> Result<(), String> {
    let id = Uuid::parse_str(upload_id.as_str()).map_err(|err| err.to_string())?;
    let connection_pool = app_context
        .db_connection
        .get()
        .cloned()
        .ok_or_else(|| "database connection is not initialized".to_string())?;
    let repository = UploadRepository::new(connection_pool);
    let upload = repository
        .find_by_id(id)
        .map_err(|err| err.to_string())?
        .ok_or_else(|| t!("errors.upload.not_found").to_string())?;

    copy_upload_link_to_clipboard(&upload, &app_handle)
}

#[tauri::command]
pub fn list_active_uploads(uploads_state: State<'_, UploadsState>) -> Vec<UploadRequest> {
    debug!("get uploads");
    uploads_state.get_all_in_progress()
}

#[tauri::command]
pub fn get_archived_uploads(
    app_context: State<'_, AppContext>,
    server_id: String,
) -> Result<Vec<Upload>, String> {
    debug!("get archived uploads for server {}", server_id);
    let connection_pool = app_context
        .db_connection
        .get()
        .cloned()
        .ok_or_else(|| "database connection is not initialized".to_string())?;
    let repository = UploadRepository::new(connection_pool);

    repository
        .find_by_server_id(server_id)
        .map_err(|err| err.to_string())
}
