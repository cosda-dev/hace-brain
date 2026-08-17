// Dynamic Brain Capacity Orchestration Layer (P8)
// Implements adaptive compute allocation based on workload and available resources

use crate::context::ScalingAction;
use crate::replay_context::ReplayContext;

// Re-export DynamicBrain types from context for backward compatibility
pub use crate::context::{DynamicBrain, CapacityProfile, LoadMetrics, ScalingPolicy};

/// Extend RuntimeContext to include dynamic brain
pub trait DynamicBrainExt {
    fn dynamic_brain(&mut self) -> &mut DynamicBrain;
    fn dynamic_brain_ref(&self) -> &DynamicBrain;
}