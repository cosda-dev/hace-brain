// brain/cli/src/lib.rs
// Public API cua hace-brain-cli crate.
// KHONG re-export BrainMasterRuntime truc tiep â€” CLI chi goi brain.rs dispatcher.

pub mod command;
pub mod brain;
pub mod prompt;
pub mod replay;
pub mod model;
pub mod executor;
pub mod profile;
pub mod benchmark;

pub use brain::{BrainCmd, BrainCliError};
