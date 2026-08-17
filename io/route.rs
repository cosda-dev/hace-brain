// Route Resolver - routes SIO to correct layer
use super::{BrainRoute, SioIntent, RouteResolver};

/// Default route resolver
pub struct BrainRouteResolver;

impl BrainRouteResolver {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for BrainRouteResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl RouteResolver for BrainRouteResolver {
    fn resolve(&self, intent: &SioIntent) -> BrainRoute {
        match intent.action.as_str() {
            "infer" | "reason" | "chat" => BrainRoute::Local,
            "orchestrate" => BrainRoute::Soul,
            "execute" => BrainRoute::Coge,
            "run" => BrainRoute::Runtime,
            _ => BrainRoute::External,
        }
    }
}