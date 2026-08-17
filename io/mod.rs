// Brain I/O - Bridge to HACE I/O layers
pub mod rac;
pub mod raci;
pub mod racin;
pub mod racex;
pub mod route;
pub mod sio;

use alloc::string::String;
use alloc::vec::Vec;

/// Brain Route - determines execution path
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrainRoute {
    Local,
    Soul,
    Coge,
    Runtime,
    External,
}

/// Route Resolver - resolves intent to route
pub trait RouteResolver {
    fn resolve(&self, intent: &SioIntent) -> BrainRoute;
}

/// SIO Intent (minimal for bridge)
pub struct SioIntent {
    pub action: String,
    pub target: String,
    pub confidence: f32,
}

impl SioIntent {
    pub fn new(action: &str, target: &str) -> Self {
        Self {
            action: action.to_string(),
            target: target.to_string(),
            confidence: 0.5,
        }
    }
}