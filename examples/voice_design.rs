//! Voice design example.
//!
//! This example demonstrates how to use the voice design feature to create
//! custom voices using text descriptions.
//!
//! Note: Voice design model does not support the `audio.voice` parameter.
//! The voice is designed purely through the user_message description.
//!
//! Run with: cargo run --example voice_design

use {mimo_api::Client, tokio::fs::write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable XIAOMI_API_KEY
    let client = Client::from_env()?;

    println!("Voice Design Example");
    println!("====================\n");

    // Example 1: Design a young male voice
    println!("1. Designing a young male voice");
    println!("---------------------------------");
    println!("Description: Give me a young male tone, energetic and friendly.");

    let response = client
        .v25_tts_voice_design("Hello, I'm a customized AI assistant.")
        .user_message("Give me a young male tone, energetic and friendly.")
        .send()
        .await?;

    let audio = response.audio()?;
    println!("Audio ID: {}", audio.id);

    if let Some(transcript) = audio.transcript() {
        println!("Transcript: {}", transcript);
    }

    let audio_bytes = audio.decode_data()?;
    let output_path = "output_voice_design_male.wav";
    write(output_path, &audio_bytes).await?;
    println!("Audio saved to: {}", output_path);
    println!("File size: {} bytes\n", audio_bytes.len());

    // Example 2: Design a soft female voice
    println!("2. Designing a soft female voice");
    println!("---------------------------------");
    println!("Description: Create a soft, gentle female voice, warm and professional.");

    let response = client
        .v25_tts_voice_design("Welcome to our service center, how can I help you today?")
        .user_message("Create a soft, gentle female voice, warm and professional.")
        .format(mimo_api::AudioFormat::Mp3) // Output as MP3
        .send()
        .await?;

    let audio = response.audio()?;
    let audio_bytes = audio.decode_data()?;
    let output_path = "output_voice_design_female.mp3";
    write(output_path, &audio_bytes).await?;
    println!("Audio saved to: {}", output_path);
    println!("File size: {} bytes\n", audio_bytes.len());

    // Example 3: Design a voice with specific characteristics for storytelling
    println!("3. Designing a voice with specific characteristics");
    println!("---------------------------------------------------");
    println!("Description: I need a mature male voice, deep and steady, perfect for storytelling.");

    let response = client
        .v25_tts_voice_design("This is a custom voice designed for audiobook narration.")
        .user_message("I need a mature male voice, deep and steady, perfect for storytelling.")
        .send()
        .await?;

    let audio = response.audio()?;
    let audio_bytes = audio.decode_data()?;
    let output_path = "output_voice_design_story.mp3";
    write(output_path, &audio_bytes).await?;
    println!("Audio saved to: {}", output_path);
    println!("File size: {} bytes\n", audio_bytes.len());

    // Example 4: Design a voice using Chinese description (anime style)
    println!("4. Designing a voice using Chinese description (anime style)");
    println!("----------------------------------------------------------------");
    println!("Description: 请给我一个可爱的少女音，清脆明亮，适合配音动漫角色。");

    let response = client
        .v25_tts_voice_design("你好，我是一个由人工智能生成的自定义音色。")
        .user_message("请给我一个可爱的少女音，清脆明亮，适合配音动漫角色。")
        .send()
        .await?;

    let audio = response.audio()?;
    let audio_bytes = audio.decode_data()?;
    let output_path = "output_voice_design_anime.wav";
    write(output_path, &audio_bytes).await?;
    println!("Audio saved to: {}", output_path);
    println!("File size: {} bytes\n", audio_bytes.len());

    // Example 5: Design a child voice
    println!("5. Designing a child voice");
    println!("---------------------------");
    println!("Description: Create a cute child voice, about 8 years old, cheerful and innocent.");

    let response = client
        .v25_tts_voice_design("Hello! I'm so excited to tell you about my day!")
        .user_message("Create a cute child voice, about 8 years old, cheerful and innocent.")
        .send()
        .await?;

    let audio = response.audio()?;
    let audio_bytes = audio.decode_data()?;
    let output_path = "output_voice_design_child.wav";
    write(output_path, &audio_bytes).await?;
    println!("Audio saved to: {}", output_path);
    println!("File size: {} bytes\n", audio_bytes.len());

    println!("All voice design examples completed successfully!");

    Ok(())
}
