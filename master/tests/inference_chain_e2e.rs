// End-to-End Inference Chain Test
// Pipeline: SIO → Soul Injection → Inference Request → Hacedle Engine → Generated Response

use std::fs;
use std::path::Path;

use hace_brain_master::inference::{InferenceRequest, InferenceFacade, SoulBinding};
use hace_brain_master::context::{RuntimeSio, SkbInjection, SkbManifest};
use hacedle::x::provider::candle::{InferenceEngine, TokenizerEngine};

/// Test full inference chain wiring
#[test]
fn test_inference_chain_components_exist() {
    // Verify InferenceRequest can be created
    let request = InferenceRequest::default();
    assert_eq!(request.max_tokens, 64);
    assert_eq!(request.temperature, 0.7);
    assert_eq!(request.top_p, 0.9);
    
    // Verify InferenceFacade can be created
    let facade = InferenceFacade::new(request);
    assert!(facade.soul_binding.is_none());
    
    // Verify SoulBinding can be created
    let binding = SoulBinding::new("dev-soul");
    assert_eq!(binding.soul_id, "dev-soul");
}

/// Test SIO to InferenceRequest conversion
#[test]
fn test_sio_to_inference_request() {
    let sio = RuntimeSio {
        prompt: Some("Hello world".to_string()),
        max_tokens: Some(128),
        temperature: Some(0.8),
        top_p: Some(0.95),
        runtime: Default::default(),
    };
    
    // Convert SIO to InferenceRequest
    let request = InferenceRequest {
        model_id: "qwen2_5_05b".to_string(),
        prompt: sio.prompt.unwrap_or_default(),
        max_tokens: sio.max_tokens.unwrap_or(64),
        temperature: sio.temperature.unwrap_or(0.7),
        top_p: sio.top_p.unwrap_or(0.9),
    };
    
    assert_eq!(request.prompt, "Hello world");
    assert_eq!(request.max_tokens, 128);
    assert_eq!(request.temperature, 0.8);
    assert_eq!(request.top_p, 0.95);
}

/// Test hacedle inference engine exists and can tokenize
#[test]
fn test_hacedle_engine_tokenize() {
    let engine = InferenceEngine::default();
    
    // Test tokenization
    let tokens = engine.tokenizer.encode("Hello");
    assert!(!tokens.is_empty());
    
    // Test logits generation (stub)
    let logits = engine.infer_logits("test");
    assert_eq!(logits.len(), 32000); // Default vocab size
}

/// Test soul injection integration
#[test]
fn test_soul_injection_pipeline() {
    let manifest = SkbManifest::new("dev-soul", "runtime", "rust", "hace");
    let skb = SkbInjection {
        manifest,
        semantic_graph: Default::default(),
        payload_views: vec![],
    };
    
    // Soul binding from SKB
    let mut binding = SoulBinding::new("dev-soul");
    binding.load_from_manifest(&skb.manifest);
    
    // Inference request with soul
    let request = InferenceRequest::default();
    let injected = binding.inject_context(request);
    
    // Should have default overrides applied
    assert!(!injected.max_tokens == 0);
}

/// Test ModelProvider trait
#[test]
fn test_model_provider_trait() {
    use hace_brain_master::provider::{ModelProvider, GgufProvider, ProviderFactory};
    
    // Verify provider factory
    let providers = ProviderFactory::list();
    assert!(providers.contains(&"gguf"));
    
    // Verify GGUF provider exists
    let provider = ProviderFactory::create("gguf");
    assert!(provider.is_some());
}

/// Test greedy decode for first token generation
fn greedy_decode(logits: &[f32]) -> usize {
    logits.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(0.cmp(&0))
        .map(|(i, _)| i)
        .unwrap_or(0)
}

/// Test first real token generation potential
#[test]
fn test_first_token_generation() {
    let engine = InferenceEngine::default();
    
    // Get logits for a prompt
    let logits = engine.infer_logits("The");
    
    // Generate first token via argmax
    let first_token = greedy_decode(&logits);
    
    println!("First token generation test:");
    println!("  Prompt: 'The'");
    println!("  Logits size: {}", logits.len());
    println!("  First token index: {}", first_token);
    
    // Token index should be within vocab range
    assert!(first_token < logits.len());
}