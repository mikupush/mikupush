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

use serde::Serialize;
use uuid::Uuid;
use mikupush_common::Progress;

#[derive(Debug, Copy, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressTrack {
    pub upload_id: Uuid,
    pub progress: f32,
    pub total_size: u64,
    pub uploaded_bytes: u64,
    pub rate_bytes: u64,
    last_uploaded_bytes: u64,
}

impl ProgressTrack {
    pub fn new(upload_id: Uuid, total_size: u64) -> Self {
        Self {
            upload_id,
            progress: 0.0,
            total_size,
            uploaded_bytes: 0,
            rate_bytes: 0,
            last_uploaded_bytes: 0,
        }
    }

    pub fn update(&mut self, uploaded_bytes: u64) -> Self {
        self.uploaded_bytes = uploaded_bytes;
        self.progress = self.calculate_progress(uploaded_bytes);
        self.rate_bytes = self.calculate_rate(uploaded_bytes);
        self.clone()
    }

    fn calculate_progress(&self, uploaded_bytes: u64) -> f32 {
        if self.total_size > 0 {
            uploaded_bytes as f32 / self.total_size as f32
        } else {
            0.0
        }
    }

    fn calculate_rate(&mut self, uploaded_bytes: u64) -> u64 {
        let rate_bytes = uploaded_bytes.saturating_sub(self.last_uploaded_bytes);
        self.last_uploaded_bytes = uploaded_bytes;
        rate_bytes
    }
}

impl From<ProgressTrack> for Progress {
    fn from(track: ProgressTrack) -> Self {
        Self {
            progress: track.progress,
            uploaded_bytes: track.uploaded_bytes,
            rate_bytes: track.rate_bytes,
            total_size: track.total_size,
        }
    }
}