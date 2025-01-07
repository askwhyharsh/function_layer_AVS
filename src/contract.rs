use ethers::{
    prelude::*,
    providers::{Provider, Http},
    types::{Bytes, U256},
    contract::Contract,
};
use serde::{Deserialize, Serialize};
use ethers::abi::Abi;
use ethers::middleware::SignerMiddleware;
use ethers::signers::Wallet;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task(pub String, pub u32);

pub struct ContractClient {
    contract: Contract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
}

impl ContractClient {
    pub async fn new(
        address: &str,
        rpc_url: &str,
        abi: &str,
        private_key: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let provider = Provider::<Http>::connect(rpc_url).await;
        let wallet: LocalWallet = private_key.parse()?;
        
        // Create a SignerMiddleware
        let client = SignerMiddleware::new(provider, wallet);
        let client = Arc::new(client);

        let address: Address = address.parse()?;
        let abi: Abi = serde_json::from_str(abi)?;
        let contract = Contract::new(address, abi, client);
        
        Ok(Self { contract })
    }

    // pub async fn get_event_stream<T: EthEvent>(&self) -> Result<EventStream<T>, Box<dyn std::error::Error>> {
    //     // Implementation for event streaming
    //     todo!()
    // }

    pub fn contract(&self) -> &Contract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>> {
        &self.contract
    }
}
