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