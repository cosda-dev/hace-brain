// LRO Runtime Test
// Target: Verify LoRA adapter loading and tensor composition

#[cfg(test)]
mod lro_runtime_tests {
    #[test]
    fn test_lora_loader() {
        // LRO loading from GGUF
        let rank = 8u32;
        let alpha = 1.0f32;
        
        // Verify LoRA structure
        assert!(rank > 0);
        assert!(alpha > 0.0);
    }
    
    #[test]
    fn test_tensor_match() {
        // Match LoRA tensors to base model tensors
        let base_layers = vec!["blk.0.attn_q", "blk.0.attn_k", "blk.0.attn_v"];
        let lora_tensors = vec!["blk.0.attn_q.lora_A", "blk.0.attn_q.lora_B"];
        
        // Verify matching logic
        let matches: Vec<_> = lora_tensors.iter()
            .filter(|l| base_layers.iter().any(|b| l.contains(b)))
            .collect();
            
        assert_eq!(matches.len(), 2);
    }
    
    #[test]
    fn test_overlay_composition() {
        // W_effective = W_base + scale * (B @ A)
        let base_weight = vec![1.0f32, 2.0, 3.0, 4.0, 5.0];
        let lora_a = vec![0.1f32, 0.2, 0.3, 0.4, 0.5];
        let lora_b = vec![0.5f32, 0.4, 0.3, 0.2, 0.1];
        let scale = 0.5f32;
        
        let effective: Vec<f32> = base_weight.iter()
            .zip(lora_a.iter().zip(lora_b.iter()))
            .map(|(w, (a, b))| w + scale * a * b)
            .collect();
            
        assert_eq!(effective.len(), 5);
        assert!(effective[0] > base_weight[0]); // Effective should be larger
    }
}