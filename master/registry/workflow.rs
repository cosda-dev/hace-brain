use alloc::string::String;
use alloc::vec::Vec;

pub struct WorkflowRegistry {
    workflows: Vec<WorkflowEntry>,
}

impl WorkflowRegistry {
    pub fn new() -> Self {
        Self { workflows: Vec::new() }
    }
}

#[derive(Debug, Clone)]
pub struct WorkflowEntry {
    pub id: String,
    pub steps: Vec<String>,
}

impl Default for WorkflowRegistry {
    fn default() -> Self {
        Self::new()
    }
}