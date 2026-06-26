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
use crate::client::{Client, HealthCheckStatus};
use crate::encoder::encode_image_base64;
use crate::resources::ResourceType;
use crate::server::{Server, ServerRepository, map_server_icon_into_base64};
use crate::state::SelectedServerState;
use log::{debug, warn};
use rust_i18n::t;
use tauri::{AppHandle, State};
use uuid::Uuid;

type ServerResult<T> = Result<T, String>;

#[tauri::command]
pub fn find_all_servers(
    app_handle: AppHandle,
    app_context: State<AppContext>,
) -> ServerResult<Vec<Server>> {
    let connection_pool = app_context.db_connection.get().cloned().ok_or_else(|| {
        warn!("can't get all servers because database connection pool is not initialized");
        t!("errors.database.internal_error")
    })?;

    let server_repository = ServerRepository::new(connection_pool);
    let servers = server_repository
        .find_all()
        .map_err(|err| err.to_string())?;

    Ok(servers)
}

#[tauri::command]
pub fn server_icon_url(app_handle: AppHandle, icon: String) -> Result<String, String> {
    debug!("encoding server icon to base64 url: {}", icon);
    let path = ResourceType::ServerIcon
        .dir_path(&app_handle)
        .map_err(|err| {
            warn!("unable to get server icons directory path: {}", err);
            t!("errors.file_system.server_icon_access").to_string()
        })?;

    let icon_path = path.join(icon);
    if !icon_path.exists() {
        warn!(
            "server icon file not found: {}",
            icon_path.to_string_lossy()
        );
        return Err(t!("errors.server.server_icon_not_found").to_string());
    }

    let base64 = encode_image_base64(icon_path).map_err(|err| {
        warn!("failed to encode server icon to base64: {}", err);
        return t!("errors.server.server_icon_encoding").to_string();
    })?;

    Ok(base64)
}

#[tauri::command]
pub fn delete_server(app_context: State<'_, AppContext>, id: String) -> ServerResult<()> {
    debug!("deleting server: {:?}", id);
    let connection_pool = app_context.db_connection.get().cloned().ok_or_else(|| {
        warn!("can't delete server because database connection pool is not initialized");
        t!("errors.database.internal_error")
    })?;

    let server_repository = ServerRepository::new(connection_pool);
    let parsed_id = Uuid::parse_str(&id)
        .map_err(|_| t!("errors.server.invalid_server_id"))?;

    server_repository.delete(parsed_id)
        .map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn check_server_health(server: Server) -> ServerResult<()> {
    let health_check_status = Client::new(server.clone())
        .check_health()
        .await
        .map_err(|err| {
            warn!("unable to check server health: {}", err);
            t!("errors.server.health_check").to_string()
        })?;

    if !matches!(health_check_status, HealthCheckStatus::Up) {
        warn!("server {} health check is down", server.id);
        return Err(t!("errors.server.health_check_down").to_string());
    }

    Ok(())
}

#[tauri::command]
pub async fn set_connected_server(
    app_context: State<'_, AppContext>,
    current_server_state: State<'_, SelectedServerState>,
    id: String,
) -> ServerResult<()> {
    let parsed_id = Uuid::parse_str(&id).map_err(|_| t!("errors.server.invalid_server_id"))?;
    let connection_pool = app_context.db_connection.get().cloned().ok_or_else(|| {
        warn!("can't set connected server because database connection pool is not initialized");
        t!("errors.database.internal_error")
    })?;

    let server_repository = ServerRepository::new(connection_pool);
    let server = server_repository.find_by_id(parsed_id).map_err(|err| {
        warn!("unable to find server by id: {}", err);
        t!("errors.database.internal_error")
    })?;
    let mut server = match server {
        Some(server) => server,
        None => {
            debug!("server with id {} not found", parsed_id);
            return Err(t!("errors.server.server_not_found").to_string());
        }
    };

    let health_check_status = Client::new(server.clone())
        .check_health()
        .await
        .map_err(|err| {
            warn!("unable to check server health: {}", err);
            t!("errors.server.health_check")
        })?;

    if !matches!(health_check_status, HealthCheckStatus::Up) {
        warn!("server {} health check is down", server.id);
        return Err(t!("errors.server.health_check_down").to_string());
    }

    server_repository
        .update_connected(server.id)
        .map_err(|err| {
            warn!("unable to update connected server: {}", err);
            t!("errors.server.change_server")
        })?;

    server.healthy = true;
    server_repository.save(server.clone()).map_err(|err| {
        warn!("unable to update connected server health: {}", err);
        t!("errors.server.change_server")
    })?;
    current_server_state.set_server(server.clone());
    debug!("current server set to {} - {}", server.id, server.name);

    Ok(())
}

#[tauri::command]
pub fn get_connected_server(
    current_server_state: State<SelectedServerState>,
) -> ServerResult<Server> {
    debug!("get current connected server");
    let connected_server = current_server_state.server.lock().map_err(|err| {
        let message = err.to_string();
        warn!("can't get connected server: {}", message);
        t!("errors.database.internal_error")
    })?;

    debug!(
        "got current connected server {} - {}",
        connected_server.id, connected_server.name
    );
    Ok(connected_server.clone())
}

#[tauri::command]
pub fn get_server_by_url(
    app_context: State<AppContext>,
    url: String,
) -> ServerResult<Option<Server>> {
    let connection_pool = app_context.db_connection.get().cloned().ok_or_else(|| {
        warn!("can't get server by url because database connection pool is not initialized");
        t!("errors.database.internal_error")
    })?;

    let server_repository = ServerRepository::new(connection_pool);
    let servers = server_repository.find_by_url(url).map_err(|err| {
        warn!("unable to find server by url: {}", err);
        t!("errors.server.get_server")
    })?;
    let server = servers.into_iter().next();

    Ok(server)
}

#[tauri::command]
pub fn get_server_by_id(
    app_context: State<AppContext>,
    id: String,
) -> ServerResult<Option<Server>> {
    let connection_pool = app_context.db_connection.get().cloned().ok_or_else(|| {
        warn!("can't get server by id because database connection pool is not initialized");
        t!("errors.database.internal_error")
    })?;

    let parsed_id = Uuid::parse_str(&id).map_err(|_| t!("errors.server.invalid_server_id"))?;
    let server_repository = ServerRepository::new(connection_pool);
    let server = server_repository.find_by_id(parsed_id).map_err(|err| {
        warn!("unable to find server by id: {}", err);
        t!("errors.server.get_server")
    })?;

    Ok(server)
}

#[tauri::command]
pub fn create_server(app_context: State<AppContext>, new_server: Server) -> ServerResult<Server> {
    debug!("creating new server: {:?}", new_server);
    let connection_pool = app_context.db_connection.get().cloned().ok_or_else(|| {
        warn!("can't create server because database connection pool is not initialized");
        t!("errors.database.internal_error")
    })?;

    let server_repository = ServerRepository::new(connection_pool);

    server_repository.save(new_server.clone()).map_err(|err| {
        warn!("unable to create new server: {}", err);
        t!("errors.server.create_server")
    })?;

    debug!("server with id {} created", new_server.id);
    Ok(new_server)
}

#[tauri::command]
pub fn update_server(app_context: State<AppContext>, server: Server) -> ServerResult<Server> {
    debug!("updating server: {:?}", server);
    let connection_pool = app_context.db_connection.get().cloned().ok_or_else(|| {
        warn!("can't update server because database connection pool is not initialized");
        t!("errors.database.internal_error")
    })?;

    let server_repository = ServerRepository::new(connection_pool);

    server_repository.save(server.clone()).map_err(|err| {
        warn!("unable to update server: {}", err);
        t!("errors.server.update_server")
    })?;

    debug!("server with id {} updated", server.id);
    Ok(server)
}
