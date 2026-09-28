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

use super::{Progress, Upload};
use crate::cipher;
use crate::mime_type::detect_mime_type;
use crate::server::Server;
use serde::{Deserialize, Serialize};
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct UploadPreferences {
    #[serde(default = "default_encryption")]
    pub encryption: bool,
}

fn default_encryption() -> bool {
    true
}

impl Default for UploadPreferences {
    fn default() -> Self {
        Self {
            encryption: default_encryption(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadRequestError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadRequest {
    #[serde(default)]
    pub preferences: UploadPreferences,
    pub progress: Progress,
    pub error: Option<UploadRequestError>,
    pub upload: Upload,
    pub finished: bool,
    pub canceled: bool,
    pub chunked: bool,
    pub chunk_size: u64,
}

impl UploadRequest {
    pub fn new(
        id: Uuid,
        name: String,
        size: u64,
        mime_type: String,
        path: String,
        server: Server,
        directory: bool,
        preferences: UploadPreferences,
    ) -> Self {
        let encryption_key = preferences.encryption.then(cipher::generate_key);

        Self {
            preferences,
            progress: Progress::new(id, size),
            error: None,
            upload: Upload::new(
                id,
                name,
                size,
                mime_type,
                path,
                server,
                directory,
                encryption_key,
            ),
            finished: false,
            canceled: false,
            chunked: false,
            chunk_size: 0,
        }
    }

    pub fn upload_by_chunks(&self, chunk_size: u64) -> Self {
        let mut this = self.clone();
        this.chunked = true;
        this.chunk_size = chunk_size;
        this
    }

    pub fn with_upload_file(&self, path: String, size: u64, mime_type: String) -> Self {
        let mut this = self.clone();
        this.upload.path = path;
        this.upload.size = size;
        this.upload.mime_type = mime_type;
        this.progress = Progress::new(this.upload.id, size);
        this
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
        this.error = Some(UploadRequestError {
            code,
            message: error,
        });
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
        this.progress = Progress::from_upload(&this.upload);
        this.upload.status = super::status::Status::Enqueued;
        this.finished = false;
        this.canceled = false;
        this.error = None;
        this
    }

    pub fn from_file_path(
        path: String,
        server: Server,
        preferences: UploadPreferences,
    ) -> Result<Self, String> {
        let path = Path::new(&path);
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| "Failed to get file name".to_string())?
            .to_string();
        let metadata =
            std::fs::metadata(&path).map_err(|e| format!("Failed to get file metadata: {}", e))?;
        let size = metadata.len();
        let mime_type = match detect_mime_type(path.to_path_buf()) {
            Ok(mime_type) => mime_type.to_string(),
            Err(err) => return Err(err.to_string()),
        };

        Ok(Self::new(
            Uuid::new_v4(),
            file_name,
            size,
            mime_type,
            path.to_str().unwrap().to_string(),
            server,
            false,
            preferences,
        ))
    }

    pub fn from_directory_path(
        path: String,
        server: Server,
        preferences: UploadPreferences,
    ) -> Result<Self, String> {
        let path = Path::new(&path);
        let directory_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| "Failed to get directory name".to_string())?;
        let name = format!("{}.zip", directory_name);

        Ok(Self::new(
            Uuid::new_v4(),
            name,
            0,
            "application/zip".to_string(),
            path.to_str().unwrap().to_string(),
            server,
            true,
            preferences,
        ))
    }

    pub fn from_path(
        path: String,
        server: Server,
        preferences: UploadPreferences,
    ) -> Result<Self, String> {
        let metadata =
            std::fs::metadata(&path).map_err(|e| format!("Failed to get path metadata: {}", e))?;

        if metadata.is_dir() {
            return Self::from_directory_path(path, server, preferences);
        }

        Self::from_file_path(path, server, preferences)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upload_preferences_are_owned_and_survive_transfer_updates() {
        let mut defaults = UploadPreferences { encryption: true };
        let request = UploadRequest::new(
            Uuid::new_v4(),
            "test.txt".into(),
            10,
            "text/plain".into(),
            "/tmp/test.txt".into(),
            Server::test(),
            false,
            defaults.clone(),
        );
        defaults.encryption = false;
        assert_ne!(request.preferences, defaults);
        assert!(request.preferences.encryption);
        assert_eq!(request.upload.encryption_key.as_ref().unwrap().len(), 64);

        let compressed =
            request.with_upload_file("/tmp/test.zip".into(), 20, "application/zip".into());
        let failed = compressed
            .upload_by_chunks(5)
            .finish_with_error("test".into(), "test".into());
        assert!(failed.reset_progress().preferences.encryption);
        assert_eq!(failed.upload.encryption_key, request.upload.encryption_key);

        let serialized = serde_json::to_string(&request).unwrap();
        let restored: UploadRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(restored.preferences, request.preferences);
        assert_eq!(
            restored.upload.encryption_key,
            request.upload.encryption_key
        );
    }

    #[test]
    fn request_without_encryption_has_no_key_or_link_fragment() {
        let request = UploadRequest::new(
            Uuid::new_v4(),
            "test.txt".into(),
            10,
            "text/plain".into(),
            "/tmp/test.txt".into(),
            Server::test(),
            false,
            UploadPreferences { encryption: false },
        );

        assert!(request.upload.encryption_key.is_none());
        assert!(!request.upload.url.contains('#'));
        assert_eq!(request.upload.encrypted_size(), request.upload.size);
    }

    #[test]
    fn older_requests_and_missing_preference_fields_use_defaults() {
        let request = UploadRequest::new(
            Uuid::new_v4(),
            "test.txt".into(),
            10,
            "text/plain".into(),
            "/tmp/test.txt".into(),
            Server::test(),
            false,
            UploadPreferences::default(),
        );
        let mut value = serde_json::to_value(request).unwrap();
        value.as_object_mut().unwrap().remove("preferences");
        let restored: UploadRequest = serde_json::from_value(value).unwrap();
        assert!(restored.preferences.encryption);
        assert_eq!(
            serde_json::from_str::<UploadPreferences>("{}").unwrap(),
            UploadPreferences::default()
        );
    }
}
