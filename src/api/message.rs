use serde::{ Deserialize, Serialize };

/// Enum representing different roles in a conversation
///
/// Variants correspond to standard chat completion roles:
/// - System: Sets assistant's behavior
/// - User: End-user input
/// - Assistant: AI-generated responses
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RoleType {
    User,
    System,
    Assistant,
}

impl RoleType {
    /// Converts enum variant to its lowercase string representation
    pub fn as_str(&self) -> &str {
        match self {
            RoleType::User => "user",
            RoleType::System => "system",
            RoleType::Assistant => "assistant",
        }
    }
}

/// Represents a single message in a conversation chain
///
/// Contains both the message content and the sender's role type.
/// Used for building context-aware chat completions.
#[derive(Debug, Serialize, Deserialize)]
pub struct Prompt {
    /// Role type of the message sender
    pub role: RoleType,
    /// Actual text content of the message
    pub content: String,
}

impl Prompt {
    /// Creates a new message with specified role and content
    ///
    /// # Example
    /// ```
    /// let system_msg = Prompt::new(RoleType::System, "You are helpful");
    /// ```
    pub fn new(role: RoleType, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }

    /// Creates a system-level instruction message
    ///
    /// Shorthand for `new(RoleType::System, ...)`
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

    /// Updates message content using builder pattern
    ///
    /// # Example
    /// ```
    /// let msg = Prompt::user("Hello").with_content("Hi there!");
    /// ```
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    /// Gets character count of the content (not byte length)
    pub fn len(&self) -> usize {
        self.content.len()
    }

    /// Checks if content is empty string
    ///
    /// Returns true when content has zero characters
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Creates cloned message with replaced content
    ///
    /// Preserves original role type while replacing text content
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
