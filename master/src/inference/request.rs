// Inference Request Types

use alloc::string::String;

/// Inference request builder
pub struct InferenceRequestBuilder {
    model_id: String,
    prompt: String,
    max_tokens: u32,
    temperature: f32,
    top_p: f32,
    stop_sequences: Vec<String>,
}

impl InferenceRequestBuilder {
    pub fn new() -> Self {
        Self {
            model_id: String::new(),
            prompt: String::new(),
            max_tokens: 64,
            temperature: 0.7,
            top_p: 0.9,
            stop_sequences: Vec::new(),
        }
    }

    pub fn model(mut self, id: &str) -> Self {
        self.model_id = id.to_string();
        self
    }

    pub fn prompt(mut self, text: &str) -> Self {
        self.prompt = text.to_string();
        self
    }

    pub fn max_tokens(mut self, n: u32) -> Self {
        self.max_tokens = n;
        self
    }

    pub fn temperature(mut self, t: f32) -> Self {
        self.temperature = t;
        self
    }

    pub fn top_p(mut self, p: f32) -> Self {
        self.top_p = p;
        self
    }
}

impl Default for InferenceRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}