// KV Cache - Attention cache for inference
// G6: cache key/value pairs during inference

use alloc::vec::Vec;

/// KV Cache entry
#[derive(Debug, Clone)]
pub struct KvCacheEntry {
    pub key: Vec<f32>,
    pub value: Vec<f32>,
}

/// KV Cache for attention
pub struct KvCache {
    entries: Vec<KvCacheEntry>,
    max_entries: usize,
}

impl KvCache {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }
    
    /// Add entry
    pub fn add(&mut self, key: Vec<f32>, value: Vec<f32>) {
        if self.entries.len() < self.max_entries {
            self.entries.push(KvCacheEntry { key, value });
        }
    }
    
    /// Get entries
    pub fn get(&self) -> &[KvCacheEntry] {
        &self.entries
    }
    
    /// Entry count
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl Default for KvCache {
    fn default() -> Self {
        Self::new(4096) // Default context length
    }
}