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
    copy_upload_link_to_clipboard, emit_uploads_changed_event, persist_completed_upload,
    show_upload_notification, update_upload_request_and_emit,
};
use super::queue::UploadQueueJob;
use crate::client::{Client, ClientError, FILE_INFO_ERROR_NOT_EXISTS, FileStatus, FileUploadError};
use crate::state::UploadsState;
use crate::upload::status::Status;
use crate::upload::{Progress, UploadRequest};
use crate::window::is_main_window_visible;
use log::{debug, info, warn};
use rust_i18n::t;
use std::sync::{LazyLock, OnceLock};
use tauri::{AppHandle, Manager};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

static UPLOAD_PROGRESS_SENDER: LazyLock<tokio::sync::watch::Sender<Progress>> =
    LazyLock::new(|| {
        let (sender, _) = tokio::sync::watch::channel(Progress::new(Uuid::nil(), 0));
        sender
    });
static UPLOAD_PROGRESS_LISTENER_STARTED: OnceLock<()> = OnceLock::new();

async fn perform_upload_transfer(
    app_handle: &AppHandle,
    client: Client,
    request: UploadRequest,
) -> Result<(), FileUploadError> {
    let state = app_handle.state::<UploadsState>();
    let upload_id = request.upload.id.clone().to_string();
    debug!("launching file upload for upload with id {}", upload_id);
    let cancellation_token = CancellationToken::new();
    let sender = shared_progress_sender();

    state.add_cancellation_token(upload_id.clone(), cancellation_token.clone());

    let result = client.upload(&request, cancellation_token, sender).await;
    state.remove_cancellation_token(upload_id.clone());

    debug!(
        "upload task finished for upload with id {}; success: {}",
        upload_id,
        result.is_ok()
    );
    result
}

pub async fn process_queued_upload(app_handle: &AppHandle, item: UploadQueueJob) {
    let upload_id = item.request.upload.id.to_string();
    let state = app_handle.state::<UploadsState>();
    let Some(mut request) = state.get_request(upload_id.clone()) else {
        debug!(
            "skipping queued upload {} because it is not in state",
            upload_id
        );
        return;
    };

    request.upload.status = Status::InProgress;
    request.finished = false;
    request.canceled = false;
    request.error = None;
    state.update_request(request.clone());
    emit_uploads_changed_event(app_handle, state.get_all_in_progress());

    let client = Client::new(item.server);
    if item.retry {
        let info = client.info(request.upload.id).await;
        if let Err(error) = info.clone() {
            if error.code() != FILE_INFO_ERROR_NOT_EXISTS {
                debug!("error retrieving file info during upload retry: {}", error);
                request.upload.status = Status::Failed;
                update_upload_request_and_emit(
                    app_handle,
                    request.finish_with_error(error.code(), error.to_string()),
                );
                return;
            }

            debug!(
                "upload with id {} is not registered, registering again",
                request.upload.id
            );
            if let Err(error) = client.create(&request.clone().upload).await {
                warn!("error registering file {:?}", error);
                fail_upload(app_handle, error, upload_id, item.always_notify);
                return;
            }
        }

        if let Ok(info) = info
            && info.status == FileStatus::Uploaded
        {
            debug!(
                "upload with id {} is already uploaded, marking retry as finished",
                request.upload.id
            );
            complete_upload(app_handle, upload_id, item.always_notify);
            return;
        }
    } else if let Err(error) = client.create(&request.clone().upload).await {
        warn!("error registering file {:?}", error);
        fail_upload(app_handle, error.into(), upload_id, item.always_notify);
        return;
    }

    match perform_upload_transfer(app_handle, client, request.clone()).await {
        Ok(_) => complete_upload(app_handle, upload_id, item.always_notify),
        Err(error) => fail_upload(app_handle, error, upload_id, item.always_notify),
    }
}

fn shared_progress_sender() -> tokio::sync::watch::Sender<Progress> {
    UPLOAD_PROGRESS_SENDER.clone()
}

pub fn start_upload_progress_sync(app_handle: AppHandle) {
    if UPLOAD_PROGRESS_LISTENER_STARTED.set(()).is_err() {
        debug!("upload progress listener already started");
        return;
    }

    let mut receiver = UPLOAD_PROGRESS_SENDER.subscribe();
    tauri::async_runtime::spawn(async move {
        while receiver.changed().await.is_ok() {
            let progress = *receiver.borrow();
            if progress.upload_id.is_nil() {
                continue;
            }

            let state = app_handle.state::<UploadsState>();
            let upload_id = progress.upload_id.to_string();
            let Some(request) = state.get_request(upload_id.clone()) else {
                warn!(
                    "upload request with id {} not found during progress listen",
                    upload_id
                );
                continue;
            };

            let request = request.update_progress(progress);
            state.update_request(request.clone());
            emit_uploads_changed_event(&app_handle, state.get_all_in_progress());
        }
    });
}

fn complete_upload(app_handle: &AppHandle, upload_id: String, always_notify: bool) {
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
    request.upload.status = Status::Completed;
    request = request.finish();
    state.update_request(request.clone());
    persist_completed_upload(app_handle, &request);

    emit_uploads_changed_event(app_handle, state.get_all_in_progress());

    show_upload_notification(
        app_handle,
        t!(
            "notifications.upload.success.title",
            name = request.upload.name
        )
        .to_string(),
        t!(
            "notifications.upload.success.body",
            name = request.upload.name
        )
        .to_string(),
        always_notify,
    );

    if !is_main_window_visible(app_handle) {
        if let Err(err) = copy_upload_link_to_clipboard(&request.upload, app_handle) {
            warn!("failed to copy upload link: {}", err);
        }
    }

    #[cfg(target_os = "macos")]
    {
        let path = request.upload.path;
        if path.contains(MACOS_APP_GROUP_ID) {
            if let Err(err) = std::fs::remove_file(&path) {
                warn!("failed to remove file ({}): {}", path, err);
            }
        }
    }
}

fn fail_upload(
    app_handle: &AppHandle,
    error: FileUploadError,
    upload_id: String,
    always_notify: bool,
) {
    warn!("upload with id {} failed: {}", upload_id, error);
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
        request.upload.status = Status::Aborted;
        request = request.canceled();
    } else {
        request.upload.status = Status::Failed;
        request = request.finish_with_error(error.code(), error.to_string());
    }

    state.update_request(request.clone());
    emit_uploads_changed_event(app_handle, state.get_all_in_progress());

    show_upload_notification(
        app_handle,
        t!(
            "notifications.upload.error.title",
            name = request.upload.name
        )
        .to_string(),
        t!(
            "notifications.upload.error.body",
            name = request.upload.name
        )
        .to_string(),
        always_notify,
    );
}
