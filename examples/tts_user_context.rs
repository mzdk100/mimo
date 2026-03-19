//! TTS with user context example.
//!
//! This example shows how to use a user message to influence the TTS output.
//!
//! Run with: cargo run --example tts_user_context

use {
    mimo::{Client, Voice},
    tokio::fs::write,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;

    // The user message can help set the context for the TTS
    println!("TTS with user context example");
    println!("==============================\n");

    // Example: User asks a question, assistant responds with TTS
    let response = client
        .tts("是的，我吃过午饭了。今天吃了一个三明治，味道还不错。")
        .voice(Voice::DefaultZh)
        .user_message("你好，MiMo，你吃午饭了吗？")
        .send()
        .await?;

    let audio = response.audio()?;
    let audio_bytes = audio.decode_data()?;
    write("output_with_context.wav", &audio_bytes).await?;

    println!("Generated audio with user context.");
    println!("User asked: 你好，MiMo，你吃午饭了吗？");
    println!("Assistant responded: 是的，我吃过午饭了。今天吃了一个三明治，味道还不错。");
    println!("Saved to: output_with_context.wav");

    Ok(())
}
