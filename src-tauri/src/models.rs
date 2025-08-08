use std::fmt::Display;
use std::path::Path;
use mimetype_detector::detect_file;
use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use mikupush_entity::upload;
use rand::random;

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
    pub size: i64,
    pub mime_type: String,
    pub path: String,
    pub url: Option<String>,
    pub created_at: DateTimeUtc,
    pub status: UploadStatus,
}

impl Upload {
    pub fn new(
        id: Uuid,
        name: String,
        size: i64,
        mime_type: String,
        path: String
    ) -> Self {
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
            "/path/to/zip".to_string()
        )
    }
}

impl From<upload::Model> for Upload {
    fn from(model: upload::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            size: model.size,
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
            size: model.size,
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
    pub error: Option<String>,
    pub upload: Upload,
    pub status: String,
}

impl UploadRequest {
    pub fn new(
        id: Uuid,
        name: String,
        size: i64,
        mime_type:
        String,
        path: String
    ) -> Self {
        Self {
            progress: 0.0,
            error: None,
            upload: Upload::new(id, name, size, mime_type, path),
            status: UploadStatus::Pending.to_string(),
        }
    }

    pub fn from_file_path(path: String) -> Result<Self, String> {
        let path = Path::new(&path);
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| "Failed to get file name".to_string())?
            .to_string();
        let metadata = std::fs::metadata(&path)
            .map_err(|e| format!("Failed to get file metadata: {}", e))?;
        let size = metadata.len() as i64;
        let mime_type = match detect_file(path.to_str().unwrap()) {
            Ok(mime_type) => mime_type.to_string(),
            Err(_) => "application/octet-stream".to_string(),
        };

        Ok(Self::new(
            Uuid::new_v4(),
            file_name,
            size,
            mime_type,
            path.to_str().unwrap().to_string()
        ))
    }
    
    pub fn set_progress(&mut self, progress: f64) {
        self.progress = progress;
        if progress > 0.0 && progress < 1.0 {
            self.status = UploadStatus::InProgress.to_string();
        }
    }
    
    pub fn set_completed(&mut self) {
        self.progress = 1.0;
        self.status = UploadStatus::Completed.to_string();
    }

    pub fn set_failed(&mut self, error: String) {
        self.status = UploadStatus::Failed.to_string();
        self.error = Some(error);
    }
    
    pub fn set_aborted(&mut self) {
        self.status = UploadStatus::Aborted.to_string();
    }
    
    pub fn is_completed(&self) -> bool {
        self.status == UploadStatus::Completed.to_string()
    }
    
    pub fn is_failed(&self) -> bool {
        self.status == UploadStatus::Failed.to_string()
    }
    
    pub fn is_aborted(&self) -> bool {
        self.status == UploadStatus::Aborted.to_string()
    }
}
