use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub base_url: String,
    pub name: String,
}

impl Default for Server {
    fn default() -> Self {
        #[cfg(debug_assertions)]
        let base_url = "http://localhost:8080".to_string();

        #[cfg(not(debug_assertions))]
        let base_url = "https://mikupush.io".to_string();

        Self {
            base_url,
            name: "mikupush.io".to_string(),
        }
    }
}