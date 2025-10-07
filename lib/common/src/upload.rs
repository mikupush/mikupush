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

use rand::random;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::status::Status;
use crate::date_time::DateTimeUtc;

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
    pub fn new(id: Uuid, name: String, size: u64, mime_type: String, path: String, server_id: Uuid) -> Self {
        Self {
            id,
            name,
            size,
            mime_type,
            path,
            server_id,
            url: "".to_string(),
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
            Uuid::new_v4()
        )
    }
}