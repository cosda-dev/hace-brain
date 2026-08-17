// Brain Provider - FES PI Layer
// Delegates to FEM modules: fem/gguf, fem/hacedle, fem/tokenizer

pub mod candle;
pub mod mock;
pub mod gguf;

use alloc::string::String;

pub trait BrainProvider {
    fn name(&self) -> &str;
    fn execute(&self, sio: &super::context::RuntimeSio) -> Result<super::outcome::SioOutcome, ProviderError>;
}

#[derive(Default, Clone)]
pub struct ProviderCapability {
    pub inference: bool,
    pub embedding: bool,
    pub reasoning: bool,
    pub streaming: bool,
}

#[derive(Debug, Clone)]
pub enum ProviderError {
    ModelNotLoaded,
    InvalidInput,
    ExecutionFailed,
}

pub struct ProviderFactory;

impl ProviderFactory {
    pub fn create(provider_id: &str) -> Box<dyn BrainProvider> {
        match provider_id {
            "mock" => Box::new(mock::MockProvider::new()),
            "hacedle" | "candle" => Box::new(candle::CandleBrain::new()),
            _ => Box::new(mock::MockProvider::new()),
        }
    }
}