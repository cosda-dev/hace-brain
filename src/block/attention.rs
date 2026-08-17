
// Attention Block Implementation  
// Pipeline: QKV projection -> RoPE -> QK^T -> Softmax -> Attention

use crate::op::{rope, matmul};

pub fn attention_forward(
    input: &[f32],
    q_weight: &[f32],
    k_weight: &[f32],
    v_weight: &[f32],
    q_bias: &[f32],
    k_bias: &[f32],
    v_bias: &[f32],
    n_heads: usize,
    head_dim: usize,
    seq_len: usize,
    pos: usize,
) -> Vec<f32> {
    let hidden_size = n_heads * head_dim;
    
    // QKV projections
    let mut q = vec![0.0f32; hidden_size];
    let mut k = vec![0.0f32; hidden_size];
    let mut v = vec![0.0f32; hidden_size];
    
    for i in 0..hidden_size {
        for j in 0..input.len() {
            q[i] += q_weight[i * input.len() + j] * input[j];
            k[i] += k_weight[i * input.len() + j] * input[j];
            v[i] += v_weight[i * input.len() + j] * input[j];
        }
        q[i] += q_bias.get(i).copied().unwrap_or(0.0);
        k[i] += k_bias.get(i).copied().unwrap_or(0.0);
        v[i] += v_bias.get(i).copied().unwrap_or(0.0);
    }
    
    // Apply RoPE pairwise
    rope::apply_rope_pairwise(&mut q, pos, head_dim);
    rope::apply_rope_pairwise(&mut k, pos, head_dim);
    
    // Reshape for attention: Q [n_heads, head_dim], K/V [n_kv_heads, head_dim]
    let n_kv_heads = 2; // Qwen2.5
    let scores = compute_attention_scores(&q, &k, n_heads, n_kv_heads, head_dim);
    let attn = softmax(&scores);
    
    // Weighted sum
    let output = attention_weighted_sum(&attn, &v, n_heads, n_kv_heads, head_dim);
    
    output
}

fn compute_attention_scores(q: &[f32], k: &[f32], n_heads: usize, n_kv_heads: usize, head_dim: usize) -> Vec<f32> {
    let scores = vec![0.0f32; n_heads * n_heads]; // [14, 14] for GQA
    for i in 0..n_heads {
        for j in 0..n_heads {
            let kv_idx = j / (n_heads / n_kv_heads);
            let mut sum = 0.0f32;
            for d in 0..head_dim {
                sum += q[i * head_dim + d] * k[kv_idx * head_dim + d];
            }
            scores[i * n_heads + j] = sum / (head_dim as f32).sqrt();
        }
    }
    scores
}

fn softmax(scores: &[f32]) -> Vec<f32> {
    let n = scores.len() / 14; // rows = n_heads
    let mut out = vec![0.0f32; scores.len()];
    
    for row in 0..n {
        let start = row * 14;
        let row_scores = &scores[start..start + 14];
        let max_val = row_scores.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let sum: f32 = row_scores.iter().map(|x| (*x - max_val).exp()).sum();
        
        for (i, s) in row_scores.iter().enumerate() {
            out[start + i] = (*s - max_val).exp() / sum;
        }
    }
    out
}

fn attention_weighted_sum(scores: &[f32], v: &[f32], n_heads: usize, n_kv_heads: usize, head_dim: usize) -> Vec<f32> {
    let output = vec![0.0f32; n_heads * head_dim];
    for i in 0..n_heads {
        for d in 0..head_dim {
            let mut sum = 0.0f32;
            for j in 0..n_heads {
                let kv_idx = j / (n_heads / n_kv_heads);
                sum += scores[i * n_heads + j] * v[kv_idx * head_dim + d];
            }
            output[i * head_dim + d] = sum;
        }
    }
    output
}

