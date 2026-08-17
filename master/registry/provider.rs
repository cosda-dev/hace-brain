use alloc::string::String;
use alloc::vec::Vec;

pub struct ProviderRegistry {
    providers: Vec<ProviderEntry>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self { providers: Vec::new() }
    }

    pub fn register(&mut self, entry: ProviderEntry) {
        self.providers.push(entry);
    }

    pub fn select(&self, capability: &str) -> Option<&ProviderEntry> {
        self.providers.iter().find(|p| p.capabilities.contains(&capability.to_string()))
    }
}

#[derive(Debug, Clone)]
pub struct ProviderEntry {
    pub id: String,
    pub capabilities: Vec<String>,
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}