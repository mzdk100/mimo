//! Web search example demonstrating real-time information retrieval.
//!
//! Usage: XIAOMI_API_KEY=your_key cargo run --example web_search

use mimo_api::{ChatRequest, Client, Tool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable
    let client = Client::from_env()?;

    println!("Creating web search request...\n");

    // Create a chat request with web search tool
    let request = ChatRequest::flash()
        .system(
            "You are a helpful assistant with access to web search. ".to_owned()
                + "Use it to provide up-to-date information.",
        )
        .user("What are the latest developments in AI technology this week?")
        .web_search_enabled(true)
        .tool(
            Tool::web_search()
                .max_keyword(3)
                .force_search(true)
                .limit(5),
        );

    println!("Sending request to MiMo API with web search enabled...\n");
    let response = client.chat(request).await?;

    // Print the response
    println!("Response:");
    println!("--------");
    println!("{}", response.choices[0].message.content);

    // Print web search annotations if available
    if let Some(annotations) = &response.choices[0].message.annotations {
        println!("\nSources:");
        println!("--------");
        for annotation in annotations {
            if let Some(title) = &annotation.title {
                println!("- {}", title);
            }
            if let Some(url) = &annotation.url {
                println!("  URL: {}", url);
            }
            if let Some(site_name) = &annotation.site_name {
                println!("  Site: {}", site_name);
            }
            if let Some(summary) = &annotation.summary {
                // Truncate summary if too long (by characters, not bytes)
                let truncated = if summary.chars().count() > 200 {
                    format!("{}...", summary.chars().take(200).collect::<String>())
                } else {
                    summary.clone()
                };
                println!("  Summary: {}", truncated);
            }
            if let Some(publish_time) = &annotation.publish_time {
                println!("  Published: {}", publish_time);
            }
            println!();
        }
    }

    // Print web search usage
    if let Some(usage) = &response.usage
        && let Some(web_usage) = &usage.web_search_usage
    {
        println!("Web Search Usage:");
        println!("  API calls: {}", web_usage.tool_usage);
        println!("  Pages retrieved: {}", web_usage.page_usage);
    }

    Ok(())
}
