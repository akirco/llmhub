//! Error handling module for LLM operations

use thiserror::Error;
#[derive(Debug, Error)]
pub enum LLMError {
    #[error("HTTP request failed: {0}")] RequestError(#[from] reqwest::Error),

    #[error("Rate limit exceeded. Please wait {0} seconds before retrying")] RateLimitError(u64),

    #[error("Invalid configuration: {0}")] ConfigError(String),

    #[error("API error: {0}")] ApiError(String),

    #[error("Parse error: {0}")] ParseError(String),

    #[error(transparent)] ProviderError(#[from] crate::api::providers::ModelProviderError),

    #[error("Response decode error: {0}")] DecodeError(String),

    #[error("Stream processing error: {0}")] StreamError(String),
}

impl LLMError {
    pub fn user_friendly_message(&self) -> String {
        match self {
            LLMError::RequestError(e) => {
                if e.is_timeout() {
                    "Request timed out, please check your network connection".to_string()
                } else if e.is_connect() {
                    "Failed to connect to API server, please check your network".to_string()
                } else {
                    format!("HTTP request failure: {}", e)
                }
            }
            LLMError::RateLimitError(seconds) => {
                format!("API rate limit exceeded. Please wait {} seconds before retrying", seconds)
            }
            LLMError::ConfigError(msg) =>
                format!("Configuration error: {}. Please check your API keys and settings", msg),
            LLMError::ApiError(msg) => {
                if msg.contains("invalid_api_key") || msg.contains("authentication") {
                    "Invalid API key. Please verify your credentials".to_string()
                } else if msg.contains("insufficient_quota") {
                    "API quota exhausted. Please check your account balance".to_string()
                } else {
                    format!("API operation failed: {}", msg)
                }
            }
            LLMError::ParseError(msg) => format!("Data parsing failed: {}", msg),
            LLMError::ProviderError(e) =>
                match e {
                    crate::api::providers::ModelProviderError::UnsupportedApiType(api_type) =>
                        format!("Provider {} does not support this model", api_type),
                }
            LLMError::DecodeError(msg) =>
                format!("Failed to decode API response: {}. This might be a temporary issue, please try again.", msg),
            LLMError::StreamError(msg) =>
                format!("Stream processing error: {}. The connection might have been interrupted.", msg),
        }
    }
}

/// Result type alias for LLM operations
pub type Result<T> = std::result::Result<T, LLMError>;
