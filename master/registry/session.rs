use alloc::string::String;
use alloc::vec::Vec;

pub struct SessionRegistry {
    sessions: Vec<SessionEntry>,
}

impl SessionRegistry {
    pub fn new() -> Self {
        Self { sessions: Vec::new() }
    }

    pub fn add(&mut self, entry: SessionEntry) {
        self.sessions.push(entry);
    }
}

#[derive(Debug, Clone)]
pub struct SessionEntry {
    pub id: String,
    pub state: String,
}

impl Default for SessionRegistry {
    fn default() -> Self {
        Self::new()
    }
}