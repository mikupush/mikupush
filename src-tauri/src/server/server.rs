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
use crate::date_time::DateTimeUtc;
use crate::server::{ServerRepository, server_icon_url};
use crate::state::SelectedServerState;
use log::{debug, warn};
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use url::Url;
use uuid::Uuid;

type ServerResult<T> = Result<T, String>;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub id: Uuid,
    pub url: String,
    pub name: String,
    pub icon: Option<String>,
    pub alias: Option<String>,
    pub use_alias: bool,
    pub added_at: DateTimeUtc,
    pub connected_at: Option<DateTimeUtc>,
    pub testing: bool,
    pub connected: bool,
    pub healthy: bool,
}

impl Server {
    pub fn new(id: Uuid, url: String, name: String) -> Self {
        Self {
            id,
            url,
            name,
            icon: None,
            alias: None,
            use_alias: false,
            added_at: chrono::Utc::now(),
            connected_at: None,
            testing: false,
            connected: false,
            healthy: false,
        }
    }

    pub fn new_from_url(url: String) -> Result<Self, url::ParseError> {
        let url_str = url.clone();
        let base_url = Url::parse(url.clone().as_str())?;
        let host = base_url.host_str().unwrap_or("").to_string();

        Ok(Self::new(Uuid::new_v4(), url_str, host))
    }

    pub fn test() -> Self {
        Server::new(
            Uuid::new_v4(),
            "http://localhost:8080".to_string(),
            "Test Server".to_string(),
        )
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new(Uuid::new_v4(), "".to_string(), "".to_string())
    }
}

pub fn initialize_current_server_state(app_handle: &AppHandle) -> ServerResult<()> {
    let app_context = app_handle.state::<AppContext>();
    let current_server = app_handle.state::<SelectedServerState>();

    let connection_pool = app_context.db_connection.get().cloned().ok_or_else(|| {
        warn!(
            "can't initialize current server because database connection pool is not initialized"
        );
        t!("errors.database.internal_error")
    })?;

    let server_repository = ServerRepository::new(connection_pool);
    let connected_server = server_repository.find_connected().map_err(|err| {
        warn!("unable to find connected server: {}", err);
        t!("errors.server.get_current_server").to_string()
    })?;

    let find_first_server = || -> Option<Server> {
        let all_servers = server_repository.find_all();
        if let Err(err) = &all_servers {
            warn!("unable to find all servers: {}", err);
            return None;
        }

        all_servers.ok()?.first().map(Clone::clone)
    };

    let connected_server = connected_server.or_else(find_first_server);

    if let Some(server) = connected_server {
        current_server.set_server(server.clone());
        debug!(
            "server initialization complete, current server is {} - {}",
            server.id, server.name
        );
    }

    Ok(())
}

pub(crate) fn map_server_icon_into_base64(app_handle: &AppHandle, server: &Server) -> Server {
    let mut server = server.clone();
    if let Some(icon) = server.icon {
        server.icon = server_icon_url(app_handle.clone(), icon).ok()
    }

    server
}
