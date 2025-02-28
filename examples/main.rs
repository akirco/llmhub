use std::io::Write;

use futures::StreamExt;
use llmhub::{
    LLMClient,
    api::{ config::ProviderConfig, message::Prompt, providers::ApiProvider },
    models::models::{ DEEPSEEK, Model },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ProviderConfig::new(
        ApiProvider::Volcengine,
        None,
        Some("a5926646-a0b1-4d4a-b617-6488ca3xdabc".to_string())
    );

    let client = LLMClient::new(config);

    let model = Model::Deepseek(DEEPSEEK::R1Volcengine);

    let message = Prompt::user("一句话介绍Rust语言");

    println!("Send stream message to {}", model.as_str());

    let mut stream = client.chat_with_stream(model.clone(), message.clone(), None, None).await?;

    let mut main_content = String::new();
    let mut reasoning_content = String::new();

    while let Some(response_result) = stream.next().await {
        match response_result {
            Ok(response) => {
                if let Some(content) = response.main_content() {
                    print!("{}", content);
                    std::io::stdout().flush()?;
                    main_content.push_str(&content);
                }

                if let Some(reasoning) = response.reasoning_content() {
                    print!("{}", reasoning);
                    std::io::stdout().flush()?;
                    reasoning_content.push_str(&reasoning);
                }
            }
            Err(e) => {
                eprintln!("\nError: {}", e.user_friendly_message());
                break;
            }
        }
    }

    println!("Main Content: {}", main_content);
    if !reasoning_content.is_empty() {
        println!("Reasoning: {}", reasoning_content);
    }

    println!("\nsend general message to {}", model.as_str());
    match client.chat_without_stream(model.clone(), message, None, None).await {
        Ok(response) => {
            if let Some(content) = response.message_content() {
                println!("Main Content: {}", content);
            }
            if let Some(reasoning) = response.message_reasoning() {
                println!("Reasoning: {}", reasoning);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
