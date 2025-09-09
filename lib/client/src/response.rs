use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::error::Error;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn from_string(content: String) -> Result<ErrorResponse, Box<dyn Error>> {
        Ok(serde_json::from_str(&content)?)
    }
}

pub enum HealthCheckStatus {
    Up,
    Down,
}

impl HealthCheckStatus {
    pub fn from_string(content: String) -> Result<HealthCheckStatus, Box<dyn Error>> {
        let json: Value = serde_json::from_str(&content)?;
        let status = json["status"].as_str().unwrap_or("down");

        match status {
            "up" => Ok(HealthCheckStatus::Up),
            "down" => Ok(HealthCheckStatus::Down),
            _ => Err(format!("Invalid health check status: {}", status).into()),
        }
    }
}
