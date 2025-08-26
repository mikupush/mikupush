use crate::events::*;
use mikupush_common::{Progress, Upload, UploadRequest};
use mikupush_client::{Client, UploadError};
use crate::state::{SelectedServerState, UploadsState};
use log::{debug, info, warn};
use tauri::{AppHandle, Emitter, Manager, State, Window};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::DialogExt;
use uuid::Uuid;

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
    let request = UploadRequest::from_file_path(file_path)?;
    let upload_id = request.upload.id.clone().to_string();
    let in_progress_uploads = app_state.add_request(request.clone());
    let app_handle_clone = app_handle.clone();
    let client = server_state.clone().client();

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
    debug!("Equeue many files to uploads: {}", paths.join(";"));
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
pub async fn delete_upload(
    server_state: State<'_, SelectedServerState>,
    uploads_state: State<'_, UploadsState>,
    upload_id: String,
) -> Result<Vec<UploadRequest>, String> {
    debug!("Deleting upload with id {}", upload_id.clone());

    let id = Uuid::parse_str(upload_id.as_str()).map_err(|err| err.to_string())?;
    let client = server_state.client();
    client.delete(id).await.map_err(|err| err.to_string())?;
    let uploads = uploads_state.delete_request(upload_id.clone());

    debug!("deleted upload with id {}", upload_id.clone());
    Ok(uploads)
}

#[tauri::command]
pub async fn find_all_uploads() -> Result<Vec<Upload>, String> {
    //let db = app_state.db.lock().unwrap();
    //db.find_all_uploads().map_err(|e| e.to_string())
    Ok(vec![])
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
    state: State<'_, SelectedServerState>,
    upload_id: String,
) -> Result<(), String> {
    let current_server = state.server.lock().unwrap().clone();
    let link = format!("{}/u/{}", current_server.base_url, upload_id);

    let result = app_handle.clipboard().write_text(link);
    if let Err(error) = result {
        warn!(
            "failed to copy link to the cipboard for upload id {}: {}",
            upload_id,
            error.to_string()
        );
        return Err(format!("failed copy to clipboard: {}", error.to_string()));
    }

    Ok(())
}

async fn upload_file(
    window: Window,
    app_handle: AppHandle,
    client: Client,
    request: UploadRequest,
) -> Result<(), UploadError> {
    let state = app_handle.state::<UploadsState>();
    let upload_id = request.upload.id.clone().to_string();
    let task = client.upload(&request).await?;

    state.add_cancellation_token(upload_id.clone(), task.cancellation_token.clone());

    let mut progress_receiver = task.progress_receiver.clone();
    let app_handle_clone = app_handle.clone();
    let upload_id_clone = upload_id.clone();
    let handle = task.start();

    tokio::spawn(async move {
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

    let result = handle.await?;
    state.remove_cancellation_token(upload_id.clone());
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
    emit_uploads_changed(&window, state.get_all_in_progress())
}

fn handle_upload_failed(
    window: Window,
    app_handle: AppHandle,
    error: UploadError,
    upload_id: String,
) {
    info!("upload with id {} failed {}", upload_id, error);
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
    if error == UploadError::Canceled {
        request = request.canceled();
    } else {
        request = request.finish_with_error(error.to_string());
    }

    state.update_request(request.clone());
    emit_uploads_changed(&window, state.get_all_in_progress())
}

fn emit_uploads_changed(window: &Window, requests: Vec<UploadRequest>) {
    match window.emit(UPLOADS_CHANGED_EVENT, requests) {
        Ok(_) => debug!("event {} emited", UPLOADS_CHANGED_EVENT),
        Err(error) => warn!("event {} failed emited: {}", UPLOADS_CHANGED_EVENT, error),
    }
}