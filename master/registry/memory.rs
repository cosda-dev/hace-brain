use alloc::string::String;
use alloc::vec::Vec;

pub struct MemoryRegistry {
    entries: Vec<MemoryEntry>,
}

impl MemoryRegistry {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn store(&mut self, entry: MemoryEntry) {
        self.entries.push(entry);
    }
}

#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub key: String,
    pub value: String,
}

impl Default for MemoryRegistry {
    fn default() -> Self {
        Self::new()
    }
}