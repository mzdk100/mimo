//! Deep thinking mode example demonstrating chain-of-thought reasoning.
//!
//! Usage: XIAOMI_API_KEY=your_key cargo run --example thinking_mode
//!
//! Note: Thinking mode is best supported by MiMo V2 Pro model.

use mimo::{ChatRequest, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable
    let client = Client::from_env()?;

    println!("Creating thinking mode request...\n");

    // Create a chat request with thinking mode enabled
    let request = ChatRequest::pro()
        .system("You are an expert problem solver. Think through problems step by step.")
        .user(
            "If a train travels 120 km in 2 hours, then stops for 30 minutes, ".to_owned()
                + "and then travels another 90 km in 1.5 hours, what is the average speed?",
        )
        .enable_thinking()
        .max_completion_tokens(4096);

    println!("Sending request to MiMo API with thinking enabled...\n");
    let response = client.chat(request).await?;

    // Print reasoning content if available
    if let Some(reasoning) = &response.choices[0].message.reasoning_content {
        println!("Reasoning Process:");
        println!("------------------");
        println!("{}", reasoning);
        println!();
    }

    println!("Final Answer:");
    println!("-------------");
    println!("{}", response.choices[0].message.content);

    // Print usage info
    if let Some(usage) = response.usage {
        println!();
        if let Some(details) = usage.completion_tokens_details {
            println!("Reasoning tokens: {}", details.reasoning_tokens);
        }
        println!("Total tokens: {}", usage.total_tokens);
    }

    Ok(())
}
