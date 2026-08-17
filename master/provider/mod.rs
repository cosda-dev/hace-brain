mod descriptor;
mod registry;
mod factory;
mod selector;

pub mod candle;
pub mod llama;
pub mod hacetral;

pub use descriptor::{ProviderDescriptor, ProviderType, Capability};
pub use registry::ProviderRegistry;
pub use factory::ProviderFactory;
pub use selector::ProviderSelector;

pub trait BrainProvider {
    fn descriptor(&self) -> &ProviderDescriptor;
    fn initialize(&mut self) -> Result<(), ProviderError>;
    fn execute(&self, _sio: &super::context::RuntimeSio) -> Result<super::outcome::SioOutcome, ProviderError>;
}

#[derive(Debug, Clone)]
pub struct ProviderError(pub &'static str);

impl From<&'static str> for ProviderError {
    fn from(e: &'static str) -> Self {
        ProviderError(e)
    }
}