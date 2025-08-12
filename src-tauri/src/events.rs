use serde::Serialize;
use uuid::Uuid;

pub const UPLOAD_LIST_CHANGED_EVENT: &str = "uploads-list-changed";
pub const UPLOAD_PROGRESS_CHANGE_EVENT: &str = "upload-progress-changed";
pub const UPLOAD_FAILED_EVENT: &str = "upload-failed";
pub const UPLOAD_FINISH_EVENT: &str = "upload-finish";
pub const UPLOAD_CHANGED_EVENT: &str = "upload-changed";

#[derive(Debug, Clone, Serialize)]
pub struct UploadFailedEvent {
    pub upload_id: Uuid,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UploadFinishEvent {
    pub upload_id: Uuid,
}