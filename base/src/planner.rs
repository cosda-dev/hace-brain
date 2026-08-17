//! Planner for Brain Base Layer
//!
//! This module handles converting intents into executable plans.

use crate::intent::Intent;
use serde::{Deserialize, Serialize};

/// Represents an execution plan derived from an intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: String,
    pub intent_id: String,
    pub steps: Vec<PlanStep>,
    pub required_resources: Vec<String>,
}

/// A step in an execution plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    pub id: String,
    pub description: String,
    pub action: String,
    pub parameters: std::collections::HashMap<String, String>,
}

impl Plan {
    /// Creates a new Plan
    pub fn new(intent_id: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            intent_id,
            steps: Vec::new(),
            required_resources: Vec::new(),
        }
    }

    /// Adds a step to the plan
    pub fn add_step(&mut self, description: String, action: String, parameters: std::collections::HashMap<String, String>) {
        let step = PlanStep {
            id: uuid::Uuid::new_v4().to_string(),
            description,
            action,
            parameters,
        };
        self.steps.push(step);
    }

    /// Adds a required resource to the plan
    pub fn add_required_resource(&mut self, resource: String) {
        self.required_resources.push(resource);
    }
}

/// Builds an execution plan from an intent
pub fn build_plan(intent: &Intent) -> anyhow::Result<Plan> {
    // Very simplified plan building for demonstration
    // In a real implementation, this would be much more complex
    
    let mut plan = Plan::new(intent.id.clone());
    
    // Add a simple step based on the intent action
    let mut params = std::collections::HashMap::new();
    params.insert("target".to_string(), intent.target.clone());
    
    plan.add_step(
        format!("Execute {} on {}", intent.action, intent.target),
        intent.action.clone(),
        params,
    );
    
    Ok(plan)
}