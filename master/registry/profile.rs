use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileRegistry {
    profiles: Vec<ProfileEntry>,
}

impl ProfileRegistry {
    pub fn new() -> Self {
        Self { profiles: Vec::new() }
    }

    pub fn get(&self, profile_id: &str) -> Option<&ProfileEntry> {
        self.profiles.iter().find(|p| p.id == profile_id)
    }
}

#[derive(Debug, Clone)]
pub struct ProfileEntry {
    pub id: String,
    pub ruleset: String,
}

impl Default for ProfileRegistry {
    fn default() -> Self {
        Self::new()
    }
}