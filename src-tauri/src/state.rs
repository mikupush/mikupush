use crate::models::UploadRequest;
use log::warn;
use std::{collections::HashMap, sync::Mutex};
use uuid::Uuid;

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

    pub fn add_request(&self, upload_request: UploadRequest) -> Vec<UploadRequest> {
        let upload = upload_request.upload.clone();
        let id = upload.id.clone().to_string();

        let in_progress = self.in_progress.lock();
        if let Err(error) = in_progress {
            warn!(
                "can't lock in_progress property: {}, returning empty vector",
                error
            );
            return vec![];
        }

        let mut in_progress = in_progress.unwrap();
        if let None = in_progress.get_mut(&id) {
            in_progress.insert(id, upload_request);
        }

        Self::sorted_in_progress(&in_progress)
    }

    pub fn update_request(&self, upload_request: UploadRequest) {
        let id = upload_request.upload.id.clone().to_string();
        let in_progress = self.in_progress.lock();
        if let Err(error) = in_progress {
            warn!(
                "can't lock in_progress property: {}, skipping update operation",
                error
            );
            return;
        }

        let mut in_progress = in_progress.unwrap();
        if let Some(_) = in_progress.get_mut(&id) {
            in_progress.insert(id, upload_request);
        }
    }

    pub fn get_request(&self, id: String) -> Option<UploadRequest> {
        let in_progress = self.in_progress.lock();
        if let Err(error) = in_progress {
            warn!(
                "can't lock in_progress property: {}, returning None optional",
                error
            );
            return None;
        }

        let in_progress = in_progress.unwrap();
        in_progress
            .get(&id)
            .map(|upload_request| upload_request.clone())
    }

    fn sorted_in_progress(in_progress: &HashMap<String, UploadRequest>) -> Vec<UploadRequest> {
        let mut items: Vec<UploadRequest> = in_progress.values().cloned().collect();

        items.sort_by(|a, b| a.upload.created_at.cmp(&b.upload.created_at));
        items.reverse();

        items
    }
}
