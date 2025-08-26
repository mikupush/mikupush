use mikupush_common::UploadRequest;
use log::warn;
use std::{collections::HashMap, sync::Mutex};
use tokio_util::sync::CancellationToken;
use mikupush_client::{Client, Server};

#[derive(Debug)]
pub struct UploadsState {
    in_progress: Mutex<HashMap<String, UploadRequest>>,
    cancellation_tokens: Mutex<HashMap<String, CancellationToken>>,
}

impl UploadsState {
    pub fn new() -> Self {
        Self {
            in_progress: Mutex::new(HashMap::new()),
            cancellation_tokens: Mutex::new(HashMap::new()),
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

    pub fn delete_request(&self, id: String) -> Vec<UploadRequest> {
        let mut in_progress = self.in_progress.lock().unwrap();
        in_progress.remove(&id);
        Self::sorted_in_progress(&in_progress)
    }

    pub fn add_cancellation_token(&self, id: String, token: CancellationToken) {
        let mut tokens = self.cancellation_tokens.lock().unwrap();
        tokens.insert(id, token);
    }

    pub fn cancel_upload(&self, id: String) {
        let mut tokens = self.cancellation_tokens.lock().unwrap();

        if let Some(task) = tokens.get(&id) {
            task.cancel();
        }

        tokens.remove(&id);
    }

    pub fn remove_cancellation_token(&self, id: String) {
        let mut tokens = self.cancellation_tokens.lock().unwrap();
        tokens.remove(&id);
    }

    fn sorted_in_progress(in_progress: &HashMap<String, UploadRequest>) -> Vec<UploadRequest> {
        let mut items: Vec<UploadRequest> = in_progress.values().cloned().collect();

        items.sort_by(|a, b| a.upload.created_at.cmp(&b.upload.created_at));
        items.reverse();

        items
    }
}

#[derive(Debug)]
pub struct SelectedServerState {
    pub server: Mutex<Server>,
}

impl SelectedServerState {
    pub fn new() -> Self {
        Self {
            server: Mutex::new(Server::default()),
        }
    }

    pub fn client(&self) -> Client {
        Client::new(self.server.lock().unwrap().clone())
    }
}
