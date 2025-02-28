pub mod api;
pub mod models;
pub mod utils;

use crate::{
    api::{
        config::ProviderConfig, message::Prompt, providers::ApiProvider, request::RequestBody,
        request::RequestHeader, request::RequestOptions, request::RequestUrl, response::Response,
        session::ChatSession,
    },
    models::models::Model,
    utils::{error::LLMError, error::Result},
};
use bytes::Bytes;
use futures::stream::once;
use futures::{Stream, StreamExt, future};
use log;
use reqwest::Client as HttpClient;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct LLMClient {
    http_client: HttpClient,
    config: Arc<RwLock<ProviderConfig>>,
    rate_limiter: Arc<RwLock<std::collections::HashMap<String, tokio::time::Instant>>>,
}

impl LLMClient {
    /// Creates a new LLM client with the specified provider configuration
    pub fn new(config: ProviderConfig) -> Self {
        Self {
            http_client: HttpClient::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_else(|_| HttpClient::new()),
            config: Arc::new(RwLock::new(config)),
            rate_limiter: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Sends a streaming request and returns a stream of responses
    pub async fn send_stream_request(
        &self,
        request: RequestBody,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Response>> + Send>>> {
        // Pre-request validation
        self.check_rate_limit(&request.provider).await?;

        // Build request URL using existing RequestUrl struct
        let request_url = RequestUrl::new(&request.provider, request.api_type)?;

        println!("{}", request_url.url);
        println!("{}", serde_json::to_string_pretty(&request).unwrap());

        // Get API key and build headers using RequestHeader struct
        let api_key = self
            .config
            .read()
            .await
            .api_key
            .as_ref()
            .ok_or_else(|| LLMError::ConfigError("API key not set".to_string()))?
            .clone();

        let headers = RequestHeader::new(api_key);

        // Send HTTP request
        let response = self
            .http_client
            .post(&request_url.url)
            .header("Authorization", headers.authorization)
            .header("Content-Type", headers.content_type.unwrap_or_default())
            .header("Accept", headers.accept.unwrap_or_default())
            .json(&request)
            .send()
            .await
            .map_err(LLMError::RequestError)?;

        // Check response status
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(LLMError::ApiError(error_text));
        }

        // Create byte stream from response
        let stream = response
            .bytes_stream()
            .flat_map(|chunk_result| Self::process_chunk(chunk_result));

        Ok(Box::pin(stream))
    }

    /// Helper method to process each chunk of the stream
    fn process_chunk(
        chunk_result: std::result::Result<Bytes, reqwest::Error>,
    ) -> Pin<Box<dyn Stream<Item = Result<Response>> + Send>> {
        match chunk_result {
            Ok(chunk) => {
                let chunk_str = String::from_utf8_lossy(&chunk);
                let messages: Vec<String> = chunk_str
                    .lines()
                    .filter(|line| !line.is_empty())
                    .map(|line| line.to_string())
                    .collect();

                Box::pin(futures::stream::iter(
                    messages
                        .into_iter()
                        .filter_map(|msg| Self::parse_sse_message(&msg)),
                ))
            }
            Err(e) => Box::pin(once(future::ready(Err(LLMError::RequestError(e))))),
        }
    }

    /// Helper method to parse SSE messages
    fn parse_sse_message(message: &str) -> Option<Result<Response>> {
        if !message.starts_with("data: ") {
            return None;
        }

        let data = &message[6..];
        if data == "[DONE]" {
            return None;
        }

        match serde_json::from_str::<Response>(data) {
            Ok(response) => Some(Ok(response)),
            Err(e) => {
                log::warn!("Failed to parse response: {} (error: {})", data, e);
                Some(Err(LLMError::ParseError(e.to_string())))
            }
        }
    }

    /// Checks if the request is within rate limits
    async fn check_rate_limit(&self, provider: &ApiProvider) -> Result<()> {
        let mut rate_limiter = self.rate_limiter.write().await;
        let now = tokio::time::Instant::now();

        if let Some(last_request) = rate_limiter.get(&provider.to_string()) {
            if now.duration_since(*last_request).as_secs() < 1 {
                return Err(crate::utils::error::LLMError::RateLimitError(1));
            }
        }

        rate_limiter.insert(provider.to_string(), now);
        Ok(())
    }

    /// Chat with stream using specific provider
    pub async fn chat_with_stream(
        &self,
        model: Model,
        message: Prompt,
        provider: Option<ApiProvider>,
        options: Option<RequestOptions>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Response>> + Send>>> {
        let provider = provider.unwrap_or_else(|| model.provider());

        // Create request body using new RequestBodyBuilder
        let request = RequestBody::new()
            .model(model)
            .provider(provider)
            .options(options)
            .api_type(api::providers::ApiType::Chat)
            .add_message(message)
            .stream(true)
            .build()?;

        // Send streaming request
        self.send_stream_request(request).await
    }

    pub async fn create_chat_session(
        &self,
        model: Model,
        provider: Option<ApiProvider>,
    ) -> ChatSession {
        ChatSession::new(model, provider)
    }

    pub async fn update_config(&self, new_config: ProviderConfig) {
        let mut config = self.config.write().await;
        *config = new_config;
    }

    /// Sends a non-streaming request and returns a single response
    async fn send_request(&self, request: RequestBody) -> Result<Response> {
        // Pre-request validation
        self.check_rate_limit(&request.provider).await?;

        // Build request URL
        let request_url = RequestUrl::new(&request.provider, request.api_type)?;
        println!("{}", request_url.url);
        println!("{}", serde_json::to_string_pretty(&request).unwrap());
        // Get API key and build headers
        let api_key = self
            .config
            .read()
            .await
            .api_key
            .as_ref()
            .ok_or_else(|| LLMError::ConfigError("API key not set".to_string()))?
            .clone();

        let headers = RequestHeader::new(api_key);

        // Send HTTP request
        let response = self
            .http_client
            .post(&request_url.url)
            .header("Authorization", headers.authorization)
            .header("Content-Type", headers.content_type.unwrap_or_default())
            .header("Accept", headers.accept.unwrap_or_default())
            .json(&request)
            .send()
            .await
            .map_err(LLMError::RequestError)?;

        // Check response status
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(LLMError::ApiError(error_text));
        }

        // Parse response
        let response_data = response
            .json::<Response>()
            .await
            .map_err(|e| LLMError::ParseError(e.to_string()))?;

        Ok(response_data)
    }

    /// Chat without stream using specific provider
    pub async fn chat_without_stream(
        &self,
        model: Model,
        message: Prompt,
        provider: Option<ApiProvider>,
        options: Option<RequestOptions>,
    ) -> Result<Response> {
        let provider = provider.unwrap_or_else(|| model.provider());

        // Create request body
        let request = RequestBody::new()
            .model(model)
            .provider(provider)
            .options(options)
            .api_type(api::providers::ApiType::Chat)
            .add_message(message)
            .stream(false)
            .build()?;

        // Send request
        self.send_request(request).await
    }
}
