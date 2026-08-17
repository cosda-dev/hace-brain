use alloc::string::String;
use alloc::vec::Vec;

pub struct PolicyRegistry {
    policies: Vec<PolicyEntry>,
}

impl PolicyRegistry {
    pub fn new() -> Self {
        Self { policies: Vec::new() }
    }

    pub fn get(&self, policy_id: &str) -> Option<&PolicyEntry> {
        self.policies.iter().find(|p| p.id == policy_id)
    }
}

#[derive(Debug, Clone)]
pub struct PolicyEntry {
    pub id: String,
    pub ruleset: String,
}

impl Default for PolicyRegistry {
    fn default() -> Self {
        Self::new()
    }
}