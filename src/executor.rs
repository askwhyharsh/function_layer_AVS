use crate::config::Config;
use ethers::types::U256;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct CodeModule {
    function: String,
    imports: String,
}

pub struct Executor {
    config: Config,
    contract_address: String,
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
        code_json: String,
        res_count: U256,
    ) -> Result<()> {
        // Check if language is supported
        if !self.config.is_language_supported(&code_lang) {
            return Err(eyre::eyre!("Unsupported language: {}", code_lang));
        }

        println!("Request ID: {:?}", request_id);
        println!("Code language: {}", code_lang);
        println!("Code JSON: {}", code_json);
        println!("Result count: {:?}", res_count);

        // Check current submission count from contract using request_id
        let current_count = self.get_submission_count(request_id).await?;
        if current_count >= res_count.as_u64() {
            return Ok(());
        }
        return Ok(());
        // Parse code module
        let code_module: CodeModule = serde_json::from_str(&code_json)?;
        println!("Code module: {:?}", code_module);

        // Execute based on language
        let result = match code_lang.as_str() {
            "js" => {
                self.execute_js(&code_module).await?
            }
            "go" => {
                self.execute_go(&code_module).await?
            }
            _ => return Err(eyre::eyre!("Unsupported language")),
        };

        // Submit result to contract
        self.submit_result(result).await?;

        Ok(())
    }

    async fn get_submission_count(&self, request_id: U256) -> Result<u64> {
        // TODO: Implement contract call to get current submission count using request_id
        Ok(0)
    }

    async fn execute_js(&self, code_module: &CodeModule) -> Result<ExecutionResult> {
        // TODO: Implement JS execution handler
        todo!()
    }

    async fn execute_go(&self, code_module: &CodeModule) -> Result<ExecutionResult> {
        // TODO: Implement Go execution handler
        todo!()
    }

    async fn submit_result(&self, result: ExecutionResult) -> Result<()> {
        // TODO: Implement contract submission
        Ok(())
    }
}

#[derive(Debug)]
enum ExecutionResult {
    UintArray(Vec<U256>),
    BoolArray(Vec<bool>),
    Uint(U256),
    Bool(bool),
    Bytes(Vec<u8>),
}
