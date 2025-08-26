use serde::Serialize;
use mikupush_common::Progress;

pub const UPLOADS_CHANGED_EVENT: &str = "uploads-changed";

#[derive(Debug, Clone, Serialize)]
pub struct ProgressEvent {
    pub upload_id: String,
    pub progress: Progress
}