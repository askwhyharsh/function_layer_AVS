use ethers::{
    prelude::*,
    types::{Bytes, U256},
    utils::keccak256,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ethers::abi::Token;
use tokio;
use crate::contract::ContractClient; // Assuming these exist in contract.rs
use std::fs;
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub code_arweave_uri: String,
    pub language: String,
    pub response_count: U256,
    pub task_created_block: u32,
    pub request_id: u32,
}

pub async fn respond_to_task(
    contract_address: &str,
    rpc_url: &str,
    task: Task,
    response_string: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Read ABI from file
    let abi = fs::read_to_string("abi/abi.json")?;
    
    // Read private key from environment
    let private_key = env::var("OPERATOR_PRIVATE_KEY")
        .expect("OPERATOR_PRIVATE_KEY must be set in environment");

    // Create new contract client for this response
    let contract_client = ContractClient::new(
        contract_address,
        rpc_url,
        &abi,
        &private_key,
    ).await?;

    // Create wallet for signing
    let wallet = private_key.parse::<LocalWallet>()?;

    // Create signature with correct parameters
    let encoded = ethers::abi::encode(&[
        Token::String(response_string.clone()),
        Token::String(task.code_arweave_uri.clone()),
        Token::String(task.language.clone()),
        Token::Uint(task.response_count),
    ]);
    let message_hash = keccak256(&encoded);
    let signature = wallet.sign_message(&message_hash).await?;

    // Submit response with task struct
    contract_client
        .contract()
        .method::<_, ()>(
            "respondToTask",
            (
                (
                    task.code_arweave_uri,
                    task.response_count,
                    task.language,
                    task.task_created_block,
                ),
                task.request_id,
                response_string.to_string(),
                Bytes::from(signature.to_vec())
            )
        )?
        .send()
        .await?;

    println!("Responded to task: index={}", &task.request_id);

    Ok(())
}