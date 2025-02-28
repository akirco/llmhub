use thiserror::Error;

#[derive(Debug, Error)]
pub enum LLMError {
    #[error("HTTP request failed: {0}")] RequestError(#[from] reqwest::Error),

    #[error("Rate limit exceeded. Please wait {0} seconds before retrying")] RateLimitError(u64),

    #[error("Invalid configuration: {0}")] ConfigError(String),

    #[error("API error: {0}")] ApiError(String),

    #[error("Network error: {0}")] NetworkError(String),

    #[error("Parse error: {0}")] ParseError(String),

    #[error("Model error: {0}")] ModelError(String),

    #[error("Token limit exceeded: {0}")] TokenLimitError(String),

    #[error("Concurrent request limit reached: {0}")] ConcurrencyLimitError(String),

    #[error(transparent)] ProviderError(#[from] crate::api::providers::ModelProviderError),
}

pub type Result<T> = std::result::Result<T, LLMError>;

impl LLMError {
    /// Returns a user-friendly error message based on the error type
    pub fn user_friendly_message(&self) -> String {
        match self {
            LLMError::RequestError(e) => {
                if e.is_timeout() {
                    "请求超时，请检查您的网络连接".to_string()
                } else if e.is_connect() {
                    "无法连接到API服务器，请检查您的网络连接".to_string()
                } else {
                    format!("HTTP请求失败: {}", e)
                }
            }
            LLMError::RateLimitError(seconds) => {
                format!("API请求频率过高，请等待{}秒后再试", seconds)
            }
            LLMError::ConfigError(msg) => format!("配置错误: {}. 请检查您的API密钥和配置", msg),
            LLMError::ApiError(msg) => {
                if msg.contains("invalid_api_key") || msg.contains("authentication") {
                    "API密钥无效，请检查您的API密钥".to_string()
                } else if msg.contains("insufficient_quota") {
                    "API配额不足，请检查您的账户余额".to_string()
                } else {
                    format!("API错误: {}", msg)
                }
            }
            LLMError::NetworkError(msg) => format!("网络错误: {}", msg),
            LLMError::ParseError(msg) => format!("解析错误: {}", msg),
            LLMError::ProviderError(e) =>
                match e {
                    crate::api::providers::ModelProviderError::UnsupportedApiType(api_type) =>
                        format!("提供商 {} 不支持模型", api_type),
                    crate::api::providers::ModelProviderError::MissingConfiguration(provider) => {
                        format!("缺少提供商 {} 的配置", provider)
                    }
                }
            LLMError::ModelError(_) => todo!(),
            LLMError::TokenLimitError(_) => todo!(),
            LLMError::ConcurrencyLimitError(_) => todo!(),
        }
    }
}
