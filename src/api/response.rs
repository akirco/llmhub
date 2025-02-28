use serde::{ Deserialize, Serialize };

/// Represents the common response structure for all API calls
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    /// Unique identifier for the response
    pub id: String,
    /// Type of object returned (e.g., "chat.completion")
    pub object: String,
    /// Unix timestamp of when the response was created
    pub created: i64,
    /// Model identifier used for generation
    pub model: String,
    /// List of generated response choices
    pub choices: Vec<Choice>,
    /// Token usage statistics
    pub usage: Option<Usage>,
    /// System fingerprint for debugging purposes
    #[serde(rename = "system_fingerprint")]
    pub system_fingerprint: Option<String>,
}

impl Response {
    /// Get the primary content from the first choice
    pub fn content(&self) -> Option<String> {
        self.choices.first().and_then(|c| {
            c.delta.as_ref().and_then(|d| {
                d.content
                    .as_ref()
                    .or(d.reasoning_content.as_ref())
                    .map(|s| s.to_owned())
            })
        })
    }

    /// Get all contents from all choices
    pub fn all_contents(&self) -> Vec<&str> {
        self.choices
            .iter()
            .filter_map(|choice| choice.delta.as_ref().and_then(|d| d.content.as_deref()))
            .collect()
    }

    /// Check if the response is complete
    pub fn is_complete(&self) -> bool {
        self.choices.iter().any(|choice| choice.finish_reason.is_some())
    }

    /// Get the message content from the first choice for non-streaming response
    pub fn message(&self) -> Option<String> {
        self.choices.first().and_then(|c| {
            c.message.as_ref().and_then(|m| {
                m.content
                    .as_ref()
                    .or(m.reasoning_content.as_ref())
                    .map(|s| s.to_owned())
            })
        })
    }

    /// Get the total tokens used
    pub fn total_tokens(&self) -> Option<u32> {
        self.usage.as_ref().and_then(|usage| usage.total_tokens)
    }

    /// Get the main content from the first choice
    pub fn main_content(&self) -> Option<String> {
        self.choices
            .first()
            .and_then(|c| {
                c.delta.as_ref().and_then(|d| { d.content.as_ref().map(|s| s.to_owned()) })
            })
    }

    /// Get the reasoning content from the first choice
    pub fn reasoning_content(&self) -> Option<String> {
        self.choices
            .first()
            .and_then(|c| {
                c.delta
                    .as_ref()
                    .and_then(|d| { d.reasoning_content.as_ref().map(|s| s.to_owned()) })
            })
    }

    /// Get the message main content for non-streaming response
    pub fn message_content(&self) -> Option<String> {
        self.choices
            .first()
            .and_then(|c| {
                c.message.as_ref().and_then(|d| { d.content.as_ref().map(|s| s.to_owned()) })
            })
    }

    /// Get the message reasoning content for non-streaming response
    pub fn message_reasoning(&self) -> Option<String> {
        self.choices
            .first()
            .and_then(|c| {
                c.message
                    .as_ref()
                    .and_then(|d| { d.reasoning_content.as_ref().map(|s| s.to_owned()) })
            })
    }
}

/**
 * Choice structure
 */
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Choice {
    /// Index of the choice in the response
    pub index: Option<i32>,
    /// sse response ,request body stream:true
    pub delta: Option<Delta>,
    /// general response,request body stream:false
    pub message: Option<Delta>,
    /// Log probabilities of the generated tokens
    pub logprobs: Option<serde_json::Value>,
    /// Reason why the generation stopped
    #[serde(rename = "finish_reason")]
    pub finish_reason: Option<String>,
}

impl Choice {
    /// Check if this choice is finished
    pub fn is_finished(&self) -> bool {
        self.finish_reason.is_some()
    }

    /// Get the content if available
    pub fn content(&self) -> Option<&str> {
        self.delta.as_ref().and_then(|d| d.content.as_deref())
    }
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct Delta {
    /// Generated text content
    pub content: Option<String>,
    /// Reasoning or explanation content
    #[serde(rename = "reasoning_content")]
    pub reasoning_content: Option<String>,
    /// Role associated with the content (e.g., "assistant")
    pub role: Option<String>,
}

impl Delta {
    /// Create a new Delta with content
    pub fn with_content(content: impl Into<String>) -> Self {
        Self {
            content: Some(content.into()),
            ..Default::default()
        }
    }

    /// Add reasoning content
    pub fn with_reasoning(mut self, reasoning: impl Into<String>) -> Self {
        self.reasoning_content = Some(reasoning.into());
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Usage {
    /// Number of tokens used in the prompt
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: Option<u32>,
    /// Number of tokens generated in the completion
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: Option<u32>,
    /// Total number of tokens used
    #[serde(rename = "total_tokens")]
    pub total_tokens: Option<u32>,
    /// Detailed breakdown of prompt token usage
    #[serde(rename = "prompt_tokens_details")]
    pub prompt_tokens_details: Option<PromptTokensDetails>,
    /// Number of tokens served from cache
    #[serde(rename = "prompt_cache_hit_tokens")]
    pub prompt_cache_hit_tokens: Option<u32>,
    /// Number of tokens not served from cache
    #[serde(rename = "prompt_cache_miss_tokens")]
    pub prompt_cache_miss_tokens: Option<u32>,
}

impl Usage {
    /// Calculate the percentage of cache hits
    pub fn cache_hit_ratio(&self) -> Option<f32> {
        match (self.prompt_cache_hit_tokens, self.prompt_cache_miss_tokens) {
            (Some(hits), Some(misses)) => {
                let total = hits + misses;
                if total > 0 {
                    Some((hits as f32) / (total as f32))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PromptTokensDetails {
    /// Number of tokens served from cache
    #[serde(rename = "cached_tokens")]
    pub cached_tokens: u32,
}
