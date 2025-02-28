use crate::api::providers::ApiProvider;
use serde::{ Deserialize, Serialize };

// ChatGLM
///https://open.bigmodel.cn/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CHATGLM {
    Glm4Plus,
    Glm4Air,
    Glm4Long,
    Glm4AirX,
    Glm4FlashX,
    Glm4Flash,
    Glm4vPlus,
    Glm4v,
    Glm4vFlash,
    GlmZeroPreviewNew,
    GlmRealtime,
    Glm4Voice,
    CogView4,
    CogView3Flash,
    CogVideoX2,
    CogVideoXFlash,
    Glm4AllTools,
    CodeGeeX4,
    GlmEmbedding2,
    GlmEmbedding3,
}

impl CHATGLM {
    pub fn as_str(&self) -> &str {
        match self {
            CHATGLM::Glm4Plus => "glm-4-plus",
            CHATGLM::Glm4Air => "glm-4-air",
            CHATGLM::Glm4Long => "glm-4-long",
            CHATGLM::Glm4AirX => "glm-4-airx",
            CHATGLM::Glm4FlashX => "glm-4-flashx",
            CHATGLM::Glm4Flash => "glm-4-flash",
            CHATGLM::Glm4AllTools => "glm-4-alltools",
            CHATGLM::Glm4vPlus => "glm-4v-plus",
            CHATGLM::Glm4v => "glm-4v",
            CHATGLM::Glm4vFlash => "glm-4v-flash",
            CHATGLM::GlmRealtime => "glm-realtime",
            CHATGLM::Glm4Voice => "glm-4-voice",
            CHATGLM::CogView4 => "cogview-4",
            CHATGLM::CogView3Flash => "cogview-3-flash",
            CHATGLM::CogVideoX2 => "cogvideox-2",
            CHATGLM::CogVideoXFlash => "cogvideox-flash",
            CHATGLM::CodeGeeX4 => "codegeex-4",
            CHATGLM::GlmEmbedding2 => "embedding-2",
            CHATGLM::GlmEmbedding3 => "embedding-3",
            CHATGLM::GlmZeroPreviewNew => "glm-zero-preview",
        }
    }
    pub fn provider(&self) -> ApiProvider {
        match self {
            CHATGLM::Glm4Plus => ApiProvider::ZhipuAI,
            CHATGLM::Glm4Air => ApiProvider::ZhipuAI,
            CHATGLM::Glm4Long => ApiProvider::ZhipuAI,
            CHATGLM::Glm4AirX => ApiProvider::ZhipuAI,
            CHATGLM::Glm4FlashX => ApiProvider::ZhipuAI,
            CHATGLM::Glm4Flash => ApiProvider::ZhipuAI,
            CHATGLM::Glm4vPlus => ApiProvider::ZhipuAI,
            CHATGLM::Glm4v => ApiProvider::ZhipuAI,
            CHATGLM::Glm4vFlash => ApiProvider::ZhipuAI,
            CHATGLM::GlmZeroPreviewNew => ApiProvider::ZhipuAI,
            CHATGLM::GlmRealtime => ApiProvider::ZhipuAI,
            CHATGLM::Glm4Voice => ApiProvider::ZhipuAI,
            CHATGLM::CogView4 => ApiProvider::ZhipuAI,
            CHATGLM::CogView3Flash => ApiProvider::ZhipuAI,
            CHATGLM::CogVideoX2 => ApiProvider::ZhipuAI,
            CHATGLM::CogVideoXFlash => ApiProvider::ZhipuAI,
            CHATGLM::Glm4AllTools => ApiProvider::ZhipuAI,
            CHATGLM::CodeGeeX4 => ApiProvider::ZhipuAI,
            CHATGLM::GlmEmbedding2 => ApiProvider::ZhipuAI,
            CHATGLM::GlmEmbedding3 => ApiProvider::ZhipuAI,
        }
    }
}

// ChatGPT
///https://platform.openai.com/docs/models
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CHATGPT {
    V4o,
    V4oMini,
    VO1Mini,
    VO1Preview,
}

impl CHATGPT {
    pub fn as_str(&self) -> &str {
        match self {
            CHATGPT::V4o => "gpt-4o",
            CHATGPT::V4oMini => "gpt-4o-mini",
            CHATGPT::VO1Mini => "o1-mini",
            CHATGPT::VO1Preview => "o1-preview",
        }
    }
    pub fn provider(&self) -> ApiProvider {
        match self {
            CHATGPT::V4o => ApiProvider::OpenAI,
            CHATGPT::V4oMini => ApiProvider::OpenAI,
            CHATGPT::VO1Mini => ApiProvider::OpenAI,
            CHATGPT::VO1Preview => ApiProvider::OpenAI,
        }
    }
}

/// Claude
///https://docs.anthropic.com/en/api/getting-started
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CLAUDE {
    Haiku3_5,
    Sonnet3_5,
    Sonnet3_7,
    Opus3,
}

impl CLAUDE {
    pub fn as_str(&self) -> &str {
        match self {
            CLAUDE::Haiku3_5 => "claude-3-5-haiku-20241022",
            CLAUDE::Sonnet3_5 => "claude-3-5-sonnet-20241022",
            CLAUDE::Opus3 => "claude-3-opus-20240229",
            CLAUDE::Sonnet3_7 => "claude-3-7-sonnet-20250219",
        }
    }
    pub fn provider(&self) -> ApiProvider {
        match self {
            CLAUDE::Haiku3_5 => ApiProvider::Anthropic,
            CLAUDE::Sonnet3_5 => ApiProvider::Anthropic,
            CLAUDE::Opus3 => ApiProvider::Anthropic,
            CLAUDE::Sonnet3_7 => ApiProvider::Anthropic,
        }
    }
}

/// DeepSeek
///
/**
 * official: https://api-docs.deepseek.com/
 * siliconflow: https://siliconflow.cn/zh-cn/models
 * tencent: https://cloud.tencent.com/document/product/1772/115963
 * volcengine: https://console.volcengine.com/ark/region:ark+cn-beijing/model?vendor=Bytedance&view=LIST_VIEW
 * baidu: https://cloud.baidu.com/doc/WENXINWORKSHOP/s/Fm2vrveyu
 * aliyun: https://help.aliyun.com/zh/model-studio/getting-started/models
 */
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DEEPSEEK {
    V3Official,
    R1Official,
    R1Siliconflow,
    V3Siliconflow,
    R1Tencent,
    V3Tencent,
    R1Volcengine,
    V3Volcengine,
    R1Baidu,
    V3Baidu,
    R1Aliyun,
    V3Aliyun,
}

impl DEEPSEEK {
    pub fn as_str(&self) -> &str {
        match self {
            DEEPSEEK::V3Official => "deepseek-chat",
            DEEPSEEK::R1Official => "deepseek-reasoner",
            DEEPSEEK::R1Siliconflow => "deepseek-ai/DeepSeek-R1",
            DEEPSEEK::V3Siliconflow => "deepseek-ai/DeepSeek-V3",
            DEEPSEEK::R1Tencent => "deepseek-r1",
            DEEPSEEK::V3Tencent => "deepseek-v3",
            DEEPSEEK::R1Volcengine => "deepseek-r1-250120",
            DEEPSEEK::V3Volcengine => "deepseek-v3-241226",
            DEEPSEEK::R1Baidu => "deepseek-r1",
            DEEPSEEK::V3Baidu => "deepseek-v3",
            DEEPSEEK::R1Aliyun => "deepseek-r1",
            DEEPSEEK::V3Aliyun => "deepseek-v3",
        }
    }

    // Get the recommended provider for this model variant
    pub fn provider(&self) -> ApiProvider {
        match self {
            DEEPSEEK::V3Official => ApiProvider::Deepseek,
            DEEPSEEK::R1Official => ApiProvider::Deepseek,
            DEEPSEEK::R1Siliconflow => ApiProvider::Siliconflow,
            DEEPSEEK::V3Siliconflow => ApiProvider::Siliconflow,
            DEEPSEEK::R1Tencent => ApiProvider::Tencent,
            DEEPSEEK::V3Tencent => ApiProvider::Tencent,
            DEEPSEEK::R1Volcengine => ApiProvider::Volcengine,
            DEEPSEEK::V3Volcengine => ApiProvider::Volcengine,
            DEEPSEEK::R1Baidu => ApiProvider::Qianfan,
            DEEPSEEK::V3Baidu => ApiProvider::Qianfan,
            DEEPSEEK::R1Aliyun => ApiProvider::ALIBAILIAN,
            DEEPSEEK::V3Aliyun => ApiProvider::ALIBAILIAN,
        }
    }
}

/// Grok
/// https://docs.x.ai/docs/overview#featured-models
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GROK {
    Grok2Latest,
}

impl GROK {
    pub fn as_str(&self) -> &str {
        match self {
            GROK::Grok2Latest => "grok-2-latest",
        }
    }
    pub fn provider(&self) -> ApiProvider {
        match self {
            GROK::Grok2Latest => ApiProvider::XAI,
        }
    }
}

/// Qwen
/// https://help.aliyun.com/zh/model-studio/getting-started/models
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum QWEN {
    Qwen25_7BInstruct,
    Qwen25_14BInstruct1m,
    QwenCoderPlusLatest,
}

impl QWEN {
    pub fn as_str(&self) -> &str {
        match self {
            QWEN::Qwen25_7BInstruct => "qwen2.5-72b-instruct",
            QWEN::Qwen25_14BInstruct1m => "qwen2.5-14b-instruct-1m",
            QWEN::QwenCoderPlusLatest => "qwen-coder-plus-latest",
        }
    }
    pub fn provider(&self) -> ApiProvider {
        match self {
            QWEN::Qwen25_7BInstruct => ApiProvider::ALIBAILIAN,
            QWEN::Qwen25_14BInstruct1m => ApiProvider::ALIBAILIAN,
            QWEN::QwenCoderPlusLatest => ApiProvider::ALIBAILIAN,
        }
    }
}

/// Doubao
/// https://console.volcengine.com/ark/region:ark+cn-beijing/model/detail?Id=doubao-1-5-pro-32k
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DOUBAO {
    Doubao1_5Pro32k250115,
}

impl DOUBAO {
    pub fn as_str(&self) -> &str {
        match self {
            DOUBAO::Doubao1_5Pro32k250115 => "doubao-1-5-pro-32k-250115",
        }
    }
    pub fn provider(&self) -> ApiProvider {
        match self {
            DOUBAO::Doubao1_5Pro32k250115 => ApiProvider::Volcengine,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub fn as_str(&self) -> &str {
        match self {
            Model::ChatGLM(m) => m.as_str(),
            Model::ChatGPT(m) => m.as_str(),
            Model::Claude(m) => m.as_str(),
            Model::Deepseek(m) => m.as_str(),
            Model::Grok(m) => m.as_str(),
            Model::Qwen(m) => m.as_str(),
            Model::Doubao(m) => m.as_str(),
        }
    }

    pub fn provider(&self) -> ApiProvider {
        match self {
            Model::ChatGPT(m) => m.provider(),
            Model::Claude(m) => m.provider(),
            Model::ChatGLM(m) => m.provider(),
            Model::Deepseek(m) => m.provider(),
            Model::Qwen(m) => m.provider(),
            Model::Grok(m) => m.provider(),
            Model::Doubao(m) => m.provider(),
        }
    }
}
