use super::{BrainPlugin, PluginResult};

pub struct SkillAnalyst;

impl SkillAnalyst {
    pub fn new() -> Self { Self }
}

impl BrainPlugin for SkillAnalyst {
    fn execute(&self, _ctx: &super::super::super::context::RuntimeSio) -> PluginResult {
        PluginResult::default()
    }
}