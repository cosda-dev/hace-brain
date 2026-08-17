// End-to-end test with actual GGUF models

#[cfg(test)]
mod end_to_end_tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_load_qwen2_5_05b_metadata() {
        // Use the small Qwen2.5-0.5B model for faster testing
        let model_path = r"D:\host\llama-models\Qwen2.5-0.5B-Instruct-Q4_K_M.gguf";
        
        // Verify the file exists
        assert!(Path::new(model_path).exists(), "Model file not found at {}", model_path);
        
        // Test that our GGUF loader can read metadata
        let loader = super::super::provider::gguf::loader::BrainGgufLoader::new();
        let verification = loader.verify(model_path).expect("Failed to verify GGUF model");
        
        // Basic sanity checks on the metadata
        assert!(!verification.architecture.is_empty(), "Architecture should not be empty");
        assert_eq!(verification.architecture, "qwen2", "Expected qwen2 architecture");
        assert!(verification.vocab_size > 0, "Vocab size should be positive");
        assert!(verification.context_length > 0, "Context length should be positive");
        assert!(verification.hidden_size > 0, "Hidden size should be positive");
        assert!(verification.n_layer > 0, "Number of layers should be positive");
        assert!(verification.n_head > 0, "Number of heads should be positive");
        assert_eq!(verification.model_path, model_path, "Model path should match");
        
        println!("GGUF Model Verification:");
        println!("  Architecture: {}", verification.architecture);
        println!("  Vocab size: {}", verification.vocab_size);
        println!("  Context length: {}", verification.context_length);
        println!("  Hidden size: {}", verification.hidden_size);
        println!("  Layers: {}", verification.n_layer);
        println!("  Heads: {}", verification.n_head);
        println!("  Tensor count: {}", verification.tensor_count);
        println!("  Metadata count: {}", verification.metadata_count);
    }

    #[test]
    fn test_model_registry_with_real_gguf() {
        let mut registry = ModelRegistry::new();
        let model_path = r"D:\host\llama-models\Qwen2.5-0.5B-Instruct-Q4_K_M.gguf";
        let model_name = "qwen2_5_05b";
        
        // Register the model (this will verify and store metadata)
        let result = registry.register_model(model_name, model_path);
        assert!(result.is_ok(), "Failed to register model: {}", result.err().unwrap());
        
        // Retrieve the model info
        let model_info = registry.get_model(model_name).expect("Model not found after registration");
        assert_eq!(model_info.name, model_name);
        assert_eq!(model_info.path, model_path);
        assert!(!model_info.is_loaded, "Model should not be loaded initially");
        
        // Check that we got reasonable metadata
        assert_eq!(model_info.verification.architecture, "qwen2");
        assert!(model_info.verification.vocab_size > 10000);
        assert!(model_info.verification.context_length >= 32768); // Qwen2.5 has 32768
        
        // Mark as loaded
        let load_result = registry.mark_loaded(model_name);
        assert!(load_result.is_ok(), "Failed to mark model as loaded");
        
        let loaded_info = registry.get_model(model_name).expect("Model not found");
        assert!(loaded_info.is_loaded, "Model should be marked as loaded");
    }

    #[test]
    fn test_tokenizer_with_gguf_vocab() {
        // This test would require loading the actual tokenizer vocab from the GGUF
        // For now, we test that our tokenizer works with the BrainTokenizerImpl
        // In a full implementation, we would extract the vocab from GGUF and configure the tokenizer
        
        let tokenizer = BrainTokenizerImpl::new();
        let text = "Hello, world!";
        let tokens = tokenizer.encode(text);
        let decoded = tokenizer.decode(&tokens);
        
        // With our current ASCII-based tokenizer, we expect exact roundtrip for ASCII text
        assert_eq!(decoded, text);
        println!("Tokenized '{}' -> {:?} -> '{}'", text, tokens, decoded);
    }
}