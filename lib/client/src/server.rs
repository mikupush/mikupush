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