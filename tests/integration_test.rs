//! Integration tests for the MiMo API client.
//!
//! These tests require a valid XIAOMI_API_KEY environment variable.
//! Run with: cargo test --test integration_test -- --ignored

use mimo_api::{ChatRequest, Client, Model, Tool};

#[tokio::test]
#[ignore = "Requires XIAOMI_API_KEY environment variable"]
async fn test_basic_chat() {
    let client = Client::from_env().expect("XIAOMI_API_KEY must be set");

    let request = ChatRequest::new(Model::MiMoV2Flash.as_str())
        .system("You are a helpful assistant. Be very brief.")
        .user("Say 'Hello' in one word.");

    let response = client
        .chat(request)
        .await
        .expect("Request should succeed");

    assert!(!response.choices.is_empty());
    assert!(!response.choices[0].message.content.is_empty());
}

#[tokio::test]
#[ignore = "Requires XIAOMI_API_KEY environment variable"]
async fn test_streaming_chat() {
    use futures::StreamExt;

    let client = Client::from_env().expect("XIAOMI_API_KEY must be set");

    let request = ChatRequest::new(Model::MiMoV2Flash.as_str())
        .user("Count from 1 to 5.");

    let stream = client.chat_stream(request).await.expect("Stream should start");

    let mut chunks = 0;
    futures::pin_mut!(stream);
    while let Some(chunk) = stream.next().await {
        if chunk.is_ok() {
            chunks += 1;
        }
    }

    assert!(chunks > 0, "Should receive at least one chunk");
}

#[tokio::test]
#[ignore = "Requires XIAOMI_API_KEY environment variable"]
async fn test_tool_calling() {
    let client = Client::from_env().expect("XIAOMI_API_KEY must be set");

    let tool = Tool::function(
        "get_time",
        "Get the current time for a given timezone",
    );

    let request = ChatRequest::new(Model::MiMoV2Flash.as_str())
        .user("What time is it in Tokyo?")
        .tool(tool);

    let response = client
        .chat(request)
        .await
        .expect("Request should succeed");

    assert!(!response.choices.is_empty());
}

#[tokio::test]
#[ignore = "Requires XIAOMI_API_KEY environment variable"]
async fn test_web_search() {
    let client = Client::from_env().expect("XIAOMI_API_KEY must be set");

    let request = ChatRequest::new(Model::MiMoV2Flash.as_str())
        .user("What is the latest news about AI?")
        .tool(Tool::web_search());

    let response = client
        .chat(request)
        .await
        .expect("Request should succeed");

    assert!(!response.choices.is_empty());
}
