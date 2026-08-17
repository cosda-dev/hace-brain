// Brain Kernel - FES TI Layer
pub mod hacedle;

use async_trait::async_trait;
use alloc::string::String;
use alloc::vec::Vec;

use crate::context::RuntimeSio;
use crate::outcome::SioOutcome;

/// BrainKernel trait - core inference interface
#[async_trait]
pub trait BrainKernel: Send + Sync {
    fn model_id(&self) -> &str;
    fn is_local(&self) -> bool { true }
    fn max_context_tokens(&self) -> u32 { 4096 }
    
    async fn reason(&self, ctx: &RuntimeSio) -> Result<SioOutcome, &'static str>;
    fn estimate_aep(&self, ctx: &RuntimeSio) -> u64;
}

/// BrainRuntime trait - extends BrainKernel
#[async_trait]
pub trait BrainRuntime: BrainKernel {
    fn ce_id(&self) -> &str;
    
    async fn load(&self, artifact: BrainArtifact) -> Result<(), &'static str>;
    async fn embed(&self, text: &str) -> Result<Embedding, &'static str>;
    async fn unload(&self) -> Result<(), &'static str>;
    fn is_ready(&self) -> bool;
}

/// BrainArtifact - model artifact for loading
pub struct BrainArtifact {
    pub model_path: String,
    pub model_type: String,
}

/// Embedding output
pub struct Embedding {
    pub vectors: Vec<f32>,
    pub dim: usize,
}