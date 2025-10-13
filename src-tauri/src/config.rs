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

use std::fmt::{Display, Formatter};
use log::{debug, warn};
use tauri::{State};
use mikupush_common::{ConfigKey, ConfigValue};
use mikupush_database::{ConfigRepository, DbPool};
use crate::AppContext;

#[derive(Debug, Clone)]
pub enum ConfigurationError {
    SaveError { message: String },
}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigurationError::SaveError { message } => write!(f, "{}", message),
        }
    }
}

impl std::error::Error for ConfigurationError {}

#[derive(Debug, Clone)]
pub struct Configuration {
    config_repository: ConfigRepository,
}

impl Configuration {
    pub fn new(connection_pool: DbPool) -> Self {
        Self { config_repository: ConfigRepository::new(connection_pool) }
    }

    pub fn apply(&self, key: ConfigKey, value: ConfigValue) -> Result<(), ConfigurationError> {
        debug!("saving config: {} = {}", key, value);
        self.config_repository.save((key, value))
            .map_err(|e| ConfigurationError::SaveError { message: e.to_string() })
    }

    pub fn get(&self, key: ConfigKey) -> ConfigValue {
        let configuration = match self.config_repository.find_by_key(key) {
            Ok(value) => value,
            Err(err) => {
                warn!("error on getting configuration key {}: {}", key, err);
                debug!("fallback to default value for configuration key: {}", key);
                Some((key, key.default_value()))
            }
        };

        if configuration.is_none() {
            debug!("configuration key {} not found", key);
            debug!("using default value for configuration key: {}", key);
            key.default_value()
        } else {
            let (_, value) = configuration.unwrap();
            debug!("get configuration key: {} = {}", key, value);
            value
        }
    }
}

#[tauri::command]
pub fn get_config_value(app_context: State<AppContext>, key: String) -> Result<String, String> {
    let connection_pool = app_context.db_connection.get();
    if connection_pool.is_none() {
        warn!("can't create instance of Configuration because database connection pool is not initialized");
        return Err("database connection pool is not initialized".to_string());
    }

    let configuration = Configuration::new(connection_pool.unwrap().clone());
    let key = ConfigKey::from_string(key);
    if key.is_none() {
        return Err("invalid configuration key".to_string());
    }

    Ok(configuration.get(key.unwrap()))
}

#[tauri::command]
pub fn set_config_value(app_context: State<AppContext>, key: String, value: String) -> Result<(), String> {
    let connection_pool = app_context.db_connection.get();
    if connection_pool.is_none() {
        warn!("can't create instance of Configuration because database connection pool is not initialized");
        return Err("database connection pool is not initialized".to_string());
    }

    let configuration = Configuration::new(connection_pool.unwrap().clone());
    let key = ConfigKey::from_string(key);
    if key.is_none() {
        return Err("invalid configuration key".to_string());
    }

    configuration.apply(key.unwrap(), value).map_err(|e| e.to_string())
}
