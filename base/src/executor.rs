//! Executor for Brain Base Layer
//!
//! This module provides abstract execution capabilities.

use crate::planner::Plan;
use serde::{Deserialize, Serialize};

/// Result of an execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub plan_id: String,
    pub success: bool,
    pub output: String,
    pub duration_ms: u64,
}

/// Trait for execution backends
pub trait Executor {
    fn execute(&self, plan: &Plan) -> anyhow::Result<ExecutionResult>;
}

/// A simple local executor for demonstration
pub struct LocalExecutor;

impl Executor for LocalExecutor {
    fn execute(&self, plan: &Plan) -> anyhow::Result<ExecutionResult> {
        // Simulate execution
        let start = std::time::Instant::now();
        
        // Very simplified execution for demonstration
        let output = format!("Executed plan {} with {} steps", plan.id, plan.steps.len());
        
        let duration = start.elapsed();
        
        Ok(ExecutionResult {
            plan_id: plan.id.clone(),
            success: true,
            output,
            duration_ms: duration.as_millis() as u64,
        })
    }
}

/// Executes a plan using the provided executor
pub fn execute<E: Executor>(executor: &E, plan: &Plan) -> anyhow::Result<ExecutionResult> {
    executor.execute(plan)
}