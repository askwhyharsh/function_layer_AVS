// mod function_layer;
use function_layer::config::Config;
use function_layer::listener::EventListener;
use std::error::Error;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok(); // Load .env file at start
    // Initialize configuration
    let config = Config::new(
        &std::env::var("RPC_URL").expect("ETH_NODE_URL must be set in .env"),
        &std::env::var("EXECUTOR_CONTRACT").expect("CONTRACT_ADDRESS must be set in .env"),
        &std::env::var("OPERATOR_ADDRESS").expect("OPERATOR_ADDRESS must be set in .env")
    );

    println!("Starting compute request listener...");
    println!("Supported languages: {:?}", config.supported_languages);
    println!("Operator address: {}", config.operator_address);
    let mut listener = EventListener::new(config).await?;
    // Start listening for events
    println!("Listening for compute requests...");
    listener.start_listening().await?;
    Ok(())
}


// use function_layer::js_executor::JsExecutor;
// use function_layer::executor::CodeModule;

// #[tokio::main]
// async fn main() -> eyre::Result<()> {
//     // Create a simple JavaScript function that adds two numbers
//     let code_module = CodeModule {
//         imports: String::new(), // No imports needed for this simple example
//         function: String::from("() => { return 5 + 3; }"),
//     };

//     // Create executor and run the code
//     let executor = JsExecutor::new();
//     let result = executor.execute(&code_module).await?;
    
//     println!("Result: {:?}", result); // Should print Result: Uint(8)
    
//     Ok(())
// }