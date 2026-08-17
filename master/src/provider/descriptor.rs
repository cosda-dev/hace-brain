// Model Provider Descriptor - Unified interface for model loading
// NOTE: Currently delegates to hacedle until fem/gguf is ready for production
// TODO: Wire to fem-gguf when ModelLoader trait is complete

use alloc::string::String;
use alloc::vec::Vec;

/// Model provider trait - abstracts GGUF, ONNX, Safetensors
// TEMPORARY: Will delegate to fem/gguf trait when ready
pub trait ModelProvider {
    fn id(&self) -> &str;
    fn load(&self, model_path: &str) -> Result<ModelHandle, ProviderError>;
    fn supports(&self, format: &str) -> bool {
        format == "gguf"
    }
}

/// Model handle returned by provider
pub struct ModelHandle {
    pub model_id: String,
    pub tensor_count: usize,
    pub vocab_size: usize,
    pub context_length: usize,
    pub hidden_size: usize,
    pub n_layer: usize,
    pub n_head: usize,
}

/// Provider error types
#[derive(Debug, Clone)]
pub enum ProviderError {
    LoadFailed(String),
    InvalidPath(String),
    Unsupported(String),
}

/// GGUF Provider implementation
// TEMPORARY: Uses hacedle directly until fem/gguf is production ready
pub struct GgufProvider;

impl ModelProvider for GgufProvider {
    fn id(&self) -> &str {
        "gguf"
    }

    fn load(&self, model_path: &str) -> Result<ModelHandle, ProviderError> {
        use hacedle::x::loader::gguf::GgufLoader;
        
        let loader = GgufLoader::load(model_path)
            .map_err(|e| ProviderError::LoadFailed(e))?;
        
        Ok(ModelHandle {
            model_id: model_path.to_string(),
            tensor_count: loader.tensor_count(),
            vocab_size: 151936,  // Default Qwen vocab size
            context_length: 32768,
            hidden_size: 896,    // Default Qwen hidden size
            n_layer: 24,
            n_head: 14,
        })
    }

    fn supports(&self, format: &str) -> bool {
        format == "gguf" || format == "ggml"
    }
}

/// ONNX Provider implementation
pub struct OnnxProvider;

impl ModelProvider for OnnxProvider {
    fn id(&self) -> &str {
        "onnx"
    }

    fn load(&self, _model_path: &str) -> Result<ModelHandle, ProviderError> {
        Err(ProviderError::Unsupported("ONNX provider not yet implemented".to_string()))
    }
}

/// Provider factory
pub struct ProviderFactory;

impl ProviderFactory {
    pub fn create(id: &str) -> Option<&'static dyn ModelProvider> {
        match id {
            "gguf" => Some(&GgufProvider),
            "onnx" => Some(&OnnxProvider),
            _ => None,
        }
    }

    pub fn list() -> Vec<&'static str> {
        vec!["gguf", "onnx"]
    }
}