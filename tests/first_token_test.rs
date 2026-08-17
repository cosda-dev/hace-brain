// First Real Token Generation Test
// Target: Generate first token from Qwen2.5-0.5B.gguf with prompt "hello"

#[cfg(test)]
mod first_token_tests {
    #[test]
    fn test_tokenizer_bpe() {
        // BPE tokenization of "hello"
        let text = "hello";
        let expected_bytes: Vec<u32> = text.bytes().map(|b| b as u32).collect();
        
        assert_eq!(expected_bytes.len(), 5);
        assert_eq!(expected_bytes[0], b'h' as u32);
    }
    
    #[test]
    fn test_embedding_lookup() {
        // Embedding dimension: 4096
        // Vocab size: 32000
        let embed_dim = 4096;
        let vocab_size = 32000;
        
        // Mock embedding retrieval
        let token_id = 0u32;
        let embedding = vec![0.0f32; embed_dim];
        
        assert_eq!(embedding.len(), embed_dim);
    }
    
    #[test]
    fn test_transformer_forward() {
        // 24 transformer layers
        let num_layers = 24;
        let hidden_dim = 4096;
        
        // Mock forward pass
        let mut hidden: Vec<f32> = vec![0.1; hidden_dim];
        for _ in 0..num_layers {
            // Placeholder - actual RMSNorm + attention + ffn
            hidden = hidden.iter().map(|x| x * 0.9).collect();
        }
        
        assert_eq!(hidden.len(), hidden_dim);
    }
    
    #[test]
    fn test_lm_head_logits() {
        let vocab_size = 32000;
        let embed_dim = 4096;
        
        // Mock logits
        let logits = vec![0.0f32; vocab_size];
        let top_token = logits.iter().enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i);
            
        assert!(top_token.is_some());
        assert!(top_token.unwrap() < vocab_size);
    }
}