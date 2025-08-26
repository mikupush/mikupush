use rand::random;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::status::Status;
use crate::date_time::DateTimeUtc;

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
    pub status: Status,
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
            status: Status::Pending,
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