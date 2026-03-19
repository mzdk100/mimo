//! Tool types for the MiMo API.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// A tool that can be called by the model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Tool type
    #[serde(rename = "type")]
    pub tool_type: ToolType,
    /// Function tool (if type is function)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<FunctionTool>,
    /// Maximum number of keywords for web search (web_search only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keyword: Option<u32>,
    /// Force search even if model thinks it's unnecessary (web_search only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_search: Option<bool>,
    /// Limit the number of search results (web_search only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// User location for localized search (web_search only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_location: Option<UserLocation>,
}

impl Tool {
    /// Create a new function tool.
    pub fn function(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            tool_type: ToolType::Function,
            function: Some(FunctionTool::new(name, description)),
            max_keyword: None,
            force_search: None,
            limit: None,
            user_location: None,
        }
    }

    /// Create a new function tool with parameters.
    pub fn function_with_params(
        name: impl Into<String>,
        description: impl Into<String>,
        parameters: HashMap<String, Value>,
    ) -> Self {
        Self {
            tool_type: ToolType::Function,
            function: Some(FunctionTool::with_params(name, description, parameters)),
            max_keyword: None,
            force_search: None,
            limit: None,
            user_location: None,
        }
    }

    /// Create a web search tool.
    ///
    /// **Note:** You must first enable the "联网服务插件" (Web Search Plugin)
    /// in the MiMo console and set `web_search_enabled(true)` in your
    /// `ChatRequest` before using this feature.
    pub fn web_search() -> Self {
        Self {
            tool_type: ToolType::WebSearch,
            function: None,
            max_keyword: None,
            force_search: None,
            limit: None,
            user_location: None,
        }
    }

    /// Set maximum number of keywords for web search.
    pub fn max_keyword(mut self, max: u32) -> Self {
        self.max_keyword = Some(max);
        self
    }

    /// Set whether to force search.
    pub fn force_search(mut self, force: bool) -> Self {
        self.force_search = Some(force);
        self
    }

    /// Set the result limit for web search.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the user location for localized search.
    pub fn user_location(mut self, location: UserLocation) -> Self {
        self.user_location = Some(location);
        self
    }

    /// Set whether to use strict mode.
    pub fn strict(mut self, strict: bool) -> Self {
        if let Some(ref mut function) = self.function {
            function.strict = Some(strict);
        }
        self
    }
}

/// Tool type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ToolType {
    /// Function tool
    Function,
    /// Web search tool
    WebSearch,
}

/// User location for localized web search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLocation {
    /// Location type (e.g., "approximate")
    #[serde(rename = "type")]
    pub location_type: String,
    /// Country name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Region/Province name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// City name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
}

impl UserLocation {
    /// Create a new user location.
    pub fn new(location_type: impl Into<String>) -> Self {
        Self {
            location_type: location_type.into(),
            country: None,
            region: None,
            city: None,
        }
    }

    /// Create an approximate location.
    pub fn approximate() -> Self {
        Self::new("approximate")
    }

    /// Set the country.
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Set the region.
    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    /// Set the city.
    pub fn city(mut self, city: impl Into<String>) -> Self {
        self.city = Some(city.into());
        self
    }
}

/// Function tool definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionTool {
    /// Function name (alphanumeric, underscore, hyphen; max 64 chars)
    pub name: String,
    /// Function description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Function parameters (JSON Schema)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Value>>,
    /// Whether to use strict mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

impl FunctionTool {
    /// Create a new function tool.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: Some(description.into()),
            parameters: None,
            strict: None,
        }
    }

    /// Create a new function tool with parameters.
    pub fn with_params(
        name: impl Into<String>,
        description: impl Into<String>,
        parameters: HashMap<String, Value>,
    ) -> Self {
        Self {
            name: name.into(),
            description: Some(description.into()),
            parameters: Some(parameters),
            strict: None,
        }
    }

    /// Set the parameters.
    pub fn parameters(mut self, parameters: HashMap<String, Value>) -> Self {
        self.parameters = Some(parameters);
        self
    }

    /// Set strict mode.
    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = Some(strict);
        self
    }
}

/// Builder for creating JSON Schema parameters.
#[derive(Debug, Clone, Default)]
pub struct ParameterBuilder {
    params: HashMap<String, Value>,
}

impl ParameterBuilder {
    /// Create a new parameter builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the schema type.
    pub fn type_object(mut self) -> Self {
        self.params.insert("type".to_string(), Value::String("object".to_string()));
        self
    }

    /// Add a required property.
    pub fn required_property(mut self, name: &str, schema: HashMap<String, Value>) -> Self {
        // Add to properties
        let properties = self
            .params
            .entry("properties".to_string())
            .or_insert_with(|| Value::Object(serde_json::Map::new()));
        
        if let Value::Object(props) = properties {
            props.insert(name.to_string(), Value::Object(schema.into_iter().collect()));
        }

        // Add to required
        let required = self
            .params
            .entry("required".to_string())
            .or_insert_with(|| Value::Array(Vec::new()));
        
        if let Value::Array(req) = required {
            req.push(Value::String(name.to_string()));
        }

        self
    }

    /// Add an optional property.
    pub fn optional_property(mut self, name: &str, schema: HashMap<String, Value>) -> Self {
        let properties = self
            .params
            .entry("properties".to_string())
            .or_insert_with(|| Value::Object(serde_json::Map::new()));
        
        if let Value::Object(props) = properties {
            props.insert(name.to_string(), Value::Object(schema.into_iter().collect()));
        }

        self
    }

    /// Build the parameters.
    pub fn build(self) -> HashMap<String, Value> {
        self.params
    }
}

/// Helper functions for creating property schemas.
pub mod schema {
    use serde_json::Value;
    use std::collections::HashMap;

    /// Create a string property schema.
    pub fn string() -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("type".to_string(), Value::String("string".to_string()));
        map
    }

    /// Create a string property with description.
    pub fn string_with_description(desc: &str) -> HashMap<String, Value> {
        let mut map = string();
        map.insert("description".to_string(), Value::String(desc.to_string()));
        map
    }

    /// Create a number property schema.
    pub fn number() -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("type".to_string(), Value::String("number".to_string()));
        map
    }

    /// Create an integer property schema.
    pub fn integer() -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("type".to_string(), Value::String("integer".to_string()));
        map
    }

    /// Create a boolean property schema.
    pub fn boolean() -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("type".to_string(), Value::String("boolean".to_string()));
        map
    }

    /// Create an array property schema.
    pub fn array(items: HashMap<String, Value>) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("type".to_string(), Value::String("array".to_string()));
        map.insert("items".to_string(), Value::Object(items.into_iter().collect()));
        map
    }

    /// Create an enum property schema.
    pub fn enum_values(values: &[&str]) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("type".to_string(), Value::String("string".to_string()));
        map.insert(
            "enum".to_string(),
            Value::Array(values.iter().map(|v| Value::String(v.to_string())).collect()),
        );
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let tool = Tool::function("get_weather", "Get the current weather");
        assert_eq!(tool.tool_type, ToolType::Function);
        assert!(tool.function.is_some());
    }

    #[test]
    fn test_web_search_tool() {
        let tool = Tool::web_search();
        assert_eq!(tool.tool_type, ToolType::WebSearch);
        assert!(tool.function.is_none());
    }

    #[test]
    fn test_tool_serialization() {
        let tool = Tool::function("get_weather", "Get weather info");
        let json = serde_json::to_string(&tool).unwrap();
        assert!(json.contains("\"type\":\"function\""));
        assert!(json.contains("\"name\":\"get_weather\""));
    }

    #[test]
    fn test_tool_with_parameters() {
        let mut params = HashMap::new();
        params.insert("type".to_string(), Value::String("object".to_string()));
        
        let tool = Tool::function_with_params(
            "get_weather",
            "Get weather for a location",
            params.clone(),
        );
        
        assert!(tool.function.as_ref().unwrap().parameters.is_some());
    }

    #[test]
    fn test_parameter_builder() {
        let params = ParameterBuilder::new()
            .type_object()
            .required_property("location", schema::string())
            .build();

        assert_eq!(params.get("type").unwrap(), &Value::String("object".to_string()));
    }

    #[test]
    fn test_schema_helpers() {
        let s = schema::string();
        assert_eq!(s.get("type").unwrap(), &Value::String("string".to_string()));

        let n = schema::number();
        assert_eq!(n.get("type").unwrap(), &Value::String("number".to_string()));

        let e = schema::enum_values(&["a", "b", "c"]);
        assert!(e.contains_key("enum"));
    }

    #[test]
    fn test_strict_mode() {
        let tool = Tool::function("test", "test function").strict(true);
        assert_eq!(tool.function.unwrap().strict, Some(true));
    }

    #[test]
    fn test_web_search_with_options() {
        let tool = Tool::web_search()
            .max_keyword(3)
            .force_search(true)
            .limit(5);
        
        assert_eq!(tool.tool_type, ToolType::WebSearch);
        assert_eq!(tool.max_keyword, Some(3));
        assert_eq!(tool.force_search, Some(true));
        assert_eq!(tool.limit, Some(5));
    }

    #[test]
    fn test_web_search_serialization() {
        let tool = Tool::web_search()
            .max_keyword(3)
            .force_search(true);
        
        let json = serde_json::to_string(&tool).unwrap();
        assert!(json.contains("\"type\":\"web_search\""));
        assert!(json.contains("\"max_keyword\":3"));
        assert!(json.contains("\"force_search\":true"));
    }

    #[test]
    fn test_user_location() {
        let location = UserLocation::approximate()
            .country("China")
            .region("Hubei")
            .city("Wuhan");
        
        assert_eq!(location.location_type, "approximate");
        assert_eq!(location.country, Some("China".to_string()));
        assert_eq!(location.region, Some("Hubei".to_string()));
        assert_eq!(location.city, Some("Wuhan".to_string()));
    }

    #[test]
    fn test_user_location_serialization() {
        let location = UserLocation::approximate()
            .country("China")
            .city("Beijing");
        
        let json = serde_json::to_string(&location).unwrap();
        assert!(json.contains("\"type\":\"approximate\""));
        assert!(json.contains("\"country\":\"China\""));
        assert!(json.contains("\"city\":\"Beijing\""));
        assert!(!json.contains("region")); // None should not be serialized
    }

    #[test]
    fn test_web_search_with_location() {
        let tool = Tool::web_search()
            .user_location(
                UserLocation::approximate()
                    .country("China")
                    .city("Shanghai"),
            );
        
        assert!(tool.user_location.is_some());
        let loc = tool.user_location.unwrap();
        assert_eq!(loc.country, Some("China".to_string()));
        assert_eq!(loc.city, Some("Shanghai".to_string()));
    }
}
