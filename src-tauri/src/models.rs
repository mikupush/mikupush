use mikupush_entity::upload;
use mimetype_detector::detect_file;
use rand::random;
use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::path::Path;
use uuid::Uuid;

use crate::server::ProgressEvent;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum UploadStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Aborted,
}

impl Display for UploadStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            UploadStatus::Pending => "pending".to_string(),
            UploadStatus::InProgress => "inProgress".to_string(),
            UploadStatus::Completed => "completed".to_string(),
            UploadStatus::Failed => "failed".to_string(),
            UploadStatus::Aborted => "aborted".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<String> for UploadStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "pending" => UploadStatus::Pending,
            "inProgress" => UploadStatus::InProgress,
            "completed" => UploadStatus::Completed,
            "failed" => UploadStatus::Failed,
            "aborted" => UploadStatus::Aborted,
            _ => UploadStatus::Failed,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Upload {
    pub id: Uuid,
    pub name: String,
    pub size: u64,
    pub mime_type: String,
    pub path: String,
    pub url: Option<String>,
    pub created_at: DateTimeUtc,
    pub status: UploadStatus,
}

impl Upload {
    pub fn new(id: Uuid, name: String, size: u64, mime_type: String, path: String) -> Self {
        Self {
            id,
            name,
            size,
            mime_type,
            path,
            url: None,
            created_at: chrono::Utc::now(),
            status: UploadStatus::Pending,
        }
    }

    pub fn test() -> Self {
        Upload::new(
            Uuid::new_v4(),
            "test.zip".to_string(),
            random(),
            "application/zip".to_string(),
            "/path/to/zip".to_string(),
        )
    }
}

impl From<upload::Model> for Upload {
    fn from(model: upload::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            size: model.size as u64,
            mime_type: model.mime_type,
            path: model.path,
            url: model.url,
            created_at: model.created_at.and_utc(),
            status: model.status.into(),
        }
    }
}

impl From<Upload> for upload::Model {
    fn from(model: Upload) -> Self {
        Self {
            id: model.id,
            name: model.name,
            size: model.size as i64,
            mime_type: model.mime_type,
            path: model.path,
            url: model.url,
            created_at: model.created_at.naive_utc(),
            status: model.status.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadRequest {
    pub progress: f64,
    pub uploaded_bytes: u64,
    pub rate_bytes: u64,
    pub error: Option<String>,
    pub upload: Upload,
    pub finished: bool,
    pub canceled: bool,
}

impl UploadRequest {
    pub fn new(id: Uuid, name: String, size: u64, mime_type: String, path: String) -> Self {
        Self {
            progress: 0.0,
            uploaded_bytes: 0,
            rate_bytes: 0,
            error: None,
            upload: Upload::new(id, name, size, mime_type, path),
            finished: false,
            canceled: false,
        }
    }

    pub fn from_file_path(path: String) -> Result<Self, String> {
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
        ))
    }

    pub fn update_progress(&self, event: ProgressEvent) -> Self {
        let mut this = self.clone();
        this.progress = event.progress as f64;
        this.uploaded_bytes = event.uploaded_bytes;
        this.rate_bytes = event.rate_bytes;
        this
    }

    pub fn finish(&self) -> Self {
        let mut this = self.clone();
        this.finished = true;
        this.error = None;
        this
    }

    pub fn finish_with_error(&self, error: String) -> Self {
        let mut this = self.clone();
        this.error = Some(error);
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
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::models::UploadRequest;

    impl UploadRequest {
        pub fn test() -> Self {
            Self::new(
                Uuid::new_v4(),
                "hatsune-miku.jpg".to_string(),
                1790390,
                "image/jpeg".to_string(),
                "tests/examples/hatsune-miku.jpg".to_string(),
            )
        }
    }
}
