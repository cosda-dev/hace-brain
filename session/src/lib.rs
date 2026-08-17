// Brain Session Module - Session Time Machine

pub mod replay_record;

pub use replay_record::ReplayRecord;

use alloc::string::String;
use alloc::vec::Vec;

/// Brain Session - manages session history for replay
pub struct BrainSession {
    pub session_id: String,
    pub history: Vec<ReplayRecord>,
}

impl BrainSession {
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            history: Vec::new(),
        }
    }

    pub fn add_turn(&mut self, input: &str, output: &str) {
        let turn_id = self.history.len() as u64;
        self.history.push(ReplayRecord::new(turn_id, input, output));
    }

    pub fn get_history(&self) -> &[ReplayRecord] {
        &self.history
    }
}