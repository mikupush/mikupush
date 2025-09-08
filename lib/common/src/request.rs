use std::path::Path;
use mimetype_detector::detect_file;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::progress::Progress;
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
    pub fn new(id: Uuid, name: String, size: u64, mime_type: String, path: String) -> Self {
        Self {
            progress: Progress::new(size),
            error: None,
            upload: Upload::new(id, name, size, mime_type, path),
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
}