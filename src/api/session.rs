use crate::api::message::Prompt;
use crate::api::providers::ApiProvider;
use crate::models::models::Model;

#[derive(Debug, Clone)]
pub struct ChatSession {
    pub model: Model,
    pub provider: ApiProvider,
    pub messages: Vec<Prompt>,
    pub max_history: usize,
}

impl ChatSession {
    pub fn new(model: Model, provider: Option<ApiProvider>) -> Self {
        Self {
            model: model.clone(),
            provider: provider.unwrap_or_else(|| model.provider()),
            messages: Vec::new(),
            max_history: 20, // 默认保留20条消息
        }
    }

    pub fn add_message(&mut self, message: Prompt) {
        self.messages.push(message);
        // 如果超出最大历史记录限制，移除最早的消息
        while self.messages.len() > self.max_history {
            self.messages.remove(0);
        }
    }

    pub fn set_max_history(&mut self, max: usize) {
        self.max_history = max;
        // 立即应用新的限制
        while self.messages.len() > self.max_history {
            self.messages.remove(0);
        }
    }

    pub fn clear_history(&mut self) {
        self.messages.clear();
    }

    pub fn get_messages(&self) -> &[Prompt] {
        &self.messages
    }
}
