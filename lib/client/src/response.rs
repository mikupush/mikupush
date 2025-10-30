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