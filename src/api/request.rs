use super::message::Prompt;
use super::providers::{ ApiProvider, ApiType };
use crate::models::models::Model;
use crate::utils::error::{ LLMError, Result };
use serde::Serialize;

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

/**
 * Request header
 * authorization: `Bearer ${API_KEY}`
 * content_type: Content-Type header value, default: application/json
 * accept: Accept header value, default: application/json
 */
#[derive(Debug, Serialize)]
pub struct RequestHeader {
    pub authorization: String,
    pub content_type: Option<String>,
    pub accept: Option<String>,
}

impl RequestHeader {
    pub fn new(api_key: String) -> Self {
        Self {
            authorization: format!("Bearer {}", api_key),
            content_type: Some("application/json".to_string()),
            accept: Some("application/json".to_string()),
        }
    }
}

/**
 * Request URL
 * url: URL for the API request
 * provider: API provider
 * api_type: API type
 * url = api_url + route
 */

#[derive(Debug, Serialize)]
pub struct RequestUrl {
    pub url: String,
}

impl RequestUrl {
    /// Constructs a complete request URL from provider and route
    pub fn new(provider: &ApiProvider, api_type: ApiType) -> Result<Self> {
        let config = provider.get_endpoint_config();
        let route = config.get_route(api_type)?;
        let url = format!("{}{}", config.api_url.trim_end_matches('/'), route);

        Ok(Self { url })
    }
}

/**
 * request body nullable values
 */
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct RequestOptions {
    /**
     * Whether or not to store the output of this chat completion request for use in our model distillation or evals products.
     * default: false
     * `provider: [openai]`
     */
    pub store: Option<bool>,
    /**
     * Constrains effort on reasoning for reasoning models. Currently supported values are low, medium, and high. Reducing reasoning effort can result in faster responses and fewer tokens used on reasoning in a response.
     * default: medium
     * `provider: [openai]`
     */
    pub reasoning_effort: Option<String>,
    /**
     * default: null
     * `provider: [openai,baidu]`
     */
    pub metadata: Option<serde_json::Value>,

    /**
     * control the word repetition in the generated text.
     * options: -2.0 to 2.0
     * default: 0
     * `provider: [openai,volcengine,siliconflow]`
     */
    pub frequency_penalty: Option<f32>,
    /**
     * 人为干预模型特定词汇出现的概率
     * options: -100 to 100
     * default: null
     * `provider: [openai,volcengine]`
     * example: {
     *   "ai":10,
     *    "暴力":-100
     * }
     */
    pub logit_bias: Option<serde_json::Value>,
    /**
     * Whether to return log probabilities of the output tokens or not. If true, returns the log probabilities of each output token returned in the content of message.
     * default: false
     * `provider: [openai,volcengine]`
     */
    pub logprobs: Option<bool>,
    /**
     * An integer between 0 and 20 specifying the number of most likely tokens to return at each token position, each with an associated log probability.
     * logprobs must be set to true if this parameter is used.
     * `provider: [openai,volcengine]`
     */
    pub top_logprobs: Option<u32>,
    /**
     * This value is now deprecated in favor of max_completion_tokens,
     * and is not compatible with o1 series models.
     *  openai is using `max_completion_tokens`
     * `provider: [volcengine,siliconflow,siliconflow,zhipu]`
     */
    pub max_tokens: Option<u32>,
    /**
     * An upper bound for the number of tokens that can be generated for a completion,
     * including visible output tokens and reasoning tokens.
     * `provider: [openai,baidu]`
     */
    pub max_completion_tokens: Option<u32>,
    /**
     * How many chat completion choices to generate for each input message.
     * Note that you will be charged based on the number of generated tokens across all of the choices. Keep n as 1 to minimize costs.
     * `provider: [openai,siliconflow]`
     */
    pub n: Option<u32>,
    /**
     * Output types that you would like the model to generate for this request.
     * Most models are capable of generating text,
     * which is the default: `["text"]`
     * `provider: [openai]`
     */
    pub modalities: Option<Vec<String>>,
    /**
     * `{ type:String,content:String | Vec<String>}`
     * `provider: [openai]`
     */
    pub prediction: Option<serde_json::Value>,
    /**
     * modalities: `["audio"]`
     * `{ voice:String,format:String}`
     * `provider: [openai]`
     */
    pub audio: Option<serde_json::Value>,
    /**
     * Number between -2.0 and 2.0.
     * Positive values penalize new tokens based on whether they appear in the text so far,
     * increasing the model's likelihood to talk about new topics.
     * `provider: [openai,volcengine,baidu]`
     */
    pub presence_penalty: Option<f32>,
    /**
     * text or json_object
     * default: text
     * `provider: [openai,siliconflow,zhipu,baidu]`
     */
    pub response_format: Option<ResponseFormat>,

    /**
     * beta feature
     * `provider: [openai,baidu]`
     */
    pub seed: Option<u32>,
    /**
     * `auto` or `default`
     * `provider: [openai]`
     */
    pub service_tier: Option<String>,
    /**
     *  value `stop_xxx`
     * `provider: [openai,volcengine,siliconflow,zhipu,baidu]`
     */
    pub stop: Option<String>,
    /**
     * is this a streaming request
     * default: false
     * `provider: [openai,volcengine,siliconflow,zhipu,baidu]`
     */
    pub stream: Option<bool>,
    /**
     * `{include_usage:Boolean}`
     * `provider: [openai,volcengine,baidu]`
     */
    pub stream_options: Option<serde_json::Value>,
    /**
     * What sampling temperature to use, between 0 and 2.
     * Higher values like 0.8 will make the output more random,
     * while lower values like 0.2 will make it more focused and
     * deterministic. We generally recommend altering this or top_p
     * but not both.
     * `provider: [openai,volcengine,siliconflow,zhipu,baidu]`
     */
    pub temperature: Option<f32>,
    /**
     * An alternative to sampling with temperature,
     * called nucleus sampling, where the model considers the results
     * of the tokens with top_p probability mass.
     * So 0.1 means only the tokens comprising the top 10% probability
     * mass are considered.
     * We generally recommend altering this or temperature but not both.
     * `provider: [openai,volcengine,siliconflow,zhipu,baidu]`
     */
    pub top_p: Option<f32>,
    /**
     * unknown
     * `provider:[siliconflow]`
     */
    pub top_k: Option<u32>,
    /**
     * `{
     * type:"function",
     * function:{
     * description:String,
     * name:String,
     * strict:Boolean,
     * parameters:json
     * }`
     * `provider: [openai,volcengine,zhipu,baidu]`
     */
    pub tools: Option<serde_json::Value>,
    /**
     * `provider: [openai,zhipu,baidu]`
     */
    pub tool_choice: Option<serde_json::Value>,
    /**
     * `provider: [openai,baidu]`
     */
    pub user: Option<String>,
}

impl Default for RequestOptions {
    fn default() -> Self {
        Self {
            store: None,
            reasoning_effort: None,
            metadata: None,
            frequency_penalty: None,
            logit_bias: None,
            logprobs: None,
            top_logprobs: None,
            max_tokens: None,
            max_completion_tokens: None,
            n: None,
            modalities: None,
            prediction: None,
            audio: None,
            presence_penalty: None,
            response_format: None,
            seed: None,
            service_tier: None,
            stop: None,
            stream: None,
            stream_options: None,
            temperature: None,
            top_p: None,
            top_k: None,
            tools: None,
            tool_choice: None,
            user: None,
        }
    }
}
/**
 * Request body
 * model: Model name ,required
 * messages: List of messages, required
 */

#[derive(Debug, Serialize)]
pub struct RequestBody {
    pub model: String,
    pub messages: Vec<Prompt>,
    #[serde(flatten)]
    pub options: RequestOptions,
    #[serde(skip)]
    pub provider: ApiProvider,
    #[serde(skip)]
    pub api_type: ApiType,
}

impl RequestBody {
    /// Creates a new request builder with default configuration
    pub fn new() -> Self {
        Self {
            model: String::new(),
            messages: Vec::new(),
            options: RequestOptions::default(),
            provider: ApiProvider::OpenAI, // default provider
            api_type: ApiType::Chat, // default type
        }
    }

    // Builder methods for model and provider settings
    pub fn model(mut self, model: Model) -> Self {
        self.model = model.as_str().to_string();
        self
    }

    pub fn provider(mut self, provider: ApiProvider) -> Self {
        self.provider = provider;
        self
    }

    pub fn api_type(mut self, api_type: ApiType) -> Self {
        self.api_type = api_type;
        self
    }

    // Message handling
    pub fn add_message(mut self, message: Prompt) -> Self {
        self.messages.push(message);
        self
    }

    pub fn add_messages(mut self, messages: Vec<Prompt>) -> Self {
        self.messages.extend(messages);
        self
    }

    /// Request options
    pub fn options(mut self, options: Option<RequestOptions>) -> Self {
        self.options = options.unwrap_or_default();
        self
    }

    // Configuration methods
    pub fn temperature(mut self, temp: f32) -> Self {
        self.options.temperature = Some(temp.clamp(0.0, 2.0));
        self
    }

    pub fn stream(mut self, enable: bool) -> Self {
        self.options.stream = Some(enable);
        self
    }

    pub fn response_format(mut self, format: ResponseType) -> Self {
        self.options.response_format = Some(ResponseFormat {
            response_type: format,
        });
        self
    }

    pub fn max_tokens(mut self, tokens: u32) -> Self {
        self.options.max_tokens = Some(tokens);
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.options.top_p = Some(top_p.clamp(0.0, 1.0));
        self
    }

    pub fn frequency_penalty(mut self, penalty: f32) -> Self {
        self.options.frequency_penalty = Some(penalty.clamp(-2.0, 2.0));
        self
    }

    pub fn presence_penalty(mut self, penalty: f32) -> Self {
        self.options.presence_penalty = Some(penalty.clamp(-2.0, 2.0));
        self
    }

    pub fn tools(mut self, tools: serde_json::Value) -> Self {
        self.options.tools = Some(tools);
        self
    }

    /// Validates and builds the request
    fn validate(&self) -> Result<()> {
        if !self.provider.supports_type(self.api_type) {
            return Err(
                LLMError::ConfigError(
                    format!("Provider {:?} does not support {:?}", self.provider, self.api_type)
                )
            );
        }

        if self.model.is_empty() {
            return Err(LLMError::ConfigError("Model not set".to_string()));
        }

        if self.messages.is_empty() {
            return Err(LLMError::ConfigError("No messages provided".to_string()));
        }

        Ok(())
    }

    /// Builds the final request body
    /// println!("{}", serde_json::to_string_pretty(&data).unwrap());
    pub fn build(self) -> Result<RequestBody> {
        self.validate()?;

        Ok(RequestBody {
            model: self.model,
            messages: self.messages,
            options: self.options,
            provider: self.provider,
            api_type: self.api_type,
        })
    }
}

/// Implement the `Default` trait for `RequestBody`.
/// This allows creating a `RequestBody` instance with default values using `RequestBody::default()`.
impl Default for RequestBody {
    /// Returns a new `RequestBody` instance with default configuration.
    /// It uses the `new` method to initialize the default values.
    fn default() -> Self {
        Self::new()
    }
}
