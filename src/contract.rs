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
    provider: Provider<Http>,
}

impl ContractClient {
    pub async fn new(
        contract_address: &str,
        rpc_url: &str,
        abi: &str,
        private_key: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let chain_id = provider.get_chainid().await?.as_u64();
        let wallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
        
        // Create a SignerMiddleware
        let client = SignerMiddleware::new(provider.clone(), wallet);
        let client = Arc::new(client);

        let address: Address = contract_address.parse()?;
        let abi: Abi = serde_json::from_str(abi)?;
        let contract = Contract::new(address, abi, client);
        
        Ok(Self { contract, provider: provider.clone() })
    }

    // pub async fn get_event_stream<T: EthEvent>(&self) -> Result<EventStream<T>, Box<dyn std::error::Error>> {
    //     // Implementation for event streaming
    //     todo!()
    // }

    pub fn contract(&self) -> &Contract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>> {
        &self.contract
    }

    pub fn provider(&self) -> &Provider<Http> {
        &self.provider  // Assuming you have a provider field in your struct
    }
}
