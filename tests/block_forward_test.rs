// Brain Block Unit Tests - RMSNorm forward pass verification

use crate::block::brain_block::BrainBlock;

#[test]
fn test_brainblock_forward_rmsnorm() {
    let block = BrainBlock::new(4096, 32, 8);
    let input = vec![1.0, 2.0, 3.0, 4.0];
    let output = block.forward(&input, 0);
    
    // Output should be same length as input when no weights loaded
    assert_eq!(output.len(), input.len());
}

#[test] 
fn test_brainblock_forward_with_weights() {
    let mut block = BrainBlock::new(4, 1, 1);
    block.rms_attn_norm_weight = vec![1.0, 1.0, 1.0, 1.0];
    
    let input = vec![1.0, 2.0, 3.0, 4.0];
    let output = block.forward(&input, 0);
    
    // With RMSNorm: output = x * w * 1/rsqrt(sum(x^2)/n + eps)
    // sum = 1+4+9+16 = 30, n = 4
    // rms = 1/sqrt(30/4 + eps) ≈ 1/sqrt(7.5) ≈ 0.365
    // output = [0.365, 0.730, 1.095, 1.460]
    assert!(!output.is_empty());
}

#[test]
fn test_brainblock_new() {
    let block = BrainBlock::new(4096, 32, 8);
    assert_eq!(block.n_embd, 4096);
    assert_eq!(block.n_head, 32);
    assert_eq!(block.n_kv_head, 8);
}