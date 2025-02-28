use std::io::Write;

use futures::StreamExt;
use llmhub::{
    LLMClient,
    api::{ config::ProviderConfig, message::Prompt, providers::ApiProvider },
    models::models::{ DEEPSEEK, Model },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建配置
    let config = ProviderConfig::new(
        ApiProvider::Deepseek,
        None, // 使用默认 API URL
        Some("sk-66be3533e7d74e659e4f3afa886eb766".to_string()) // 替换为你的 API key
    );

    // 创建客户端
    let client = LLMClient::new(config);

    // 创建一个聊天会话
    let model = Model::Deepseek(DEEPSEEK::R1Official);
    let mut session = client.create_chat_session(model.clone(), None).await;

    // 准备用户消息
    let message = Prompt::user("你好，请介绍一下你自己。");

    println!("发送消息到 {}...", model.as_str());

    // 发送流式请求
    let mut stream = client.chat_with_stream(model.clone(), message, None, None).await?;

    // 处理流式响应
    let mut response_text = String::new();
    while let Some(response_result) = stream.next().await {
        match response_result {
            Ok(response) => {
                if let Some(content) = response.content() {
                    // print!("{}", content);
                    std::io::stdout().flush()?;
                    response_text.push_str(&content);
                }
            }
            Err(e) => {
                eprintln!("\n错误: {}", e);
                break;
            }
        }
    }
    println!("\n");

    // 将响应添加到会话历史
    session.add_message(Prompt::assistant(response_text));

    // 发送第二条消息
    let second_message = Prompt::user("你能用中文写一首诗吗？");
    session.add_message(second_message.clone());

    println!("发送第二条消息...");

    // 使用会话中的所有消息历史
    let mut stream = client.chat_with_stream(model, second_message, None, None).await?;

    while let Some(response_result) = stream.next().await {
        match response_result {
            Ok(response) => {
                if let Some(content) = response.content() {
                    print!("{}", content);
                    std::io::stdout().flush()?;
                }
            }
            Err(e) => {
                eprintln!("\n错误: {}", e);
                break;
            }
        }
    }
    println!("\n");

    Ok(())
}
