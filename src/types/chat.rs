//! Chat request types.

use super::*;
use serde::{Deserialize, Serialize};

/// Available MiMo models.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Model {
    /// MiMo V2.5 Pro - Latest flagship model
    MiMoV25Pro,
    /// MiMo V2.5 - Balanced performance model
    MiMoV25,
    /// MiMo V2.5 TTS - Text-to-speech with preset voices
    MiMoV25Tts,
    /// MiMo V2.5 TTS VoiceDesign - Voice design via text description
    MiMoV25TtsVoiceDesign,
    /// MiMo V2.5 TTS VoiceClone - Voice cloning via audio sample
    MiMoV25TtsVoiceClone,
    /// MiMo V2 Pro - Agent-oriented flagship model
    MiMoV2Pro,
    /// MiMo V2 Omni - Multi-modal agent model
    MiMoV2Omni,
    /// MiMo V2 TTS - Text-to-speech model (legacy)
    MiMoV2Tts,
    /// MiMo V2 Flash - Fast and efficient model
    MiMoV2Flash,
}

impl Model {
    /// Get the model identifier string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Model::MiMoV25Pro => "mimo-v2.5-pro",
            Model::MiMoV25 => "mimo-v2.5",
            Model::MiMoV25Tts => "mimo-v2.5-tts",
            Model::MiMoV25TtsVoiceDesign => "mimo-v2.5-tts-voicedesign",
            Model::MiMoV25TtsVoiceClone => "mimo-v2.5-tts-voiceclone",
            Model::MiMoV2Pro => "mimo-v2-pro",
            Model::MiMoV2Omni => "mimo-v2-omni",
            Model::MiMoV2Tts => "mimo-v2-tts",
            Model::MiMoV2Flash => "mimo-v2-flash",
        }
    }
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for Model {
    fn from(s: &str) -> Self {
        match s {
            "mimo-v2.5-pro" => Model::MiMoV25Pro,
            "mimo-v2.5" => Model::MiMoV25,
            "mimo-v2.5-tts" => Model::MiMoV25Tts,
            "mimo-v2.5-tts-voicedesign" => Model::MiMoV25TtsVoiceDesign,
            "mimo-v2.5-tts-voiceclone" => Model::MiMoV25TtsVoiceClone,
            "mimo-v2-pro" => Model::MiMoV2Pro,
            "mimo-v2-omni" => Model::MiMoV2Omni,
            "mimo-v2-tts" => Model::MiMoV2Tts,
            "mimo-v2-flash" => Model::MiMoV2Flash,
            _ => Model::MiMoV2Flash,
        }
    }
}

/// Thinking mode configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ThinkingType {
    /// Enable thinking mode (deep reasoning)
    Enabled,
    /// Disable thinking mode
    Disabled,
}

/// Thinking configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thinking {
    /// Whether to enable thinking mode
    #[serde(rename = "type")]
    pub thinking_type: ThinkingType,
}

impl Thinking {
    /// Create a new thinking configuration.
    pub fn new(thinking_type: ThinkingType) -> Self {
        Self { thinking_type }
    }

    /// Enable thinking mode.
    pub fn enabled() -> Self {
        Self::new(ThinkingType::Enabled)
    }

    /// Disable thinking mode.
    pub fn disabled() -> Self {
        Self::new(ThinkingType::Disabled)
    }
}

/// Tool choice configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoice {
    /// Let the model decide whether to call tools
    Auto,
}

/// Response format type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseFormatType {
    /// Plain text response
    Text,
    /// JSON object response
    JsonObject,
}

/// Response format configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormat {
    /// The format type
    #[serde(rename = "type")]
    pub format_type: ResponseFormatType,
}

impl ResponseFormat {
    /// Create a new response format.
    pub fn new(format_type: ResponseFormatType) -> Self {
        Self { format_type }
    }

    /// Create a text response format.
    pub fn text() -> Self {
        Self::new(ResponseFormatType::Text)
    }

    /// Create a JSON object response format.
    pub fn json_object() -> Self {
        Self::new(ResponseFormatType::JsonObject)
    }
}

/// Stop sequence configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Stop {
    /// Single stop sequence
    Single(String),
    /// Multiple stop sequences (max 4)
    Multiple(Vec<String>),
}

/// Chat completion request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    /// The model to use for generation
    pub model: String,
    /// List of messages in the conversation
    pub messages: Vec<Message>,
    /// Audio output parameters (for TTS)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,
    /// Frequency penalty (-2.0 to 2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    /// Maximum completion tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,
    /// Presence penalty (-2.0 to 2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// Response format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// Stop sequences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,
    /// Enable streaming response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Thinking mode configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<Thinking>,
    /// Sampling temperature (0 to 1.5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Tool choice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// List of tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// Top-p sampling (0.01 to 1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Enable web search capability.
    ///
    /// **Note:** You must first enable the "联网服务插件" (Web Search Plugin)
    /// in the MiMo console before using this feature. If the plugin is not
    /// enabled, setting this to `true` will result in a 400 error.
    #[serde(skip_serializing_if = "Option::is_none", rename = "webSearchEnabled")]
    pub web_search_enabled: Option<bool>,
}

impl Default for ChatRequest {
    fn default() -> Self {
        Self {
            model: "mimo-v2-flash".to_string(),
            messages: Vec::new(),
            audio: None,
            frequency_penalty: None,
            max_completion_tokens: None,
            presence_penalty: None,
            response_format: None,
            stop: None,
            stream: None,
            thinking: None,
            temperature: None,
            tool_choice: None,
            tools: None,
            top_p: None,
            web_search_enabled: None,
        }
    }
}

impl ChatRequest {
    /// Create a new chat request with the specified model.
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            messages: Vec::new(),
            audio: None,
            frequency_penalty: None,
            max_completion_tokens: None,
            presence_penalty: None,
            response_format: None,
            stop: None,
            stream: None,
            thinking: None,
            temperature: None,
            tool_choice: None,
            tools: None,
            top_p: None,
            web_search_enabled: None,
        }
    }

    /// Create a chat request with the MiMo V2 Flash model.
    pub fn flash() -> Self {
        Self::new(Model::MiMoV2Flash.as_str())
    }

    /// Create a chat request with the MiMo V2 Pro model.
    pub fn pro() -> Self {
        Self::new(Model::MiMoV2Pro.as_str())
    }

    /// Create a chat request with the MiMo V2.5 Pro model.
    pub fn v25_pro() -> Self {
        Self::new(Model::MiMoV25Pro.as_str())
    }

    /// Create a chat request with the MiMo V2.5 model.
    pub fn v25() -> Self {
        Self::new(Model::MiMoV25.as_str())
    }

    /// Create a chat request with the MiMo V2 Omni model.
    pub fn omni() -> Self {
        Self::new(Model::MiMoV2Omni.as_str())
    }

    /// Create a chat request with the MiMo V2.5 TTS model (preset voices).
    pub fn v25_tts() -> Self {
        Self::new(Model::MiMoV25Tts.as_str())
    }

    /// Create a chat request with the MiMo V2.5 TTS VoiceDesign model.
    pub fn v25_tts_voicedesign() -> Self {
        Self::new(Model::MiMoV25TtsVoiceDesign.as_str())
    }

    /// Create a chat request with the MiMo V2.5 TTS VoiceClone model.
    pub fn v25_tts_voiceclone() -> Self {
        Self::new(Model::MiMoV25TtsVoiceClone.as_str())
    }

    /// Create a chat request with the MiMo V2 TTS model (legacy).
    pub fn tts() -> Self {
        Self::new(Model::MiMoV2Tts.as_str())
    }

    /// Set the model.
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Add a message to the conversation.
    pub fn message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
    }

    /// Add multiple messages to the conversation.
    pub fn messages(mut self, messages: Vec<Message>) -> Self {
        self.messages = messages;
        self
    }

    /// Add a system message.
    pub fn system(mut self, content: impl Into<String>) -> Self {
        self.messages
            .push(Message::system(MessageContent::Text(content.into())));
        self
    }

    /// Add a user message.
    pub fn user(mut self, content: impl Into<String>) -> Self {
        self.messages
            .push(Message::user(MessageContent::Text(content.into())));
        self
    }

    /// Add an assistant message.
    pub fn assistant(mut self, content: impl Into<String>) -> Self {
        self.messages
            .push(Message::assistant(MessageContent::Text(content.into())));
        self
    }

    /// Set audio output parameters (for TTS).
    pub fn audio(mut self, audio: Audio) -> Self {
        self.audio = Some(audio);
        self
    }

    /// Set the frequency penalty.
    pub fn frequency_penalty(mut self, penalty: f32) -> Self {
        self.frequency_penalty = Some(penalty);
        self
    }

    /// Set the maximum completion tokens.
    pub fn max_completion_tokens(mut self, tokens: u32) -> Self {
        self.max_completion_tokens = Some(tokens);
        self
    }

    /// Set the presence penalty.
    pub fn presence_penalty(mut self, penalty: f32) -> Self {
        self.presence_penalty = Some(penalty);
        self
    }

    /// Set the response format.
    pub fn response_format(mut self, format: ResponseFormat) -> Self {
        self.response_format = Some(format);
        self
    }

    /// Set the stop sequences.
    pub fn stop(mut self, stop: Stop) -> Self {
        self.stop = Some(stop);
        self
    }

    /// Enable or disable streaming.
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    /// Set the thinking mode.
    pub fn thinking(mut self, thinking: Thinking) -> Self {
        self.thinking = Some(thinking);
        self
    }

    /// Enable thinking mode.
    pub fn enable_thinking(mut self) -> Self {
        self.thinking = Some(Thinking::enabled());
        self
    }

    /// Disable thinking mode.
    pub fn disable_thinking(mut self) -> Self {
        self.thinking = Some(Thinking::disabled());
        self
    }

    /// Set the temperature.
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set the tool choice.
    pub fn tool_choice(mut self, choice: ToolChoice) -> Self {
        self.tool_choice = Some(choice);
        self
    }

    /// Add a tool.
    pub fn tool(mut self, tool: Tool) -> Self {
        if self.tools.is_none() {
            self.tools = Some(Vec::new());
        }
        self.tools.as_mut().unwrap().push(tool);
        self
    }

    /// Set the tools.
    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }

    /// Set the top-p.
    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Enable or disable web search.
    ///
    /// **Note:** You must first enable the "联网服务插件" (Web Search Plugin)
    /// in the MiMo console before using this feature. If the plugin is not
    /// enabled, setting this to `true` will result in a 400 error.
    pub fn web_search_enabled(mut self, enabled: bool) -> Self {
        self.web_search_enabled = Some(enabled);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_as_str() {
        assert_eq!(Model::MiMoV25Pro.as_str(), "mimo-v2.5-pro");
        assert_eq!(Model::MiMoV25.as_str(), "mimo-v2.5");
        assert_eq!(Model::MiMoV25Tts.as_str(), "mimo-v2.5-tts");
        assert_eq!(
            Model::MiMoV25TtsVoiceDesign.as_str(),
            "mimo-v2.5-tts-voicedesign"
        );
        assert_eq!(
            Model::MiMoV25TtsVoiceClone.as_str(),
            "mimo-v2.5-tts-voiceclone"
        );
        assert_eq!(Model::MiMoV2Pro.as_str(), "mimo-v2-pro");
        assert_eq!(Model::MiMoV2Omni.as_str(), "mimo-v2-omni");
        assert_eq!(Model::MiMoV2Tts.as_str(), "mimo-v2-tts");
        assert_eq!(Model::MiMoV2Flash.as_str(), "mimo-v2-flash");
    }

    #[test]
    fn test_model_from_str() {
        assert_eq!(Model::from("mimo-v2.5-pro"), Model::MiMoV25Pro);
        assert_eq!(Model::from("mimo-v2.5-tts"), Model::MiMoV25Tts);
        assert_eq!(Model::from("mimo-v2-pro"), Model::MiMoV2Pro);
        assert_eq!(Model::from("mimo-v2-flash"), Model::MiMoV2Flash);
        assert_eq!(Model::from("unknown"), Model::MiMoV2Flash);
    }

    #[test]
    fn test_model_display() {
        assert_eq!(format!("{}", Model::MiMoV25Pro), "mimo-v2.5-pro");
    }

    #[test]
    fn test_thinking() {
        let enabled = Thinking::enabled();
        assert_eq!(enabled.thinking_type, ThinkingType::Enabled);

        let disabled = Thinking::disabled();
        assert_eq!(disabled.thinking_type, ThinkingType::Disabled);
    }

    #[test]
    fn test_response_format() {
        let text = ResponseFormat::text();
        assert_eq!(text.format_type, ResponseFormatType::Text);

        let json = ResponseFormat::json_object();
        assert_eq!(json.format_type, ResponseFormatType::JsonObject);
    }

    #[test]
    fn test_chat_request_builder() {
        let request = ChatRequest::flash()
            .system("You are a helpful assistant.")
            .user("Hello!")
            .temperature(0.7)
            .max_completion_tokens(1024);

        assert_eq!(request.model, "mimo-v2-flash");
        assert_eq!(request.messages.len(), 2);
        assert_eq!(request.temperature, Some(0.7));
        assert_eq!(request.max_completion_tokens, Some(1024));
    }

    #[test]
    fn test_chat_request_serialization() {
        let request = ChatRequest::new("mimo-v2-flash")
            .user("Hello!")
            .temperature(0.5);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"model\":\"mimo-v2-flash\""));
        assert!(json.contains("\"temperature\":0.5"));
    }
}
