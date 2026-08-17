use alloc::string::String;
use alloc::vec::Vec;

use super::descriptor::{ProviderDescriptor, ProviderType, Capability};
use super::BrainProvider;

pub struct ProviderConfig {
    pub id: String,
    pub provider_type: ProviderType,
    pub capabilities: Vec<Capability>,
}

pub struct ProviderFactory;

impl ProviderFactory {
    pub fn create(_cfg: &ProviderConfig) -> Option<alloc::sync::Arc<dyn BrainProvider>> {
        Some(alloc::sync::Arc::new(DummyProvider::default()))
    }
}

struct DummyProvider {
    descriptor: ProviderDescriptor,
}

impl Default for DummyProvider {
    fn default() -> Self {
        Self {
            descriptor: ProviderDescriptor::default(),
        }
    }
}

impl BrainProvider for DummyProvider {
    fn descriptor(&self) -> &ProviderDescriptor {
        &self.descriptor
    }

    fn initialize(&mut self) -> Result<(), super::ProviderError> {
        Ok(())
    }

    fn execute(&self, _sio: &super::super::context::RuntimeSio) -> Result<super::super::outcome::SioOutcome, super::ProviderError> {
        Ok(super::super::outcome::SioOutcome::default())
    }
}