use crate::models::{Upload, UploadRequest, UploadStatus};
use log::debug;
use reqwest::multipart::{Form, Part};
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Listener, Manager, State, Window};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tokio::sync::mpsc;
use uuid::Uuid;
use crate::{AppState, GenericResult};

#[tauri::command]
pub async fn resolve_file_path(path: String) -> Result<String, String> {
    // In Tauri, we don't need to resolve web file paths like in Electron
    // We can use the path directly
    Ok(path)
}

#[tauri::command]
pub async fn enqueue_upload(
    window: Window,
    app_state: State<'_, AppState>,
    file_path: String,
) -> Result<(), String> {
    let mut requests = app_state.uploads.lock()
        .map_err(|e| format!("failed to get current uploads list: {}", e.to_string()))?;
    let request = UploadRequest::from_file_path(file_path)?;

    // Start upload in background
    let request_clone = request.clone();
    let window_clone = window.clone();
    requests.push(Mutex::new(request));

    tauri::async_runtime::spawn(async move {
        // Upload the file
        match upload_file(window_clone, request_clone).await {
            Ok(_) => {
                // Success is handled by the progress updates
            }
            Err(e) => {
                // Handle error
                println!("Upload failed: {}", e);
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn enqueue_many_uploads(
    window: Window,
    app_state: State<'_, AppState>,
    paths: Vec<String>,
) -> Result<(), String> {
    for path in paths {
        enqueue_upload(window.clone(), app_state.clone(), path).await?;
    }

    Ok(())
}

#[tauri::command]
pub async fn retry_upload(
    window: Window,
    app_state: State<'_, AppState>,
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
pub async fn delete_upload(
    app_state: State<'_, AppState>,
    upload_id: String,
) -> Result<(), String> {
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
pub async fn find_all_uploads(app_state: State<'_, AppState>) -> Result<Vec<Upload>, String> {
    //let db = app_state.db.lock().unwrap();
    //db.find_all_uploads().map_err(|e| e.to_string())
    Ok(vec![])
}

#[tauri::command]
pub async fn copy_upload_link(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
    upload_id: String,
) -> Result<(), String> {
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

// Helper function to upload a file
async fn upload_file(window: Window, mut request: UploadRequest) -> Result<(), String> {
    // debug!("Uploading file");
    // // Create a channel to handle abort signals
    // let (abort_tx, mut abort_rx) = mpsc::channel::<()>(1);
    //
    // // Listen for abort events
    // let abort_id = request.id.clone();
    // let abort_handler = window.listen(&format!("abort-upload-{}", abort_id), move |_| {
    //     let _ = abort_tx.try_send(());
    // });
    //
    // // Get server URL based on environment
    // let server_url = if cfg!(debug_assertions) {
    //     "http://localhost:8080/upload"
    // } else {
    //     "https://mikupush.io/upload"
    // };
    //
    // // Open the file
    // let file = match File::open(&request.path) {
    //     Ok(file) => file,
    //     Err(e) => {
    //         request.set_failed(format!("Failed to open file: {}", e));
    //         emit_progress(&window, &request)?;
    //         return Err(e.to_string());
    //     }
    // };

    // Create a client
    // let client = reqwest::Client::new();

    // // Start the upload
    // let mut response = match client.post(server_url)
    //     .multipart(form)
    //     .send()
    //     .await {
    //         Ok(response) => response,
    //         Err(e) => {
    //             request.set_failed(format!("Failed to start upload: {}", e));
    //             emit_progress(&window, &request)?;
    //             return Err(e.to_string());
    //         }
    //     };
    //
    // // Process the response
    // if response.status().is_success() {
    //     // Get the URL from the response
    //     let response_json: serde_json::Value = match response.json().await {
    //         Ok(json) => json,
    //         Err(e) => {
    //             request.set_failed(format!("Failed to parse response: {}", e));
    //             emit_progress(&window, &request)?;
    //             return Err(e.to_string());
    //         }
    //     };
    //
    //     // Extract the URL
    //     let url = match response_json.get("url") {
    //         Some(url) => match url.as_str() {
    //             Some(url_str) => url_str.to_string(),
    //             None => {
    //                 request.set_failed("Invalid URL in response".to_string());
    //                 emit_progress(&window, &request)?;
    //                 return Err("Invalid URL in response".to_string());
    //             }
    //         },
    //         None => {
    //             request.set_failed("No URL in response".to_string());
    //             emit_progress(&window, &request)?;
    //             return Err("No URL in response".to_string());
    //         }
    //     };
    //
    //     // Mark as completed
    //     request.set_completed(url);
    //     emit_progress(&window, &request)?;
    //
    //     // Save to database
    //     if let Some(upload) = &request.upload {
    //         let app_handle = window.app_handle();
    //         let app_state = app_handle.state::<AppState>();
    //         let db = app_state.db.lock().unwrap();
    //         if let Err(e) = db.save_upload(upload) {
    //             println!("Failed to save upload to database: {}", e);
    //         }
    //     }
    //
    //     Ok(())
    // } else {
    //     // Handle error response
    //     let error_text = response.text().await
    //         .unwrap_or_else(|e| format!("Failed to read error response: {}", e));
    //
    //     request.set_failed(format!("Upload failed: {}", error_text));
    //     emit_progress(&window, &request)?;
    //
    //     Err(error_text)
    // }
    Ok(())
}

// Helper function to emit progress updates
fn emit_progress(window: &Window, request: &UploadRequest) -> Result<(), String> {
    window
        .emit("upload-progress", request)
        .map_err(|e| e.to_string())
}
