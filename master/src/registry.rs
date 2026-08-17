// Provider Registry - manages providers
use alloc::collections::BTreeMap;
use alloc::string::String;

use super::provider::BrainProvider;

pub struct ProviderRegistry {
    providers: BTreeMap<String, &'static dyn BrainProvider>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, provider: &'static dyn BrainProvider) {
        self.providers.insert(name.to_string(), provider);
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}