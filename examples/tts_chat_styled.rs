//! Text-to-Speech (TTS) example demonstrating audio synthesis with MiMo API.
//! Chat-styled interface.
//!
//! Usage: XIAOMI_API_KEY=your_key cargo run --example tts
//!
//! This example uses MiMo V2 TTS model to convert text to speech.

use mimo_api::{Audio, ChatRequest, Client, Voice};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable
    let client = Client::from_env()?;

    println!("=== MiMo Text-to-Speech Example ===\n");

    // Example 1: Basic TTS with default voice
    println!("1. Basic TTS with default voice (WAV format):");
    println!("   Text: '你好，欢迎使用小米MiMo语音合成服务。'\n");

    let request = ChatRequest::tts()
        .user("你好，欢迎使用小米MiMo语音合成服务。")
        .assistant("")  // TTS model requires assistant message
        .audio(Audio::wav());

    match client.chat(request).await {
        Ok(response) => {
            if let Some(audio) = &response.choices[0].message.audio {
                println!("   Audio ID: {}", audio.id);
                println!("   Audio data length: {} bytes", audio.data.len());
                if let Some(ref transcript) = audio.transcript {
                    println!("   Transcript: {}", transcript);
                }
                println!("   ✓ Audio synthesis successful!\n");
            } else {
                println!("   No audio data in response\n");
            }
        }
        Err(e) => {
            eprintln!("   Error: {}", e);
        }
    }

    // Example 2: TTS with Chinese voice
    println!("2. TTS with Chinese voice:");
    println!("   Text: '今天天气真不错，适合出去散步。'\n");

    let request = ChatRequest::tts()
        .user("今天天气真不错，适合出去散步。")
        .assistant("")  // TTS model requires assistant message
        .audio(Audio::mp3().voice(Voice::DefaultZh));

    match client.chat(request).await {
        Ok(response) => {
            if let Some(audio) = &response.choices[0].message.audio {
                println!("   Format: MP3");
                println!("   Voice: Default Chinese");
                println!("   Audio data length: {} bytes", audio.data.len());
                println!("   ✓ Audio synthesis successful!\n");
            }
        }
        Err(e) => {
            eprintln!("   Error: {}", e);
        }
    }

    // Example 3: TTS with English voice
    println!("3. TTS with English voice:");
    println!("   Text: 'Hello, this is a test of the MiMo text-to-speech system.'\n");

    let request = ChatRequest::tts()
        .user("Hello, this is a test of the MiMo text-to-speech system.")
        .assistant("")  // TTS model requires assistant message
        .audio(Audio::wav().voice(Voice::DefaultEn));

    match client.chat(request).await {
        Ok(response) => {
            if let Some(audio) = &response.choices[0].message.audio {
                println!("   Format: WAV");
                println!("   Voice: Default English");
                println!("   Audio data length: {} bytes", audio.data.len());
                println!("   ✓ Audio synthesis successful!\n");
            }
        }
        Err(e) => {
            eprintln!("   Error: {}", e);
        }
    }

    // Example 4: Long text TTS
    println!("4. Long text TTS:");
    let long_text = "MiMo是一个强大的大语言模型，支持多种功能，包括对话、翻译、代码生成等。\
                     它还支持语音合成功能，可以将文字转换为自然的语音输出。\
                     MiMo的语音合成支持多种声音和格式，适用于各种应用场景。";
    println!("   Text: '{}...'\n", long_text.chars().take(50).collect::<String>());

    let request = ChatRequest::tts()
        .user(long_text)
        .assistant("")  // TTS model requires assistant message
        .audio(Audio::wav().voice(Voice::MimoDefault));

    match client.chat(request).await {
        Ok(response) => {
            if let Some(audio) = &response.choices[0].message.audio {
                println!("   Voice: MiMo Default");
                println!("   Audio data length: {} bytes", audio.data.len());
                println!("   ✓ Long text synthesis successful!\n");
            }
        }
        Err(e) => {
            eprintln!("   Error: {}", e);
        }
    }

    println!("=== TTS Example Complete ===");
    println!("\nNote: In production, you would decode the base64 audio data");
    println!("and save it to a file for playback.");

    Ok(())
}
