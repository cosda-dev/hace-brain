// Runtime Dispatcher - dispatches to providers

use async_trait::async_trait;

use super::outcome::SioOutcome;
use super::context::RuntimeSio;
use super::provider::{BrainProvider, ProviderError, ProviderFactory};

/// Runtime Dispatcher - dispatches to providers
pub struct RuntimeDispatcher {
    provider: Box<dyn BrainProvider>,
}

impl RuntimeDispatcher {
    pub fn new() -> Self {
        Self {
            provider: ProviderFactory::create("mock"),
        }
    }

    pub fn dispatch(&self, sio: &RuntimeSio) -> Result<SioOutcome, ProviderError> {
        self.provider.execute(sio)
    }
}

impl Default for RuntimeDispatcher {
    fn default() -> Self {
        Self::new()
    }
}