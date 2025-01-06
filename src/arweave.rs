use reqwest;
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArweaveError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub struct ArweaveClient {
    gateway_url: String,
}

impl ArweaveClient {
    pub fn new(gateway_url: Option<String>) -> Self {
        Self {
            gateway_url: gateway_url.unwrap_or_else(|| "https://arweave.net".to_string()),
        }
    }

    pub async fn get_transaction_data(&self, tx_id: &str) -> Result<Value, ArweaveError> {
        let url = format!("{}/{}", self.gateway_url, tx_id);
        
        let response = reqwest::get(&url).await?;
        let json: Value = response.json().await?;
        
        Ok(json)
    }
}