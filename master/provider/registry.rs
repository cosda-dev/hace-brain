use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

use super::descriptor::{ProviderDescriptor, Capability};
use super::BrainProvider;

pub struct ProviderRegistry {
    providers: BTreeMap<String, ProviderEntry>,
}

struct ProviderEntry {
    descriptor: ProviderDescriptor,
    enabled: bool,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, provider: &dyn BrainProvider) {
        let desc = provider.descriptor();
        self.providers.insert(
            desc.id.clone(),
            ProviderEntry {
                descriptor: desc.clone(),
                enabled: true,
            }
        );
    }

    pub fn get(&self, id: &str) -> Option<&ProviderDescriptor> {
        self.providers.get(id).map(|e| &e.descriptor)
    }

    pub fn list(&self) -> Vec<ProviderDescriptor> {
        self.providers.values().map(|e| e.descriptor.clone()).collect()
    }

    pub fn select(&self, capability: Capability) -> Option<String> {
        self.providers
            .values()
            .filter(|e| e.enabled && e.descriptor.capabilities.contains(&capability))
            .max_by_key(|e| e.descriptor.priority)
            .map(|e| e.descriptor.id.clone())
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}