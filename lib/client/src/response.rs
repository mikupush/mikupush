use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

impl ErrorResponse {
    pub async fn from_response(response: Response) -> Result<ErrorResponse, Box<dyn Error>> {
        let response_body = response.text().await?;
        Ok(serde_json::from_str(&response_body)?)
    }
}

pub enum HealthCheckStatus {
    Up,
    Down,
}

impl HealthCheckStatus {
    pub async fn from_response(response: Response) -> Result<HealthCheckStatus, Box<dyn Error>> {
        let response_body = response.text().await?;
        match response_body.as_str() {
            "up" => Ok(HealthCheckStatus::Up),
            "down" => Ok(HealthCheckStatus::Down),
            _ => Err(format!("Invalid health check status: {}", response_body).into()),
        }
    }
}
