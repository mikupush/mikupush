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

use serde::{Deserialize, Serialize};
use std::error::Error;
use chrono::NaiveDateTime;
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn from_string(content: String) -> Result<ErrorResponse, Box<dyn Error>> {
        Ok(serde_json::from_str(&content)?)
    }
}

pub enum HealthCheckStatus {
    Up,
    Down,
}

impl HealthCheckStatus {
    pub fn from_string(content: String) -> Result<HealthCheckStatus, Box<dyn Error>> {
        let json: Value = serde_json::from_str(&content)?;
        let status = json["status"].as_str().unwrap_or("down");

        match status {
            "up" => Ok(HealthCheckStatus::Up),
            "down" => Ok(HealthCheckStatus::Down),
            _ => Err(format!("Invalid health check status: {}", status).into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileStatus {
    WaitingForUpload,
    Uploaded
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub id: Uuid,
    pub name: String,
    pub mime_type: String,
    pub size: i64,
    pub uploaded_at: NaiveDateTime,
    pub status: FileStatus
}