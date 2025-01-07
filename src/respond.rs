use crate::contract::ContractClient; // Assuming these exist in contract.rs
use ethers::abi::Token;
use ethers::{
    abi::encode_packed,
    prelude::*,
    types::{Bytes, U256},
    utils::keccak256,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub code_arweave_uri: String,
    pub language: String,
    pub response_count: U256,
    pub task_created_block: u32,
    pub request_id: u32,
}

pub async fn create_signature_for_task(
    wallet: &LocalWallet,
    response_string: &str,
    task: &Task,
) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
    let packed = encode_packed(&[
        Token::String(response_string.to_string()),
        Token::Uint(task.request_id.into()),
    ])?;

    let message_hash = keccak256(packed);

    // Remove hardcoded chain ID - use wallet's existing chain ID
    let signature = wallet.sign_message(&message_hash).await?;

    // When recovering, use the same hash
    let recovered = signature
        .recover(&message_hash[..])
        .expect("Failed to recover signer");

    // Get the wallet's address to compare
    let signer_address = wallet.address();

    assert_eq!(
        recovered, signer_address,
        "Recovered signer doesn't match wallet address"
    );

    Ok(Bytes::from(signature.to_vec()))
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
    let private_key =
        env::var("OPERATOR_PRIVATE_KEY").expect("OPERATOR_PRIVATE_KEY must be set in environment");

    // Create new contract client for this response
    let contract_client =
        ContractClient::new(contract_address, rpc_url, &abi, &private_key).await?;

    // Create wallet for signing with the correct chain ID
    let wallet = private_key
        .parse::<LocalWallet>()?
        .with_chain_id(contract_client.provider().get_chainid().await?.as_u64());

    let signature = create_signature_for_task(&wallet, &response_string, &task).await?;

    // Submit response with task struct
    let result = contract_client
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
                response_string,
                signature,
            ),
        )?
        .send()
        .await?
        .clone();

    Ok(())
}
