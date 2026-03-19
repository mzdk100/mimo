//! Basic chat example demonstrating simple conversation with MiMo API.
//!
//! Usage: XIAOMI_API_KEY=your_key cargo run --example basic_chat

use mimo_api::{ChatRequest, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable
    let client = Client::from_env()?;

    println!("Creating chat request...\n");

    // Create a simple chat request
    let request = ChatRequest::flash()
        .system("You are a helpful assistant. Be concise.")
        .user("What is the capital of France?");

    // Send the request
    println!("Sending request to MiMo API...\n");
    let response = client.chat(request).await?;

    // Print the response
    println!("Response:");
    println!("--------");
    println!("{}", response.choices[0].message.content);
    println!();

    // Print usage info
    if let Some(usage) = response.usage {
        println!("Token Usage:");
        println!("  Prompt tokens: {}", usage.prompt_tokens);
        println!("  Completion tokens: {}", usage.completion_tokens);
        println!("  Total tokens: {}", usage.total_tokens);
    }

    Ok(())
}
