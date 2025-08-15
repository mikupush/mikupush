use std::{collections::HashMap, sync::Mutex};

use crate::models::UploadRequest;

#[derive(Debug)]
pub struct UploadsState {
    in_progress: Mutex<HashMap<String, UploadRequest>>,
}

impl UploadsState {
    pub fn new() -> Self {
        Self {
            in_progress: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_request(&self, upload_request: UploadRequest) -> Result<Vec<UploadRequest>, String> {
        let upload = upload_request.upload.clone();
        let id = upload.id.clone().to_string();

        let mut in_progress = self
            .in_progress
            .lock()
            .map_err(|err| format!("can't lock in_progress property: {}", err.to_string()))?;

        if let None = in_progress.get_mut(&id) {
            in_progress.insert(id, upload_request);
        }

        Ok(Self::sorted_in_progress(&in_progress))
    }

    fn sorted_in_progress(in_progress: &HashMap<String, UploadRequest>) -> Vec<UploadRequest> {
        let mut items: Vec<UploadRequest> = in_progress.values().cloned().collect();

        items.sort_by(|a, b| a.upload.created_at.cmp(&b.upload.created_at));
        items.reverse();

        items
    }
}
