//! Streaming TTS example.
//!
//! This example demonstrates the streaming TTS API that reduces
//! boilerplate code compared to the manual stream processing.
//!
//! Run with: cargo run --example streaming_tts

use {
    futures::StreamExt,
    mimo_api::{Client, Voice},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable XIAOMI_API_KEY
    let client = Client::from_env()?;

    println!("Simplified Streaming TTS Example");
    println!("=================================\n");

    // Example 1: Basic streaming with collect() method
    println!("1. Basic streaming with collect()");
    println!("----------------------------------");
    println!("Synthesizing: Hello, this is a simple streaming TTS example.\n");

    let mut stream = client
        .tts_stream("Hello, this is a simple streaming TTS example.")
        .voice(Voice::DefaultEn)
        .send()
        .await?;

    // Collect all audio chunks into a single byte vector
    let audio_bytes = stream.collect_audio().await?;

    println!("Total bytes collected: {}", audio_bytes.len());
    println!("Total chunks: {}", stream.chunk_count());

    // Save to file
    tokio::fs::write("output_simple_1.pcm", &audio_bytes).await?;
    println!("Saved to: output_simple_1.pcm\n");

    // Example 2: Streaming with save_to_file() method
    println!("2. Streaming with save_to_file()");
    println!("--------------------------------");

    let text = "The save_to_file method makes it even easier to save streaming audio to a file.";

    println!("Synthesizing: {}\n", text);

    let mut stream = client
        .tts_stream(text)
        .voice(Voice::DefaultEn)
        .send()
        .await?;

    // Save directly to file with one method call
    stream.save_to_file("output_simple_2.pcm").await?;

    println!("Total bytes: {}", stream.total_bytes());
    println!("Total chunks: {}", stream.chunk_count());
    println!("Saved to: output_simple_2.pcm\n");

    // Example 3: Streaming with manual iteration
    println!("3. Streaming with manual iteration");
    println!("----------------------------------");

    let text = "You can also iterate over chunks manually for more control.";

    println!("Synthesizing: {}\n", text);

    let mut stream = client
        .tts_stream(text)
        .voice(Voice::DefaultEn)
        .send()
        .await?;

    let mut total_bytes = 0u64;
    let mut chunk_count = 0u32;

    // Use standard StreamExt to iterate
    while let Some(result) = stream.next().await {
        match result {
            Ok(audio_bytes) => {
                total_bytes += audio_bytes.len() as u64;
                chunk_count += 1;

                println!(
                    "  Received chunk {}: {} bytes",
                    chunk_count,
                    audio_bytes.len()
                );
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    println!("\nTotal bytes: {}", total_bytes);
    println!("Total chunks: {}", chunk_count);
    println!(
        "Final stats - Total: {}, Chunks: {}",
        stream.total_bytes(),
        stream.chunk_count()
    );

    Ok(())
}
