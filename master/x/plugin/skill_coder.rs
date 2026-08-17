use super::{BrainPlugin, PluginResult};

pub struct SkillCoder;

impl SkillCoder {
    pub fn new() -> Self { Self }
}

impl BrainPlugin for SkillCoder {
    fn execute(&self, _ctx: &super::super::super::context::RuntimeSio) -> PluginResult {
        PluginResult::default()
    }
}