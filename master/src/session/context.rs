// Session Context - manages inference sessions

use crate::context::{Turn, TurnHistory};
use crate::runtime::kv_cache::KvCache;
use alloc::string::String;
use alloc::vec::Vec;

/// Session context for maintaining state
pub struct SessionContext {
    pub session_id: String,
    pub model_id: String,
    pub kv_cache: KvCache,
    pub turn_history: TurnHistory,
}

impl SessionContext {
    pub fn new(model_id: &str) -> Self {
        Self {
            session_id: String::new(),
            model_id: model_id.to_string(),
            kv_cache: KvCache::default(),
            turn_history: TurnHistory::new(),
        }
    }
    
    pub fn add_turn(&mut self, prompt: String, response: String) {
        self.turn_history.add(prompt, response);
    }
}

impl Default for SessionContext {
    fn default() -> Self {
        Self::new("default")
    }
}