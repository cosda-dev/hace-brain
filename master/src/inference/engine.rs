// Inference Engine Wrapper - Delegates to hacedle InferenceEngine (P5)

use alloc::string::String;
use alloc::vec::Vec;

use hacedle::x::provider::candle::{InferenceEngine as HacedleInferenceEngine, LogitsProcessor};

/// Brain Inference Engine - wraps hacedle InferenceEngine
pub struct BrainInferenceEngine {
    inference_engine: HacedleInferenceEngine,
}

impl BrainInferenceEngine {
    pub fn new() -> Self {
        Self {
            inference_engine: HacedleInferenceEngine::default(),
        }
    }
    
    /// Configure the inference engine with model parameters
    pub fn configure(&mut self, context_size: u32, top_p: f32, temperature: f32, vocab_size: usize, embed_dim: usize) {
        self.inference_engine = HacedleInferenceEngine {
            context_size,
            top_p,
            temperature,
            embed: hacedle::x::provider::candle::EmbedEngine::new(),
            transformer: hacedle::x::provider::candle::Transformer24::new(embed_dim, 32, 8, vocab_size),
            lm_head: hacedle::x::provider::candle::LMHead::new(vocab_size, embed_dim),
            tokenizer: hacedle::x::provider::candle::BpeTokenizer::new(),
            logits_processor: LogitsProcessor::new(temperature, top_p, 50),
        };
    }
}

#[async_trait::async_trait]
impl super::InferenceEngine for BrainInferenceEngine {
    async fn infer(&self, request: super::InferenceRequest) -> Result<super::InferenceResponse, &'static str> {
        // Tokenize
        let tokens = self.inference_engine.tokenizer.encode(&request.prompt);
        
        // Generate tokens
        let mut generated_tokens = Vec::new();
        let mut all_tokens = tokens.clone();
        
        for _ in 0..request.max_tokens {
            // Get logits for current tokens
            let logits = self.inference_engine.infer_logits(&self.inference_engine.tokenizer.decode(&all_tokens));
            
            // Process logits (temperature, top-p, top-k)
            let processed = self.inference_engine.logits_processor.process(logits);
            
            // Simple argmax sampling (M5: First Token)
            let next_token = processed.iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(i, _)| i as u32)
                .unwrap_or(0);
                
            all_tokens.push(next_token);
            generated_tokens.push(next_token);
        }
        
        // Decode generated tokens (M6: First Word, M7: First Sentence)
        let generated_text = self.inference_engine.tokenizer.decode(&generated_tokens);
        
        Ok(super::InferenceResponse {
            tokens: generated_tokens,
            text: generated_text,
        })
    }
}

impl Default for BrainInferenceEngine {
    fn default() -> Self {
        Self::new()
    }
}