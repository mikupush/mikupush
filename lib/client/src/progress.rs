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