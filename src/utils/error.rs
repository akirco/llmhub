//! Error handling module for llmhub operations.

use thiserror::Error;

/// The primary error type for the llmhub crate.
#[derive(Error, Debug)]
pub enum LlmHubError {
    /// Error related to configuration, like a missing API key or invalid settings.
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Error originating from the underlying network client (reqwest).
    #[error("Network request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    /// Error when the API provider's rate limit is exceeded.
    #[error("Rate limit exceeded. Please wait {0} seconds before retrying.")]
    RateLimitError(u64),

    /// Error during serialization or deserialization of data.
    #[error("Failed to (de)serialize data: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// A generic error returned by the API provider.
    #[error("API error: {0}")]
    ApiError(String),

    /// Error related to unsupported providers, models, or API types.
    #[error("Provider or model error: {0}")]
    ProviderError(String),

    /// Error that occurs while processing a response stream.
    #[error("Stream processing error: {0}")]
    StreamError(String),

    /// An error occurred in the session logic.
    #[error("Session error: {0}")]
    SessionError(String),

    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

/// A specialized `Result` type for llmhub operations.
pub type Result<T> = std::result::Result<T, LlmHubError>;
