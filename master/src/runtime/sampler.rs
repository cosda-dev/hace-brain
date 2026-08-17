// Sampler - Token selection from logits
// G8: first token generation

use alloc::vec::Vec;

/// Sampler for token selection
pub struct Sampler;

impl Sampler {
    /// Argmax sampling - select token with highest logit
    pub fn argmax(logits: &[f32]) -> usize {
        logits.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Top-k sampling
    pub fn top_k(logits: &[f32], k: usize) -> Vec<(usize, f32)> {
        let mut indexed: Vec<(usize, f32)> = logits.iter().enumerate()
            .map(|(i, &v)| (i, v))
            .collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        indexed[..k.min(indexed.len())].to_vec()
    }

    /// Top-p (nucleus) sampling
    pub fn top_p(logits: &[f32], p: f32) -> Vec<(usize, f32)> {
        let mut sorted = Self::top_k(logits, logits.len());
        let mut cumulative = 0.0f32;
        let mut cutoff = logits.len();
        
        for (i, (_, prob)) in sorted.iter().enumerate() {
            let exp_prob = prob.exp().min(1.0);
            cumulative += exp_prob;
            if cumulative >= p {
                cutoff = i + 1;
                break;
            }
        }
        
        sorted[..cutoff].to_vec()
    }
}