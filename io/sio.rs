// SIO - Standardized Intent Object
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

use super::{BrainRoute, SioIntent};

/// SIO - Standardized Intent Object
pub struct Sio {
    pub intent: SioIntent,
    pub context: Context,
    pub payload: Payload,
    pub route: BrainRoute,
    pub metadata: Metadata,
}

impl Sio {
    pub fn new(intent: SioIntent) -> Self {
        Self {
            intent,
            context: Context::default(),
            payload: Payload::default(),
            route: BrainRoute::Local,
            metadata: Metadata::default(),
        }
    }
}

/// Context
#[derive(Default)]
pub struct Context {
    pub hid: String,      // Human ID
    pub sid: String,      // Session ID
    pub pid: String,      // Profile ID
    pub ttl: u64,         // Time to live
}

/// Payload
pub enum Payload {
    Text(String),
    Json(BTreeMap<String, String>),
    Binary(Vec<u8>),
}

impl Default for Payload {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

/// Metadata
#[derive(Default)]
pub struct Metadata {
    pub timestamp: u64,
    pub source: String,
    pub priority: Priority,
}

/// Priority
#[derive(Debug, Clone, Copy, Default)]
pub enum Priority {
    Low,
    #[default]
    Medium,
    High,
    Critical,
}