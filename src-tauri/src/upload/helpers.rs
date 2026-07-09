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

use crate::events::*;
use crate::state::UploadsState;
use crate::upload::{Upload, UploadRepository, UploadRequest};
use crate::window::is_main_window_visible;
use crate::{AppContext, MAIN_WINDOW};
use log::{debug, warn};
use rust_i18n::t;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_notification::NotificationExt;

#[cfg(target_os = "macos")]
pub const MACOS_APP_GROUP_ID: &str = "group.io.mikupush.client";

pub fn copy_upload_link_to_clipboard(
    upload: &Upload,
    app_handle: &AppHandle,
) -> Result<(), String> {
    let result = app_handle.clipboard().write_text(upload.url.clone());

    if let Err(error) = result {
        warn!(
            "failed to copy link to the clipboard for upload id {}: {}",
            upload.id,
            error.to_string()
        );
        return Err(t!("errors.upload.copy_link").to_string());
    }

    Ok(())
}

pub fn emit_uploads_changed_event(app_handle: &AppHandle, requests: Vec<UploadRequest>) {
    if let Some(window) = app_handle.get_webview_window(MAIN_WINDOW) {
        match window.emit(UPLOADS_CHANGED_EVENT, requests) {
            Ok(_) => debug!("event {} emited", UPLOADS_CHANGED_EVENT),
            Err(error) => warn!("event {} failed emited: {}", UPLOADS_CHANGED_EVENT, error),
        }
    }
}

pub fn persist_completed_upload(app_handle: &AppHandle, request: &UploadRequest) {
    let app_context = app_handle.state::<AppContext>();
    let Some(connection_pool) = app_context.db_connection.get().cloned() else {
        warn!(
            "failed to persist completed upload {}: database connection is not initialized",
            request.upload.id
        );
        return;
    };

    let repository = UploadRepository::new(connection_pool);
    if let Err(err) = repository.save(request.upload.clone()) {
        warn!(
            "failed to persist completed upload {}: {}",
            request.upload.id, err
        );
    }
}

pub fn show_upload_notification(app_handle: &AppHandle, title: String, body: String, always: bool) {
    let app_handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        debug!("showing notification: {} - {}", title, body);

        if !always && is_main_window_visible(&app_handle) {
            debug!("skipping notification because window is visible");
            return;
        }

        let result = app_handle
            .notification()
            .builder()
            .title(title)
            .body(body)
            .show();

        if let Err(error) = result {
            warn!("failed to show notification: {}", error.to_string());
        }
    });
}

pub fn update_upload_request_and_emit(app_handle: &AppHandle, upload_request: UploadRequest) {
    let state = app_handle.state::<UploadsState>();
    state.update_request(upload_request.clone());
    emit_uploads_changed_event(app_handle, state.get_all_in_progress());
}
