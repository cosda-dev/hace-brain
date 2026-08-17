// Model registry and inference test (P3, P5, M5-M7)

pub use hace_brain_master::{ModelRegistry, ModelInfo, BrainInferenceEngine, InferenceRequest};

#[cfg(test)]
mod model_registry_tests {
    use super::*;
    
    #[test]
    fn test_model_registry_basic() {
        let mut registry = ModelRegistry::new();
        
        // Register a model (this would normally load from actual GGUF file)
        let result = registry.register_model("test-model", "/fake/path/to/model.gguf");
        // This will fail because we can't actually verify the fake path, but we can test the structure
        assert!(result.is_err()); // Expected to fail on fake path
        
        // Test with a mock verification for structure testing
        // In real implementation, we'd mock the BrainGgufLoader or use a test GGUF
    }
    
    #[test]
    fn test_model_info_structure() {
        use hace_brain_master::provider::gguf::loader::ModelVerification;
        
        let verification = ModelVerification {
            architecture: "qwen2".to_string(),
            tensor_count: 100,
            metadata_count: 50,
            context_length: 32768,
            vocab_size: 32000,
            hidden_size: 4096,
            n_layer: 32,
            n_head: 32,
            model_path: "/test/model.gguf".to_string(),
        };
        
        let model_info = ModelInfo {
            name: "test-model".to_string(),
            path: "/test/model.gguf".to_string(),
            verification,
            is_loaded: false,
        };
        
        assert_eq!(model_info.name, "test-model");
        assert_eq!(model_info.verification.architecture, "qwen2");
        assert_eq!(model_info.verification.vocab_size, 32000);
        assert!(!model_info.is_loaded);
    }
}

#[cfg(test)]
mod inference_tests {
    use super::*;
    
    #[test]
    fn test_inference_engine_structure() {
        let engine = BrainInferenceEngine::new();
        // Just test that it creates successfully
        assert!(std::mem::size_of_val(&engine) > 0);
    }
    
    #[test]
    fn test_inference_request_default() {
        let request = InferenceRequest::default();
        assert_eq!(request.max_tokens, 64);
        assert_eq!(request.temperature, 0.7);
        assert_eq!(request.top_p, 0.9);
        assert!(request.model_id.is_empty());
        assert!(request.prompt.is_empty());
    }
}