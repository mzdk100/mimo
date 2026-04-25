//! Basic text-to-speech example.
//!
//! This example demonstrates how to use the TTS (text-to-speech) API to convert
//! text to audio, showcasing all available preset voices.
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

    println!("Preset Voices Demo");
    println!("==================\n");

    // Part 1: Voices supported by mimo-v2-tts model
    println!("=== MiMo V2 TTS Model (mimo-v2-tts) ===\n");

    let v2_voices: Vec<(&str, Voice, &str)> = vec![
        (
            "MiMo Default (balanced tone)",
            Voice::MimoDefault,
            "Hello, I am MiMo, your AI assistant.",
        ),
        (
            "Default English Female",
            Voice::DefaultEn,
            "Hello, this is the default English voice.",
        ),
        (
            "Default Chinese Female",
            Voice::DefaultZh,
            "你好，这是默认中文音色。",
        ),
    ];

    for (description, voice, text) in &v2_voices {
        println!("Voice: {}", description);
        println!("-------------------------------------------");

        println!("Text: {}", text);

        // Use mimo-v2-tts model
        let response = client.tts(*text).voice(voice.clone()).send().await?;

        // Get the audio data
        let audio = response.audio()?;
        println!("Audio ID: {}", audio.id);

        if let Some(transcript) = audio.transcript() {
            println!("Transcript: {}", transcript);
        }

        // Decode and save the audio
        let audio_bytes = audio.decode_data()?;
        let output_path = format!("output_tts_{}.wav", get_voice_name(voice));
        write(&output_path, &audio_bytes).await?;
        println!("Audio saved to: {}", output_path);
        println!("File size: {} bytes\n", audio_bytes.len());
    }

    // Part 2: Voices supported by mimo-v2.5-tts model
    println!("\n=== MiMo V2.5 TTS Model (mimo-v2.5-tts) ===\n");

    let v25_voices: Vec<(&str, Voice, &str, &str)> = vec![
        // (description, voice enum, English text, Chinese text)
        (
            "冰糖 (Chinese Female)",
            Voice::Bingtang,
            "Hello, I'm Bingtang.",
            "你好，我是冰糖，一个温柔的女声。",
        ),
        (
            "茉莉 (Chinese Female)",
            Voice::Moli,
            "Hello, I'm Moli.",
            "你好，我是茉莉，声音清脆悦耳。",
        ),
        (
            "苏打 (Chinese Male)",
            Voice::Suda,
            "Hello, I'm Suda.",
            "你好，我是苏打，一个沉稳的男声。",
        ),
        (
            "白桦 (Chinese Male)",
            Voice::Baihua,
            "Hello, I'm Baihua.",
            "你好，我是白桦，声音浑厚有力。",
        ),
        (
            "Mia (English Female)",
            Voice::Mia,
            "Hello, I'm Mia. Nice to meet you!",
            "你好，我是Mia，很高兴认识你！",
        ),
        (
            "Chloe (English Female)",
            Voice::Chloe,
            "Hi there! I'm Chloe, how can I help you today?",
            "嗨，我是Chloe，今天有什么可以帮你的吗？",
        ),
        (
            "Milo (English Male)",
            Voice::Milo,
            "Hey! I'm Milo. Let's get started!",
            "嘿，我是Milo，让我们开始吧！",
        ),
        (
            "Dean (English Male)",
            Voice::Dean,
            "Hello, I'm Dean. I have a deep and steady voice.",
            "你好，我是Dean，我的声音深沉稳重。",
        ),
    ];

    for (description, voice, en_text, zh_text) in &v25_voices {
        println!("Voice: {}", description);
        println!("-------------------------------------------");

        // Use appropriate text based on voice type
        let text = if description.contains("Chinese")
            || description.contains("冰糖")
            || description.contains("茉莉")
            || description.contains("苏打")
            || description.contains("白桦")
        {
            *zh_text
        } else {
            *en_text
        };

        println!("Text: {}", text);

        // Use mimo-v2.5-tts model
        let response = client.v25_tts(text).voice(voice.clone()).send().await?;

        // Get the audio data
        let audio = response.audio()?;
        println!("Audio ID: {}", audio.id);

        if let Some(transcript) = audio.transcript() {
            println!("Transcript: {}", transcript);
        }

        // Decode and save the audio
        let audio_bytes = audio.decode_data()?;
        let output_path = format!("output_tts_{}.wav", get_voice_name(voice));
        write(&output_path, &audio_bytes).await?;
        println!("Audio saved to: {}", output_path);
        println!("File size: {} bytes\n", audio_bytes.len());
    }

    println!("\nAll preset voices demo completed!");
    println!("\nGenerated files:");
    println!("\n[mimo-v2-tts model]");
    for (description, voice, _) in &v2_voices {
        let name = get_voice_name(voice);
        println!("  - output_tts_{}.wav  ({})", name, description);
    }
    println!("\n[mimo-v2.5-tts model]");
    for (description, voice, _, _) in &v25_voices {
        let name = get_voice_name(voice);
        println!("  - output_tts_{}.wav  ({})", name, description);
    }

    Ok(())
}

/// Get the string name of a voice for file naming
fn get_voice_name(voice: &Voice) -> String {
    match voice {
        Voice::MimoDefault => "mimo_default",
        Voice::DefaultEn => "default_en",
        Voice::DefaultZh => "default_zh",
        Voice::Bingtang => "bingtang",
        Voice::Moli => "moli",
        Voice::Suda => "suda",
        Voice::Baihua => "baihua",
        Voice::Mia => "mia",
        Voice::Chloe => "chloe",
        Voice::Milo => "milo",
        Voice::Dean => "dean",
        Voice::Custom(_) => "custom",
    }
    .to_string()
}
