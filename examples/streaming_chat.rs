//! Streaming chat example demonstrating real-time response streaming.
//!
//! Usage: XIAOMI_API_KEY=your_key cargo run --example streaming_chat

use futures::StreamExt;
use mimo::{ChatRequest, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable
    let client = Client::from_env()?;

    println!("Creating streaming chat request...\n");

    // Create a chat request with streaming enabled
    let request = ChatRequest::flash()
        .system("You are a creative storyteller.")
        .user("Tell me a short story about a robot learning to paint.");

    println!("Starting stream...\n");
    println!("Response:");
    println!("--------");

    // Get the stream
    let mut stream = client.chat_stream(request).await?;

    // Process chunks as they arrive
    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(chunk) => {
                // Check if choices array is not empty
                if !chunk.choices.is_empty()
                    && let Some(content) = &chunk.choices[0].delta.content
                {
                    print!("{}", content);
                }
            }
            Err(e) => {
                eprintln!("\nError: {}", e);
                break;
            }
        }
    }

    println!("\n\nStream complete!");
    Ok(())
}
