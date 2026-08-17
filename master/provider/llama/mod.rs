use alloc::string::String;
use alloc::vec::Vec;

use super::super::{ProviderDescriptor, ProviderType, Capability, BrainProvider, ProviderError};
use super::super::super::context::RuntimeSio;
use super::super::super::outcome::SioOutcome;

pub struct LlamaProvider {
    descriptor: ProviderDescriptor,
}

impl LlamaProvider {
    pub fn new() -> Self {
        Self {
            descriptor: ProviderDescriptor {
                id: String::from("llama"),
                provider_type: ProviderType::Llama,
                capabilities: vec![Capability::Inference],
                priority: 80,
                enabled: true,
            },
        }
    }
}

impl Default for LlamaProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl BrainProvider for LlamaProvider {
    fn descriptor(&self) -> &ProviderDescriptor {
        &self.descriptor
    }

    fn initialize(&mut self) -> Result<(), ProviderError> {
        Ok(())
    }

    fn execute(&self, _sio: &RuntimeSio) -> Result<SioOutcome, ProviderError> {
        Ok(SioOutcome::default())
    }
}