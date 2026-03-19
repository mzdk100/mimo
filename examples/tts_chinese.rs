//! Chinese text-to-speech example with style control.
//!
//! This example demonstrates TTS with Chinese voice and style control.
//!
//! Run with: cargo run --example tts_chinese

use {
    mimo::{Client, Voice},
    tokio::fs::write,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;

    // Example 1: Normal Chinese TTS
    println!("Example 1: Normal Chinese TTS");
    let response = client
        .tts("你好，我是MiMo，很高兴认识你！")
        .voice(Voice::DefaultZh)
        .send()
        .await?;

    let audio = response.audio()?;
    let audio_bytes = audio.decode_data()?;
    write("output_chinese.wav", &audio_bytes).await?;
    println!("Saved: output_chinese.wav");

    // Example 2: TTS with happy style
    println!("\nExample 2: TTS with happy style");
    let response = client
        .tts_styled("开心", "明天就是周五了，真开心！周末可以好好休息了。")
        .voice(Voice::DefaultZh)
        .send()
        .await?;

    let audio = response.audio()?;
    let audio_bytes = audio.decode_data()?;
    write("output_happy.wav", &audio_bytes).await?;
    println!("Saved: output_happy.wav");

    // Example 3: TTS with Northeastern dialect
    println!("\nExample 3: TTS with Northeastern dialect");
    let response = client
        .tts_styled("东北话", "哎呀妈呀，这天儿也忒冷了吧！")
        .voice(Voice::DefaultZh)
        .send()
        .await?;

    let audio = response.audio()?;
    let audio_bytes = audio.decode_data()?;
    write("output_dialect.wav", &audio_bytes).await?;
    println!("Saved: output_dialect.wav");

    // Example 4: TTS with whisper style
    println!("\nExample 4: TTS with whisper style");
    let response = client
        .tts_styled("悄悄话", "这是一个秘密，不要告诉别人哦。")
        .voice(Voice::DefaultZh)
        .send()
        .await?;

    let audio = response.audio()?;
    let audio_bytes = audio.decode_data()?;
    write("output_whisper.wav", &audio_bytes).await?;
    println!("Saved: output_whisper.wav");

    println!("\nAll TTS examples completed successfully!");

    Ok(())
}
