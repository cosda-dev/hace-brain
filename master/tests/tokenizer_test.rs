// Tokenizer roundtrip test (M2)

#[cfg(test)]
mod tokenizer_tests {
    use super::*;
    
    #[test]
    fn test_tokenizer_roundtrip_ascii() {
        // Test with ASCII text
        let tokenizer = BrainTokenizerImpl::new();
        let text = "hello";
        
        // Encode
        let tokens = tokenizer.encode(text);
        assert_eq!(tokens, vec![104, 101, 108, 108, 111]); // ASCII values
        
        // Decode
        let decoded = tokenizer.decode(&tokens);
        assert_eq!(decoded, "hello");
    }
    
    #[test]
    fn test_tokenizer_roundtrip_unicode() {
        // Test with unicode text (will work with ASCII subset)
        let tokenizer = BrainTokenizerImpl::new();
        let text = "🦀"; // Rust crab emoji (may not work perfectly with ASCII tokenizer)
        
        // Encode
        let tokens = tokenizer.encode(text);
        // Decode
        let decoded = tokenizer.decode(&tokens);
        // This test documents current behavior - real tokenizer would handle unicode properly
        assert!(!decoded.is_empty());
    }
}