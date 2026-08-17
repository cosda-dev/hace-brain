// Inference Engine - FES abstraction over providers

use alloc::string::String;
use alloc::vec::Vec;

use crate::context::RuntimeSio;
use crate::outcome::SioOutcome;

pub mod request;
pub mod response;
pub mod engine;
pub mod soul_binding;

pub use engine::BrainInferenceEngine;
pub use soul_binding::{SoulBinding, InferenceFacade};

pub struct InferenceRequest {
    pub model_id: String,
    pub prompt: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
}

impl Default for InferenceRequest {
    fn default() -> Self {
        Self {
            model_id: String::new(),
            prompt: String::new(),
            max_tokens: 64,
            temperature: 0.7,
            top_p: 0.9,
        }
    }
}

#[async_trait::async_trait]
pub trait InferenceEngine {
    async fn infer(&self, request: InferenceRequest) -> Result<InferenceResponse, &'static str>;
}

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