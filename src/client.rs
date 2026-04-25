//! HTTP client for the MiMo API.

use {
    crate::{
        error::{Error, Result},
        types::*,
    },
    eventsource_stream::Eventsource,
    futures::{StreamExt, stream::BoxStream},
    reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue},
    std::env,
    tokio::{fs::File, io::AsyncWriteExt},
};

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
    /// use mimo_api::Client;
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
    /// use mimo_api::Client;
    ///
    /// // Assuming XIAOMI_API_KEY is set in environment
    /// let client = Client::from_env()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
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
    /// use mimo_api::{Client, ChatRequest, Message};
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
    /// use mimo_api::{Client, ChatRequest, Message};
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
    /// use mimo_api::{Client, Voice};
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
    ///     tokio::fs::write("output.wav", audio_bytes).await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn tts(&self, text: impl Into<String>) -> TtsRequestBuilder {
        TtsRequestBuilder::new(self.clone(), Model::MiMoV2Tts.as_str(), text.into())
    }

    /// Create a text-to-speech request builder with styled text.
    ///
    /// This method allows you to apply style controls to the synthesized speech.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use mimo_api::{Client, Voice};
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
    ///     tokio::fs::write("output.wav", audio_bytes).await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn tts_styled(&self, style: &str, text: &str) -> TtsRequestBuilder {
        TtsRequestBuilder::new(
            self.clone(),
            Model::MiMoV2Tts.as_str(),
            styled_text(style, text),
        )
    }

    /// Create a text-to-speech request builder using the MiMo V2.5 TTS model.
    ///
    /// This method uses the updated TTS model with more preset voices.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to synthesize.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use mimo_api::{Client, Voice};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::from_env()?;
    ///
    ///     let response = client.v25_tts("Hello, world!")
    ///         .voice(Voice::Mia)
    ///         .send()
    ///         .await?;
    ///
    ///     let audio = response.audio()?;
    ///     let audio_bytes = audio.decode_data()?;
    ///     tokio::fs::write("output.wav", audio_bytes).await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn v25_tts(&self, text: impl Into<String>) -> TtsRequestBuilder {
        TtsRequestBuilder::new(self.clone(), Model::MiMoV25Tts.as_str(), text.into())
    }

    /// Create a TTS request builder with voice design (MiMo V2.5 TTS VoiceDesign).
    ///
    /// This method uses text description to design a custom voice.
    /// The `user_message` is REQUIRED and should contain the voice description.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to synthesize.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use mimo_api::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::from_env()?;
    ///
    ///     let response = client.v25_tts_voice_design("Hello, world!")
    ///         .user_message("Give me a young male tone.")
    ///         .send()
    ///         .await?;
    ///
    ///     let audio = response.audio()?;
    ///     let audio_bytes = audio.decode_data()?;
    ///     tokio::fs::write("output.wav", audio_bytes).await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn v25_tts_voice_design(&self, text: impl Into<String>) -> TtsRequestBuilder {
        TtsRequestBuilder::new(
            self.clone(),
            Model::MiMoV25TtsVoiceDesign.as_str(),
            text.into(),
        )
    }

    /// Create a TTS request builder with voice clone (MiMo V2.5 TTS VoiceClone).
    ///
    /// This method uses an audio sample to clone a voice.
    /// Use `Voice::custom()` or `Voice::from_audio_file()` to set the voice.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to synthesize.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use mimo_api::{Client, Voice};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::from_env()?;
    ///
    ///     let voice = Voice::from_audio_file("voice_sample.mp3").await?;
    ///
    ///     let response = client.v25_tts_voice_clone("Hello, world!")
    ///         .voice(voice)
    ///         .send()
    ///         .await?;
    ///
    ///     let audio = response.audio()?;
    ///     let audio_bytes = audio.decode_data()?;
    ///     tokio::fs::write("output.wav", audio_bytes).await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn v25_tts_voice_clone(&self, text: impl Into<String>) -> TtsRequestBuilder {
        TtsRequestBuilder::new(
            self.clone(),
            Model::MiMoV25TtsVoiceClone.as_str(),
            text.into(),
        )
    }

    /// Create a streaming text-to-speech request builder.
    ///
    /// This method creates a builder for streaming speech synthesis using the `mimo-v2-tts` model.
    /// Streaming TTS delivers audio data in real-time chunks.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to synthesize. This text will be placed in an `assistant` message.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use mimo_api::{Client, Voice};
    /// use futures::StreamExt;
    /// use tokio::fs::File;
    /// use tokio::io::AsyncWriteExt;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::from_env()?;
    ///
    ///     let mut stream = client.tts_stream("Hello, world!")
    ///         .voice(Voice::DefaultEn)
    ///         .send()
    ///         .await?;
    ///
    ///     let mut file = File::create("output.pcm").await?;
    ///     let mut total_bytes = 0;
    ///
    ///     while let Some(chunk) = stream.next().await {
    ///         let audio_bytes = chunk?;
    ///         file.write_all(&audio_bytes).await?;
    ///         total_bytes += audio_bytes.len();
    ///     }
    ///
    ///     println!("Total bytes: {}", total_bytes);
    ///     Ok(())
    /// }
    /// ```
    pub fn tts_stream(&self, text: impl Into<String>) -> StreamingTtsRequestBuilder {
        StreamingTtsRequestBuilder::new(self.clone(), Model::MiMoV2Tts.as_str(), text.into())
    }

    /// Create a streaming text-to-speech request builder with styled text.
    ///
    /// This method allows you to apply style controls to the streaming synthesized speech.
    ///
    /// # Arguments
    ///
    /// * `style` - The style to apply (e.g., "开心", "悲伤", "变快", "变慢")
    /// * `text` - The text to synthesize
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use mimo_api::{Client, Voice};
    /// use futures::StreamExt;
    /// use tokio::fs::File;
    /// use tokio::io::AsyncWriteExt;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::from_env()?;
    ///
    ///     // Synthesize speech with "开心" (happy) style
    ///     let mut stream = client.tts_styled_stream("开心", "明天就是周五了，真开心！")
    ///         .voice(Voice::DefaultZh)
    ///         .send()
    ///         .await?;
    ///
    ///     let mut file = File::create("output.pcm").await?;
    ///     let mut total_bytes = 0;
    ///
    ///     while let Some(chunk) = stream.next().await {
    ///         let audio_bytes = chunk?;
    ///         file.write_all(&audio_bytes).await?;
    ///         total_bytes += audio_bytes.len();
    ///     }
    ///
    ///     println!("Total bytes: {}", total_bytes);
    ///     Ok(())
    /// }
    /// ```
    pub fn tts_styled_stream(&self, style: &str, text: &str) -> StreamingTtsRequestBuilder {
        StreamingTtsRequestBuilder::new(
            self.clone(),
            Model::MiMoV2Tts.as_str(),
            styled_text(style, text),
        )
    }

    /// Create a streaming TTS request builder using MiMo V2.5 TTS model.
    ///
    /// Note: Low-latency streaming for V2.5 TTS series is not yet available.
    /// The streaming API currently returns results in compatibility mode.
    pub fn v25_tts_stream(&self, text: impl Into<String>) -> StreamingTtsRequestBuilder {
        StreamingTtsRequestBuilder::new(self.clone(), Model::MiMoV25Tts.as_str(), text.into())
    }

    /// Create a streaming TTS request builder with voice design.
    ///
    /// Note: Low-latency streaming for V2.5 TTS series is not yet available.
    pub fn v25_tts_voice_design_stream(
        &self,
        text: impl Into<String>,
    ) -> StreamingTtsRequestBuilder {
        StreamingTtsRequestBuilder::new(
            self.clone(),
            Model::MiMoV25TtsVoiceDesign.as_str(),
            text.into(),
        )
    }

    /// Create a streaming TTS request builder with voice clone.
    ///
    /// Note: Low-latency streaming for V2.5 TTS series is not yet available.
    pub fn v25_tts_voice_clone_stream(
        &self,
        text: impl Into<String>,
    ) -> StreamingTtsRequestBuilder {
        StreamingTtsRequestBuilder::new(
            self.clone(),
            Model::MiMoV25TtsVoiceClone.as_str(),
            text.into(),
        )
    }
}

/// Builder for text-to-speech requests.
///
/// This builder provides a fluent API for configuring TTS requests.
#[derive(Debug, Clone)]
pub struct TtsRequestBuilder {
    client: Client,
    model: String,
    text: String,
    user_message: Option<String>,
    voice: Voice,
    format: AudioFormat,
}

impl TtsRequestBuilder {
    /// Create a new TTS request builder.
    fn new(client: Client, model: impl Into<String>, text: String) -> Self {
        Self {
            client,
            model: model.into(),
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
    /// use mimo_api::{Client, Voice, AudioFormat};
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

        // Voice design model does not support audio.voice parameter
        // Model name is "mimo-v2.5-tts-voicedesign" (no hyphen between voice and design)
        let is_voice_design = self.model.contains("voicedesign");

        let audio = if is_voice_design {
            // Voice design model only supports format, not voice
            Some(Audio {
                format: Some(self.format),
                voice: None,
            })
        } else {
            Some(Audio {
                format: Some(self.format),
                voice: Some(self.voice),
            })
        };

        let request = ChatRequest {
            model: self.model,
            messages,
            audio,
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

/// Builder for streaming text-to-speech requests.
///
/// This builder provides a fluent API for configuring streaming TTS requests.
#[derive(Debug, Clone)]
pub struct StreamingTtsRequestBuilder {
    client: Client,
    model: String,
    text: String,
    user_message: Option<String>,
    voice: Voice,
}

impl StreamingTtsRequestBuilder {
    /// Create a new streaming TTS request builder.
    fn new(client: Client, model: impl Into<String>, text: String) -> Self {
        Self {
            client,
            model: model.into(),
            text,
            user_message: None,
            voice: Voice::default(),
        }
    }

    /// Set the voice for synthesis.
    ///
    /// Available voices:
    /// - `Voice::MimoDefault` - MiMo default voice (balanced tone)
    /// - `Voice::DefaultEn` - Default English female voice
    /// - `Voice::DefaultZh` - Default Chinese female voice
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use mimo_api::{Client, Voice};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::from_env()?;
    ///
    ///     let stream = client.tts_stream("Hello!")
    ///         .voice(Voice::DefaultEn)
    ///         .send()
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn voice(mut self, voice: Voice) -> Self {
        self.voice = voice;
        self
    }

    /// Add a user message to influence the synthesis style.
    ///
    /// The user message can help adjust the tone and style of the synthesized speech.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use mimo_api::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::from_env()?;
    ///
    ///     let stream = client.tts_stream("Hello there!")
    ///         .user_message("Speak in a friendly, conversational tone")
    ///         .send()
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn user_message(mut self, message: impl Into<String>) -> Self {
        self.user_message = Some(message.into());
        self
    }

    /// Send the streaming TTS request and return the response stream.
    ///
    /// # Returns
    ///
    /// A `StreamingTtsResponse` that yields audio data chunks.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use mimo_api::{Client, Voice};
    /// use futures::StreamExt;
    /// use tokio::fs::File;
    /// use tokio::io::AsyncWriteExt;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::from_env()?;
    ///
    ///     let mut stream = client.tts_stream("Hello, world!")
    ///         .voice(Voice::DefaultEn)
    ///         .send()
    ///         .await?;
    ///
    ///     let mut file = File::create("output.pcm").await?;
    ///     let mut total_bytes = 0;
    ///
    ///     while let Some(result) = stream.next().await {
    ///         let audio_bytes = result?;
    ///         file.write_all(&audio_bytes).await?;
    ///         total_bytes += audio_bytes.len();
    ///     }
    ///
    ///     println!("Total bytes: {}", total_bytes);
    ///     Ok(())
    /// }
    /// ```
    pub async fn send(self) -> Result<StreamingTtsResponse> {
        let mut messages = Vec::new();

        // Add optional user message
        if let Some(user_msg) = self.user_message {
            messages.push(Message::user(MessageContent::Text(user_msg)));
        }

        // Add assistant message with text to synthesize
        messages.push(Message::assistant(MessageContent::Text(self.text)));

        let request = ChatRequest {
            model: self.model,
            messages,
            stream: Some(true),
            audio: Some(Audio {
                format: Some(AudioFormat::Pcm16), // PCM16 is recommended for streaming
                voice: Some(self.voice),
            }),
            ..Default::default()
        };

        let stream = self.client.chat_stream(request).await?;
        Ok(StreamingTtsResponse::new(stream))
    }
}

/// Response from a streaming text-to-speech request.
///
/// This type wraps the underlying stream and provides convenience methods
/// for consuming audio data.
pub struct StreamingTtsResponse {
    stream: BoxStream<'static, Result<StreamChunk>>,
    total_bytes: u64,
    chunk_count: u32,
}

impl StreamingTtsResponse {
    /// Create a new streaming TTS response.
    fn new(stream: BoxStream<'static, Result<StreamChunk>>) -> Self {
        Self {
            stream,
            total_bytes: 0,
            chunk_count: 0,
        }
    }

    /// Collect all audio chunks and return them as a single byte vector.
    ///
    /// This is a convenience method for non-streaming use cases where you
    /// want to wait for all audio data before processing it.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use mimo_api::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::from_env()?;
    ///
    ///     let mut stream = client.tts_stream("Hello, world!").send().await?;
    ///     let audio_bytes = stream.collect_audio().await?;
    ///
    ///     tokio::fs::write("output.pcm", &audio_bytes).await?;
    ///     println!("Total bytes: {}", audio_bytes.len());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn collect_audio(&mut self) -> Result<Vec<u8>> {
        let mut all_bytes = Vec::new();

        while let Some(chunk) = self.stream.next().await {
            if let Some(audio_bytes) = self.process_chunk(chunk?)? {
                all_bytes.extend(audio_bytes);
            }
        }

        Ok(all_bytes)
    }

    /// Save all audio chunks to a file.
    ///
    /// This is a convenience method that collects all audio data and writes it to a file.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use mimo_api::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::from_env()?;
    ///
    ///     let mut stream: mimo_api::StreamingTtsResponse = client.tts_stream("Hello, world!").send().await?;
    ///     stream.save_to_file("output.pcm").await?;
    ///
    ///     println!("Audio saved to file");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn save_to_file<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<()> {
        let mut file = File::create(path).await?;

        while let Some(chunk) = self.stream.next().await {
            if let Some(audio_bytes) = self.process_chunk(chunk?)? {
                file.write_all(&audio_bytes).await?;
            }
        }

        file.flush().await?;
        Ok(())
    }

    /// Process a stream chunk and return audio bytes if present.
    fn process_chunk(&mut self, chunk: StreamChunk) -> Result<Option<Vec<u8>>> {
        if !chunk.choices.is_empty()
            && let Some(audio) = &chunk.choices[0].delta.audio
        {
            let bytes = audio.decode_data()?;
            self.total_bytes += bytes.len() as u64;
            self.chunk_count += 1;
            return Ok(Some(bytes));
        }
        Ok(None)
    }

    /// Get the total number of bytes received so far.
    pub fn total_bytes(&self) -> u64 {
        self.total_bytes
    }

    /// Get the number of audio chunks received so far.
    pub fn chunk_count(&self) -> u32 {
        self.chunk_count
    }
}

impl futures::Stream for StreamingTtsResponse {
    type Item = Result<Vec<u8>>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        // Process chunks until we find one with audio data or the stream ends
        loop {
            match std::pin::Pin::new(&mut self.stream).poll_next(cx) {
                std::task::Poll::Ready(Some(Ok(chunk))) => {
                    // Check if this is the final chunk with finish_reason
                    let is_final = chunk
                        .choices
                        .first()
                        .and_then(|c| c.finish_reason.as_ref())
                        .is_some();

                    match self.process_chunk(chunk) {
                        Ok(Some(bytes)) => {
                            // Return audio data from this chunk
                            return std::task::Poll::Ready(Some(Ok(bytes)));
                        }
                        Ok(None) => {
                            // No audio data in this chunk
                            if is_final {
                                // Stream has ended, no more audio data
                                return std::task::Poll::Ready(None);
                            }
                            // Continue to next chunk
                            continue;
                        }
                        Err(e) => return std::task::Poll::Ready(Some(Err(e))),
                    }
                }
                std::task::Poll::Ready(Some(Err(e))) => {
                    let error_msg = format!("Stream error: {}", e);
                    return std::task::Poll::Ready(Some(Err(Error::StreamError(error_msg))));
                }
                std::task::Poll::Ready(None) => {
                    // Stream has ended normally
                    return std::task::Poll::Ready(None);
                }
                std::task::Poll::Pending => return std::task::Poll::Pending,
            }
        }
    }
}
