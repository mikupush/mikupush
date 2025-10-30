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

use crate::state::SelectedServerState;
use crate::AppContext;
use log::{debug, warn};
use mikupush_common::Server;
use mikupush_database::ServerRepository;
use rust_i18n::t;
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;

type ServerResult<T> = Result<T, String>;

#[tauri::command]
pub fn set_connected_server(
    app_context: State<AppContext>,
    current_server_state: State<SelectedServerState>,
    id: String,
) -> ServerResult<()> {
    let parsed_id = Uuid::parse_str(&id)
        .map_err(|_| t!("errors.server.invalid_server_id"))?;
    let connection_pool = app_context
        .db_connection
        .get()
        .cloned()
        .ok_or_else(|| {
            warn!("can't set connected server because database connection pool is not initialized");
            t!("errors.database.internal_error")
        })?;

    let server_repository = ServerRepository::new(connection_pool);
    let server = server_repository
        .find_by_id(parsed_id)
        .map_err(|err| {
            warn!("unable to find server by id: {}", err);
            t!("errors.database.internal_error")
        })?;
    let server = match server {
        Some(server) => server,
        None => {
            debug!("server with id {} not found", parsed_id);
            return Err(t!("errors.server.server_not_found").to_string());
        }
    };

    server_repository
        .update_connected(server.id)
        .map_err(|err| {
            warn!("unable to update connected server: {}", err);
            t!("errors.server.change_server")
        })?;
    current_server_state.set_server(server.clone());
    debug!("current server set to {} - {}", server.id, server.name);

    Ok(())
}

#[tauri::command]
pub fn get_connected_server(current_server_state: State<SelectedServerState>) -> ServerResult<Server> {
    debug!("get current connected server");
    let connected_server = current_server_state
        .server
        .lock()
        .map_err(|err| {
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
    let connection_pool = app_context
        .db_connection
        .get()
        .cloned()
        .ok_or_else(|| {
            warn!("can't get server by url because database connection pool is not initialized");
            t!("errors.database.internal_error")
        })?;

    let server_repository = ServerRepository::new(connection_pool);
    let servers = server_repository
        .find_by_url(url)
        .map_err(|err| {
            warn!("unable to find server by url: {}", err);
            t!("errors.server.get_server")
        })?;
    let server = servers.into_iter().next();

    Ok(server)
}

#[tauri::command]
pub fn get_server_by_id(app_context: State<AppContext>, id: String) -> ServerResult<Option<Server>> {
    let connection_pool = app_context
        .db_connection
        .get()
        .cloned()
        .ok_or_else(|| {
            warn!("can't get server by id because database connection pool is not initialized");
            t!("errors.database.internal_error")
        })?;

    let parsed_id = Uuid::parse_str(&id)
        .map_err(|_| t!("errors.server.invalid_server_id"))?;
    let server_repository = ServerRepository::new(connection_pool);
    let server = server_repository
        .find_by_id(parsed_id)
        .map_err(|err| {
            warn!("unable to find server by id: {}", err);
            t!("errors.server.get_server")
        })?;

    Ok(server)
}

#[tauri::command]
pub fn create_server(app_context: State<AppContext>, new_server: Server) -> ServerResult<Server> {
    debug!("creating new server: {:?}", new_server);
    let connection_pool = app_context
        .db_connection
        .get()
        .cloned()
        .ok_or_else(|| {
            warn!("can't create server because database connection pool is not initialized");
            t!("errors.database.internal_error")
        })?;

    let server_repository = ServerRepository::new(connection_pool);

    server_repository
        .save(new_server.clone())
        .map_err(|err| {
            warn!("unable to create new server: {}", err);
            t!("errors.server.create_server")
        })?;

    debug!("server with id {} created", new_server.id);
    Ok(new_server)
}

pub fn initialize_current_server_state(app_handle: &AppHandle) -> ServerResult<()> {
    let app_context = app_handle.state::<AppContext>();
    let current_server = app_handle.state::<SelectedServerState>();

    let connection_pool = app_context
        .db_connection
        .get()
        .cloned()
        .ok_or_else(|| {
            warn!("can't initialize current server because database connection pool is not initialized");
            t!("errors.database.internal_error")
        })?;

    let server_repository = ServerRepository::new(connection_pool);
    let connected_server = server_repository
        .find_connected()
        .map_err(|err| {
            warn!("unable to find connected server: {}", err);
            t!("errors.server.get_current_server").to_string()
        })?;

    let connected_server = match connected_server {
        Some(server) => server,
        None => {
            warn!("connected server not found");
            return Err(t!("errors.server.server_not_found").to_string());
        }
    };

    current_server.set_server(connected_server.clone());
    debug!("server initialization complete, current server is {} - {}", connected_server.id, connected_server.name);

    Ok(())
}