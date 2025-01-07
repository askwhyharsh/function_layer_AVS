use crate::executor::{CodeModule, ExecutionResult};
use ethers::types::U256;
use eyre::Result;
use std::process::Command;
use tempfile::TempDir;
use tokio::fs::write;

pub struct JsExecutor;

impl JsExecutor {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(&self, code_module: &CodeModule) -> Result<ExecutionResult> {
        // Check for .env usage in imports and function
        if code_module.imports.contains(".env") || code_module.function.contains(".env") {
            return Err(eyre::eyre!("Usage of .env is not allowed for security reasons"));
        }

        // Ensure temp/js directory exists
        std::fs::create_dir_all("temp/js")?;
        
        let temp_dir = TempDir::new_in("temp/js")?;
        let temp_path = temp_dir.path();
        // println!("Temp path: {:?}", temp_path);
        // Create package.json
        let package_json = r#"{
            "type": "module",
            "dependencies": {}
        }"#;
        write(temp_path.join("package.json"), package_json).await?;

        // Create the JS module file
        let js_code = format!(
            "{}\n\nexport const run = {}",
            code_module.imports,
            code_module.function
        );
        write(temp_path.join("index.js"), js_code).await?;

        // Create the executor script
        let executor_code = r#"
import { run } from './index.js';

async function main() {
    try {
        const result = await run();
        console.log(JSON.stringify(result));
    } catch (error) {
        console.error(error);
        process.exit(1);
    }
}

main();
"#;
        write(temp_path.join("executor.js"), executor_code).await?;

        // Install dependencies if any are specified
        if !code_module.imports.is_empty() {
            Command::new("npm")
                .arg("install")
                .current_dir(temp_path)
                .status()?;
        }

        // Execute the JS code
        let output = Command::new("node")
            .arg("executor.js")
            .current_dir(temp_path)
            .output()?; 

        println!("Output: {}", String::from_utf8_lossy(&output.stdout));

        if !output.status.success() {
            return Err(eyre::eyre!(
                "JavaScript execution failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        // Parse the output
        let result_str = String::from_utf8(output.stdout)?;
        let result: serde_json::Value = serde_json::from_str(&result_str)?;

        // Convert the result to ExecutionResult
        let execution_result = match result {
            // Arrays: Only supports arrays of numbers (converted to U256) or booleans
            serde_json::Value::Array(arr) => {
                if arr.iter().all(|v| v.is_number()) {
                    ExecutionResult::UintArray(
                        arr.iter()
                            .map(|v| U256::from(v.as_u64().unwrap()))
                            .collect(),
                    )
                } else if arr.iter().all(|v| v.is_boolean()) {
                    ExecutionResult::BoolArray(
                        arr.iter()
                            .map(|v| v.as_bool().unwrap())
                            .collect(),
                    )
                } else {
                    return Err(eyre::eyre!("Unsupported array type"));
                }
            }
            // Numbers: Only supports unsigned integers (converted to U256)
            serde_json::Value::Number(n) => ExecutionResult::Uint(U256::from(n.as_u64().ok_or_else(|| {
                eyre::eyre!("Number conversion failed")
            })?)),
            // Booleans
            serde_json::Value::Bool(b) => ExecutionResult::Bool(b),
            // Everything else is unsupported
            _ => return Err(eyre::eyre!("Unsupported return type")),
        };

        // TempDir will be automatically cleaned up when it goes out of scope
        Ok(execution_result)
    }
}
