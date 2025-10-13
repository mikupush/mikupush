/// Copyright 2025 Miku Push! Team
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///     http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.

use std::io;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub enum ResourceType {
    ServerIcon
}

impl ResourceType {
    pub fn dir_path(&self, app_handle: &AppHandle) -> PathBuf {
        let app_data_dir = app_handle.path()
            .app_data_dir()
            .unwrap();

        match self {
            Self::ServerIcon => app_data_dir.join("server-icon")
        }
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

pub fn unpack_resource(app_handle: &AppHandle, resource: Resource) -> io::Result<()> {
    let resource_dir = resource.resource_type().dir_path(app_handle);

    if !resource_dir.exists() {
        std::fs::create_dir_all(resource_dir.clone())?
    }

    let file_path = resource_dir.join(resource.file_name());
    std::fs::write(file_path, resource.bytes())
}

pub fn unpack_resources(app_handle: &AppHandle) -> io::Result<()> {
    unpack_resource(app_handle, Resource::MikupushSvg)?;
    Ok(())
}