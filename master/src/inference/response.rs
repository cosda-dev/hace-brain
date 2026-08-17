// Inference Response Types

use alloc::string::String;
use alloc::vec::Vec;

/// Inference response
pub struct InferenceResponse {
    pub tokens: Vec<u32>,
    pub text: String,
}

impl Default for InferenceResponse {
    fn default() -> Self {
        Self {
            tokens: Vec::new(),
            text: String::new(),
        }
    }
}

/// Usage statistics
#[derive(Default)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
}

/// Stream chunk for streaming generation
pub struct StreamChunk {
    pub token: u32,
    pub text: String,
    pub finished: bool,
}