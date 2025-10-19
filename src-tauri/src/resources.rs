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

use std::io;
use std::path::PathBuf;
use log::{debug, warn};
use rust_i18n::t;
use tauri::{AppHandle, Manager};
use mikupush_common::encode_image_base64;

pub enum ResourceType {
    ServerIcon
}

impl ResourceType {
    pub fn dir_path(&self, app_handle: &AppHandle) -> Result<PathBuf, String> {
        let app_data_dir = match app_handle.path().app_data_dir() {
            Ok(path) => path,
            Err(err) => {
                return Err(format!("failed to get app data dir: {}", err));
            }
        };

        Ok(match self {
            Self::ServerIcon => app_data_dir.join("server-icon")
        })
    }
}

pub enum Resource {
    MikupushSvg
}

impl Resource {
    pub fn file_name(&self) -> String {
        match self {
            Self::MikupushSvg => "mikupush.svg".to_string()
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        match self {
            Self::MikupushSvg => include_bytes!("../assets/mikupush.svg").to_vec()
        }
    }

    pub fn resource_type(&self) -> ResourceType {
        match self {
            Self::MikupushSvg => ResourceType::ServerIcon
        }
    }
}

pub fn unpack_resource(app_handle: &AppHandle, resource: Resource) -> Result<(), String> {
    let resource_dir = resource.resource_type().dir_path(app_handle)?;

    if !resource_dir.exists() {
        std::fs::create_dir_all(resource_dir.clone())
            .map_err(|err| format!("failed to create resource dir: {}", err))?;
    }

    let file_path = resource_dir.join(resource.file_name());
    std::fs::write(file_path, resource.bytes())
        .map_err(|err| format!("failed to write resource: {}", err))?;
    Ok(())
}

pub fn unpack_resources(app_handle: &AppHandle) -> Result<(), String> {
    unpack_resource(app_handle, Resource::MikupushSvg)?;
    Ok(())
}

#[tauri::command]
pub fn server_icon_url(app_handle: AppHandle, icon: String) -> Result<String, String> {
    debug!("encoding server icon to base64 url: {}", icon);
    let path = ResourceType::ServerIcon.dir_path(&app_handle).map_err(|err| {
        warn!("unable to get server icons directory path: {}", err);
        t!("errors.file_system.server_icon_access").to_string()
    })?;

    let icon_path = path.join(icon);
    if !icon_path.exists() {
        warn!("server icon file not found: {}", icon_path.to_string_lossy());
        return Err(t!("errors.server.server_icon_not_found").to_string());
    }

    let base64 = encode_image_base64(icon_path).map_err(|err| {
        warn!("failed to encode server icon to base64: {}", err);
        return t!("errors.server.server_icon_encoding").to_string();
    })?;

    Ok(base64)
}