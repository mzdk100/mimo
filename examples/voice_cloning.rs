//! Voice cloning example.
//!
//! This example demonstrates how to use the voice cloning feature to clone
//! a voice from an audio sample.
//!
//! Run with: cargo run --example voice_cloning
//!
//! Note: You need to provide an audio file (MP3 or WAV) for voice cloning.
//! Place the audio file in the examples directory and update the path below.

use {
    base64::prelude::*,
    mimo_api::{AudioFormat, Client, Voice},
    tokio::fs::{read, write},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable XIAOMI_API_KEY
    let client = Client::from_env()?;

    println!("Voice Cloning Example");
    println!("=====================\n");

    // Note: Replace "voice_sample.mp3" with the path to your audio file
    let sample_path = "examples/bajie.mp3";

    println!("Loading voice sample from: {}", sample_path);

    // Create a custom voice from the audio file
    let custom_voice = Voice::from_audio_file(sample_path).await?;
    println!("Voice cloned successfully!");

    // Example 1: Clone voice and synthesize English text
    println!("\n1. Cloning voice for English synthesis");
    println!("----------------------------------------");

    let response = client
        .v25_tts_voice_clone("Hello, this is a cloned voice speaking.")
        .voice(custom_voice.clone())
        .send()
        .await?;

    let audio = response.audio()?;
    println!("Audio ID: {}", audio.id);

    let audio_bytes = audio.decode_data()?;
    let output_path = "output_voice_clone_en.wav";
    write(output_path, &audio_bytes).await?;
    println!("Audio saved to: {}", output_path);
    println!("File size: {} bytes", audio_bytes.len());

    // Example 2: Clone voice and synthesize Chinese text
    println!("\n2. Cloning voice for Chinese synthesis");
    println!("----------------------------------------");

    let response = client
        .v25_tts_voice_clone("大家好，这是一个克隆音色的语音合成测试。")
        .voice(custom_voice.clone())
        .format(AudioFormat::Mp3)
        .send()
        .await?;

    let audio = response.audio()?;
    let audio_bytes = audio.decode_data()?;
    let output_path = "output_voice_clone_zh.mp3";
    write(output_path, &audio_bytes).await?;
    println!("Audio saved to: {}", output_path);
    println!("File size: {} bytes", audio_bytes.len());

    // Example 3: Using Voice::custom() with base64 data directly
    println!("\n3. Using Voice::custom() with base64-encoded audio");
    println!("----------------------------------------------------");

    // Read and encode the audio file manually
    let audio_data = read(sample_path).await?;
    let base64_audio = BASE64_STANDARD.encode(&audio_data);
    let mime_type = if sample_path.ends_with(".mp3") {
        "audio/mpeg"
    } else {
        "audio/wav"
    };
    let voice_str = format!("data:{};base64,{}", mime_type, base64_audio);

    let custom_voice2 = Voice::custom(voice_str);

    let response = client
        .v25_tts_voice_clone("This voice was created using Voice::custom() method.")
        .voice(custom_voice2)
        .send()
        .await?;

    let audio = response.audio()?;
    let audio_bytes = audio.decode_data()?;
    let output_path = "output_voice_clone_custom.wav";
    write(output_path, &audio_bytes).await?;
    println!("Audio saved to: {}", output_path);
    println!("File size: {} bytes", audio_bytes.len());

    // Example 4: Voice cloning with user message for style adjustment
    println!("\n4. Voice cloning with style adjustment");
    println!("---------------------------------------");

    let response = client
        .v25_tts_voice_clone("This is the cloned voice with adjusted speaking style.")
        .voice(custom_voice)
        .user_message("Speak clearly and at a moderate pace.")
        .send()
        .await?;

    let audio = response.audio()?;
    let audio_bytes = audio.decode_data()?;
    let output_path = "output_voice_clone_styled.wav";
    write(output_path, &audio_bytes).await?;
    println!("Audio saved to: {}", output_path);
    println!("File size: {} bytes", audio_bytes.len());

    println!("\nAll voice cloning examples completed successfully!");

    Ok(())
}
