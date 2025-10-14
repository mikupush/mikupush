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

use std::fmt::{Display, Formatter};


use crate::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigKey {
    Theme,
    StartOnSystemStartup,
    StartMinimized,
}

impl ConfigKey {
    pub fn key(&self) -> String {
        match self {
            ConfigKey::Theme => "theme".to_string(),
            ConfigKey::StartOnSystemStartup => "start_on_system_startup".to_string(),
            ConfigKey::StartMinimized => "start_minimized".to_string(),
        }
    }

    pub fn default_value(&self) -> ConfigValue {
        match self {
            ConfigKey::Theme => Theme::default().to_string(),
            ConfigKey::StartOnSystemStartup => false.to_string(),
            ConfigKey::StartMinimized => false.to_string(),
        }
    }

    pub fn from_string(key: String) -> Option<Self> {
        match key.as_str() {
            "theme" => Some(ConfigKey::Theme),
            "start_on_system_startup" => Some(ConfigKey::StartOnSystemStartup),
            "start_minimized" => Some(ConfigKey::StartMinimized),
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