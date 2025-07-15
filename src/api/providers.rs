use crate::utils::error::{LlmHubError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use strum_macros::{Display, EnumString};

#[derive(Debug, Serialize, Deserialize, Copy, PartialEq, Eq, Clone, Hash, Display, EnumString)]
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
    GOOGLE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
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
    pub fn default_path(&self) -> &'static str {
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

#[derive(Debug, Clone, PartialEq)]
pub struct EndpointConfig {
    pub base_url: String,
    pub supported_types: Vec<ApiType>,
    pub custom_paths: HashMap<ApiType, String>,
}

impl EndpointConfig {
    pub fn get_url(&self, api_type: ApiType) -> Result<String> {
        if !self.supported_types.contains(&api_type) {
            return Err(LlmHubError::ProviderError(format!(
                "API type '{}' is not supported by provider.",
                api_type
            )));
        }
        let path = self
            .custom_paths
            .get(&api_type)
            .map(|s| s.as_str())
            .unwrap_or_else(|| api_type.default_path());
        Ok(format!("{}{}", self.base_url, path))
    }
}

impl ApiProvider {
    pub fn base_url(&self) -> &str {
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
            ApiProvider::GOOGLE => "https://generativelanguage.googleapis.com/v1beta/openai/",
        }
    }

    pub fn get_endpoint_config(&self) -> EndpointConfig {
        let base_url = self.base_url().trim_end_matches('/').to_string();
        let (supported_types, custom_paths) = match self {
            ApiProvider::OpenAI => (
                vec![
                    ApiType::Chat,
                    ApiType::ImageGeneration,
                    ApiType::ImageEdit,
                    ApiType::Embedding,
                    ApiType::AudioSpeech,
                    ApiType::AudioTranscription,
                    ApiType::AudioTranslation,
                    ApiType::ListModels,
                ],
                HashMap::new(),
            ),
            ApiProvider::Anthropic => (
                vec![ApiType::Chat],
                HashMap::from([(ApiType::Chat, "/messages".to_string())]),
            ),
            ApiProvider::Siliconflow => (
                vec![
                    ApiType::Chat,
                    ApiType::ImageGeneration,
                    ApiType::Embedding,
                    ApiType::AudioSpeech,
                    ApiType::AudioTranscription,
                ],
                HashMap::new(),
            ),
            _ => (vec![ApiType::Chat], HashMap::new()),
        };
        EndpointConfig {
            base_url,
            supported_types,
            custom_paths,
        }
    }
}
