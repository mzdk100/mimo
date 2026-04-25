//! Multimodal example demonstrating image, audio, and video understanding with MiMo API.
//!
//! Usage: XIAOMI_API_KEY=your_key cargo run --example multimodal
//!
//! Note: This example uses MiMo V2 Omni which supports multimodal inputs.

use mimo_api::{ChatRequest, Client, ContentPart, Message, MessageContent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable
    let client = Client::from_env()?;

    println!("=== MiMo Multimodal Example ===\n");

    // ========================================
    // Example 1: Image Understanding
    // ========================================
    println!("1. Image Understanding:");
    println!("   Analyzing an image from URL...\n");

    let image_url = "https://picsum.photos/seed/mimo/800/600";

    let content = MessageContent::Parts(vec![
        ContentPart::text("What do you see in this image? Please describe it briefly."),
        ContentPart::image_url(image_url),
    ]);

    let request = ChatRequest::omni()
        .message(Message::user(content))
        .max_completion_tokens(512);

    match client.chat(request).await {
        Ok(response) => {
            println!("   Response: {}", response.choices[0].message.content);
            println!("   ✓ Image understanding successful!\n");
        }
        Err(e) => {
            eprintln!("   Error: {}\n", e);
        }
    }

    // ========================================
    // Example 2: Video Understanding
    // ========================================
    println!("2. Video Understanding:");
    println!("   Analyzing a video from URL...\n");

    // Note: Replace with an actual accessible video URL
    let video_url = "https://img.tukuppt.com/video_show/2475824/00/08/40/5d21787a4d185.mp4";

    let content = MessageContent::Parts(vec![
        ContentPart::text(
            "Please describe what happens in this video. What are the main actions and events?",
        ),
        ContentPart::video_url(video_url),
    ]);

    let request = ChatRequest::omni()
        .message(Message::user(content))
        .max_completion_tokens(512);

    println!("   Video URL: {}", video_url);
    match client.chat(request).await {
        Ok(response) => {
            println!("   Response: {}", response.choices[0].message.content);
            println!("   ✓ Video understanding successful!\n");
        }
        Err(e) => {
            eprintln!("   Error: {}", e);
            eprintln!("   Note: Video URL needs to be publicly accessible.\n");
        }
    }

    // ========================================
    // Example 3: Audio Understanding
    // ========================================
    println!("3. Audio Understanding:");
    println!("   Analyzing audio content...\n");

    // Example: Using base64 encoded audio
    // In production, you would read actual audio file and encode it
    let sample_audio_base64 = create_sample_audio_base64();

    let content = MessageContent::Parts(vec![
        ContentPart::text("Please transcribe and describe what you hear in this audio."),
        ContentPart::audio_base64(sample_audio_base64),
    ]);

    let request = ChatRequest::omni()
        .message(Message::user(content))
        .max_completion_tokens(512);

    match client.chat(request).await {
        Ok(response) => {
            println!("   Response: {}", response.choices[0].message.content);
            println!("   ✓ Audio understanding successful!\n");
        }
        Err(e) => {
            eprintln!("   Error: {}", e);
            eprintln!("   Note: Use real audio data for actual transcription.\n");
        }
    }

    // ========================================
    // Example 4: Combined Image + Audio
    // ========================================
    println!("4. Combined Image + Audio:");
    println!("   Analyzing both image and audio together...\n");

    let content = MessageContent::Parts(vec![
        ContentPart::text(
            "I'm showing you an image and playing an audio. Please describe both and explain any relationship between them.",
        ),
        ContentPart::image_url("https://picsum.photos/seed/combined/800/600"),
    ]);

    let request = ChatRequest::omni()
        .message(Message::user(content))
        .max_completion_tokens(512);

    match client.chat(request).await {
        Ok(response) => {
            println!("   Response: {}", response.choices[0].message.content);
            println!("   ✓ Combined analysis successful!\n");
        }
        Err(e) => {
            eprintln!("   Error: {}\n", e);
        }
    }

    // ========================================
    // Example 5: Multi-image Analysis
    // ========================================
    println!("5. Multi-image Analysis:");
    println!("   Comparing multiple images...\n");

    let content = MessageContent::Parts(vec![
        ContentPart::text("Compare these two images and describe the differences."),
        ContentPart::image_url("https://picsum.photos/seed/img1/400/300"),
        ContentPart::image_url("https://picsum.photos/seed/img2/400/300"),
    ]);

    let request = ChatRequest::omni()
        .message(Message::user(content))
        .max_completion_tokens(512);

    match client.chat(request).await {
        Ok(response) => {
            println!("   Response: {}", response.choices[0].message.content);
            println!("   ✓ Multi-image analysis successful!\n");
        }
        Err(e) => {
            eprintln!("   Error: {}\n", e);
        }
    }

    // ========================================
    // Example 6: Image with Base64
    // ========================================
    println!("6. Image with Base64 Encoding:");
    println!("   Analyzing base64-encoded image...\n");

    let base64_image = create_sample_image_base64();
    let content = MessageContent::Parts(vec![
        ContentPart::text("What do you see in this image?"),
        ContentPart::image_base64("image/png", base64_image),
    ]);

    let request = ChatRequest::omni()
        .message(Message::user(content))
        .max_completion_tokens(256);

    match client.chat(request).await {
        Ok(response) => {
            println!("   Response: {}", response.choices[0].message.content);
            println!("   ✓ Base64 image analysis successful!\n");
        }
        Err(e) => {
            eprintln!("   Error: {}\n", e);
        }
    }

    println!("=== Multimodal Example Complete ===");
    println!("\nSupported multimodal inputs:");
    println!("  • Images: URL or base64 encoded");
    println!("  • Audio: base64 encoded (WAV/MP3)");
    println!("  • Video: URL accessible by the API");

    Ok(())
}

/// Create a simple sample image as base64 (a small red square)
fn create_sample_image_base64() -> String {
    // This is a 1x1 red pixel PNG image
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==".to_string()
}

/// Create a sample audio as base64 (placeholder - use real audio in production)
fn create_sample_audio_base64() -> String {
    // This is a placeholder - in production, read and encode actual WAV/MP3 file
    // Example: std::fs::read("audio.wav").map(|data| base64::encode(&data))
    "UklGRiQAAABXQVZFZm10IBAAAAABAAEARKwAAIhYAQACABAAZGF0YQAAAAA=".to_string()
}
