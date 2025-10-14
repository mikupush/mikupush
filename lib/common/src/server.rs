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

use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;
use crate::date_time::DateTimeUtc;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub id: Uuid,
    pub url: String,
    pub name: String,
    pub icon: Option<String>,
    pub alias: Option<String>,
    pub added_at: DateTimeUtc,
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
            added_at: chrono::Utc::now(),
            testing: false,
            connected: false,
            healthy: false,
        }
    }

    pub fn new_from_url(url: String) -> Result<Self, url::ParseError> {
        let url_str = url.clone();
        let base_url = Url::parse(url.clone().as_str())?;
        let host = base_url.host_str()
            .unwrap_or("")
            .to_string();

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
        Self::new(
            Uuid::new_v4(),
            "".to_string(),
            "".to_string()
        )
    }
}