use crate::api::message::Prompt;
use crate::api::providers::ApiProvider;
use crate::models::models::Model;

/// Represents a chat session with a language model
#[derive(Debug, Clone)]
pub struct ChatSession {
    /// The model used for this session
    pub model: Model,
    /// The API provider for this session
    pub provider: ApiProvider,
    /// History of messages in this session
    pub messages: Vec<Prompt>,
    /// Maximum number of messages to keep in history
    pub max_history: usize,
}

impl ChatSession {
    /// Creates a new chat session with the specified model and optional provider
    pub fn new(model: Model, provider: Option<ApiProvider>) -> Self {
        Self {
            model: model.clone(),
            provider: provider.unwrap_or_else(|| model.provider()),
            messages: Vec::new(),
            max_history: 20,
        }
    }

    /// Adds a new message to the session and maintains history limit
    pub fn add_message(&mut self, message: Prompt) {
        self.messages.push(message);
        // if messages.len() > max, remove the oldest messages
        while self.messages.len() > self.max_history {
            self.messages.remove(0);
        }
    }

    /// Sets the maximum number of messages to keep in history
    pub fn set_max_history(&mut self, max: usize) {
        self.max_history = max;
        //if messages.len() > max, remove the oldest messages
        while self.messages.len() > self.max_history {
            self.messages.remove(0);
        }
    }

    /// Clears all message history in this session
    pub fn clear_history(&mut self) {
        self.messages.clear();
    }

    /// Returns a reference to the current message history
    pub fn get_messages(&self) -> &[Prompt] {
        &self.messages
    }

    pub fn with_context(mut self, context: String) -> Self {
        self.add_message(Prompt::system(context));
        self
    }
}
