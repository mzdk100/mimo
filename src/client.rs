//! HTTP client for the MiMo API.

use crate::error::{Error, Result};
use crate::types::*;
use eventsource_stream::Eventsource;
use futures::stream::BoxStream;
use futures::StreamExt;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::env;

const API_BASE_URL: &str = "https://api.xiaomimimo.com/v1";
const ENV_API_KEY: &str = "XIAOMI_API_KEY";

/// HTTP client for the MiMo API.
#[derive(Debug, Clone)]
pub struct Client {
    /// The underlying HTTP client.
    http_client: reqwest::Client,
    /// The API key for authentication.
    api_key: String,
    /// The base URL for the API.
    base_url: String,
}

impl Client {
    /// Create a new client with the given API key.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mimo::Client;
    ///
    /// let client = Client::new("your-api-key");
    /// ```
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            api_key: api_key.into(),
            base_url: API_BASE_URL.to_string(),
        }
    }

    /// Create a new client from the `XIAOMI_API_KEY` environment variable.
    ///
    /// # Errors
    ///
    /// Returns an error if the `XIAOMI_API_KEY` environment variable is not set.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use mimo::Client;
    ///
    /// // Assuming XIAOMI_API_KEY is set in environment
    /// let client = Client::from_env()?;
    /// # Ok::<(), mimo::Error>(())
    /// ```
    pub fn from_env() -> Result<Self> {
        let api_key = env::var(ENV_API_KEY).map_err(|_| Error::MissingApiKey)?;
        Ok(Self::new(api_key))
    }

    /// Set a custom base URL for the API.
    ///
    /// This is useful for testing or using a custom API endpoint.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Build headers for the request.
    fn build_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            "api-key",
            HeaderValue::from_str(&self.api_key)
                .map_err(|_| Error::InvalidParameter("Invalid API key".into()))?,
        );
        Ok(headers)
    }

    /// Send a chat completion request.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use mimo::{Client, ChatRequest, Message};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::from_env()?;
    ///     let request = ChatRequest::new("mimo-v2-flash")
    ///         .message(Message::user("Hello!"));
    ///     let response = client.chat(request).await?;
    ///     println!("{}", response.choices[0].message.content);
    ///     Ok(())
    /// }
    /// ```
    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let url = format!("{}/chat/completions", self.base_url);
        let headers = self.build_headers()?;

        let response = self
            .http_client
            .post(&url)
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::api_error(status.as_u16(), error_text));
        }

        response.json().await.map_err(Error::from)
    }

    /// Send a chat completion request with streaming response.
    ///
    /// Returns a stream of `StreamChunk` objects.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use mimo::{Client, ChatRequest, Message};
    /// use futures::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::from_env()?;
    ///     let request = ChatRequest::new("mimo-v2-flash")
    ///         .message(Message::user("Tell me a story."))
    ///         .stream(true);
    ///     
    ///     let mut stream = client.chat_stream(request).await?;
    ///     while let Some(chunk) = stream.next().await {
    ///         match chunk {
    ///             Ok(chunk) => {
    ///                 if let Some(content) = &chunk.choices[0].delta.content {
    ///                     print!("{}", content);
    ///                 }
    ///             }
    ///             Err(e) => eprintln!("Error: {}", e),
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> Result<BoxStream<'static, Result<StreamChunk>>> {
        let mut request = request;
        request.stream = Some(true);

        let url = format!("{}/chat/completions", self.base_url);
        let headers = self.build_headers()?;

        let response = self
            .http_client
            .post(&url)
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::api_error(status.as_u16(), error_text));
        }

        let stream = response
            .bytes_stream()
            .eventsource()
            .filter_map(|event| async move {
                match event {
                    Ok(event) => {
                        if event.data == "[DONE]" {
                            None
                        } else {
                            match serde_json::from_str::<StreamChunk>(&event.data) {
                                Ok(chunk) => Some(Ok(chunk)),
                                Err(e) => Some(Err(Error::StreamError(e.to_string()))),
                            }
                        }
                    }
                    Err(e) => Some(Err(Error::StreamError(e.to_string()))),
                }
            })
            .boxed();

        Ok(stream)
    }

    /// Create a text-to-speech request builder.
    ///
    /// This method creates a builder for synthesizing speech from text using the `mimo-v2-tts` model.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to synthesize. This text will be placed in an `assistant` message.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use mimo::{Client, Voice};
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
    pub fn tts(&self, text: impl Into<String>) -> TtsRequestBuilder {
        TtsRequestBuilder::new(self.clone(), text.into())
    }

    /// Create a text-to-speech request builder with styled text.
    ///
    /// This method allows you to apply style controls to the synthesized speech.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use mimo::{Client, Voice};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::from_env()?;
    ///     
    ///     // Synthesize speech with "开心" (happy) style
    ///     let response = client.tts_styled("开心", "明天就是周五了，真开心！")
    ///         .voice(Voice::DefaultZh)
    ///         .send()
    ///         .await?;
    ///     
    ///     let audio = response.audio()?;
    ///     let audio_bytes = audio.decode_data()?;
    ///     std::fs::write("output.wav", audio_bytes)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn tts_styled(&self, style: &str, text: &str) -> TtsRequestBuilder {
        TtsRequestBuilder::new(self.clone(), styled_text(style, text))
    }
}

/// Builder for text-to-speech requests.
///
/// This builder provides a fluent API for configuring TTS requests.
#[derive(Debug, Clone)]
pub struct TtsRequestBuilder {
    client: Client,
    text: String,
    user_message: Option<String>,
    voice: Voice,
    format: AudioFormat,
}

impl TtsRequestBuilder {
    /// Create a new TTS request builder.
    fn new(client: Client, text: String) -> Self {
        Self {
            client,
            text,
            user_message: None,
            voice: Voice::default(),
            format: AudioFormat::default(),
        }
    }

    /// Set the voice for synthesis.
    ///
    /// Available voices:
    /// - `Voice::MimoDefault` - MiMo default voice (balanced tone)
    /// - `Voice::DefaultEn` - Default English female voice
    /// - `Voice::DefaultZh` - Default Chinese female voice
    pub fn voice(mut self, voice: Voice) -> Self {
        self.voice = voice;
        self
    }

    /// Set the audio output format.
    ///
    /// Available formats:
    /// - `AudioFormat::Wav` - WAV format (recommended for high quality)
    /// - `AudioFormat::Mp3` - MP3 format (smaller file size)
    /// - `AudioFormat::Pcm` - PCM format (for streaming)
    pub fn format(mut self, format: AudioFormat) -> Self {
        self.format = format;
        self
    }

    /// Add a user message to influence the synthesis style.
    ///
    /// The user message can help adjust the tone and style of the synthesized speech.
    pub fn user_message(mut self, message: impl Into<String>) -> Self {
        self.user_message = Some(message.into());
        self
    }

    /// Send the TTS request and return the response.
    ///
    /// # Returns
    ///
    /// A `TtsResponse` containing the synthesized audio data.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use mimo::{Client, Voice, AudioFormat};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::from_env()?;
    ///     
    ///     let response = client.tts("Hello, world!")
    ///         .voice(Voice::DefaultEn)
    ///         .format(AudioFormat::Mp3)
    ///         .send()
    ///         .await?;
    ///     
    ///     let audio = response.audio()?;
    ///     println!("Audio ID: {}", audio.id);
    ///     println!("Transcript: {:?}", audio.transcript());
    ///     Ok(())
    /// }
    /// ```
    pub async fn send(self) -> Result<TtsResponse> {
        let mut messages = Vec::new();

        // Add optional user message
        if let Some(user_msg) = self.user_message {
            messages.push(Message::user(MessageContent::Text(user_msg)));
        }

        // Add assistant message with text to synthesize
        messages.push(Message::assistant(MessageContent::Text(self.text)));

        let request = ChatRequest {
            model: Model::MiMoV2Tts.to_string(),
            messages,
            audio: Some(Audio {
                format: Some(self.format),
                voice: Some(self.voice),
            }),
            ..Default::default()
        };

        let response = self.client.chat(request).await?;
        Ok(TtsResponse(response))
    }
}

/// Response from a text-to-speech request.
#[derive(Debug, Clone)]
pub struct TtsResponse(pub ChatResponse);

impl TtsResponse {
    /// Get the audio data from the response.
    ///
    /// # Errors
    ///
    /// Returns an error if no audio data is present in the response.
    pub fn audio(&self) -> Result<&ResponseAudio> {
        self.0
            .choices
            .first()
            .and_then(|c| c.message.audio.as_ref())
            .ok_or_else(|| Error::InvalidResponse("No audio data in response".into()))
    }

    /// Get the content text from the response.
    pub fn content(&self) -> Option<&str> {
        self.0.choices.first().map(|c| c.message.content.as_str())
    }

    /// Get the underlying chat response.
    pub fn into_inner(self) -> ChatResponse {
        self.0
    }
}
