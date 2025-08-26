use crate::error::{ClientError, ServerResponseError, UploadError};
use crate::server::Server;
use crate::upload::UploadTask;
use log::debug;
use mikupush_common::{Upload, UploadRequest};
use serde_json::json;
use uuid::Uuid;

pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(server: Server) -> Self {
        debug!("using server client with base url: {}", server.base_url);
        Self {
            base_url: server.base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn create(&self, upload: &Upload) -> Result<(), ClientError> {
        let data = json!({
            "uuid": upload.id,
            "name": upload.name,
            "mime_type": upload.mime_type,
            "size": upload.size
        });

        let url = format!("{}/api/file", self.base_url);
        let response = self.client.post(&url).json(&data).send().await?;

        if response.status() != 200 {
            return Err(ServerResponseError::from_response(response).into());
        }

        Ok(())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), ClientError> {
        let url = format!("{}/api/file/{}", self.base_url, id);
        let response = self.client.delete(&url).send().await?;

        if response.status() != 200 {
            return Err(ServerResponseError::from_response(response).into());
        }

        Ok(())
    }

    pub async fn upload(&self, request: &UploadRequest) -> Result<UploadTask, UploadError> {
        if request.upload.mime_type.is_empty() {
            return Err(UploadError::IO {
                message: "unknown file type".to_string(),
            });
        }

        let client = self.client.clone();
        let url = format!("{}/api/file/{}/upload", self.base_url, request.upload.id);

        UploadTask::new(url, request.upload.clone(), client).await
            .map_err(|err| UploadError::IO { message: err.to_string() })
    }
}