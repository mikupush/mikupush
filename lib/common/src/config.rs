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

use std::fmt::{Display, Formatter};


use crate::Theme;

pub const CONFIG_TRUE_VALUE: &str = "true";
pub const CONFIG_FALSE_VALUE: &str = "false";
pub const CONFIG_CHUNK_SIZE_DEFAULT: u64 = 50 * 1024 * 1024; // 50 MB

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigKey {
    Theme,
    StartOnSystemStartup,
    StartMinimized,
    UploadInChunks,
    UploadChunkSize,
}

impl ConfigKey {
    pub fn key(&self) -> String {
        match self {
            ConfigKey::Theme => "theme".to_string(),
            ConfigKey::StartOnSystemStartup => "start_on_system_startup".to_string(),
            ConfigKey::StartMinimized => "start_minimized".to_string(),
            ConfigKey::UploadInChunks => "upload_in_chunks".to_string(),
            ConfigKey::UploadChunkSize => "upload_chunk_size".to_string(),
        }
    }

    pub fn default_value(&self) -> ConfigValue {
        match self {
            ConfigKey::Theme => Theme::default().to_string(),
            ConfigKey::StartOnSystemStartup => CONFIG_FALSE_VALUE.to_string(),
            ConfigKey::StartMinimized => CONFIG_FALSE_VALUE.to_string(),
            ConfigKey::UploadInChunks => CONFIG_TRUE_VALUE.to_string(),
            ConfigKey::UploadChunkSize => "50".to_string(),
        }
    }

    pub fn from_string(key: String) -> Option<Self> {
        match key.as_str() {
            "theme" => Some(ConfigKey::Theme),
            "start_on_system_startup" => Some(ConfigKey::StartOnSystemStartup),
            "start_minimized" => Some(ConfigKey::StartMinimized),
            "upload_in_chunks" => Some(ConfigKey::UploadInChunks),
            "upload_chunk_size" => Some(ConfigKey::UploadChunkSize),
            _ => None,
        }
    }
}

impl Display for ConfigKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key())
    }
}

pub type ConfigValue = String;
pub type ConfigKeyValue = (ConfigKey, ConfigValue);
pub type ConfigMap = std::collections::HashMap<ConfigKey, ConfigValue>;