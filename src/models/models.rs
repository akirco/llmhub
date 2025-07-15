use crate::api::providers::ApiProvider;
use serde::{Serialize, Serializer};
use strum_macros::{Display, EnumString};

// --- Model-specific Enums ---

#[derive(Debug, Clone, PartialEq, Display, EnumString)]
pub enum CHATGLM {
    #[strum(serialize = "glm-4-plus")]
    Glm4Plus,
    #[strum(serialize = "glm-4-air")]
    Glm4Air,
    #[strum(serialize = "glm-4-long")]
    Glm4Long,
    #[strum(serialize = "glm-4-airx")]
    Glm4AirX,
    #[strum(serialize = "glm-4-flashx")]
    Glm4FlashX,
    #[strum(serialize = "glm-4-flash")]
    Glm4Flash,
    #[strum(serialize = "glm-4-alltools")]
    Glm4AllTools,
    #[strum(serialize = "glm-4v-plus")]
    Glm4vPlus,
    #[strum(serialize = "glm-4v")]
    Glm4v,
    #[strum(serialize = "glm-4v-flash")]
    Glm4vFlash,
    #[strum(serialize = "glm-realtime")]
    GlmRealtime,
    #[strum(serialize = "glm-4-voice")]
    Glm4Voice,
    #[strum(serialize = "cogview-4")]
    CogView4,
    #[strum(serialize = "cogview-3-flash")]
    CogView3Flash,
    #[strum(serialize = "cogvideox-2")]
    CogVideoX2,
    #[strum(serialize = "cogvideox-flash")]
    CogVideoXFlash,
    #[strum(serialize = "codegeex-4")]
    CodeGeeX4,
    #[strum(serialize = "embedding-2")]
    GlmEmbedding2,
    #[strum(serialize = "embedding-3")]
    GlmEmbedding3,
    #[strum(serialize = "glm-zero-preview")]
    GlmZeroPreviewNew,
}
impl CHATGLM {
    pub fn provider(&self) -> ApiProvider {
        ApiProvider::ZhipuAI
    }
}

#[derive(Debug, Clone, PartialEq, Display, EnumString)]
pub enum CHATGPT {
    #[strum(serialize = "gpt-4o")]
    V4o,
    #[strum(serialize = "gpt-4o-mini")]
    V4oMini,
    #[strum(serialize = "o1-mini")]
    VO1Mini,
    #[strum(serialize = "o1-preview")]
    VO1Preview,
}
impl CHATGPT {
    pub fn provider(&self) -> ApiProvider {
        ApiProvider::OpenAI
    }
}

#[derive(Debug, Clone, PartialEq, Display, EnumString)]
pub enum CLAUDE {
    #[strum(serialize = "claude-3-5-haiku-20241022")]
    Haiku3_5,
    #[strum(serialize = "claude-3-5-sonnet-20241022")]
    Sonnet3_5,
    #[strum(serialize = "claude-3-opus-20240229")]
    Opus3,
    #[strum(serialize = "claude-3-7-sonnet-20250219")]
    Sonnet3_7,
}
impl CLAUDE {
    pub fn provider(&self) -> ApiProvider {
        ApiProvider::Anthropic
    }
}

#[derive(Debug, Clone, PartialEq, Display, EnumString)]
pub enum DEEPSEEK {
    #[strum(serialize = "deepseek-chat")]
    V3Official,
    #[strum(serialize = "deepseek-reasoner")]
    R1Official,
    #[strum(serialize = "deepseek-ai/DeepSeek-R1")]
    R1Siliconflow,
    #[strum(serialize = "deepseek-ai/DeepSeek-V3")]
    V3Siliconflow,
    #[strum(serialize = "deepseek-r1-tencent")]
    R1Tencent,
    #[strum(serialize = "deepseek-v3-tencent")]
    V3Tencent,
    #[strum(serialize = "deepseek-r1-volcengine")]
    R1Volcengine,
    #[strum(serialize = "deepseek-v3-volcengine")]
    V3Volcengine,
    #[strum(serialize = "deepseek-r1-baidu")]
    R1Baidu,
    #[strum(serialize = "deepseek-v3-baidu")]
    V3Baidu,
    #[strum(serialize = "deepseek-r1-aliyun")]
    R1Aliyun,
    #[strum(serialize = "deepseek-v3-aliyun")]
    V3Aliyun,
}
impl DEEPSEEK {
    pub fn provider(&self) -> ApiProvider {
        match self {
            DEEPSEEK::V3Official | DEEPSEEK::R1Official => ApiProvider::Deepseek,
            DEEPSEEK::R1Siliconflow | DEEPSEEK::V3Siliconflow => ApiProvider::Siliconflow,
            DEEPSEEK::R1Tencent | DEEPSEEK::V3Tencent => ApiProvider::Tencent,
            DEEPSEEK::R1Volcengine | DEEPSEEK::V3Volcengine => ApiProvider::Volcengine,
            DEEPSEEK::R1Baidu | DEEPSEEK::V3Baidu => ApiProvider::Qianfan,
            DEEPSEEK::R1Aliyun | DEEPSEEK::V3Aliyun => ApiProvider::ALIBAILIAN,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Display, EnumString)]
pub enum GROK {
    #[strum(serialize = "grok-2-latest")]
    Grok2Latest,
}
impl GROK {
    pub fn provider(&self) -> ApiProvider {
        ApiProvider::XAI
    }
}

#[derive(Debug, Clone, PartialEq, Display, EnumString)]
pub enum QWEN {
    #[strum(serialize = "qwen2.5-72b-instruct")]
    Qwen25_7BInstruct,
    #[strum(serialize = "qwen2.5-14b-instruct-1m")]
    Qwen25_14BInstruct1m,
    #[strum(serialize = "qwen-coder-plus-latest")]
    QwenCoderPlusLatest,
}
impl QWEN {
    pub fn provider(&self) -> ApiProvider {
        ApiProvider::ALIBAILIAN
    }
}

#[derive(Debug, Clone, PartialEq, Display, EnumString)]
pub enum DOUBAO {
    #[strum(serialize = "doubao-1-5-pro-32k-250115")]
    Doubao1_5Pro32k250115,
}
impl DOUBAO {
    pub fn provider(&self) -> ApiProvider {
        ApiProvider::Volcengine
    }
}

#[derive(Debug, Clone, PartialEq, Display, EnumString)]
pub enum GEMINI {
    #[strum(serialize = "gemini-2.5-flash")]
    Flash2_5,
    #[strum(serialize = "gemini-2.5-pro")]
    Pro2_5,
}
impl GEMINI {
    pub fn provider(&self) -> ApiProvider {
        ApiProvider::Volcengine
    }
}

// --- Top-level Model Enum ---

#[derive(Debug, Clone, PartialEq)]
pub enum Model {
    ChatGLM(CHATGLM),
    ChatGPT(CHATGPT),
    Claude(CLAUDE),
    Deepseek(DEEPSEEK),
    Grok(GROK),
    Qwen(QWEN),
    Doubao(DOUBAO),
}

impl Model {
    pub fn provider(&self) -> ApiProvider {
        match self {
            Model::ChatGLM(m) => m.provider(),
            Model::ChatGPT(m) => m.provider(),
            Model::Claude(m) => m.provider(),
            Model::Deepseek(m) => m.provider(),
            Model::Grok(m) => m.provider(),
            Model::Qwen(m) => m.provider(),
            Model::Doubao(m) => m.provider(),
        }
    }
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Model::ChatGLM(m) => write!(f, "{}", m),
            Model::ChatGPT(m) => write!(f, "{}", m),
            Model::Claude(m) => write!(f, "{}", m),
            Model::Deepseek(m) => write!(f, "{}", m),
            Model::Grok(m) => write!(f, "{}", m),
            Model::Qwen(m) => write!(f, "{}", m),
            Model::Doubao(m) => write!(f, "{}", m),
        }
    }
}

impl Serialize for Model {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
