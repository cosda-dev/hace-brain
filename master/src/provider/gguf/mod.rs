// GGUF Provider Module - delegates to hacedle loader
// Per CSA audit: MUST reuse hacedle::x::loader::gguf, not reimplement
// See: brain/canon/hookpoints/gguf-loader.ail, brain/canon/asi/gguf-loader.ail

pub mod loader;

// Re-exports from hacedle (mandatory reuse per RULE_04)
// These types are OWNED by hacedle, brain/master is STEWARD only
pub use loader::{BrainGgufLoader, ModelVerification};