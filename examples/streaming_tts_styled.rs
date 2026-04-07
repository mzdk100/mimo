//! Streaming TTS with style example.
//!
//! This example demonstrates the streaming TTS API with style controls.
//!
//! Run with: cargo run --example streaming_tts_styled

use {
    futures::StreamExt,
    mimo_api::{Client, Voice},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable XIAOMI_API_KEY
    let client = Client::from_env()?;

    println!("Streaming TTS with Style Example");
    println!("=================================\n");

    // Example 1: Happy style with streaming
    println!("1. Happy style streaming");
    println!("------------------------");
    println!("Style: 开心");
    println!("Text: 明天就是周五了，真开心！\n");

    let mut stream = client
        .tts_styled_stream("开心", "明天就是周五了，真开心！")
        .voice(Voice::DefaultZh)
        .send()
        .await?;

    let mut total_bytes = 0u64;
    let mut chunk_count = 0u32;

    while let Some(result) = stream.next().await {
        match result {
            Ok(audio_bytes) => {
                let bytes_len = audio_bytes.len();
                total_bytes += bytes_len as u64;
                chunk_count += 1;
                println!("  Received chunk {}: {} bytes", chunk_count, bytes_len);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    println!("\nTotal bytes: {}", total_bytes);
    println!("Total chunks: {}", chunk_count);

    // Example 2: Fast speech style with streaming
    println!("2. Fast speech style streaming");
    println!("------------------------------");
    println!("Style: 变快");
    println!("Text: 这个示例展示了带风格控制的流式语音合成。\n");

    let mut stream = client
        .tts_styled_stream("变快", "这个示例展示了带风格控制的流式语音合成。")
        .voice(Voice::DefaultZh)
        .send()
        .await?;

    let audio_bytes = stream.collect_audio().await?;

    println!("Total bytes: {}", audio_bytes.len());
    println!("Total chunks: {}", stream.chunk_count());

    tokio::fs::write("output_fast.pcm", &audio_bytes).await?;
    println!("Saved to: output_fast.pcm\n");

    // Example 3: Multiple styles with streaming
    println!("3. Multiple styles streaming");
    println!("-----------------------------");
    println!("Style: 开心, 变快");
    println!("Text: 周末马上就要到了，真是太棒了！\n");

    let text_with_style = mimo_api::styled_text("开心", "周末马上就要到了，真是太棒了！");

    let mut stream = client
        .tts_stream(text_with_style)
        .voice(Voice::DefaultZh)
        .send()
        .await?;

    stream.save_to_file("output_happy_multi.pcm").await?;

    println!("Total bytes: {}", stream.total_bytes());
    println!("Total chunks: {}", stream.chunk_count());
    println!("Saved to: output_happy_multi.pcm\n");

    println!("All examples completed successfully!");

    Ok(())
}
