use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RoleType {
    User,
    System,
    Assistant,
}

impl RoleType {
    pub fn as_str(&self) -> &str {
        match self {
            RoleType::User => "user",
            RoleType::System => "system",
            RoleType::Assistant => "assistant",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prompt {
    pub role: RoleType,
    pub content: String,
}

impl Prompt {
    /// Create a new prompt with specified role and content
    pub fn new(role: RoleType, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }

    /// Create a system message
    pub fn system(content: impl Into<String>) -> Self {
        Self::new(RoleType::System, content)
    }

    /// Create a user message
    pub fn user(content: impl Into<String>) -> Self {
        Self::new(RoleType::User, content)
    }

    /// Create an assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new(RoleType::Assistant, content)
    }

    /// Update the content of the prompt
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    /// Get the length of the content in characters
    pub fn len(&self) -> usize {
        self.content.len()
    }

    /// Check if the content is empty
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Clone the prompt with new content
    pub fn clone_with_content(&self, content: impl Into<String>) -> Self {
        Self::new(self.role.clone(), content)
    }
}

// Add common conversions
impl From<(RoleType, String)> for Prompt {
    fn from(tuple: (RoleType, String)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}

impl From<(RoleType, &str)> for Prompt {
    fn from(tuple: (RoleType, &str)) -> Self {
        Self::new(tuple.0, tuple.1.to_string())
    }
}

// Add Clone trait implementation
impl Clone for Prompt {
    fn clone(&self) -> Self {
        Self {
            role: self.role.clone(),
            content: self.content.clone(),
        }
    }
}
