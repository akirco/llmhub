use serde::{ Deserialize, Serialize };
use std::hash::Hash;
use std::collections::HashMap;

/**
 * API Providers
 * Official API providers & Api services platforms
 */

#[derive(Debug, Serialize, Deserialize, Copy, PartialEq, Eq, Clone, Hash)]
pub enum ApiProvider {
    Siliconflow,
    Deepseek,
    Qianfan,
    Anthropic,
    OpenAI,
    ZhipuAI,
    ALIBAILIAN,
    XAI,
    Volcengine,
    Tencent,
}

/**
 * API Endpoints
 * Endpoints for each API provider
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApiType {
    Chat,
    ImageGeneration,
    ImageEdit,
    Embedding,
    AudioSpeech,
    AudioTranscription,
    AudioTranslation,
    ListModels,
}

impl ApiType {
    /// get the default route for the API type
    pub fn default_route(&self) -> &'static str {
        match self {
            Self::Chat => "/chat/completions",
            Self::ImageGeneration => "/images/generations",
            Self::ImageEdit => "/images/edits",
            Self::Embedding => "/embeddings",
            Self::AudioSpeech => "/audio/speech",
            Self::AudioTranscription => "/audio/transcribes",
            Self::AudioTranslation => "/audio/translations",
            Self::ListModels => "/models",
        }
    }
}

/// Configuration for API endpoints
///
/// Contains the base URL, supported API types, and custom routes
/// for a specific API provider.
#[derive(Debug, Clone, PartialEq)]
pub struct EndpointConfig {
    /// Base URL for the API provider
    pub api_url: String,
    /// List of supported API types (e.g., Chat, ImageGeneration)
    pub supported_types: Vec<ApiType>,
    /// Custom routes mapping for specific API types
    ///
    /// Overrides default routes when specified
    pub custom_routes: HashMap<ApiType, String>,
}

impl EndpointConfig {
    /// Retrieves the route for a specific API type
    ///
    /// # Arguments
    /// * `api_type` - The API type to get route for
    ///
    /// # Returns
    /// - `Ok(String)` with the route if supported
    /// - `Err(ModelProviderError)` if API type is not supported
    ///
    /// # Behavior
    /// - Returns custom route if defined
    /// - Falls back to default route if no custom route exists
    pub fn get_route(&self, api_type: ApiType) -> Result<String, ModelProviderError> {
        if !self.supported_types.contains(&api_type) {
            return Err(ModelProviderError::UnsupportedApiType(format!("{:?}", api_type)));
        }
        Ok(
            self.custom_routes
                .get(&api_type)
                .cloned()
                .unwrap_or_else(|| api_type.default_route().to_string())
        )
    }
}

impl ApiProvider {
    /**
     * Returns the API URL for the provider
     * @param {ApiProvider} provider - The API provider
     * @returns {string} The API URL
     */
    pub fn apiurl(&self) -> &str {
        match self {
            ApiProvider::Siliconflow => "https://api.siliconflow.cn/v1/",
            ApiProvider::Deepseek => "https://api.deepseek.com",
            ApiProvider::Qianfan => "https://qianfan.baidubce.com/v2/",
            ApiProvider::Anthropic => "https://api.anthropic.com/v1/",
            ApiProvider::OpenAI => "https://api.openai.com/v1/",
            ApiProvider::ZhipuAI => "https://open.bigmodel.cn/api/paas/v4/",
            ApiProvider::ALIBAILIAN => "https://dashscope.aliyuncs.com/compatible-mode/v1/",
            ApiProvider::XAI => "https://api.x.ai/v1/",
            ApiProvider::Volcengine => "https://ark.cn-beijing.volces.com/api/v3/",
            ApiProvider::Tencent => "https://api.lkeap.cloud.tencent.com/v1/",
        }
    }

    /// Returns the endpoint configuration for the provider
    pub fn get_endpoint_config(&self) -> EndpointConfig {
        match self {
            ApiProvider::OpenAI =>
                EndpointConfig {
                    api_url: self.apiurl().to_string(),
                    supported_types: vec![
                        ApiType::Chat,
                        ApiType::ImageGeneration,
                        ApiType::ImageEdit,
                        ApiType::Embedding,
                        ApiType::AudioSpeech,
                        ApiType::AudioTranscription,
                        ApiType::AudioTranslation,
                        ApiType::ListModels
                    ],
                    custom_routes: HashMap::new(),
                },
            ApiProvider::Anthropic =>
                EndpointConfig {
                    api_url: self.apiurl().to_string(),
                    supported_types: vec![ApiType::Chat],
                    custom_routes: HashMap::from([(ApiType::Chat, "/messages".to_string())]),
                },
            ApiProvider::Siliconflow =>
                EndpointConfig {
                    api_url: self.apiurl().to_string(),
                    supported_types: vec![
                        ApiType::Chat,
                        ApiType::ImageGeneration,
                        ApiType::Embedding,
                        ApiType::AudioSpeech,
                        ApiType::AudioTranscription
                    ],
                    custom_routes: HashMap::new(),
                },
            ApiProvider::Deepseek =>
                EndpointConfig {
                    api_url: self.apiurl().to_string(),
                    supported_types: vec![ApiType::Chat],
                    custom_routes: HashMap::new(),
                },
            ApiProvider::Qianfan =>
                EndpointConfig {
                    api_url: self.apiurl().to_string(),
                    supported_types: vec![ApiType::Chat],
                    custom_routes: HashMap::new(),
                },
            ApiProvider::ZhipuAI =>
                EndpointConfig {
                    api_url: self.apiurl().to_string(),
                    supported_types: vec![ApiType::Chat],
                    custom_routes: HashMap::new(),
                },
            ApiProvider::ALIBAILIAN =>
                EndpointConfig {
                    api_url: self.apiurl().to_string(),
                    supported_types: vec![ApiType::Chat],
                    custom_routes: HashMap::new(),
                },
            ApiProvider::XAI =>
                EndpointConfig {
                    api_url: self.apiurl().to_string(),
                    supported_types: vec![ApiType::Chat],
                    custom_routes: HashMap::new(),
                },
            ApiProvider::Volcengine =>
                EndpointConfig {
                    api_url: self.apiurl().to_string(),
                    supported_types: vec![ApiType::Chat],
                    custom_routes: HashMap::new(),
                },
            ApiProvider::Tencent =>
                EndpointConfig {
                    api_url: self.apiurl().to_string(),
                    supported_types: vec![ApiType::Chat],
                    custom_routes: HashMap::new(),
                },
        }
    }

    /**
     * Checks if the provider supports the specified API type
     * @param {ApiProvider} provider - The API provider
     * @param {ApiType} api_type - The API type
     * @returns {boolean} Whether the provider supports the API type
     */
    pub fn supports_type(&self, api_type: ApiType) -> bool {
        self.get_endpoint_config().supported_types.contains(&api_type)
    }

    /// Gets the route for the specified API type
    pub fn get_route(&self, api_type: ApiType) -> Result<String, ModelProviderError> {
        self.get_endpoint_config().get_route(api_type)
    }

    /// Returns a list of API types supported by this provider
    pub fn get_supported_types(&self) -> Vec<ApiType> {
        self.get_endpoint_config().supported_types.clone()
    }
}

impl std::fmt::Display for ApiProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ModelProviderError {
    #[error("API type {0} is not supported")] UnsupportedApiType(String),

    #[error("Missing API configuration for provider {0:?}")] MissingConfiguration(ApiProvider),
}
