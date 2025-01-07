use eyre::Result;
use crate::config::Config;
use crate::executor::Executor;
// use ethers::abi::AbiDecode;
use ethers::providers::Provider;
use ethers::types::{H256, U256, U64};
use ethers::prelude::*;
use ethers::abi::RawLog;
use colored::*;
use log::{info, error, warn};
pub struct EventListener {
    provider: Provider<Http>,
    contract_address: String,
    executor: Executor,
    last_processed_block: U64,
    // task_created_block: U256,
}

#[derive(Debug, Clone, EthEvent)]
#[ethevent(abi = "ComputeRequestCreated(uint256,string,string,uint256,uint256)")]
pub struct ComputeRequestCreated {
    #[ethevent(indexed, name = "taskIndex")]
    pub request_id: U256,
    #[ethevent(name = "codeArweaveUri")]
    pub code_json: String,
    #[ethevent(name = "language")]
    pub code_lang: String,
    #[ethevent(name = "responseCount")]
    pub node_count: U256,
    #[ethevent(name = "taskCreatedBlock")]
    pub task_created_block: U256,
}

#[derive(Debug)]
pub struct ComputeRequest {
    pub request_id: U256,
    pub code_json: String,
    pub code_lang: String,
    pub node_count: U256,
    pub task_created_block: U256,
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
            // task_created_block: U256::zero(),
        })
    }

    pub async fn start_listening(&mut self) -> Result<()> {
        let address: Address = self.contract_address.parse()?;
        let event_signature = "ComputeRequestCreated(uint256,string,string,uint256,uint256)";
        let topic = H256::from(ethers::utils::keccak256(event_signature.as_bytes()));

        match self.provider.get_block_number().await {
            Ok(block) => {
                self.last_processed_block = block;
                println!("Starting to listen from block: {}", self.last_processed_block.to_string().cyan());
            },
            Err(e) => {
                error!("{}", format!("Failed to get initial block number: {:?}", e).red());
                return Err(eyre::eyre!("Failed to get initial block number: {}", e));
            }
        }

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
                        info!("{}", "\nNew ComputeRequestCreated event detected!".green().bold());
                        info!("Transaction hash: {}", format!("{:?}", log.transaction_hash).cyan());
                        
                        if let Ok(compute_request) = self.parse_compute_request_event(&log) {
                            // info!("{}", "Parsed Compute Request:".yellow());
                            info!("Code Ar Txn Id: {}", compute_request.code_json.cyan());
                            info!("Code Language: {}", compute_request.code_lang.cyan());
                            info!("Node Count: {}", compute_request.node_count.to_string().cyan());
                            info!("Task Created Block: {}", compute_request.task_created_block.to_string().cyan());

                            match self.executor.execute(
                                compute_request.request_id,
                                compute_request.code_lang.to_string(),
                                compute_request.code_json,
                                compute_request.node_count,
                                compute_request.task_created_block,
                            ).await {
                                Ok(_) => {
                                    info!("{}", format!("✓ Execution completed successfully for request ID: {}", 
                                        compute_request.request_id).green());
                                },
                                Err(e) => {
                                    error!("{}", format!("✗ Execution failed: {:?}", e).red());
                                    warn!("{}", "continuing...".yellow());
                                }
                            }
                        } else {
                            error!("{}", "Failed to parse compute request event".red());
                        }
                    }
                }
                
                self.last_processed_block = current_block;
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    fn parse_compute_request_event(&self, log: &Log) -> Result<ComputeRequest> {
        match <ComputeRequestCreated as EthEvent>::decode_log(&RawLog::from(log.clone())) {
            Ok(event) => {
                info!("{}", "Successfully decoded event:".green());
                info!("{:?}", event);
                Ok(ComputeRequest {
                    code_json: event.code_json,
                    code_lang: event.code_lang,
                    node_count: event.node_count,
                    request_id: event.request_id,
                    task_created_block: event.task_created_block,
                })
            },
            Err(e) => {
                error!("{}", format!("Failed to decode event: {:?}", e).red());
                Err(eyre::eyre!("Failed to decode event: {}", e))
            }
        }
    }
}
