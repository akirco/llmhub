use crate::api::message::Message;
use serde::{Deserialize, Serialize};

// --- Supporting Structs ---

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Usage {
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
    pub prompt_tokens_details: Option<PromptTokensDetails>,
    pub prompt_cache_hit_tokens: Option<u32>,
    pub prompt_cache_miss_tokens: Option<u32>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct PromptTokensDetails {
    pub cached_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCallFunction {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCall {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub tool_type: Option<String>,
    pub function: Option<ToolCallFunction>,
}

// --- Non-Streaming Response ---

#[derive(Debug, Deserialize, Clone)]
pub struct ApiResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ApiChoice>,
    pub usage: Option<Usage>,
    pub system_fingerprint: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApiChoice {
    pub index: i32,
    pub message: Message,
    pub finish_reason: Option<String>,
    pub logprobs: Option<serde_json::Value>,
}

// --- Streaming Response ---

#[derive(Debug, Deserialize, Clone)]
pub struct StreamChunk {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<StreamChoice>,
    pub usage: Option<Usage>, // Usage can appear in the last chunk
    pub system_fingerprint: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StreamChoice {
    pub index: i32,
    pub delta: StreamDelta,
    pub finish_reason: Option<String>,
    pub logprobs: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct StreamDelta {
    pub role: Option<String>,
    pub content: Option<String>,
    pub reasoning_content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
}
