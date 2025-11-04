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

use std::path::Path;
use mimetype_detector::detect_file;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::progress::Progress;
use crate::Server;
use crate::upload::Upload;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadRequestError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadRequest {
    pub progress: Progress,
    pub error: Option<UploadRequestError>,
    pub upload: Upload,
    pub finished: bool,
    pub canceled: bool,
}

impl UploadRequest {
    pub fn new(id: Uuid, name: String, size: u64, mime_type: String, path: String, server: Server) -> Self {
        Self {
            progress: Progress::new(size),
            error: None,
            upload: Upload::new(id, name, size, mime_type, path, server),
            finished: false,
            canceled: false,
        }
    }

    pub fn update_progress(&self, progress: Progress) -> Self {
        let mut this = self.clone();
        this.progress = progress;
        this
    }

    pub fn finish(&self) -> Self {
        let mut this = self.clone();
        this.finished = true;
        this.error = None;
        this
    }

    pub fn finish_with_error(&self, code: String, error: String) -> Self {
        let mut this = self.clone();
        this.error = Some(UploadRequestError { code, message: error });
        this.finished = true;
        this
    }

    pub fn canceled(&self) -> Self {
        let mut this = self.clone();
        this.canceled = true;
        this.finished = true;
        this.error = None;
        this
    }

    pub fn reset_progress(&self) -> Self {
        let mut this = self.clone();
        this.progress = Progress::new(this.upload.size);
        this.finished = false;
        this.error = None;
        this
    }

    pub fn from_file_path(path: String, server: Server) -> Result<Self, String> {
        let path = Path::new(&path);
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| "Failed to get file name".to_string())?
            .to_string();
        let metadata =
            std::fs::metadata(&path).map_err(|e| format!("Failed to get file metadata: {}", e))?;
        let size = metadata.len();
        let mime_type = match detect_file(path.to_str().unwrap()) {
            Ok(mime_type) => mime_type.to_string(),
            Err(_) => "application/octet-stream".to_string(),
        };

        Ok(Self::new(
            Uuid::new_v4(),
            file_name,
            size,
            mime_type,
            path.to_str().unwrap().to_string(),
            server,
        ))
    }
}