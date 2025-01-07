use eyre::Result;
use crate::config::Config;
use crate::executor::Executor;
// use ethers::abi::AbiDecode;
use ethers::providers::Provider;
use ethers::types::{H256, U256, U64};
use ethers::prelude::*;
use ethers::abi::RawLog;
pub struct EventListener {
    provider: Provider<Http>,
    contract_address: String,
    executor: Executor,
    last_processed_block: U64,
}

#[derive(Debug, Clone, EthEvent)]
#[ethevent(abi = "ComputeRequestCreated(uint256,string,string,uint256)")]
pub struct ComputeRequestCreated {
    #[ethevent(indexed, name = "taskIndex")]
    pub request_id: U256,
    #[ethevent(name = "codeArweaveUri")]
    pub code_json: String,
    #[ethevent(name = "language")]
    pub code_lang: String,
    #[ethevent(name = "responseCount")]
    pub node_count: U256,
}

#[derive(Debug)]
pub struct ComputeRequest {
    pub request_id: U256,
    pub code_json: String,
    pub code_lang: String,
    pub node_count: U256,
}

impl EventListener {
    pub async fn new(config1: Config) -> Result<Self> {
        let provider = Provider::<Http>::connect(&config1.rpc_url).await;
        let contract_address = config1.contract_address.clone();
        
        let executor = Executor::new(config1);
        Ok(Self { 
            provider,
            contract_address,
            executor,
            last_processed_block: U64::zero(),
        })
    }

    pub async fn start_listening(&mut self) -> Result<()> {
        let address: Address = self.contract_address.parse()?;
        let event_signature = "ComputeRequestCreated(uint256,string,string,uint256)";
        let topic = H256::from(ethers::utils::keccak256(event_signature.as_bytes()));

        self.last_processed_block = self.provider.get_block_number().await?;
        println!("Starting to listen from block: {}", self.last_processed_block);

        loop {
            let current_block = self.provider.get_block_number().await?;
            
            if current_block > self.last_processed_block {
                let filter = Filter::new()
                    .address(address)
                    .topic0(topic)
                    .from_block(self.last_processed_block + 1)
                    .to_block(current_block);

                if let Ok(logs) = self.provider.get_logs(&filter).await {
                    for log in logs {
                        println!("\nNew ComputeRequestCreated event detected!");
                        println!("Transaction hash: {:?}", log.transaction_hash);
                        
                        if let Ok(compute_request) = self.parse_compute_request_event(&log) {
                            println!("Parsed Compute Request:");
                            println!("Code JSON: {}", compute_request.code_json);
                            println!("Code Language: {}", compute_request.code_lang);
                            println!("Node Count: {}", compute_request.node_count);

                            // Execute the compute request
                            match self.executor.execute(
                                compute_request.request_id,
                                compute_request.code_lang.to_string(),
                                compute_request.code_json,
                                compute_request.node_count,
                            ).await {
                                Ok(_) => {
                                    println!("Execution completed successfully for request ID: {}", compute_request.request_id);
                                    // send a deliver smart contract call
                                },
                                Err(e) => {
                                    println!("Execution failed: {:?}", e);
                                }
                            }
                        } else {
                            println!("Failed to parse compute request event");
                        }
                    }
                }
                
                self.last_processed_block = current_block;
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    fn parse_compute_request_event(&self, log: &Log) -> Result<ComputeRequest> {
        // Attempt to decode and handle errors explicitly
        match <ComputeRequestCreated as EthEvent>::decode_log(&RawLog::from(log.clone())) {
            Ok(event) => {
                println!("Successfully decoded event: {:?}", event);
                Ok(ComputeRequest {
                    code_json: event.code_json,
                    code_lang: event.code_lang,
                    node_count: event.node_count,
                    request_id: event.request_id
                })
            },
            Err(e) => {
                println!("Failed to decode event: {:?}", e);
                Err(eyre::eyre!("Failed to decode event: {}", e))
            }
        }
    }
}
