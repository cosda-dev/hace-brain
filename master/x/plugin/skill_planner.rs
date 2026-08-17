use super::{BrainPlugin, PluginResult};

pub struct SkillPlanner;

impl SkillPlanner {
    pub fn new() -> Self { Self }
}

impl BrainPlugin for SkillPlanner {
    fn execute(&self, _ctx: &super::super::super::context::RuntimeSio) -> PluginResult {
        PluginResult::default()
    }
}