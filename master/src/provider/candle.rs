// Candle Brain Provider - FES PI Layer
// Wires hacedle inference engine through brain inference engine

use alloc::string::String;
use alloc::vec::Vec;

use super::{BrainProvider, ProviderError};
use crate::context::RuntimeSio;
use crate::outcome::SioOutcome;
use crate::inference::{BrainInferenceEngine, InferenceRequest, InferenceResponse};
use crate::tokenizer::BrainTokenizer;
use crate::model_registry::{ModelRegistry, ModelInfo};

/// Candle Brain Provider - Wrapper for Hacedle inference engine
pub struct CandleBrain {
    inference_engine: BrainInferenceEngine,
    model_registry: ModelRegistry,
    current_model: Option<String>,
}

impl CandleBrain {
    pub fn new() -> Self {
        Self {
            inference_engine: BrainInferenceEngine::new(),
            model_registry: ModelRegistry::new(),
            current_model: None,
        }
    }
    
    /// Load a GGUF model from the model registry
    pub fn load_model(&mut self, model_name: &str) -> Result<(), &'static str> {
        let model_info = self.model_registry.get_model(model_name)
            .ok_or("model_not_found")?;
        
        // Configure inference engine with model parameters
        self.inference_engine.configure(
            model_info.verification.context_length as u32,
            0.9,  // default top_p
            0.7,  // default temperature
            model_info.verification.vocab_size,
            model_info.verification.hidden_size
        );
        
        self.current_model = Some(model_name.to_string());
        self.model_registry.mark_loaded(model_name)?;
        
        Ok(())
    }
}

impl Default for CandleBrain {
    fn default() -> Self {
        Self::new()
    }
}

impl BrainProvider for CandleBrain {
    fn name(&self) -> &str {
        "candle"
    }
    
    fn execute(&self, sio: &RuntimeSio) -> Result<SioOutcome, ProviderError> {
        // Extract prompt from SIO
        let prompt = sio.prompt.as_ref()
            .ok_or(ProviderError::InvalidInput)?
            .clone();
            
        let max_tokens = sio.max_tokens.unwrap_or(64);
        let temperature = sio.temperature.unwrap_or(0.7);
        let top_p = sio.top_p.unwrap_or(0.9);
        
        // Use current model or load default
        let model_name = self.current_model.as_ref()
            .map(|s| s.as_str())
            .unwrap_or("default");
        
        // Create inference request
        let request = InferenceRequest {
            model_id: model_name.to_string(),
            prompt,
            max_tokens,
            temperature,
            top_p,
        };
        
        // Run inference (this will use the hacedle inference engine through brain wrapper)
        // Note: In a real implementation, we'd need to handle async properly
        // For now, we'll use a blocking call or mock the async
        let response = futures::executor::block_on(self.inference_engine.infer(request))
            .map_err(|_| ProviderError::ExecutionFailed)?;
        
        Ok(SioOutcome::Success {
            text: response.text,
            tokens: response.tokens,
        })
    }
}