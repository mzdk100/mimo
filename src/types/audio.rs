//! Audio types for the MiMo API.
//!
//! This module provides types for configuring audio output, particularly for
//! text-to-speech (TTS) synthesis using the `mimo-v2-tts` model.

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Audio output format.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum AudioFormat {
    /// WAV format (recommended for high quality)
    #[default]
    Wav,
    /// MP3 format (smaller file size)
    Mp3,
    /// PCM format (for streaming)
    Pcm,
}

/// Available voice options for text-to-speech.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum Voice {
    /// MiMo default voice - balanced tone
    #[default]
    MimoDefault,
    /// Default English female voice
    DefaultEn,
    /// Default Chinese female voice
    DefaultZh,
}

/// Audio output configuration for text-to-speech.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audio {
    /// Output audio format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<AudioFormat>,
    /// Voice to use for synthesis
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
}

impl Audio {
    /// Create a new audio configuration.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mimo_api::{Audio, AudioFormat, Voice};
    ///
    /// let audio = Audio::new()
    ///     .format(AudioFormat::Wav)
    ///     .voice(Voice::MimoDefault);
    /// ```
    pub fn new() -> Self {
        Self {
            format: None,
            voice: None,
        }
    }

    /// Set the audio format.
    pub fn format(mut self, format: AudioFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Set the voice for synthesis.
    pub fn voice(mut self, voice: Voice) -> Self {
        self.voice = Some(voice);
        self
    }

    /// Create audio configuration with WAV format.
    pub fn wav() -> Self {
        Self::new().format(AudioFormat::Wav)
    }

    /// Create audio configuration with MP3 format.
    pub fn mp3() -> Self {
        Self::new().format(AudioFormat::Mp3)
    }

    /// Create audio configuration with PCM format (for streaming).
    pub fn pcm() -> Self {
        Self::new().format(AudioFormat::Pcm)
    }
}

impl Default for Audio {
    fn default() -> Self {
        Self::new()
    }
}

/// Response audio data from text-to-speech.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseAudio {
    /// Audio ID
    pub id: String,
    /// Base64 encoded audio data
    pub data: String,
    /// Expiration timestamp (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    /// Audio transcript (text that was synthesized)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
}

impl ResponseAudio {
    /// Decode the base64 audio data to bytes.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use mimo_api::{Client, Audio, Voice, Message};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::from_env()?;
    ///
    ///     let response = client.tts("Hello, world!")
    ///         .voice(Voice::DefaultEn)
    ///         .send()
    ///         .await?;
    ///
    ///     let audio = response.audio()?;
    ///     let audio_bytes = audio.decode_data()?;
    ///     std::fs::write("output.wav", audio_bytes)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn decode_data(&self) -> Result<Vec<u8>> {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.decode(&self.data).map_err(Into::into)
    }

    /// Get the transcript of the synthesized text.
    pub fn transcript(&self) -> Option<&str> {
        self.transcript.as_deref()
    }

    /// Check if the audio has expired.
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            now > expires_at
        } else {
            false
        }
    }
}

/// Delta audio in a streaming response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaAudio {
    /// Audio ID
    pub id: String,
    /// Base64 encoded audio data
    pub data: String,
    /// Expiration timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    /// Audio transcript
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
}

impl DeltaAudio {
    /// Decode the base64 audio data to bytes.
    pub fn decode_data(&self) -> Result<Vec<u8>> {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.decode(&self.data).map_err(Into::into)
    }
}

/// Text-to-speech style control.
///
/// Use the `<style>` tag to control the overall style of the synthesized audio.
/// The style should be placed at the beginning of the text to be synthesized.
#[derive(Debug, Clone, Default)]
pub struct TtsStyle {
    styles: Vec<String>,
}

impl TtsStyle {
    /// Create a new TTS style builder.
    pub fn new() -> Self {
        Self { styles: Vec::new() }
    }

    /// Add a style to apply.
    ///
    /// # Available Styles
    ///
    /// - **Speed control**: "变快", "变慢"
    /// - **Emotion**: "开心", "悲伤", "生气"
    /// - **Role play**: "孙悟空", "林黛玉"
    /// - **Style change**: "悄悄话", "夹子音", "台湾腔"
    /// - **Dialect**: "东北话", "四川话", "河南话", "粤语"
    /// - **Singing**: "唱歌"
    ///
    /// # Example
    ///
    /// ```rust
    /// use mimo_api::TtsStyle;
    ///
    /// let style = TtsStyle::new()
    ///     .with_style("开心")
    ///     .with_style("变快");
    ///
    /// let text = style.apply("明天就是周五了，真开心！");
    /// assert!(text.starts_with("<style>"));
    /// ```
    pub fn with_style(mut self, style: impl Into<String>) -> Self {
        self.styles.push(style.into());
        self
    }

    /// Apply the style to the text to be synthesized.
    ///
    /// Returns the text with the style tag prepended.
    pub fn apply(&self, text: &str) -> String {
        if self.styles.is_empty() {
            text.to_string()
        } else {
            format!("<style>{}</style>{}", self.styles.join(" "), text)
        }
    }
}

/// Create styled text for TTS with the given style.
///
/// # Example
///
/// ```rust
/// use mimo_api::styled_text;
///
/// let text = styled_text("开心", "明天就是周五了，真开心！");
/// assert!(text.starts_with("<style>开心</style>"));
/// ```
pub fn styled_text(style: &str, text: &str) -> String {
    TtsStyle::new().with_style(style).apply(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;

    #[test]
    fn test_audio_format_default() {
        let format = AudioFormat::default();
        assert_eq!(format, AudioFormat::Wav);
    }

    #[test]
    fn test_voice_default() {
        let voice = Voice::default();
        assert_eq!(voice, Voice::MimoDefault);
    }

    #[test]
    fn test_audio_config() {
        let audio = Audio::wav().voice(Voice::DefaultZh);
        assert_eq!(audio.format, Some(AudioFormat::Wav));
        assert_eq!(audio.voice, Some(Voice::DefaultZh));
    }

    #[test]
    fn test_audio_serialization() {
        let audio = Audio::mp3().voice(Voice::DefaultEn);
        let json = serde_json::to_string(&audio).unwrap();
        assert!(json.contains("\"format\":\"mp3\""));
        assert!(json.contains("\"voice\":\"default_en\""));
    }

    #[test]
    fn test_audio_formats() {
        assert_eq!(Audio::wav().format, Some(AudioFormat::Wav));
        assert_eq!(Audio::mp3().format, Some(AudioFormat::Mp3));
        assert_eq!(Audio::pcm().format, Some(AudioFormat::Pcm));
    }

    #[test]
    fn test_tts_style_single() {
        let text = TtsStyle::new().with_style("开心").apply("Hello");
        assert_eq!(text, "<style>开心</style>Hello");
    }

    #[test]
    fn test_tts_style_multiple() {
        let text = TtsStyle::new().with_style("开心").with_style("变快").apply("Hello");
        assert!(text.starts_with("<style>"));
        assert!(text.contains("开心"));
        assert!(text.contains("变快"));
        assert!(text.ends_with("Hello"));
    }

    #[test]
    fn test_tts_style_empty() {
        let text = TtsStyle::new().apply("Hello");
        assert_eq!(text, "Hello");
    }

    #[test]
    fn test_styled_text_helper() {
        let text = styled_text("东北话", "哎呀妈呀");
        assert_eq!(text, "<style>东北话</style>哎呀妈呀");
    }

    #[test]
    fn test_response_audio_decode() {
        let audio = ResponseAudio {
            id: "test-id".to_string(),
            data: base64::engine::general_purpose::STANDARD.encode(b"test audio data"),
            expires_at: None,
            transcript: Some("test".to_string()),
        };

        let decoded = audio.decode_data().unwrap();
        assert_eq!(decoded, b"test audio data");
    }

    #[test]
    fn test_response_audio_transcript() {
        let audio = ResponseAudio {
            id: "test-id".to_string(),
            data: String::new(),
            expires_at: None,
            transcript: Some("Hello world".to_string()),
        };

        assert_eq!(audio.transcript(), Some("Hello world"));
    }
}
