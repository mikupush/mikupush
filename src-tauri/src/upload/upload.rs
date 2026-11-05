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

use crate::cipher;
use crate::date_time::DateTimeUtc;
use crate::server::Server;
use crate::upload::status::Status;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::random;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Upload {
    pub id: Uuid,
    pub name: String,
    pub size: u64,
    pub mime_type: String,
    pub path: String,
    pub url: String,
    pub encryption_key: Option<String>,
    pub created_at: DateTimeUtc,
    pub status: Status,
    pub server_id: Uuid,
    pub directory: bool,
}

impl Upload {
    pub fn new(
        id: Uuid,
        name: String,
        size: u64,
        mime_type: String,
        path: String,
        server: Server,
        directory: bool,
        encryption_key: Option<String>,
    ) -> Self {
        let url = format!("{}/u/{}", server.url, id);
        let url = match &encryption_key {
            Some(key) => format!("{}#{}", url, URL_SAFE_NO_PAD.encode(key.as_bytes())),
            None => url,
        };

        Self {
            id,
            name,
            size,
            mime_type,
            path,
            server_id: server.id,
            url,
            encryption_key,
            created_at: chrono::Utc::now(),
            status: Status::Enqueued,
            directory,
        }
    }

    /// Returns the file size if there isn't an encryption key, otherwise return the size including
    /// encryption extra bytes
    pub fn encrypted_size(&self) -> u64 {
        if self.encryption_key.is_none() {
            return self.size;
        }

        cipher::encrypted_size(self.size)
    }

    pub fn test() -> Self {
        Upload::new(
            Uuid::new_v4(),
            "test.zip".to_string(),
            random(),
            "application/zip".to_string(),
            "/path/to/zip".to_string(),
            Server::test(),
            false,
            None,
        )
    }
}
