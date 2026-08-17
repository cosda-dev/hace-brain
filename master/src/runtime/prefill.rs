// Prefill - Process prompt tokens into embeddings
// G6: initial context processing - delegates to hacedle

use alloc::string::String;
use alloc::vec::Vec;

use hacedle::x::provider::candle::{BpeTokenizer, TokenizerEngine, EmbedEngine};

/// Prefill processor
pub struct Prefill {
    tokenizer: BpeTokenizer,
    embed: EmbedEngine,
}

impl Prefill {
    pub fn new() -> Self {
        Self {
            tokenizer: BpeTokenizer::new(),
            embed: EmbedEngine::new(),
        }
    }

    /// Process prompt and return embeddings
    pub fn process(&self, prompt: &str) -> Vec<f32> {
        // Tokenize prompt
        let tokens = self.tokenizer.encode(prompt);
        
        // Embed tokens (stub - would use real embeddings from model)
        self.embed.embed_sequence(&tokens)
    }

    /// Tokenize prompt
    pub fn tokenize(&self, prompt: &str) -> Vec<u32> {
        self.tokenizer.encode(prompt)
    }
}

impl Default for Prefill {
    fn default() -> Self {
        Self::new()
    }
}