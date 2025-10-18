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
use crate::error::CommandError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ServerError {
    InvalidServerId { input: String, source: uuid::Error },
    ServerNotFound { id: Uuid },
    InternalError { message: String },
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::InvalidServerId { input, source } => {
                write!(f, "invalid server id '{}': {}", input, source)
            }
            ServerError::ServerNotFound { id } => {
                write!(f, "server with id {} not found", id)
            }
            ServerError::InternalError { message } => {
                write!(f, "internal error: {}", message)
            }
        }
    }
}

impl std::error::Error for ServerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ServerError::InvalidServerId { source, .. } => Some(source),
            _ => None,
        }
    }
}

type ServerResult<T> = Result<T, CommandError>;

pub const SERVER_ERROR_INVALID_SERVER_ID: &str = "invalid_server_id";
pub const SERVER_ERROR_SERVER_NOT_FOUND: &str = "server_not_found";
pub const SERVER_ERROR_INTERNAL_ERROR: &str = "internal_error";

impl ServerError {
    pub fn code(&self) -> String {
        match self {
            ServerError::InvalidServerId { .. } => SERVER_ERROR_INVALID_SERVER_ID.to_string(),
            ServerError::ServerNotFound { .. } => SERVER_ERROR_SERVER_NOT_FOUND.to_string(),
            ServerError::InternalError { .. } => SERVER_ERROR_INTERNAL_ERROR.to_string(),
        }
    }

    pub fn to_command_error(&self) -> CommandError {
        let code = self.code();
        let message = match self {
            ServerError::InternalError { message } => message.clone(),
            _ => self.to_string(),
        };

        CommandError::new(code, message)
    }
}

impl From<ServerError> for CommandError {
    fn from(error: ServerError) -> Self {
        error.to_command_error()
    }
}

#[tauri::command]
pub fn set_connected_server(
    app_context: State<AppContext>,
    current_server_state: State<SelectedServerState>,
    id: String,
) -> ServerResult<()> {
    let parsed_id = Uuid::parse_str(&id)
        .map_err(|err| ServerError::InvalidServerId { input: id.clone(), source: err })?;
    let connection_pool = app_context
        .db_connection
        .get()
        .cloned()
        .ok_or_else(|| {
            warn!("can't set connected server because database connection pool is not initialized");
            ServerError::InternalError { message: "database connection pool is not initialized".to_string() }
        })?;

    let server_repository = ServerRepository::new(connection_pool);
    let server = server_repository
        .find_by_id(parsed_id)
        .map_err(|err| ServerError::InternalError { message: err.to_string() }.to_command_error())?;
    let server = match server {
        Some(server) => server,
        None => {
            debug!("server with id {} not found", parsed_id);
            return Err(ServerError::ServerNotFound { id: parsed_id }.to_command_error());
        }
    };

    server_repository
        .update_connected(server.id)
        .map_err(|err| ServerError::InternalError { message: err.to_string() }.to_command_error())?;
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
            ServerError::InternalError { message }.to_command_error()
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
            ServerError::InternalError { message: "database connection pool is not initialized".to_string() }.to_command_error()
        })?;

    let server_repository = ServerRepository::new(connection_pool);
    let servers = server_repository
        .find_by_url(url)
        .map_err(|err| ServerError::InternalError { message: err.to_string() }.to_command_error())?;
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
            ServerError::InternalError { message: "database connection pool is not initialized".to_string() }.to_command_error()
        })?;

    let parsed_id = Uuid::parse_str(&id)
        .map_err(|err| ServerError::InvalidServerId { input: id.clone(), source: err }.to_command_error())?;
    let server_repository = ServerRepository::new(connection_pool);
    let server = server_repository
        .find_by_id(parsed_id)
        .map_err(|err| ServerError::InternalError { message: err.to_string() }.to_command_error())?;

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
            ServerError::InternalError { message: "database connection pool is not initialized".to_string() }.to_command_error()
        })?;

    let server_repository = ServerRepository::new(connection_pool);

    server_repository
        .save(new_server.clone())
        .map_err(|err| ServerError::InternalError { message: err.to_string() }.to_command_error())?;

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
            ServerError::InternalError { message: "database connection pool is not initialized".to_string() }.to_command_error()
        })?;

    let server_repository = ServerRepository::new(connection_pool);
    let connected_server = server_repository
        .find_connected()
        .map_err(|err| {
            warn!("unable to find connected server: {}", err);
            ServerError::InternalError { message: err.to_string() }.to_command_error()
        })?;

    let connected_server = match connected_server {
        Some(server) => server,
        None => {
            warn!("connected server not found");
            return Err(ServerError::InternalError { message: "connected server not found".to_string() }.to_command_error());
        }
    };

    current_server.set_server(connected_server.clone());
    debug!("server initialization complete, current server is {} - {}", connected_server.id, connected_server.name);

    Ok(())
}
