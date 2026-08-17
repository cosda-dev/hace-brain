// Brain Master Runtime - Orchestrator for Multi-Soul, Multi-Provider

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod session;
pub mod dispatcher;
pub mod outcome;
pub mod registry;
pub mod context;
pub mod provider;
pub mod kernel;
pub mod inference;
pub mod tokenizer;
pub mod runtime;
pub mod model_registry;

pub use dispatcher::RuntimeDispatcher;
pub use outcome::SioOutcome;
pub use context::{RuntimeSio, RuntimeContext, InferenceContext};
pub use kernel::{BrainKernel, BrainRuntime};
pub use model_registry::{ModelRegistry, ModelInfo};
pub use inference::BrainInferenceEngine;
pub use tokenizer::BrainTokenizerImpl;

/// Brain Master Runtime - main entry point
pub struct BrainMasterRuntime {
    dispatcher: RuntimeDispatcher,
}

impl BrainMasterRuntime {
    pub fn new() -> Self {
        Self {
            dispatcher: RuntimeDispatcher::new(),
        }
    }

    pub fn execute(&self, sio: &RuntimeSio) -> Result<SioOutcome, &'static str> {
        self.dispatcher.dispatch(sio).map_err(|_| "dispatch_failed")
    }
}

impl Default for BrainMasterRuntime {
    fn default() -> Self {
        Self::new()
    }
}