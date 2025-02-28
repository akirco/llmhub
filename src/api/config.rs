use super::providers::ApiProvider;
use serde::{ Deserialize, Serialize };

/// Configuration settings for API providers
///
/// # Fields
/// - `api_provider`: Enum variant specifying the AI service provider
/// - `api_base_url`: Optional base URL for API endpoints (can override default provider URLs)
/// - `api_key`: Authentication credential for the API service
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderConfig {
    pub api_provider: ApiProvider,
    pub api_base_url: Option<String>,
    pub api_key: Option<String>,
}

impl ProviderConfig {
    /// Creates a new provider configuration with the specified parameters
    ///
    /// # Arguments
    /// * `api_provider` - Service provider enum variant
    /// * `api_base_url` - Optional custom API endpoint URL
    /// * `api_key` - Optional authentication key
    pub fn new(
        api_provider: ApiProvider,
        api_base_url: Option<String>,
        api_key: Option<String>
    ) -> Self {
        Self {
            api_provider,
            api_base_url,
            api_key,
        }
    }

    /// Loads provider configurations from a file, creating default config if file doesn't exist
    /// Also merges configurations from environment variables
    pub fn load_from_file(path: &str) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        if !std::path::Path::new(path).exists() {
            Self::create_default_config(path)?;
        }

        let config_content = std::fs::read_to_string(path)?;
        let mut configs: Vec<Self> = serde_json::from_str(&config_content)?;

        for provider in [
            ApiProvider::OpenAI,
            ApiProvider::Anthropic,
            ApiProvider::Deepseek,
            ApiProvider::Siliconflow,
            ApiProvider::Qianfan,
            ApiProvider::ZhipuAI,
            ApiProvider::Volcengine,
            ApiProvider::XAI,
            ApiProvider::Tencent,
            ApiProvider::ALIBAILIAN,
        ].iter() {
            if let Some(env_config) = Self::from_env(*provider) {
                if let Some(existing) = configs.iter_mut().find(|c| c.api_provider == *provider) {
                    existing.api_key = env_config.api_key;
                    existing.api_base_url = env_config.api_base_url;
                } else {
                    configs.push(env_config);
                }
            }
        }

        Ok(configs)
    }

    /// Creates a default configuration file with preset providers
    pub fn create_default_config(path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let default_configs = vec![
            Self {
                api_provider: ApiProvider::OpenAI,
                api_base_url: Some(ApiProvider::OpenAI.apiurl().to_string()),
                api_key: Some("your_openai_key_here".to_string()),
            },
            Self {
                api_provider: ApiProvider::Anthropic,
                api_base_url: Some(ApiProvider::Anthropic.apiurl().to_string()),
                api_key: Some("your_anthropic_key_here".to_string()),
            },
            Self {
                api_provider: ApiProvider::Tencent,
                api_base_url: Some(ApiProvider::Tencent.apiurl().to_string()),
                api_key: Some("your_TencentTencent_key_here".to_string()),
            },
            Self {
                api_provider: ApiProvider::Qianfan,
                api_base_url: Some(ApiProvider::Qianfan.apiurl().to_string()),
                api_key: Some("your_qianfan_key_here".to_string()),
            },
            Self {
                api_provider: ApiProvider::Siliconflow,
                api_base_url: Some(ApiProvider::Siliconflow.apiurl().to_string()),
                api_key: Some("your_siliconflow_key_here".to_string()),
            },
            Self {
                api_provider: ApiProvider::Deepseek,
                api_base_url: Some(ApiProvider::Deepseek.apiurl().to_string()),
                api_key: Some("your_deepseek_key_here".to_string()),
            },
            Self {
                api_provider: ApiProvider::ZhipuAI,
                api_base_url: Some(ApiProvider::ZhipuAI.apiurl().to_string()),
                api_key: Some("your_zhipuai_key_here".to_string()),
            },
            Self {
                api_provider: ApiProvider::Volcengine,
                api_base_url: Some(ApiProvider::Volcengine.apiurl().to_string()),
                api_key: Some("your_volcengine_key_here".to_string()),
            },
            Self {
                api_provider: ApiProvider::XAI,
                api_base_url: Some(ApiProvider::XAI.apiurl().to_string()),
                api_key: Some("your_XAI_key_here".to_string()),
            }
        ];

        let config_content = serde_json::to_string_pretty(&default_configs)?;
        std::fs::write(path, config_content)?;
        Ok(())
    }

    /// Finds a provider configuration by provider type
    pub fn get_provider_config(configs: &[Self], provider: ApiProvider) -> Option<&Self> {
        configs.iter().find(|c| c.api_provider == provider)
    }

    /// Returns the default path for the configuration file
    pub fn default_config_path() -> String {
        let home_dir = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        home_dir.join(".llmhub/config.json").to_string_lossy().to_string()
    }

    /// Ensures the configuration directory exists, creating it if necessary
    pub fn ensure_config_dir() -> Result<(), Box<dyn std::error::Error>> {
        let home_dir = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        let config_dir = home_dir.join(".llmhub");

        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)?;
        }

        Ok(())
    }

    /// Sets the API key for this provider configuration
    pub fn set_api_key(&mut self, api_key: String) -> &mut Self {
        self.api_key = Some(api_key);
        self
    }

    /// Saves the configuration to a file
    pub fn save_to_file(
        &self,
        configs: &[Self],
        path: &str
    ) -> Result<(), Box<dyn std::error::Error>> {
        let config_content = serde_json::to_string_pretty(configs)?;
        std::fs::write(path, config_content)?;
        Ok(())
    }

    /// Creates a provider configuration from environment variables
    pub fn from_env(provider: ApiProvider) -> Option<Self> {
        let env_prefix = match provider {
            ApiProvider::OpenAI => "OPENAI",
            ApiProvider::Anthropic => "ANTHROPIC",
            ApiProvider::Deepseek => "DEEPSEEK",
            ApiProvider::Siliconflow => "SILICONFLOW",
            ApiProvider::Qianfan => "QIANFAN",
            ApiProvider::ZhipuAI => "ZHIPUAI",
            ApiProvider::Volcengine => "VOLCENGINE",
            ApiProvider::XAI => "XAI",
            ApiProvider::Tencent => "TENCENT",
            ApiProvider::ALIBAILIAN => "ALIBAILIAN",
        };

        let api_key_var = format!("{}_API_KEY", env_prefix);
        let api_base_url_var = format!("{}_API_BASE", env_prefix);

        let api_key = std::env::var(&api_key_var).ok();
        let api_base_url = std::env
            ::var(&api_base_url_var)
            .ok()
            .or_else(|| Some(provider.apiurl().to_string()));

        if api_key.is_none() {
            return None;
        }

        Some(Self {
            api_provider: provider,
            api_base_url,
            api_key,
        })
    }
}
