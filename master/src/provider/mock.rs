// Mock Provider - for testing end-to-end chain

use alloc::string::String;
use alloc::vec::Vec;

use super::{BrainProvider, ProviderError};
use crate::context::RuntimeSio;
use crate::outcome::SioOutcome;

/// Mock Provider for chain validation
pub struct MockProvider {
    name: String,
}

impl MockProvider {
    pub fn new() -> Self {
        Self {
            name: "mock".to_string(),
        }
    }
}

impl Default for MockProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl BrainProvider for MockProvider {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute(&self, sio: &RuntimeSio) -> Result<SioOutcome, ProviderError> {
        // Mock: echo prompt as tokens (as u32)
        let tokens: Vec<u32> = sio.prompt
            .as_ref()
            .map(|p| p.bytes().map(|b| b as u32).collect())
            .unwrap_or_default();

        Ok(SioOutcome::Success {
            text: sio.prompt.clone().unwrap_or_default(),
            tokens,
        })
    }
}