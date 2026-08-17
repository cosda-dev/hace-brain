// Brain CLI Router - Routes brain commands to executors
// Extends RR CLI pattern

use super::command::BrainAction;
use crate::executor::{BrainExecutor, ModelExecutor, LoraExecutor, ReplayExecutor};

pub struct BrainRouter;

impl BrainRouter {
    pub fn route(action: BrainAction) -> &'static str {
        match action {
            BrainAction::Run => "run",
            BrainAction::Chat => "chat",
            BrainAction::Model => "model",
            BrainAction::Lora => "lora",
            BrainAction::Kv => "kv",
            BrainAction::Replay => "replay",
            BrainAction::Inspect => "inspect",
            BrainAction::Bench => "bench",
            BrainAction::Profile => "profile",
        }
    }
}