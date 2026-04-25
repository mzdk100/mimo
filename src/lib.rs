//! # MiMo API - Xiaomi MiMo API Client for Rust
//!
//! A Rust client library for Xiaomi MiMo Open Platform API, compatible with OpenAI API format.
//!
//! ## Features
//!
//! - Chat completions (streaming and non-streaming)
//! - Function calling / Tool use
//! - Web search integration
//! - Multi-modal input (image, audio, video)
//! - Text-to-speech synthesis
//! - Structured output
//! - Deep thinking mode
//!
//! ## Example
//!
//! ```rust,no_run
//! use mimo_api::{Client, Message, ChatRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client from environment variable XIAOMI_API_KEY
//!     let client = Client::from_env()?;
//!     
//!     // Create a chat request
//!     let request = ChatRequest::new("mimo-v2-flash")
//!         .message(Message::user("Hello, introduce yourself!"));
//!     
//!     // Send the request
//!     let response = client.chat(request).await?;
//!     
//!     println!("{}", response.choices[0].message.content);
//!     Ok(())
//! }
//! ```
//!
//! ## Environment Variables
//!
//! - `XIAOMI_API_KEY`: Your Xiaomi MiMo API key (required)

pub mod client;
pub mod error;
pub mod types;

pub use client::Client;
pub use error::{Error, Result};
pub use types::*;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::client::{
        Client, StreamingTtsRequestBuilder, StreamingTtsResponse, TtsRequestBuilder, TtsResponse,
    };
    pub use crate::types::{
        Audio, AudioFormat, ChatRequest, ChatResponse, Message, MessageContent, Model,
        ResponseAudio, Role, StreamChunk, Thinking, ThinkingType, Tool, ToolChoice, TtsStyle,
        UserLocation, Voice, styled_text,
    };
}

pub use client::{StreamingTtsRequestBuilder, StreamingTtsResponse};

/// Schema helpers for creating tool parameter schemas.
pub mod schema {
    pub use crate::types::schema::*;
}
