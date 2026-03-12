use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LlmMessage {
    pub role: String,
    pub content: String,
}

#[derive(Clone)]
pub struct LlmConfig {
    pub provider: String,
    pub api_key: String,
    pub api_url: String,
    pub model: String,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: "none".to_string(),
            api_key: String::new(),
            api_url: String::new(),
            model: String::new(),
        }
    }
}

pub struct AppState {
    pub llm_config: Mutex<LlmConfig>,
    pub http_client: Client,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            llm_config: Mutex::new(LlmConfig::default()),
            http_client: Client::new(),
        }
    }
}
