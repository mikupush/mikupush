use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub progress: f32,
    pub total_size: u64,
    pub uploaded_bytes: u64,
    pub rate_bytes: u64,
}

impl Progress {
    pub fn new(total_size: u64) -> Self {
        Self {
            progress: 0.0,
            total_size,
            uploaded_bytes: 0,
            rate_bytes: 0,
        }
    }
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            progress: 0.0,
            total_size: 0,
            uploaded_bytes: 0,
            rate_bytes: 0,
        }
    }
}