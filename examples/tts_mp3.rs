//! TTS with MP3 format example.
//!
//! This example demonstrates TTS with MP3 output format for smaller file size.
//!
//! Run with: cargo run --example tts_mp3

use {
    mimo_api::{AudioFormat, Client, Voice},
    tokio::fs::write,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;

    println!("Comparing WAV vs MP3 output formats\n");

    // WAV format (higher quality, larger file)
    println!("Generating WAV format...");
    let response = client
        .tts("This is a test of the MiMo text-to-speech system. We are comparing different audio formats.")
        .voice(Voice::DefaultEn)
        .format(AudioFormat::Wav)
        .send()
        .await?;

    let audio = response.audio()?;
    let wav_bytes = audio.decode_data()?;
    write("output_format.wav", &wav_bytes).await?;
    println!("WAV file size: {} bytes", wav_bytes.len());

    // MP3 format (smaller file, good quality)
    println!("\nGenerating MP3 format...");
    let response = client
        .tts("This is a test of the MiMo text-to-speech system. We are comparing different audio formats.")
        .voice(Voice::DefaultEn)
        .format(AudioFormat::Mp3)
        .send()
        .await?;

    let audio = response.audio()?;
    let mp3_bytes = audio.decode_data()?;
    write("output_format.mp3", &mp3_bytes).await?;
    println!("MP3 file size: {} bytes", mp3_bytes.len());

    // Calculate size reduction
    let reduction = 100.0 * (1.0 - (mp3_bytes.len() as f64 / wav_bytes.len() as f64));
    println!("\nMP3 is {:.1}% smaller than WAV", reduction);

    Ok(())
}
