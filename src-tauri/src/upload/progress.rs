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

use crate::Upload;
use log::debug;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub upload_id: Uuid,
    pub progress: f32,
    pub total_size: u64,
    pub uploaded_bytes: u64,
    pub rate_bytes: u64,
    last_uploaded_bytes: u64,
}

impl Progress {
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

    pub fn from_upload(upload: &Upload) -> Self {
        Self::new(upload.id, upload.size)
    }

    pub fn update(&mut self, uploaded_bytes: u64, elapsed: Duration) -> Self {
        self.uploaded_bytes = uploaded_bytes;
        self.progress = self.calculate_progress(uploaded_bytes);
        self.rate_bytes = self.calculate_rate(uploaded_bytes, elapsed);
        self.clone()
    }

    fn calculate_progress(&self, uploaded_bytes: u64) -> f32 {
        if self.total_size > 0 {
            uploaded_bytes as f32 / self.total_size as f32
        } else {
            0.0
        }
    }

    fn calculate_rate(&mut self, uploaded_bytes: u64, elapsed: Duration) -> u64 {
        debug!("progress uploaded bytes: {}", uploaded_bytes);
        debug!("progress last uploaded bytes: {}", self.last_uploaded_bytes);
        let transferred_bytes = uploaded_bytes.saturating_sub(self.last_uploaded_bytes);
        self.last_uploaded_bytes = uploaded_bytes;

        let rate_bytes = if elapsed.is_zero() {
            0
        } else {
            (transferred_bytes as f64 / elapsed.as_secs_f64()) as u64
        };

        debug!("progress rate bytes: {}", rate_bytes);
        rate_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rate_uses_elapsed_time_including_the_first_measurement() {
        let mut progress = Progress::new(Uuid::nil(), 20_000);

        assert_eq!(
            progress.update(6_000, Duration::from_secs(3)).rate_bytes,
            2_000
        );
        assert_eq!(
            progress
                .update(11_000, Duration::from_millis(2_500))
                .rate_bytes,
            2_000
        );
        assert_eq!(
            progress.update(12_000, Duration::from_secs(2)).rate_bytes,
            500
        );
    }

    #[test]
    fn rate_handles_zero_elapsed_time_and_non_increasing_bytes() {
        let mut progress = Progress::new(Uuid::nil(), 20_000);

        assert_eq!(progress.update(1_000, Duration::ZERO).rate_bytes, 0);
        assert_eq!(progress.update(1_000, Duration::from_secs(1)).rate_bytes, 0);
        assert_eq!(progress.update(500, Duration::from_secs(1)).rate_bytes, 0);
    }
}
