use serde::Serialize;
use mikupush_common::Progress;

pub const UPLOAD_PROGRESS_CHANGE_EVENT: &str = "upload-progress-changed";
pub const UPLOAD_FAILED_EVENT: &str = "upload-failed";
pub const UPLOAD_FINISH_EVENT: &str = "upload-finish";

#[derive(Debug, Clone, Serialize)]
pub struct ProgressEvent {
    pub upload_id: String,
    pub progress: Progress
}