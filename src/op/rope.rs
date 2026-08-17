
// RoPE Implementation - Rotary Position Embedding
// Reference: https://arxiv.org/abs/2104.09864
// For Qwen2.5: theta = 1000000.0 (not 10000.0)

pub fn rope(theta: f32, pos: usize, dim: usize, x: &mut [f32]) {
    for i in 0..dim / 2 {
        let freq = theta.powf(2.0 * (i as f32) / (dim as f32));
        let inv_freq = 1.0 / freq;
        let angle = pos as f32 * inv_freq;
        let cos_val = angle.cos();
        let sin_val = angle.sin();
        
        let x1 = x[i];
        let x2 = x[i + dim / 2];
        
        x[i] = x1 * cos_val - x2 * sin_val;
        x[i + dim / 2] = x1 * sin_val + x2 * cos_val;
    }
}

pub fn apply_rope_pairwise(x: &mut [f32], pos: usize, dim: usize) {
    let theta: f32 = 1000000.0; // Qwen2.5 specific
    let n_total = x.len();
    let n_heads = n_total / dim;
    
    for head in 0..n_heads {
        let start = head * dim;
        let end = (head + 1) * dim;
        rope(theta, pos, dim, &mut x[start..end]);
    }
}

pub fn apply_rope(q: &mut [f32], k: &mut [f32], pos: usize, dim: usize) {
    apply_rope_pairwise(q, pos, dim);
    apply_rope_pairwise(k, pos, dim);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rope_basic() {
        let mut x = vec![1.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let original = x.clone();
        apply_rope(&mut x, &mut x, 0, 8);
        println!("RoPE applied: {:?}", x);
    }
}

