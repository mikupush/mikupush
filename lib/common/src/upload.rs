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

use rand::random;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::status::Status;
use crate::date_time::DateTimeUtc;
use crate::Server;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Upload {
    pub id: Uuid,
    pub name: String,
    pub size: u64,
    pub mime_type: String,
    pub path: String,
    pub url: String,
    pub created_at: DateTimeUtc,
    pub status: Status,
    pub server_id: Uuid
}

impl Upload {
    pub fn new(id: Uuid, name: String, size: u64, mime_type: String, path: String, server: Server) -> Self {
        Self {
            id,
            name,
            size,
            mime_type,
            path,
            server_id: server.id,
            url: format!("{}/u/{}", server.url, id),
            created_at: chrono::Utc::now(),
            status: Status::Pending,
        }
    }

    pub fn test() -> Self {
        Upload::new(
            Uuid::new_v4(),
            "test.zip".to_string(),
            random(),
            "application/zip".to_string(),
            "/path/to/zip".to_string(),
            Server::test(),
        )
    }
}