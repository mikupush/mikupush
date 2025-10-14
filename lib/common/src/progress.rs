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

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub progress: f32,
    pub total_size: u64,
    pub uploaded_bytes: u64,
    pub rate_bytes: u64,
}

impl Progress {
    pub fn new(total_size: u64) -> Self {
        Self {
            progress: 0.0,
            total_size,
            uploaded_bytes: 0,
            rate_bytes: 0,
        }
    }
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            progress: 0.0,
            total_size: 0,
            uploaded_bytes: 0,
            rate_bytes: 0,
        }
    }
}