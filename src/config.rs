use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub rpc_url: String,
    pub contract_address: String,
    pub supported_languages: Vec<String>,
    pub operator_address: String,
}

impl Config {
    pub fn new(rpc_url: &str, contract_address: &str, operator_address: &str) -> Self {
        Self {
            rpc_url: rpc_url.to_string(),
            contract_address: contract_address.to_string(),
            supported_languages: vec!["js".to_string(), "go".to_string()],
            operator_address: operator_address.to_string(),
        }
    }

    pub fn is_language_supported(&self, lang: &str) -> bool {
        self.supported_languages.contains(&lang.to_string())
    }
} 