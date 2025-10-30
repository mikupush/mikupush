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