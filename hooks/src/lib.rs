//! hace-brain-hooks — Brain hook-point registry
//!
//! E4: All hooks are pass-through (zero overhead).
//! E5: Activate governance by enabling Cargo features:
//!   - e5_acm_check  → before_route, before_execute gate via ACM
//!   - e5_evidence   → after_execute packs EvidenceBundle
//!
//! Hook IDs (canonical, never rename):
//!   hok://brain/before_route
//!   hok://brain/after_route
//!   hok://brain/before_execute
//!   hok://brain/after_execute
//!   hok://brain/before_response
//!   hok://brain/after_response

// ── Hook ID constants ─────────────────────────────────────────────────────────

pub const HOK_BRAIN_BEFORE_ROUTE:    &str = "hok://brain/before_route";
pub const HOK_BRAIN_AFTER_ROUTE:     &str = "hok://brain/after_route";
pub const HOK_BRAIN_BEFORE_EXECUTE:  &str = "hok://brain/before_execute";
pub const HOK_BRAIN_AFTER_EXECUTE:   &str = "hok://brain/after_execute";
pub const HOK_BRAIN_BEFORE_RESPONSE: &str = "hok://brain/before_response";
pub const HOK_BRAIN_AFTER_RESPONSE:  &str = "hok://brain/after_response";

pub const ALL_BRAIN_HOOKS: &[&str] = &[
    HOK_BRAIN_BEFORE_ROUTE,
    HOK_BRAIN_AFTER_ROUTE,
    HOK_BRAIN_BEFORE_EXECUTE,
    HOK_BRAIN_AFTER_EXECUTE,
    HOK_BRAIN_BEFORE_RESPONSE,
    HOK_BRAIN_AFTER_RESPONSE,
];

// ── Hook result ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HookOutcome {
    Continue,              // E4 default — pass-through
    Deny { reason: &'static str }, // E5 — ACM gate denied
}

// ── Hook interface ────────────────────────────────────────────────────────────

pub trait BrainHook: Send + Sync {
    fn hook_id(&self) -> &'static str;
    fn call(&self, ctx: &HookCtx) -> HookOutcome;
}

#[derive(Debug, Clone)]
pub struct HookCtx {
    pub hook_id:    &'static str,
    pub soul_id:    Option<String>,
    pub brain_id:   Option<String>,
    pub ce_id:      Option<String>,
    pub action:     String,
}

// ── PassThroughHook — E4 default for all hooks ───────────────────────────────

pub struct PassThroughHook {
    pub id: &'static str,
}

impl BrainHook for PassThroughHook {
    fn hook_id(&self) -> &'static str { self.id }
    fn call(&self, _ctx: &HookCtx) -> HookOutcome { HookOutcome::Continue }
}

// ── BrainHookRegistry ────────────────────────────────────────────────────────

pub struct BrainHookRegistry {
    hooks: Vec<Box<dyn BrainHook>>,
}

impl BrainHookRegistry {
    /// E4 default: register pass-through for all hook-points
    pub fn e4_defaults() -> Self {
        let mut reg = Self { hooks: Vec::new() };
        for &id in ALL_BRAIN_HOOKS {
            reg.hooks.push(Box::new(PassThroughHook { id }));
        }
        reg
    }

    pub fn register(&mut self, hook: Box<dyn BrainHook>) {
        self.hooks.push(hook);
    }

    /// Run hook — returns first non-Continue outcome, else Continue
    pub fn run(&self, hook_id: &str, ctx: &HookCtx) -> HookOutcome {
        for h in &self.hooks {
            if h.hook_id() == hook_id {
                let out = h.call(ctx);
                if out != HookOutcome::Continue { return out; }
            }
        }
        HookOutcome::Continue
    }

    pub fn list_registered(&self) -> Vec<&str> {
        self.hooks.iter().map(|h| h.hook_id()).collect()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e4_defaults_all_pass_through() {
        let reg = BrainHookRegistry::e4_defaults();
        let ctx = HookCtx {
            hook_id:  HOK_BRAIN_BEFORE_EXECUTE,
            soul_id:  Some("soul://coder".into()),
            brain_id: Some("ce.hacedle".into()),
            ce_id:    None,
            action:   "reason".into(),
        };
        for &id in ALL_BRAIN_HOOKS {
            assert_eq!(
                reg.run(id, &ctx),
                HookOutcome::Continue,
                "hook {id} should pass-through in E4"
            );
        }
    }

    #[test]
    fn all_hook_ids_registered_by_default() {
        let reg = BrainHookRegistry::e4_defaults();
        let registered = reg.list_registered();
        for &id in ALL_BRAIN_HOOKS {
            assert!(registered.contains(&id), "hook {id} not registered");
        }
    }
}
