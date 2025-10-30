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

use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Pending,
    InProgress,
    Completed,
    Failed,
    Aborted,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Status::Pending => "pending".to_string(),
            Status::InProgress => "inProgress".to_string(),
            Status::Completed => "completed".to_string(),
            Status::Failed => "failed".to_string(),
            Status::Aborted => "aborted".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<String> for Status {
    fn from(s: String) -> Self {
        match s.as_str() {
            "pending" => Status::Pending,
            "inProgress" => Status::InProgress,
            "completed" => Status::Completed,
            "failed" => Status::Failed,
            "aborted" => Status::Aborted,
            _ => Status::Failed,
        }
    }
}