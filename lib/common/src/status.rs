use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Pending,
    InProgress,
    Completed,
    Failed,
    Aborted,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Status::Pending => "pending".to_string(),
            Status::InProgress => "inProgress".to_string(),
            Status::Completed => "completed".to_string(),
            Status::Failed => "failed".to_string(),
            Status::Aborted => "aborted".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<String> for Status {
    fn from(s: String) -> Self {
        match s.as_str() {
            "pending" => Status::Pending,
            "inProgress" => Status::InProgress,
            "completed" => Status::Completed,
            "failed" => Status::Failed,
            "aborted" => Status::Aborted,
            _ => Status::Failed,
        }
    }
}