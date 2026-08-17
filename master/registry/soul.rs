use alloc::string::String;
use alloc::vec::Vec;

pub struct SoulRegistry {
    souls: Vec<SoulEntry>,
}

impl SoulRegistry {
    pub fn new() -> Self {
        Self { souls: Vec::new() }
    }

    pub fn register(&mut self, entry: SoulEntry) {
        self.souls.push(entry);
    }
}

#[derive(Debug, Clone)]
pub struct SoulEntry {
    pub id: String,
    pub profile: String,
}

impl Default for SoulRegistry {
    fn default() -> Self {
        Self::new()
    }
}