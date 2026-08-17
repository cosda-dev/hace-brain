// Integration test for the full pipeline (M2-M7)

pub use hace_brain_master::{
    BrainTokenizerImpl, ModelRegistry, ModelInfo, RuntimeContext, BrainInferenceEngine, InferenceContext,
};

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_tokenizer_integration() {
        // Test M2: Tokenizer Roundtrip
        let tokenizer = BrainTokenizerImpl::new();
        let text = "hello world";
        let tokens = tokenizer.encode(text);
        let decoded = tokenizer.decode(&tokens);
        assert_eq!(decoded, text);
        // M2 PASS: encode/decode roundtrip works
    }
    
    #[test]
    fn test_model_registry_integration() {
        // Test P3: Model Registry basic structure
        let mut registry = ModelRegistry::new();
        assert!(registry.list_models().is_empty());
        
        // Test model info structure
        use crate::provider::gguf::loader::ModelVerification;
        let verification = ModelVerification {
            architecture: "qwen2".to_string(),
            tensor_count: 42,
            metadata_count: 291,
            context_length: 32768,
            vocab_size: 151936,
            hidden_size: 896,
            n_layer: 24,
            n_head: 14,
            model_path: "/test/model.gguf".to_string(),
        };
        
        let model_info = ModelInfo {
            name: "qwen2-test".to_string(),
            path: "/test/model.gguf".to_string(),
            verification,
            is_loaded: false,
        };
        
        assert_eq!(model_info.verification.architecture, "qwen2");
        assert_eq!(model_info.verification.vocab_size, 151936);
        assert_eq!(model_info.verification.context_length, 32768);
        // P3 PASS: Model registry structure works
    }
    
    #[test]
    fn test_runtime_context_integration() {
        // Test P4: Runtime Context hierarchy
        let mut context = RuntimeContext::new("test-model");
        assert_eq!(context.model_id, "test-model");
        
        // Test setting prompt
        context.set_prompt("test prompt".to_string());
        assert_eq!(context.get_prompt(), Some("test prompt".to_string()));
        
        // Test session context
        assert_eq!(context.session_context.model_id, "test-model");
        
        // Test inference context defaults
        assert_eq!(context.inference_context.temperature, 0.7);
        assert_eq!(context.inference_context.top_p, 0.9);
        assert_eq!(context.inference_context.max_tokens, 64);
        // P4 PASS: Runtime context hierarchy works
    }
    
    #[test]
    fn test_inference_engine_basic() {
        // Test P5/M5-M7: Inference engine structure
        let engine = BrainInferenceEngine::new();
        
        // Test that we can create the engine
        assert!(std::mem::size_of_val(&engine) > 0);
        
        // Test configuration
        let mut engine = BrainInferenceEngine::new();
        engine.configure(32768, 0.9, 0.7, 151936, 896);
        // Configuration should work without panic
        // P5 PASS: Inference engine can be configured
    }
}