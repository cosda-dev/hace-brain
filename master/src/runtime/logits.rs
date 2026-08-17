// Logits - Output logits from model
// G7: forward pass through transformer + lmhead

use alloc::vec::Vec;

use hacedle::x::provider::candle::{InferenceEngine, LogitsProcessor};

/// Logits output from model
pub struct Logits {
    pub values: Vec<f32>,
    pub topk: Vec<(usize, f32)>,
}

impl Logits {
    /// Run forward pass through hacedle inference engine
    pub fn forward(engine: &InferenceEngine, prompt: &str) -> Self {
        // Get logits from hacedle
        let logit_values = engine.infer_logits(prompt);
        
        // Store top-5 by default (stub selection)
        let mut topk = logit_values.iter().enumerate()
            .map(|(i, &v)| (i, v))
            .collect::<Vec<_>>();
        topk.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let topk = topk[..5.min(topk.len())].to_vec();
        
        Self {
            values: logit_values,
            topk,
        }
    }

    pub fn top_k(&self, k: usize) -> &[(usize, f32)] {
        &self.topk[..k.min(self.topk.len())]
    }
    
    /// Get top token via argmax
    pub fn argmax(&self) -> usize {
        self.topk.first().map(|(i, _)| *i).unwrap_or(0)
    }
}