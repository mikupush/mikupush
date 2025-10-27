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
use tauri::utils::platform::resource_dir;
use mikupush_common::encode_image_base64;

pub enum ResourceType {
    ServerIcon,
    Document
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
            Self::ServerIcon => app_data_dir.join("server-icon"),
            Self::Document => app_data_dir
        })
    }
}

pub enum Resource {
    MikupushSvg,
    ThirdPartyLicenses
}

impl Resource {
    pub fn file_name(&self) -> String {
        match self {
            Self::MikupushSvg => "mikupush.svg".to_string(),
            Self::ThirdPartyLicenses => "THIRD_PARTY_LICENSES.html".to_string()
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        match self {
            Self::MikupushSvg => include_bytes!("../assets/mikupush.svg").to_vec(),
            Self::ThirdPartyLicenses => include_bytes!("../assets/THIRD_PARTY_LICENSES.html").to_vec()
        }
    }

    pub fn resource_type(&self) -> ResourceType {
        match self {
            Self::MikupushSvg => ResourceType::ServerIcon,
            Self::ThirdPartyLicenses => ResourceType::Document
        }
    }

    pub fn path(&self, app_handle: &AppHandle) -> Result<PathBuf, String> {
        let dir = self.resource_type().dir_path(app_handle)?;
        Ok(dir.join(self.file_name()))
    }

    pub fn from_string(string: String) -> Option<Self> {
        match string.to_lowercase().as_str() {
            "mikupush_svg" => Some(Self::MikupushSvg),
            "third_party_licenses" => Some(Self::ThirdPartyLicenses),
            _ => None
        }
    }
}

pub fn unpack_resource(app_handle: &AppHandle, resource: Resource) -> Result<(), String> {
    let resource_dir = resource.resource_type().dir_path(app_handle)?;

    if !resource_dir.exists() {
        std::fs::create_dir_all(resource_dir.clone())
            .map_err(|err| format!("failed to create resource dir: {}", err))?;
    }

    let file_path = resource.path(app_handle)?;
    if file_path.exists() {
        debug!("resource file already unpacked: {}", file_path.display());
        return Ok(());
    }

    std::fs::write(file_path, resource.bytes())
        .map_err(|err| format!("failed to write resource: {}", err))?;
    Ok(())
}

pub fn unpack_resources(app_handle: &AppHandle) -> Result<(), String> {
    unpack_resource(app_handle, Resource::MikupushSvg)?;
    unpack_resource(app_handle, Resource::ThirdPartyLicenses)?;
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

#[tauri::command]
pub fn resource_path(app_handle: AppHandle, resource: String) -> Result<String, String> {
    match Resource::from_string(resource.clone()) {
        Some(resource) => {
            let resource_dir = resource.path(&app_handle).map_err(|err| {
                warn!("unable to get resource directory path: {}", err);
                t!("errors.resource.path_not_resolved").to_string()
            })?;

            Ok(resource_dir.to_string_lossy().to_string())
        },
        None => {
            warn!("invalid resource: {}", resource);
            Err(t!("errors.resource.not_found").to_string())
        }
    }
}