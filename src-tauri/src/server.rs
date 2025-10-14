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

use log::{debug, warn};
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;
use mikupush_common::Server;
use mikupush_database::ServerRepository;
use crate::AppContext;
use crate::state::SelectedServerState;

#[tauri::command]
pub fn set_connected_server(
    app_context: State<AppContext>,
    current_server_state: State<SelectedServerState>,
    id: String
) -> Result<(), String> {
    let id = Uuid::parse_str(&id)
        .map_err(|err| format!("Invalid server id: {}", err))?;
    let connection_pool = app_context.db_connection.get();
    if connection_pool.is_none() {
        warn!("can't set connected server because database connection pool is not initialized");
        return Err("connection pool is not initialized".to_string());
    }

    let server_repository = ServerRepository::new(connection_pool.unwrap().clone());
    let server = server_repository.find_by_id(id)
        .map_err(|err| format!("can't find server: {}", err))?;
    if server.is_none() {
        debug!("server with id {} not found", id);
        return Err(format!("server with id {} not found", id));
    }

    let server = server.unwrap();
    server_repository.update_connected(server.id)
        .map_err(|err| format!("can't update connected server: {}", err))?;
    current_server_state.set_server(server.clone());
    debug!("current server set to {} - {}", server.id, server.name);

    Ok(())
}

#[tauri::command]
pub fn get_connected_server(current_server_state: State<SelectedServerState>) -> Result<Server, String> {
    debug!("get current connected server");
    let connected_server = current_server_state.server.lock()
        .map_err(|err| format!("can't get connected server: {}", err))?;

    debug!("got current connected server {} - {}", connected_server.id, connected_server.name);
    Ok(connected_server.clone())
}

#[tauri::command]
pub fn get_server_by_url(app_context: State<AppContext>, url: String) -> Result<Option<Server>, String> {
    let connection_pool = app_context.db_connection.get();
    if connection_pool.is_none() {
        warn!("can't set connected server because database connection pool is not initialized");
        return Err("connection pool is not initialized".to_string());
    }

    let server_repository = ServerRepository::new(connection_pool.unwrap().clone());
    let servers = server_repository.find_by_url(url.clone())
        .map_err(|err| format!("can't find server by url: {}", err))?;
    let server = servers.first().map(|server| server.clone());

    Ok(server)
}

#[tauri::command]
pub fn get_server_by_id(app_context: State<AppContext>, id: String) -> Result<Option<Server>, String> {
    let connection_pool = app_context.db_connection.get();
    if connection_pool.is_none() {
        warn!("can't set connected server because database connection pool is not initialized");
        return Err("connection pool is not initialized".to_string());
    }

    let server_repository = ServerRepository::new(connection_pool.unwrap().clone());
    let id = Uuid::parse_str(&id)
        .map_err(|err| format!("invalid server id: {}", err))?;
    let server = server_repository.find_by_id(id)
        .map_err(|err| format!("can't find server by id: {}", err))?;

    Ok(server)
}

#[tauri::command]
pub fn create_server(app_context: State<AppContext>, new_server: Server) -> Result<Server, String> {
    debug!("creating new server: {:?}", new_server);
    let connection_pool = app_context.db_connection.get();
    if connection_pool.is_none() {
        warn!("can't set connected server because database connection pool is not initialized");
        return Err("connection pool is not initialized".to_string());
    }

    let server_repository = ServerRepository::new(connection_pool.unwrap().clone());

    server_repository.save(new_server.clone())
        .map_err(|err| format!("can't save server: {}", err))?;

    debug!("server with id {} created", new_server.id);
    Ok(new_server)
}

pub fn initialize_current_server_state(app_handle: &AppHandle) -> Result<(), String> {
    let app_context = app_handle.state::<AppContext>();
    let current_server = app_handle.state::<SelectedServerState>();

    let connection_pool = app_context.db_connection.get();
    if connection_pool.is_none() {
        warn!("can't initialize current server because database connection pool is not initialized");
        return Err("connection pool is not initialized".to_string());
    }

    let server_repository = ServerRepository::new(connection_pool.unwrap().clone());
    let connected_server = match server_repository.find_connected() {
        Ok(server) => server,
        Err(err) => {
            warn!("unable to find connected server: {}", err);
            return Err(format!("unable to find connected server: {}", err));
        }
    };

    if connected_server.is_none() {
        warn!("connected server not found");
        return Err("connected server not found".to_string());
    }

    let connected_server = connected_server.unwrap();
    current_server.set_server(connected_server.clone());
    debug!("server initialization complete, current server is {} - {}", connected_server.id, connected_server.name);

    Ok(())
}