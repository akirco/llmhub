use std::io::Write;

use futures::StreamExt;
use llmhub::{
    LLMClient,
    api::{config::ProviderConfig, message::Prompt, providers::ApiProvider},
    models::models::{DEEPSEEK, Model},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建配置
    let config = ProviderConfig::new(
        ApiProvider::Deepseek,
        None,                                                    // 使用默认 API URL
        Some("sk-66be3533e7d74e659e4f3afa886eb766".to_string()), // 替换为你的 API key
    );

    // 创建客户端
    let client = LLMClient::new(config);

    // 创建一个聊天会话
    let model = Model::Deepseek(DEEPSEEK::R1Official);

    // 准备用户消息
    let message = Prompt::user("你是?");

    println!("发送流式消息到 {}...", model.as_str());

    // 发送流式请求
    let mut stream = client
        .chat_with_stream(model.clone(), message, None, None)
        .await?;

    // 处理流式响应
    let mut response_text = String::new();
    while let Some(response_result) = stream.next().await {
        match response_result {
            Ok(response) => {
                if let Some(content) = response.content() {
                    print!("{}", content);
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
    println!("\n流式响应结果: {}\n", response_text);

    // 非流式对话示例
    println!("发送非流式消息到 {}...", model.as_str());
    let message = Prompt::user("请用一句话介绍Rust语言");

    // 发送非流式请求
    match client
        .chat_without_stream(model.clone(), message, None, None)
        .await
    {
        Ok(response) => {
            if let Some(content) = response.message() {
                println!("非流式响应结果: {}\n", content);
            }
        }
        Err(e) => {
            eprintln!("错误: {}", e);
        }
    }

    Ok(())
}
