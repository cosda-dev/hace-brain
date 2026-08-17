use alloc::string::String;
use alloc::vec::Vec;

/// SIO Outcome - result from Brain execution
#[derive(Debug, Clone)]
pub enum SioOutcome {
    Success {
        text: String,
        tokens: Vec<u32>,
    },
    Error {
        message: String,
        details: String,
    },
}

#[derive(Debug, Clone, Default)]
pub struct Telemetry {
    pub latency_ms: u64,
    pub tokens_used: u32,
    pub provider: String,
}