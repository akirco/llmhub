//! Error handling module for LLM operations

use thiserror::Error;

/// Main error type for LLM operations
#[derive(Debug, Error)]
pub enum LLMError {
    /// HTTP request failure (via reqwest crate)
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    /// Rate limit exceeded with retry timing
    #[error("Rate limit exceeded. Please wait {0} seconds before retrying")]
    RateLimitError(u64),

    /// Configuration related errors
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
    /// API response errors
    #[error("API error: {0}")]
    ApiError(String),
    /// Network connectivity issues
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Response parsing failures
    #[error("Parse error: {0}")]
    ParseError(String),
    /// Model-specific errors
    #[error("Model error: {0}")]
    ModelError(String),

    /// Token quota exceeded
    #[error("Token limit exceeded: {0}")]
    TokenLimitError(String),
    /// Concurrent request limit reached
    #[error("Concurrent request limit reached: {0}")]
    ConcurrencyLimitError(String),

    /// Provider-specific errors
    #[error(transparent)]
    ProviderError(#[from] crate::api::providers::ModelProviderError),
}

/// Result type alias for LLM operations
pub type Result<T> = std::result::Result<T, LLMError>;

impl LLMError {
    /// Converts technical error into user-friendly message
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

            LLMError::NetworkError(msg) => format!("Network issue detected: {}", msg),
            LLMError::ParseError(msg) => format!("Data parsing failed: {}", msg),
            LLMError::ProviderError(e) =>
                match e {
                    crate::api::providers::ModelProviderError::UnsupportedApiType(api_type) =>
                        format!("Provider {} does not support this model", api_type),
                    crate::api::providers::ModelProviderError::MissingConfiguration(provider) =>
                        format!("Missing configuration for {} provider", provider),
                }
            // Implementation pending for these variants
            LLMError::ModelError(_) => todo!(),
            LLMError::TokenLimitError(_) => todo!(),
            LLMError::ConcurrencyLimitError(_) => todo!(),
        }
    }
}
