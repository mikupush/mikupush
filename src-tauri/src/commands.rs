use crate::events::*;
use crate::models::{Upload, UploadRequest};
use crate::server_client::{ServerClient, UploadError};
use crate::state::UploadsState;
use crate::GenericResult;
use log::{debug, info, warn};
use serde::Serialize;
use std::error::Error;
use std::sync::{Arc, Mutex};
use tauri::{App, AppHandle, Emitter, Manager, State, Window};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::DialogExt;
use uuid::Uuid;

#[tauri::command]
pub async fn select_files_to_upload(
    window: Window,
    app_state: State<'_, UploadsState>,
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
    let in_progress_uploads = enqueue_many_uploads(window, app_handle.clone(), app_state, files).await?;

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
    file_path: String,
) -> Result<Vec<UploadRequest>, String> {
    let request = UploadRequest::from_file_path(file_path)?;
    let upload_id = request.upload.id.clone().to_string();
    let in_progress_uploads = app_state.add_request(request.clone());
    let app_handle_clone = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(error) = register_file(request.clone()).await {
            warn!("error registering file {:?}", error);
            return;
        }

        match upload_file(window.clone(), app_handle, request.clone()).await {
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
    paths: Vec<String>,
) -> Result<Vec<UploadRequest>, String> {
    let mut in_progress_uploads: Vec<UploadRequest> = vec![];

    for path in paths {
        in_progress_uploads = enqueue_upload(
            window.clone(),
            app_handle.clone(),
            app_state.clone(),
            path
        ).await?;
    }

    Ok(in_progress_uploads)
}

#[tauri::command]
pub async fn retry_upload(
    window: Window,
    app_state: State<'_, UploadsState>,
    request: UploadRequest,
) -> Result<(), String> {
    // Create a new request with the same ID and file info
    // let mut new_request = UploadRequest::new(
    //     request.id,
    //     request.name,
    //     request.size,
    //     request.mime_type,
    //     request.path,
    // );
    //
    // // Start upload in background
    // let window_clone = window.clone();
    //
    // tauri::async_runtime::spawn(async move {
    //     // Upload the file
    //     match upload_file(window_clone, new_request).await {
    //         Ok(_) => {
    //             // Success is handled by the progress updates
    //         }
    //         Err(e) => {
    //             // Handle error
    //             println!("Upload retry failed: {}", e);
    //         }
    //     }
    // });

    Ok(())
}

#[tauri::command]
pub async fn abort_upload(app_handle: AppHandle, upload_id: String) -> Result<(), String> {
    // Emit an event to abort the upload
    app_handle
        .emit(&format!("abort-upload-{}", upload_id), ())
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_upload(upload_id: String) -> Result<(), String> {
    //let db = app_state.db.lock().unwrap();

    // Find the upload
    /*let upload = db.find_upload_by_id(&upload_id)
    .map_err(|e| e.to_string())?
    .ok_or_else(|| "Upload not found".to_string())?;*/

    // Delete from server if URL exists
    /*if let Some(url) = &upload.url {
        // In a real implementation, you would call the server API to delete the file
        println!("Would delete file from server: {}", url);
    }*/

    // Delete from database
    //db.delete_upload(&upload_id).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn find_all_uploads() -> Result<Vec<Upload>, String> {
    //let db = app_state.db.lock().unwrap();
    //db.find_all_uploads().map_err(|e| e.to_string())
    Ok(vec![])
}

#[tauri::command]
pub async fn copy_upload_link(app_handle: AppHandle, upload_id: String) -> Result<(), String> {
    //let db = app_state.db.lock().unwrap();

    // Find the upload
    /*let upload = db.find_upload_by_id(&upload_id)
    .map_err(|e| e.to_string())?
    .ok_or_else(|| "Upload not found".to_string())?;*/

    // Get the URL
    //let url = upload.url.ok_or_else(|| "Upload has no URL".to_string())?;

    // Copy to clipboard
    /*app_handle.clipboard()
    .write_text(url)
    .map_err(|e| e.to_string())?;*/

    Ok(())
}

async fn register_file(request: UploadRequest) -> GenericResult<()> {
    let client = ServerClient::new();
    client.create(&request.upload).await
}

async fn upload_file(
    window: Window,
    app_handle: AppHandle,
    request: UploadRequest,
) -> Result<(), UploadError> {
    let upload_id = request.upload.id.clone().to_string();
    let client = ServerClient::new();
    let task = client.upload(&request).await?;

    let mut progress_receiver = task.progress();
    let app_handle_clone = app_handle.clone();
    let upload_id_clone = upload_id.clone();
    tokio::spawn(async move {
        let state = app_handle_clone.state::<UploadsState>();
        let request = state.get_request(upload_id_clone.clone());
        if let None = request {
            warn!("upload request with id {} not found during progress listen", upload_id_clone);
            return;
        }

        let mut request = request.unwrap();
        while progress_receiver.changed().await.is_ok() {
            let current = *progress_receiver.borrow();
            request = request.update_progress(current.clone());
            state.update_request(request.clone());
            let _ = window.emit(UPLOAD_PROGRESS_CHANGE_EVENT, current);
        }
    });

    task.wait().await
}

// Helper function to emit progress updates
fn emit_progress(window: &Window, request: &UploadRequest) -> Result<(), String> {
    window
        .emit("upload-progress", request)
        .map_err(|e| e.to_string())
}

fn handle_upload_finish(window: Window, app_handle: AppHandle, upload_id: String) {
    info!("upload with id {} finished", upload_id);
    let state = app_handle.state::<UploadsState>();
    let request = state.get_request(upload_id.clone());
    if let None = request {
        warn!("upload request with id {} not found during finish handle", upload_id);
        return;
    }

    let mut request = request.unwrap();
    request = request.finish();
    state.update_request(request.clone());

    let _ = window.emit(UPLOAD_FINISH_EVENT, request);
    debug!("event {} emited for upload with id {}", UPLOAD_FINISH_EVENT, upload_id);
}

fn handle_upload_failed(window: Window, app_handle: AppHandle, error: UploadError, upload_id: String) {
    info!("upload with id {} failed {}", upload_id, error);
    let state = app_handle.state::<UploadsState>();
    let request = state.get_request(upload_id.clone());
    if let None = request {
        warn!("upload request with id {} not found during failed handle", upload_id);
        return;
    }

    let mut request = request.unwrap();
    if error == UploadError::Canceled {
        request = request.canceled();
    } else {
        request = request.finish_with_error(error.to_string());
    }

    state.update_request(request.clone());
    let _ = window.emit(UPLOAD_FAILED_EVENT, request);
    debug!("event {} emited for upload with id {}", UPLOAD_FAILED_EVENT, upload_id);
}
