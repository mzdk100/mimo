//! Response types for the MiMo API.

use serde::{Deserialize, Serialize};
// Import audio types from the audio module
use super::{ResponseAudio, DeltaAudio};

/// Chat completion response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    /// Unique identifier for the response
    pub id: String,
    /// List of completion choices
    pub choices: Vec<Choice>,
    /// Unix timestamp of creation
    pub created: i64,
    /// Model used for generation
    pub model: String,
    /// Object type (always "chat.completion")
    pub object: String,
    /// Token usage information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

/// A completion choice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    /// The reason the model stopped generating
    pub finish_reason: FinishReason,
    /// The index of this choice
    pub index: u32,
    /// The generated message
    pub message: ResponseMessage,
}

/// Reason for finishing generation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    /// Natural stop or stop sequence
    Stop,
    /// Max tokens reached
    Length,
    /// Tool was called
    ToolCalls,
    /// Content filtered
    ContentFilter,
}

/// Response message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMessage {
    /// Message content
    pub content: String,
    /// Reasoning content (for thinking mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
    /// Message role
    pub role: Role,
    /// Tool calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    /// Annotations (from web search)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<Annotation>>,
    /// Error message (from web search)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Audio data (for TTS)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<ResponseAudio>,
}

/// Web search annotation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    /// Logo URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Publish time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_time: Option<String>,
    /// Site name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    /// Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Annotation type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub annotation_type: Option<String>,
    /// URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Token usage information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    /// Tokens used in the completion
    pub completion_tokens: u32,
    /// Tokens used in the prompt
    pub prompt_tokens: u32,
    /// Total tokens used
    pub total_tokens: u32,
    /// Completion token details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens_details: Option<CompletionTokensDetails>,
    /// Prompt token details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<PromptTokensDetails>,
    /// Web search usage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search_usage: Option<WebSearchUsage>,
}

/// Completion tokens details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionTokensDetails {
    /// Tokens used for reasoning
    pub reasoning_tokens: u32,
}

/// Prompt tokens details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTokensDetails {
    /// Cached tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_tokens: Option<u32>,
    /// Audio tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
    /// Image tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tokens: Option<u32>,
    /// Video tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_tokens: Option<u32>,
}

/// Web search usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSearchUsage {
    /// Number of API calls
    pub tool_usage: u32,
    /// Number of pages returned
    pub page_usage: u32,
}

/// Streaming response chunk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    /// Unique identifier
    pub id: String,
    /// List of choices
    pub choices: Vec<StreamChoice>,
    /// Unix timestamp
    pub created: i64,
    /// Model used
    pub model: String,
    /// Object type (always "chat.completion.chunk")
    pub object: String,
    /// Token usage (only in final chunk)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

/// A streaming choice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChoice {
    /// Delta content
    pub delta: DeltaMessage,
    /// Reason for finishing (only in final chunk)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<FinishReason>,
    /// Choice index
    pub index: u32,
}

/// Delta message in a streaming response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaMessage {
    /// Message content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Reasoning content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
    /// Message role
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    /// Tool calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<DeltaToolCall>>,
    /// Annotations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<Annotation>>,
    /// Error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Audio data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<DeltaAudio>,
}

/// Delta tool call in a streaming response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaToolCall {
    /// Tool call index
    pub index: u32,
    /// Tool call ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Tool type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub tool_type: Option<ToolCallType>,
    /// Function call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<DeltaFunctionCall>,
}

/// Delta function call in a streaming response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaFunctionCall {
    /// Function name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Function arguments (incremental)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}

// Import types from other modules
use super::message::{ToolCall, ToolCallType};
use super::Role;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_response_deserialization() {
        let json = r#"{
            "id": "test-id",
            "choices": [{
                "finish_reason": "stop",
                "index": 0,
                "message": {
                    "content": "Hello!",
                    "role": "assistant"
                }
            }],
            "created": 1234567890,
            "model": "mimo-v2-flash",
            "object": "chat.completion"
        }"#;

        let response: ChatResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "test-id");
        assert_eq!(response.choices.len(), 1);
        assert_eq!(response.choices[0].message.content, "Hello!");
    }

    #[test]
    fn test_finish_reason_deserialization() {
        assert_eq!(
            serde_json::from_str::<FinishReason>(r#""stop""#).unwrap(),
            FinishReason::Stop
        );
        assert_eq!(
            serde_json::from_str::<FinishReason>(r#""tool_calls""#).unwrap(),
            FinishReason::ToolCalls
        );
    }

    #[test]
    fn test_usage() {
        let json = r#"{
            "completion_tokens": 100,
            "prompt_tokens": 50,
            "total_tokens": 150
        }"#;

        let usage: Usage = serde_json::from_str(json).unwrap();
        assert_eq!(usage.completion_tokens, 100);
        assert_eq!(usage.prompt_tokens, 50);
        assert_eq!(usage.total_tokens, 150);
    }

    #[test]
    fn test_stream_chunk_deserialization() {
        let json = r#"{
            "id": "chunk-id",
            "choices": [{
                "delta": {
                    "content": "Hello"
                },
                "index": 0
            }],
            "created": 1234567890,
            "model": "mimo-v2-flash",
            "object": "chat.completion.chunk"
        }"#;

        let chunk: StreamChunk = serde_json::from_str(json).unwrap();
        assert_eq!(chunk.id, "chunk-id");
        assert_eq!(chunk.choices[0].delta.content, Some("Hello".to_string()));
    }

    #[test]
    fn test_response_with_thinking() {
        let json = r#"{
            "id": "test-id",
            "choices": [{
                "finish_reason": "stop",
                "index": 0,
                "message": {
                    "content": "The answer is 42.",
                    "reasoning_content": "Let me think about this...",
                    "role": "assistant"
                }
            }],
            "created": 1234567890,
            "model": "mimo-v2-pro",
            "object": "chat.completion"
        }"#;

        let response: ChatResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response.choices[0].message.reasoning_content,
            Some("Let me think about this...".to_string())
        );
    }
}
