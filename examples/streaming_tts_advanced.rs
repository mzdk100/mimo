//! Advanced streaming TTS example with progress tracking.
//!
//! This example demonstrates a more sophisticated streaming TTS usage with
//! progress tracking, real-time statistics, and multiple streaming techniques.
//!
//! Run with: cargo run --example streaming_tts_advanced

use {
    futures::StreamExt,
    mimo_api::{Client, StreamingTtsResponse, TtsStyle, Voice},
    std::time::Instant,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;

    println!("=== Advanced Streaming TTS Example ===\n");

    // Example 1: Basic streaming with progress tracking
    println!("1. Basic streaming with progress tracking");
    println!("----------------------------------------");
    basic_streaming(&client).await?;
    println!();

    // Example 2: Streaming with style control
    println!("\n2. Streaming with style control");
    println!("--------------------------------");
    styled_streaming(&client).await?;
    println!();

    // Example 3: Streaming with user context
    println!("\n3. Streaming with user context");
    println!("-------------------------------");
    context_streaming(&client).await?;
    println!();

    println!("\n=== All examples completed! ===");

    Ok(())
}

/// Basic streaming TTS with progress tracking
async fn basic_streaming(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let text = "This is a demonstration of streaming text-to-speech. \
        The audio data is delivered in real-time chunks, allowing for \
        immediate playback as soon as the first segment arrives.";

    println!("Text: \"{}\"\n", text);

    let mut stream: StreamingTtsResponse = client
        .tts_stream(text)
        .voice(Voice::DefaultEn)
        .send()
        .await?;

    let start_time = Instant::now();
    let mut total_bytes = 0u64;
    let mut chunk_count = 0;

    println!("  Receiving chunks with progress tracking:");

    while let Some(result) = stream.next().await {
        match result {
            Ok(bytes) => {
                let len: usize = bytes.len();
                total_bytes += len as u64;
                chunk_count += 1;

                // Show progress
                let elapsed = start_time.elapsed().as_secs_f64();
                let rate = if elapsed > 0.0 {
                    (total_bytes as f64 / 1024.0) / elapsed
                } else {
                    0.0
                };

                println!(
                    "  ✓ Chunk {}: {} bytes | Total: {} bytes | Rate: {:.2} KB/s",
                    chunk_count, len, total_bytes, rate
                );
            }
            Err(e) => eprintln!("  ✗ Error: {}", e),
        }
    }

    println!(
        "\n  Total: {} chunks, {} bytes in {:.2}s",
        chunk_count,
        total_bytes,
        start_time.elapsed().as_secs_f64()
    );
    println!(
        "  Final stats - Total: {}, Chunks: {}",
        stream.total_bytes(),
        stream.chunk_count()
    );

    Ok(())
}

/// Streaming TTS with style control
async fn styled_streaming(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let text = "Welcome to this happy and cheerful streaming synthesis! \
        The style tags help control the emotion and delivery of the speech.";

    println!("Text: \"{}\"\n", text);

    // Apply style to the text
    let styled_text = TtsStyle::new()
        .with_style("开心")
        .with_style("变快")
        .apply(text);

    let mut stream: StreamingTtsResponse = client
        .tts_stream(styled_text)
        .voice(Voice::DefaultZh)
        .send()
        .await?;

    let mut total_bytes = 0u64;
    let mut chunk_count = 0;

    println!("  Receiving chunks (with '开心' + '变快' styles):");

    while let Some(result) = stream.next().await {
        match result {
            Ok(bytes) => {
                let len: usize = bytes.len();
                total_bytes += len as u64;
                chunk_count += 1;

                print!(".");
                if chunk_count % 10 == 0 {
                    print!(" [{}KB]", total_bytes / 1024);
                }
            }
            Err(e) => eprintln!("  ✗ Error: {}", e),
        }
    }

    println!();
    println!("  Total: {} chunks, {} bytes", chunk_count, total_bytes);

    Ok(())
}

/// Streaming TTS with user context
async fn context_streaming(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let text = "This speech is synthesized with user context. The user message \
        can influence the tone and style of the output audio.";

    let user_msg = "Speak in a friendly, conversational tone like you're \
        talking to a close friend.";

    println!("User context: \"{}\"", user_msg);
    println!("Text: \"{}\"\n", text);

    let mut stream: StreamingTtsResponse = client
        .tts_stream(text)
        .user_message(user_msg)
        .voice(Voice::MimoDefault)
        .send()
        .await?;

    let mut total_bytes = 0u64;
    let mut chunk_count = 0;

    println!("  Receiving chunks (with user context):");

    while let Some(result) = stream.next().await {
        match result {
            Ok(bytes) => {
                let len: usize = bytes.len();
                total_bytes += len as u64;
                chunk_count += 1;

                // Show a simple progress bar
                let progress = if chunk_count % 5 == 0 {
                    "#".repeat((chunk_count as usize / 5).min(20))
                } else {
                    "".to_string()
                };

                if !progress.is_empty() {
                    let spaces = " ".repeat(20_usize.saturating_sub(progress.len()));
                    print!("\r  Progress: [{}{}]", progress, spaces);
                }
            }
            Err(e) => eprintln!("  ✗ Error: {}", e),
        }
    }

    println!("\r  Progress: [####################]");
    println!();
    println!("  Total: {} chunks, {} bytes", chunk_count, total_bytes);

    Ok(())
}
