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
