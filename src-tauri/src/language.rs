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

use crate::AppContext;
use crate::config::{ConfigKey, ConfigRepository};
use log::{debug, warn};
use tauri::{AppHandle, Manager, State};

const FALLBACK_LANGUAGE: &str = "en";
const AVAILABLE_LANGUAGES: [&str; 2] = ["en", "es"];

#[tauri::command]
pub fn get_language(app_context: State<AppContext>) -> Result<String, String> {
    let config_repository = config_repository_from_state(&app_context)?;

    resolve_current_language(&config_repository)
}

#[tauri::command]
pub fn set_current_language(
    app_context: State<AppContext>,
    language: String,
) -> Result<(), String> {
    let language =
        normalize_available_language(&language).ok_or("selected language is not available")?;
    let config_repository = config_repository_from_state(&app_context)?;

    config_repository
        .save((ConfigKey::Language, language.clone()))
        .map_err(|err| err.to_string())?;
    rust_i18n::set_locale(&language);
    debug!("current language set to: {}", language);

    Ok(())
}

pub fn configure_current_language(app_handle: &AppHandle) -> Result<String, String> {
    let app_context = app_handle.state::<AppContext>();
    let config_repository = config_repository_from_state(&app_context)?;
    let language = resolve_current_language(&config_repository)?;

    rust_i18n::set_locale(&language);
    debug!("current rust-i18n language configured: {}", language);

    Ok(language)
}

fn config_repository_from_state(
    app_context: &State<AppContext>,
) -> Result<ConfigRepository, String> {
    let connection_pool = app_context.db_connection.get();
    if connection_pool.is_none() {
        warn!("can't get language because database connection pool is not initialized");
        return Err("database connection pool is not initialized".to_string());
    }

    Ok(ConfigRepository::new(connection_pool.unwrap().clone()))
}

fn resolve_current_language(config_repository: &ConfigRepository) -> Result<String, String> {
    let configured_language = config_repository
        .find_by_key(ConfigKey::Language)
        .map_err(|err| err.to_string())?;

    if let Some((_, language)) = configured_language {
        if let Some(language) = normalize_available_language(&language) {
            debug!("using configured language: {}", language);
            return Ok(language);
        }

        warn!("configured language is not available: {}", language);
    }

    Ok(system_language().unwrap_or_else(|| FALLBACK_LANGUAGE.to_string()))
}

fn system_language() -> Option<String> {
    tauri_plugin_os::locale().and_then(|locale| {
        let language = normalize_available_language(&locale);
        if language.is_none() {
            debug!("system language is not available: {}", locale);
        }
        language
    })
}

fn normalize_available_language(language: &str) -> Option<String> {
    let language = language
        .split(['-', '_'])
        .next()
        .unwrap_or(language)
        .to_lowercase();

    if AVAILABLE_LANGUAGES.contains(&language.as_str()) {
        Some(language)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::language::normalize_available_language;

    #[test]
    fn normalize_available_language_should_accept_plain_supported_language() {
        assert_eq!(Some("en".to_string()), normalize_available_language("en"));
        assert_eq!(Some("es".to_string()), normalize_available_language("es"));
    }

    #[test]
    fn normalize_available_language_should_accept_regional_supported_language() {
        assert_eq!(
            Some("en".to_string()),
            normalize_available_language("en-US")
        );
        assert_eq!(
            Some("es".to_string()),
            normalize_available_language("es_ES")
        );
    }

    #[test]
    fn normalize_available_language_should_reject_unsupported_language() {
        assert_eq!(None, normalize_available_language("fr-FR"));
        assert_eq!(None, normalize_available_language("enen"));
    }
}
