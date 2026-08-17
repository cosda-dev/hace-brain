// Replay Record - Session Time Machine artifact

use alloc::string::String;

/// Replay Record - stores turn input/output
#[derive(Debug, Clone)]
pub struct ReplayRecord {
    pub turn_id: u64,
    pub timestamp: u64,
    pub input: String,
    pub output: String,
}

impl ReplayRecord {
    pub fn new(turn_id: u64, input: &str, output: &str) -> Self {
        Self {
            turn_id,
            timestamp: 0, // Will use std::time when available
            input: input.to_string(),
            output: output.to_string(),
        }
    }
}