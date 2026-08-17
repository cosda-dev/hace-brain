// Hacedle BrainKernel Implementation - Wires to BrainInferenceEngine (P5)

use async_trait::async_trait;
use alloc::string::String;
use alloc::vec::Vec;

use super::{BrainKernel, BrainRuntime, BrainArtifact, Embedding};
use crate::context::RuntimeSio;
use crate::outcome::SioOutcome;
use crate::inference::{InferenceEngine, InferenceRequest, BrainInferenceEngine};

/// HacedleBrain - implements BrainKernel trait using hacedle inference
pub struct HacedleBrain {
    inference_engine: BrainInferenceEngine,
    model_path: Option<String>,
    loaded: bool,
}

impl HacedleBrain {
    pub fn new() -> Self {
        Self {
            inference_engine: BrainInferenceEngine::new(),
            model_path: None,
            loaded: false,
        }
    }
    
    /// Configure model parameters from GGUF metadata
    pub fn configure(&mut self, context_size: u32, vocab_size: usize, embed_dim: usize) {
        self.inference_engine.configure(context_size, 0.9, 0.7, vocab_size, embed_dim);
    }
}

impl Default for HacedleBrain {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BrainKernel for HacedleBrain {
    fn model_id(&self) -> &str {
        "hacedle.qwen2.5-0.5b"
    }

    fn max_context_tokens(&self) -> u32 {
        4096
    }

    async fn reason(&self, ctx: &RuntimeSio) -> Result<SioOutcome, &'static str> {
        let prompt = ctx.prompt.as_ref().ok_or("no_prompt")?;
        let max_tokens = ctx.max_tokens.unwrap_or(64);
        
        let request = InferenceRequest {
            model_id: self.model_id().to_string(),
            prompt: prompt.clone(),
            max_tokens,
            temperature: ctx.temperature.unwrap_or(0.7),
            top_p: ctx.top_p.unwrap_or(0.9),
        };
        
        let response = self.inference_engine.infer(request).await
            .map_err(|e| *e)?;
        
        Ok(SioOutcome::Success {
            text: response.text,
            tokens: response.tokens,
        })
    }

    fn estimate_aep(&self, ctx: &RuntimeSio) -> u64 {
        let tokens = ctx.max_tokens.unwrap_or(64) as u64;
        tokens * 10
    }
}

#[async_trait]
impl BrainRuntime for HacedleBrain {
    fn ce_id(&self) -> &str {
        "ce.hacedle"
    }

    async fn load(&self, artifact: BrainArtifact) -> Result<(), &'static str> {
        self.model_path = Some(artifact.model_path);
        Ok(())
    }

    async fn embed(&self, text: &str) -> Result<Embedding, &'static str> {
        // Use tokenizer to get token count as proxy for embedding
        let tokens = self.inference_engine.inference_engine.tokenizer.encode(text);
        Ok(Embedding {
            vectors: vec![0.0; tokens.len() * 4096],
            dim: 4096,
        })
    }

    async fn unload(&self) -> Result<(), &'static str> {
        self.model_path = None;
        Ok(())
    }

    fn is_ready(&self) -> bool {
        self.loaded
    }
}