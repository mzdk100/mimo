//! Basic text-to-speech example.
//!
//! This example demonstrates how to use the TTS (text-to-speech) API to convert
//! text to audio.
//!
//! Run with: cargo run --example tts_basic

use {
    mimo_api::{Client, Voice},
    tokio::fs::write,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable XIAOMI_API_KEY
    let client = Client::from_env()?;

    println!("Synthesizing speech with default voice...");

    // Create a TTS request
    let response = client
        .tts("Hello, I am MiMo, your AI assistant. How can I help you today?")
        .voice(Voice::MimoDefault)
        .send()
        .await?;

    // Get the audio data
    let audio = response.audio()?;
    println!("Audio ID: {}", audio.id);

    if let Some(transcript) = audio.transcript() {
        println!("Transcript: {}", transcript);
    }

    // Decode and save the audio
    let audio_bytes = audio.decode_data()?;
    let output_path = "output_tts.wav";
    write(output_path, &audio_bytes).await?;
    println!("Audio saved to: {}", output_path);
    println!("File size: {} bytes", audio_bytes.len());

    Ok(())
}
