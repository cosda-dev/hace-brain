use alloc::string::String;
use alloc::vec::Vec;

pub struct SkillRegistry {
    skills: Vec<SkillEntry>,
}

impl SkillRegistry {
    pub fn new() -> Self {
        Self { skills: Vec::new() }
    }

    pub fn register(&mut self, entry: SkillEntry) {
        self.skills.push(entry);
    }
}

#[derive(Debug, Clone)]
pub struct SkillEntry {
    pub id: String,
    pub capability: String,
}

impl Default for SkillRegistry {
    fn default() -> Self {
        Self::new()
    }
}