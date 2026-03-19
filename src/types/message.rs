//! Message types for the MiMo API.

use serde::{Deserialize, Serialize};

/// Message role.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// System message
    System,
    /// Developer message
    Developer,
    /// User message
    User,
    /// Assistant message
    Assistant,
    /// Tool message
    Tool,
}

/// Message content - can be text or multi-part content.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    /// Simple text content
    Text(String),
    /// Multi-part content (for multi-modal messages)
    Parts(Vec<ContentPart>),
}

impl MessageContent {
    /// Create text content.
    pub fn text(text: impl Into<String>) -> Self {
        MessageContent::Text(text.into())
    }

    /// Create multi-part content.
    pub fn parts(parts: Vec<ContentPart>) -> Self {
        MessageContent::Parts(parts)
    }
}

impl From<&str> for MessageContent {
    fn from(s: &str) -> Self {
        MessageContent::Text(s.to_string())
    }
}

impl From<String> for MessageContent {
    fn from(s: String) -> Self {
        MessageContent::Text(s)
    }
}

/// Content part for multi-modal messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentPart {
    /// Content type
    #[serde(rename = "type")]
    pub content_type: ContentType,
    /// Text content (for text type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Image URL (for image type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<ImageUrl>,
    /// Input audio (for audio type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_audio: Option<InputAudio>,
    /// Video URL (for video type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_url: Option<VideoUrl>,
}

impl ContentPart {
    /// Create a text content part.
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            content_type: ContentType::Text,
            text: Some(text.into()),
            image_url: None,
            input_audio: None,
            video_url: None,
        }
    }

    /// Create an image content part from a URL.
    pub fn image_url(url: impl Into<String>) -> Self {
        Self {
            content_type: ContentType::ImageUrl,
            text: None,
            image_url: Some(ImageUrl {
                url: url.into(),
                detail: None,
            }),
            input_audio: None,
            video_url: None,
        }
    }

    /// Create an image content part from base64 data.
    pub fn image_base64(media_type: &str, data: impl Into<String>) -> Self {
        let url = format!("data:{};base64,{}", media_type, data.into());
        Self {
            content_type: ContentType::ImageUrl,
            text: None,
            image_url: Some(ImageUrl {
                url,
                detail: None,
            }),
            input_audio: None,
            video_url: None,
        }
    }

    /// Create an audio content part from base64 data.
    pub fn audio_base64(data: impl Into<String>) -> Self {
        Self {
            content_type: ContentType::InputAudio,
            text: None,
            image_url: None,
            input_audio: Some(InputAudio {
                data: data.into(),
                format: AudioInputFormat::Wav,
            }),
            video_url: None,
        }
    }

    /// Create a video content part from a URL.
    pub fn video_url(url: impl Into<String>) -> Self {
        Self {
            content_type: ContentType::VideoUrl,
            text: None,
            image_url: None,
            input_audio: None,
            video_url: Some(VideoUrl { url: url.into() }),
        }
    }
}

/// Content type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    /// Text content
    Text,
    /// Image URL content
    ImageUrl,
    /// Input audio content
    InputAudio,
    /// Video URL content
    VideoUrl,
}

/// Image URL configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUrl {
    /// The image URL (can be a regular URL or data URL)
    pub url: String,
    /// Image detail level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<ImageDetail>,
}

/// Image detail level.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ImageDetail {
    /// Auto detect
    Auto,
    /// Low detail
    Low,
    /// High detail
    High,
}

/// Audio input format.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AudioInputFormat {
    /// WAV format
    Wav,
    /// MP3 format
    Mp3,
}

/// Input audio configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputAudio {
    /// Base64 encoded audio data
    pub data: String,
    /// Audio format
    pub format: AudioInputFormat,
}

/// Video URL configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoUrl {
    /// The video URL
    pub url: String,
}

/// A message in the conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// The role of the message author
    pub role: Role,
    /// The content of the message
    pub content: MessageContent,
    /// Optional name for the participant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tool calls (for assistant messages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    /// Tool call ID (for tool messages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    /// Reasoning content (for thinking mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
    /// Audio data (for assistant messages with audio output)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<MessageAudio>,
}

impl Message {
    /// Create a new message.
    pub fn new(role: Role, content: MessageContent) -> Self {
        Self {
            role,
            content,
            name: None,
            tool_calls: None,
            tool_call_id: None,
            reasoning_content: None,
            audio: None,
        }
    }

    /// Create a system message.
    pub fn system(content: impl Into<MessageContent>) -> Self {
        Self::new(Role::System, content.into())
    }

    /// Create a developer message.
    pub fn developer(content: impl Into<MessageContent>) -> Self {
        Self::new(Role::Developer, content.into())
    }

    /// Create a user message.
    pub fn user(content: impl Into<MessageContent>) -> Self {
        Self::new(Role::User, content.into())
    }

    /// Create an assistant message.
    pub fn assistant(content: impl Into<MessageContent>) -> Self {
        Self::new(Role::Assistant, content.into())
    }

    /// Create a tool response message.
    pub fn tool(tool_call_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: Role::Tool,
            content: MessageContent::Text(content.into()),
            name: None,
            tool_calls: None,
            tool_call_id: Some(tool_call_id.into()),
            reasoning_content: None,
            audio: None,
        }
    }

    /// Set the name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the tool calls.
    pub fn with_tool_calls(mut self, tool_calls: Vec<ToolCall>) -> Self {
        self.tool_calls = Some(tool_calls);
        self
    }

    /// Set the reasoning content.
    pub fn with_reasoning_content(mut self, content: impl Into<String>) -> Self {
        self.reasoning_content = Some(content.into());
        self
    }
}

/// Message audio data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAudio {
    /// Audio ID
    pub id: Option<String>,
    /// Base64 encoded audio data
    pub data: Option<String>,
    /// Expiration timestamp
    pub expires_at: Option<i64>,
    /// Audio transcript
    pub transcript: Option<String>,
}

/// A tool call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// The ID of the tool call
    pub id: String,
    /// The type of tool
    #[serde(rename = "type")]
    pub tool_type: ToolCallType,
    /// The function call
    pub function: FunctionCall,
}

impl ToolCall {
    /// Create a new tool call.
    pub fn new(id: impl Into<String>, function: FunctionCall) -> Self {
        Self {
            id: id.into(),
            tool_type: ToolCallType::Function,
            function,
        }
    }
}

/// Tool call type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ToolCallType {
    /// Function tool
    Function,
}

/// A function call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    /// The name of the function to call
    pub name: String,
    /// The arguments to pass to the function (JSON string)
    pub arguments: String,
}

impl FunctionCall {
    /// Create a new function call.
    pub fn new(name: impl Into<String>, arguments: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            arguments: arguments.into(),
        }
    }

    /// Parse the arguments as JSON.
    pub fn parse_arguments<T: serde::de::DeserializeOwned>(&self) -> serde_json::Result<T> {
        serde_json::from_str(&self.arguments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let msg = Message::user(MessageContent::Text("Hello".to_string()));
        assert_eq!(msg.role, Role::User);
    }

    #[test]
    fn test_message_serialization() {
        let msg = Message::user(MessageContent::Text("Hello".to_string()));
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"role\":\"user\""));
        assert!(json.contains("\"content\":\"Hello\""));
    }

    #[test]
    fn test_content_part_text() {
        let part = ContentPart::text("Hello");
        assert_eq!(part.content_type, ContentType::Text);
        assert_eq!(part.text, Some("Hello".to_string()));
    }

    #[test]
    fn test_content_part_image_url() {
        let part = ContentPart::image_url("https://example.com/image.png");
        assert_eq!(part.content_type, ContentType::ImageUrl);
        assert!(part.image_url.is_some());
    }

    #[test]
    fn test_content_part_video_url() {
        let part = ContentPart::video_url("https://example.com/video.mp4");
        assert_eq!(part.content_type, ContentType::VideoUrl);
        assert!(part.video_url.is_some());
    }

    #[test]
    fn test_function_call_parse() {
        let fc = FunctionCall::new("test", r#"{"arg": "value"}"#);
        let parsed: serde_json::Value = fc.parse_arguments().unwrap();
        assert_eq!(parsed["arg"], "value");
    }

    #[test]
    fn test_multimodal_message() {
        let content = MessageContent::Parts(vec![
            ContentPart::text("What's in this image?"),
            ContentPart::image_url("https://example.com/image.png"),
        ]);
        let msg = Message::user(content);
        assert_eq!(msg.role, Role::User);
    }

    #[test]
    fn test_tool_message() {
        let msg = Message::tool("call_123", "result data");
        assert_eq!(msg.role, Role::Tool);
        assert_eq!(msg.tool_call_id, Some("call_123".to_string()));
    }
}
