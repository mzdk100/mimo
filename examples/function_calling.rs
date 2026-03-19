//! Function calling example demonstrating tool use with MiMo API.
//!
//! Usage: XIAOMI_API_KEY=your_key cargo run --example function_calling

use mimo_api::{schema, ChatRequest, Client, Message, ParameterBuilder, Tool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable
    let client = Client::from_env()?;

    println!("Creating function calling example...\n");

    // Define a weather function tool
    let weather_params = ParameterBuilder::new()
        .type_object()
        .required_property("location", schema::string_with_description("The city and country"))
        .required_property("unit", schema::enum_values(&["celsius", "fahrenheit"]))
        .build();

    let weather_tool = Tool::function_with_params(
        "get_weather",
        "Get the current weather for a location",
        weather_params,
    );

    // Create a chat request with the tool
    let request = ChatRequest::flash()
        .system("You are a helpful weather assistant.")
        .user("What's the weather like in Tokyo, Japan in celsius?")
        .tool(weather_tool);

    println!("Sending request to MiMo API...\n");
    let response = client.chat(request).await?;

    // Check if the model wants to call a tool
    if let Some(tool_calls) = &response.choices[0].message.tool_calls {
        println!("Model requested tool calls:");
        for tool_call in tool_calls {
            println!(
                "  Function: {}",
                tool_call.function.name
            );
            println!(
                "  Arguments: {}",
                tool_call.function.arguments
            );

            // Simulate calling the weather API
            let result = simulate_weather_api(&tool_call.function.arguments);

            // Create a new request with the tool result
            println!("\nSending tool result back...\n");
            let follow_up = ChatRequest::flash()
                .system("You are a helpful weather assistant.")
                .user("What's the weather like in Tokyo, Japan in celsius?")
                .message(Message::assistant(response.choices[0].message.content.clone())
                    .with_tool_calls(tool_calls.clone()))
                .message(Message::tool(&tool_call.id, result));

            let final_response = client.chat(follow_up).await?;
            println!("Final response:");
            println!("{}", final_response.choices[0].message.content);
        }
    } else {
        println!("Response:");
        println!("{}", response.choices[0].message.content);
    }

    Ok(())
}

/// Simulate a weather API call
fn simulate_weather_api(args: &str) -> String {
    // Parse the arguments
    let parsed: serde_json::Value = serde_json::from_str(args).unwrap_or_default();
    let location = parsed["location"].as_str().unwrap_or("Unknown");
    let _unit = parsed["unit"].as_str().unwrap_or("celsius");

    // Return simulated weather data
    format!(
        r#"{{"location": "{}", "temperature": 22, "condition": "Partly cloudy", "humidity": 65}}"#,
        location
    )
}
