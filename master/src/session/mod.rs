// Session Module

pub mod context;

use alloc::string::String;

// Re-export Turn types from context module
pub use crate::context::{Turn, TurnHistory};

/// Session ID type
pub type SessionId = String;

/// Soul ID type
pub type SoulId = String;

/// Profile ID type
pub type ProfileId = String;

/// Session state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
}

impl Default for SessionState {
    fn default() -> Self {
        SessionState::Pending
    }
}