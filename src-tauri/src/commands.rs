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
    let in_progress_uploads = enqueue_many_uploads(window, app_state, files).await?;

    Ok(in_progress_uploads)
}

#[tauri::command]
pub async fn enqueue_upload(
    window: Window,
    app_state: State<'_, UploadsState>,
    file_path: String,
) -> Result<Vec<UploadRequest>, String> {
    let request = UploadRequest::from_file_path(file_path)?;
    let in_progress_uploads = app_state.add_request(request.clone())?;

    tauri::async_runtime::spawn(async move {
        // match upload_file(window.clone(), request.clone()).await {
        //     Ok(_) => handle_upload_finish(window, request),
        //     Err(error) => handle_upload_failed(window, error, request),
        // }
    });

    Ok(in_progress_uploads)
}

#[tauri::command]
pub async fn enqueue_many_uploads(
    window: Window,
    app_state: State<'_, UploadsState>,
    paths: Vec<String>,
) -> Result<Vec<UploadRequest>, String> {
    let mut in_progress_uploads: Vec<UploadRequest> = vec![];

    for path in paths {
        in_progress_uploads = enqueue_upload(window.clone(), app_state.clone(), path).await?;
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

async fn upload_file(
    window: Window,
    app_handle: AppHandle,
    request: UploadRequest,
) -> GenericResult<()> {
    let client = ServerClient::new();
    let task = client.upload(&request).await?;
    let mut progress_receiver = task.progress();

    tokio::spawn(async move {
        // let state = app_handle.state::<UploadsState>();

        while progress_receiver.changed().await.is_ok() {
            let current = *progress_receiver.borrow();
            let _ = window.emit(UPLOAD_PROGRESS_CHANGE_EVENT, current);
        }
    });

    let result = task.wait().await;
    if let Err(err) = result {
        if err != UploadError::Canceled {
            return Err(err.message().into());
        }
    }

    Ok(())
}

// Helper function to emit progress updates
fn emit_progress(window: &Window, request: &UploadRequest) -> Result<(), String> {
    window
        .emit("upload-progress", request)
        .map_err(|e| e.to_string())
}

fn handle_upload_finish(window: Window, request: UploadRequest) {
    info!("upload file success for {}", request.upload.path);
    let _ = window.emit(
        UPLOAD_FINISH_EVENT,
        UploadFinishEvent {
            upload_id: request.upload.id,
        },
    );
}

fn handle_upload_failed(window: Window, error: Box<dyn Error>, request: UploadRequest) {
    warn!("Upload failed: {}", error);
    let _ = window.emit(
        UPLOAD_FAILED_EVENT,
        UploadFailedEvent {
            upload_id: request.upload.id,
            message: error.to_string(),
        },
    );
}
