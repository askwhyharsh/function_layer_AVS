use crate::config::Config;
use ethers::types::U256;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::env;
use crate::arweave::ArweaveClient;
use crate::js_executor::JsExecutor;
use std::sync::Arc;
use crate::contract::ContractClient;
use crate::respond;
use crate::respond::Task;
use ethers::utils::hex;

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeModule {
    pub function: String,
    pub imports: String,
}

pub struct Executor {
    pub config: Config,
    pub contract_address: String,
}

impl Executor {
    pub fn new(config: Config) -> Self {
        let contract_address = env::var("EXECUTOR_CONTRACT")
            .expect("EXECUTOR_CONTRACT must be set in environment");
        println!("Executor contract address: {}", contract_address);
        Self {
            config,
            contract_address,
        }
    }

    pub async fn execute(
        &self,
        request_id: U256,
        code_lang: String,
        code_tx_id: String,
        res_count: U256,
    ) -> Result<()> {
        // Check if language is supported
        if !self.config.is_language_supported(&code_lang) {
            return Err(eyre::eyre!("Unsupported language: {}", code_lang));
        }
 
        println!("Request ID: {:?}", request_id); // this is the request id
        println!("Code language: {}", code_lang); // this is the language of the code
        println!("Arweave tx id: {}", code_tx_id); // this is arweave tx id to the code json
        println!("Result count: {:?}", res_count); // this is the number of results we expect to get
        
        // build the task struct
        let task = Task {
            code_arweave_uri: code_tx_id,
            language: code_lang,
            response_count: res_count,
            task_created_block: 0, // You might need to get this from somewhere
            request_id: request_id.as_u32(),
        };
        // Check current submission count from contract using request_id
        let current_count = self.get_submission_count(request_id).await?;
        if current_count >= res_count.as_u64() {
            return Ok(());
        }
        // get the code json from arweave
        let arweave_client = ArweaveClient::new(None);
        let code_json = arweave_client.get_transaction_data_json(&task.code_arweave_uri).await?;
        println!("Code JSON: {:?}", code_json);
        

        // return Ok(());

        // Parse code module from the retrieved JSON
        let code_module: CodeModule = serde_json::from_value(code_json)?;
        println!("Code module: {:?}", code_module);

        // Execute based on language
        let result = match task.language.as_str() {
            "js" => {
                self.execute_js(&task, &code_module).await?
            }
            "go" => {
                self.execute_go(&task, &code_module).await?
            }
            _ => return Err(eyre::eyre!("Unsupported language")),
        };

        // Submit result to contract
        self.submit_result(&task, &result).await?;

        Ok(())
    }

    async fn get_submission_count(&self, request_id: U256) -> Result<u64> {
        // TODO: Implement contract call to get current submission count using request_id
        Ok(0)
    }

    async fn execute_js(&self, task: &Task, code_module: &CodeModule) -> Result<ExecutionResult> {
        let js_executor = JsExecutor::new();
        let result = js_executor.execute(code_module).await?;
        
        // Submit result to contract
        self.submit_result(task, &result).await?;

        Ok(result)
    }

    async fn execute_go(&self, task: &Task, code_module: &CodeModule) -> Result<ExecutionResult> {
        let result = ExecutionResult::Bool(true);
        self.submit_result(&task, &result).await?;
        Ok(result)
    }

    async fn submit_result(&self, task: &Task, result: &ExecutionResult) -> Result<()> {
        // Convert ExecutionResult to string
        let response_string = match result {
            ExecutionResult::UintArray(arr) => arr.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(","),
            ExecutionResult::BoolArray(arr) => arr.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(","),
            ExecutionResult::Uint(val) => val.to_string(),
            ExecutionResult::Bool(val) => val.to_string(),
            ExecutionResult::Bytes(bytes) => hex::encode(bytes),
        };

        // Submit the result using respond module
        respond::respond_to_task(
            &self.contract_address,
            &self.config.rpc_url,
            task.clone(),
            response_string,
        ).await.map_err(|e| eyre::eyre!("{}", e))
    }


}

#[derive(Debug)]
pub enum ExecutionResult {
    UintArray(Vec<U256>),
    BoolArray(Vec<bool>),
    Uint(U256),
    Bool(bool),
    Bytes(Vec<u8>),
}
