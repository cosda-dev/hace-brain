mod skill_planner;
mod skill_coder;
mod skill_analyst;
mod skill_architect;

pub use skill_planner::SkillPlanner;
pub use skill_coder::SkillCoder;
pub use skill_analyst::SkillAnalyst;
pub use skill_architect::SkillArchitect;

use alloc::vec::Vec;
use alloc::string::String;

pub trait BrainPlugin {
    fn execute(&self, _ctx: &super::super::context::RuntimeSio) -> PluginResult;
}

pub struct PluginResult {
    pub success: bool,
    pub output: Vec<u8>,
}

impl Default for PluginResult {
    fn default() -> Self {
        Self { success: true, output: Vec::new() }
    }
}