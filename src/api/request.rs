use crate::api::message::Message;
use crate::api::session::Session;
use crate::models::models::Model;
use serde::Serialize;

// Your original ResponseType and ResponseFormat are kept.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ResponseType {
    Text,
    JsonObject,
}

#[derive(Debug, Serialize, Clone)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub response_type: ResponseType,
}

// Your comprehensive RequestOptions is kept entirely.
#[derive(Debug, Serialize, Clone, Default)]
#[serde_with::skip_serializing_none]
pub struct RequestOptions {
    pub store: Option<bool>,
    pub reasoning_effort: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub frequency_penalty: Option<f32>,
    pub logit_bias: Option<serde_json::Value>,
    pub logprobs: Option<bool>,
    pub top_logprobs: Option<u32>,
    pub max_tokens: Option<u32>,
    pub max_completion_tokens: Option<u32>,
    pub n: Option<u32>,
    pub modalities: Option<Vec<String>>,
    pub prediction: Option<serde_json::Value>,
    pub audio: Option<serde_json::Value>,
    pub presence_penalty: Option<f32>,
    pub response_format: Option<ResponseFormat>,
    pub seed: Option<u32>,
    pub service_tier: Option<String>,
    pub stop: Option<String>,
    pub stream: Option<bool>,
    pub stream_options: Option<serde_json::Value>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<u32>,
    pub tools: Option<serde_json::Value>,
    pub tool_choice: Option<serde_json::Value>,
    pub user: Option<String>,
}

/// Represents the complete, serializable request body sent to the API.
#[derive(Debug, Serialize, Clone)]
pub struct ApiRequest {
    pub model: Model,
    pub messages: Vec<Message>,
    #[serde(flatten)]
    pub options: RequestOptions,
}

impl ApiRequest {
    /// Creates a new ApiRequest with the essential fields.
    pub fn new(model: Model, session: Option<&Session>) -> Self {
        let messages = session.map_or_else(Vec::new, |s| s.get_messages().clone());
        Self {
            model,
            messages,
            options: RequestOptions::default(),
        }
    }

    /// A convenient way to modify the request options.
    pub fn with_options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }
    
    /// A convenient way to set the streaming option.
    pub fn stream(mut self, stream: bool) -> Self {
        self.options.stream = Some(stream);
        self
    }

    /// Adds a message to the request.
    pub fn add_message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
    }
}
