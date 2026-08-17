mod soul_router;
mod provider_router;
mod skill_router;

pub use soul_router::SoulRouter;
pub use provider_router::ProviderRouter;
pub use skill_router::SkillRouter;

use alloc::string::String;

pub enum RouteDecision {
    Soul(String),
    Provider(String),
    Skill(String),
}

pub trait BrainRouter {
    fn route(&self, _sio_id: &str) -> RouteDecision;
}

pub struct SoulRouter;

impl BrainRouter for SoulRouter {
    fn route(&self, _sio_id: &str) -> RouteDecision {
        RouteDecision::Soul(String::new())
    }
}

pub struct ProviderRouter;

impl BrainRouter for ProviderRouter {
    fn route(&self, _sio_id: &str) -> RouteDecision {
        RouteDecision::Provider(String::from("candle"))
    }
}

pub struct SkillRouter;

impl BrainRouter for SkillRouter {
    fn route(&self, _sio_id: &str) -> RouteDecision {
        RouteDecision::Skill(String::new())
    }
}