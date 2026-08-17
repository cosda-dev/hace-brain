// Model Registry - manages GGUF models (P3)

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::provider::gguf::loader::{BrainGgufLoader, ModelVerification};

/// Model registry for GGUF models
pub struct ModelRegistry {
    models: BTreeMap<String, ModelInfo>,
}

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub path: String,
    pub verification: ModelVerification,
    pub is_loaded: bool,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self {
            models: BTreeMap::new(),
        }
    }
    
    /// Register a GGUF model
    pub fn register_model(&mut self, name: &str, path: &str) -> Result<(), &'static str> {
        // Verify the model first
        let loader = BrainGgufLoader::new();
        let verification = loader.verify(path)?;
        
        let model_info = ModelInfo {
            name: name.to_string(),
            path: path.to_string(),
            verification,
            is_loaded: false,
        };
        
        self.models.insert(name.to_string(), model_info);
        Ok(())
    }
    
    /// Get model info by name
    pub fn get_model(&self, name: &str) -> Option<&ModelInfo> {
        self.models.get(name)
    }
    
    /// Get mutable model info by name
    pub fn get_model_mut(&mut self, name: &str) -> Option<&mut ModelInfo> {
        self.models.get_mut(name)
    }
    
    /// List all registered models
    pub fn list_models(&self) -> Vec<String> {
        self.models.keys().cloned().collect()
    }
    
    /// Mark model as loaded
    pub fn mark_loaded(&mut self, name: &str) -> Result<(), &'static str> {
        if let Some(model) = self.models.get_mut(name) {
            model.is_loaded = true;
            Ok(())
        } else {
            Err("model_not_found")
        }
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}