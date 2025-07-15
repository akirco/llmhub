use crate::api::providers::ApiType;
use crate::api::request::ApiRequest;
use crate::api::response::{ApiResponse, StreamChunk};
use crate::utils::error::{LlmHubError, Result};
use futures::{Stream, StreamExt};
use reqwest::Client as ReqwestClient;
use reqwest_eventsource::{Event, EventSource};
use std::pin::Pin;

/// A stateless, low-level client for interacting with LLM provider APIs.
#[derive(Debug, Clone)]
pub struct Client {
    http_client: ReqwestClient,
    api_key: String,
}

impl Client {
    /// Creates a new `Client`.
    pub fn new(api_key: String) -> Self {
        Self {
            http_client: ReqwestClient::new(),
            api_key,
        }
    }

    /// Sends a standard, non-streaming chat request.
    pub async fn chat(&self, request: &ApiRequest) -> Result<ApiResponse> {
        let provider = request.model.provider();
        let endpoint_config = provider.get_endpoint_config();
        let url = endpoint_config.get_url(ApiType::Chat)?;

        let response = self
            .http_client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(request)
            .send()
            .await?;

        if response.status().is_success() {
            response.json().await.map_err(LlmHubError::from)
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown API error".to_string());
            Err(LlmHubError::ApiError(error_text))
        }
    }

    /// Sends a streaming chat request.
    pub fn chat_stream(
        &self,
        request: &ApiRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamChunk>> + Send>>> {
        let provider = request.model.provider();
        let endpoint_config = provider.get_endpoint_config();
        let url = endpoint_config.get_url(ApiType::Chat)?;

        let mut es = EventSource::new(
            self.http_client
                .post(url)
                .bearer_auth(&self.api_key)
                .json(&request),
        )
        .expect("Failed to create EventSource");

        let stream = async_stream::stream! {
            while let Some(event) = es.next().await {
                match event {
                    Ok(Event::Open) => continue,
                    Ok(Event::Message(message)) => {

                        if message.data == "[DONE]" {
                            break;
                        }
                        let chunk: StreamChunk = match serde_json::from_str(&message.data) {
                            Ok(c) => {
                                c
                            },
                            Err(e) => {
                                yield Err(LlmHubError::SerializationError(e));
                                continue;
                            }
                        };
                        yield Ok(chunk);
                    }
                    Err(e) => {
                        es.close();
                        yield Err(LlmHubError::StreamError(e.to_string()));
                        break;
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }
}
