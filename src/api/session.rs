use uuid::Uuid;
use crate::api::message::Message;

pub struct Session {
    id: String,
    messages: Vec<Message>,
    max_history: usize,
}

impl Session {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            messages: Vec::new(),
            max_history: 20,
        }
    }

    pub fn with_max_history(mut self, max: usize) -> Self {
        self.max_history = max;
        self
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.truncate_history();
    }

    fn truncate_history(&mut self) {
        if self.max_history > 0 && self.messages.len() > self.max_history {
            let to_remove = self.messages.len() - self.max_history;
            self.messages.drain(0..to_remove);
        }
    }

    pub fn get_messages(&self) -> &Vec<Message> {
        &self.messages
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}