// Brain execute module - execution entry point
// CRD Directive D4: execute_brain real implementation

use crate::context::RuntimeSio;
use crate::outcome::SioOutcome;
use crate::dispatcher::RuntimeDispatcher;

pub fn execute_brain(sio: &RuntimeSio, dispatcher: &RuntimeDispatcher) -> Result<SioOutcome, &'static str> {
    dispatcher.dispatch(sio).map_err(|_| "dispatch_failed")
}

#[derive(Debug, Clone)]
pub struct Checkpoint {
    pub step: usize,
    pub tokens: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct TelemetryData {
    pub latency_ms: u64,
    pub tokens_generated: u32,
}