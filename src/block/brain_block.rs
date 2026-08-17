
// Brain Block - Transformer Layer Executable
// P1: BrainBlock struct với Transformer weights

use crate::op::{rope, matmul};

pub struct BrainBlock {
    pub n_embd: usize,
    pub n_head: usize,
    pub n_kv_head: usize,
    
    pub rms_attn_norm_weight: Vec<f32>,
    pub rms_ffn_norm_weight: Vec<f32>,
    
    pub q_proj_weight: Vec<f32>,
    pub q_proj_bias: Vec<f32>,
    
    pub k_proj_weight: Vec<f32>,
    pub k_proj_bias: Vec<f32>,
    
    pub v_proj_weight: Vec<f32>,
    pub v_proj_bias: Vec<f32>,
    
    pub o_proj_weight: Vec<f32>,
    pub o_proj_bias: Vec<f32>,
    
    pub gate_proj_weight: Vec<f32>,
    pub gate_proj_bias: Vec<f32>,
    
    pub up_proj_weight: Vec<f32>,
    pub up_proj_bias: Vec<f32>,
    
    pub down_proj_weight: Vec<f32>,
    pub down_proj_bias: Vec<f32>,
}

impl BrainBlock {
    pub fn new(n_embd: usize, n_head: usize, n_kv_head: usize) -> Self {
        Self {
            n_embd,
            n_head,
            n_kv_head,
            rms_attn_norm_weight: Vec::new(),
            rms_ffn_norm_weight: Vec::new(),
            q_proj_weight: Vec::new(),
            q_proj_bias: Vec::new(),
            k_proj_weight: Vec::new(),
            k_proj_bias: Vec::new(),
            v_proj_weight: Vec::new(),
            v_proj_bias: Vec::new(),
            o_proj_weight: Vec::new(),
            o_proj_bias: Vec::new(),
            gate_proj_weight: Vec::new(),
            gate_proj_bias: Vec::new(),
            up_proj_weight: Vec::new(),
            up_proj_bias: Vec::new(),
            down_proj_weight: Vec::new(),
            down_proj_bias: Vec::new(),
        }
    }

    pub fn forward(&self, hidden: &[f32], _pos: usize) -> Vec<f32> {
        // Placeholder: apply RMSNorm + attention + FFN
        // For now return RMSNorm normalized
        if hidden.is_empty() || self.rms_attn_norm_weight.is_empty() {
            return hidden.to_vec();
        }
        
        let len = hidden.len().min(self.rms_attn_norm_weight.len());
        let ss: f32 = hidden[..len].iter().map(|&x| x * x).sum();
        let rms = (ss / len as f32 + 1e-5).sqrt().recip();
        hidden[..len].iter()
            .zip(self.rms_attn_norm_weight[..len].iter())
            .map(|(&x, &w)| x * w * rms)
            .collect()
    }
}

