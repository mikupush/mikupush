/// Copyright 2025 Miku Push! Team
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///     http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.

use mikupush_common::Server;
use crate::upload::UploadTask;
use log::debug;
use mikupush_common::{Upload, UploadRequest};
use serde_json::{json, Value};
use uuid::Uuid;
use crate::error::{FileDeleteError, FileUploadError};
use crate::{FileInfoError, HealthCheckError};
use crate::response::{ErrorResponse, FileInfo, HealthCheckStatus};

pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(server: Server) -> Self {
        debug!("using server client with base url: {}", server.url);
        Self {
            base_url: server.url,
            client: reqwest::Client::builder()
                .user_agent("MikuPush/1.0.0")
                .build()
                .unwrap(),
        }
    }

    pub async fn create(&self, upload: &Upload) -> Result<(), FileUploadError> {
        let data = json!({
            "id": upload.id,
            "name": upload.name,
            "mime_type": upload.mime_type,
            "size": upload.size
        });

        let url = format!("{}/api/file", self.base_url);
        let response = self.client.post(&url).json(&data).send().await
            .map_err(|err| FileUploadError::ClientError { message: err.to_string()})?;
        let status = response.status().clone();
        let response_body = response.text().await
            .map_err(|err| FileUploadError::ClientError { message: err.to_string()})?;
        debug!("POST {}: {} - {}",  url, status, response_body);

        if status != 200 {
            let error_response = ErrorResponse::from_string(response_body)
                .map_err(|err| FileUploadError::ClientError { message: err.to_string()})?;
            return Err(error_response.into());
        }

        Ok(())
    }

    pub async fn info(&self, id: Uuid) -> Result<FileInfo, FileInfoError> {
        let url = format!("{}/api/file/{}", self.base_url, id);
        let response = self.client.delete(&url).send().await
            .map_err(|err| FileInfoError::ClientError { message: err.to_string()})?;
        let status = response.status().clone();
        let response_body = response.text().await
            .map_err(|err| FileInfoError::ClientError { message: err.to_string()})?;
        debug!("GET {}: {} - {}",  url, status, response_body);

        if status != 200 {
            let error_response = ErrorResponse::from_string(response_body)
                .map_err(|err| FileInfoError::ClientError { message: err.to_string()})?;
            return Err(error_response.into());
        }

        let info: FileInfo = serde_json::from_str(&response_body)
            .map_err(|err| FileInfoError::ClientError { message: err.to_string()})?;

        Ok(info)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), FileDeleteError> {
        let url = format!("{}/api/file/{}", self.base_url, id);
        let response = self.client.delete(&url).send().await
            .map_err(|err| FileDeleteError::ClientError { message: err.to_string()})?;
        let status = response.status().clone();
        let response_body = response.text().await
            .map_err(|err| FileDeleteError::ClientError { message: err.to_string()})?;
        debug!("DELETE {}: {} - {}",  url, status, response_body);

        if status != 200 {
            let error_response = ErrorResponse::from_string(response_body)
                .map_err(|err| FileDeleteError::ClientError { message: err.to_string()})?;
            return Err(error_response.into());
        }

        Ok(())
    }

    pub async fn upload(&self, request: &UploadRequest) -> Result<UploadTask, FileUploadError> {
        if request.upload.mime_type.is_empty() {
            return Err(FileUploadError::UnknownMimeType);
        }

        let client = self.client.clone();
        let url = format!("{}/api/file/{}/upload", self.base_url, request.upload.id);

        UploadTask::new(url, request.upload.clone(), client).await
            .map_err(|err| FileUploadError::ClientError { message: err.to_string() })
    }

    pub async fn check_health(&self) -> Result<HealthCheckStatus, HealthCheckError> {
        let url = format!("{}/health", self.base_url);
        let response = self.client.get(&url).send().await
            .map_err(|err| HealthCheckError { message: err.to_string()})?;
        let status = response.status().clone();
        let response_body = response.text().await
            .map_err(|err| HealthCheckError { message: err.to_string()})?;
        debug!("GET {}: {} - {}",  url, status, response_body);

        HealthCheckStatus::from_string(response_body)
            .map_err(|err| HealthCheckError { message: err.to_string()})
    }
}