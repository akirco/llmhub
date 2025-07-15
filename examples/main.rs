use anyhow::Result;
use dotenv::dotenv;
use futures::StreamExt;
use llmhub::api::client::Client;
use llmhub::api::message::{Message, Role};
use llmhub::api::request::{ApiRequest, RequestOptions};
use llmhub::api::session::Session;
use llmhub::models::models::Model;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::env;
use std::io::{stdout, Write};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let client = Client::new(api_key);
    let options = RequestOptions {
        temperature: Some(0.7),
        ..Default::default()
    };

    let mut session = Session::new();
    let mut rl = DefaultEditor::new()?;

    println!("Starting interactive chat session. Type 'exit' to end.");

    loop {
        let readline = rl.readline(">> User: ");
        match readline {
            Ok(line) => {
                if line.trim().to_lowercase() == "exit" {
                    break;
                }

                session.add_message(Message::new(Role::User, line));

                let stream_request = ApiRequest::new(
                    Model::Deepseek(llmhub::models::models::DEEPSEEK::R1Siliconflow),
                    Some(&session),
                )
                .with_options(options.clone())
                .stream(true);

                print!(">> Assistant: ");
                stdout().flush()?;

                let mut full_response = String::new();

                match client.chat_stream(&stream_request) {
                    Ok(mut stream) => {
                        while let Some(chunk_result) = stream.next().await {
                            match chunk_result {
                                Ok(chunk) => {
                                    for choice in chunk.choices {
                                        if let Some(content) = &choice.delta.content {
                                            print!("{}", content);
                                            stdout().flush()?;
                                            full_response.push_str(content);
                                        }
                                        if let Some(reasoning) = &choice.delta.reasoning_content {
                                            print!("{}", reasoning);
                                            stdout().flush()?;
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("\nStream Error: {}", e);
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error starting stream: {}", e);
                    }
                }
                println!(); // Add a newline after the response
                session.add_message(Message::new(Role::Assistant, full_response));
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("End of File");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}