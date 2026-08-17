
// Brain Operator Registry
// Alpha-3 Assembly: Delegates to hacedle/hacetral implementations

use std::collections::HashMap;
use std::sync::Arc;

pub trait Operator: Send + Sync {
    fn name(&self) -> &str;
    fn execute(&self, input: &[f32], output: &mut [f32]) -> Result<(), String>;
}

pub struct OperatorRegistry {
    operators: HashMap<String, Arc<dyn Operator>>,
}

impl OperatorRegistry {
    pub fn new() -> Self {
        Self { operators: HashMap::new() }
    }
    
    pub fn register(&mut self, name: impl Into<String>, op: Arc<dyn Operator>) {
        self.operators.insert(name.into(), op);
    }
    
    pub fn get(&self, name: &str) -> Option<Arc<dyn Operator>> {
        self.operators.get(name).cloned()
    }
}

