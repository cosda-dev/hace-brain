//! Intent Processing for Brain Base Layer
//!
//! This module handles normalizing input into structured intents.

use serde::{Deserialize, Serialize};

/// Represents a structured intent derived from input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub id: String,
    pub action: String,
    pub target: String,
    pub parameters: std::collections::HashMap<String, String>,
    pub context: std::collections::HashMap<String, String>,
}

impl Intent {
    /// Creates a new Intent
    pub fn new(id: String, action: String, target: String) -> Self {
        Self {
            id,
            action,
            target,
            parameters: std::collections::HashMap::new(),
            context: std::collections::HashMap::new(),
        }
    }

    /// Adds a parameter to the intent
    pub fn with_parameter(mut self, key: String, value: String) -> Self {
        self.parameters.insert(key, value);
        self
    }

    /// Adds context to the intent
    pub fn with_context(mut self, key: String, value: String) -> Self {
        self.context.insert(key, value);
        self
    }
}

/// Parses a raw input string into a structured Intent
pub fn parse_intent(input: &str) -> anyhow::Result<Intent> {
    // Very simplified intent parsing for demonstration
    // In a real implementation, this would use NLP or other techniques
    
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return Err(anyhow::anyhow!("Empty input"));
    }

    let action = parts[0].to_string();
    let target = if parts.len() > 1 {
        parts[1..].join(" ")
    } else {
        "".to_string()
    };

    Ok(Intent::new(
        uuid::Uuid::new_v4().to_string(),
        action,
        target,
    ))
}