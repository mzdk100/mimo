//! Error types for the MiMo API client.

use thiserror::Error;

/// Result type alias for MiMo API operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error types that can occur when using the MiMo API.
#[derive(Error, Debug)]
pub enum Error {
    /// API key is missing from environment variables.
    #[error("API key not found. Please set the XIAOMI_API_KEY environment variable.")]
    MissingApiKey,

    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// JSON serialization/deserialization failed.
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// API returned an error response.
    #[error("API error: {message} (status: {status})")]
    ApiError {
        /// HTTP status code
        status: u16,
        /// Error message from the API
        message: String,
    },

    /// Stream parsing error.
    #[error("Stream parsing error: {0}")]
    StreamError(String),

    /// Invalid parameter value.
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Invalid response from API.
    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    /// IO error.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Base64 decode error.
    #[error("Base64 decode error: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),
}

impl Error {
    /// Create a new API error.
    pub fn api_error(status: u16, message: impl Into<String>) -> Self {
        Error::ApiError {
            status,
            message: message.into(),
        }
    }
}
