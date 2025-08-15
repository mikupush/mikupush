use std::{collections::VecDeque, sync::Mutex};

use crate::models::UploadRequest;

#[derive(Debug)]
pub struct UploadsState {
    in_progress: Mutex<VecDeque<UploadRequest>>,
}

impl UploadsState {
    pub fn new() -> Self {
        Self {
            in_progress: Mutex::new(VecDeque::new()),
        }
    }

    pub fn add_request(
        &self,
        upload_request: UploadRequest,
    ) -> Result<VecDeque<UploadRequest>, String> {
        let upload = upload_request.upload.clone();
        let mut in_progress = self
            .in_progress
            .lock()
            .map_err(|err| format!("can't lock in_progress property: {}", err.to_string()))?;

        let exists = in_progress.iter().any(|item| item.upload.id == upload.id);

        if !exists {
            in_progress.push_front(upload_request);
        }

        Ok(in_progress.clone())
    }
}
