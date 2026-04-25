//! Data types for the MiMo API.

mod audio;
mod chat;
mod message;
mod response;
mod tool;

// Re-export commonly used types at the crate root
pub use audio::{Audio, AudioFormat, DeltaAudio, ResponseAudio, TtsStyle, Voice, styled_text};
pub use chat::{
    ChatRequest, Model, ResponseFormat, ResponseFormatType, Stop, Thinking, ThinkingType,
    ToolChoice,
};
pub use message::{ContentPart, ContentType, FunctionCall, Message, MessageContent, Role};
pub use response::{
    Annotation, ChatResponse, Choice, CompletionTokensDetails, DeltaFunctionCall, DeltaMessage,
    DeltaToolCall, FinishReason, PromptTokensDetails, ResponseMessage, StreamChoice, StreamChunk,
    Usage, WebSearchUsage,
};
pub use tool::{ParameterBuilder, Tool, ToolType, UserLocation};

/// Schema helpers for creating tool parameter schemas.
pub mod schema {
    pub use crate::types::tool::schema::*;
}
