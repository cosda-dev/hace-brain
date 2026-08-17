// BrainProvider implementation for Candle - stub
// CRD Directive D2: Wire provider tới BrainKernel

use alloc::string::String;
use alloc::vec::Vec;

use super::{ProviderDescriptor, ProviderType, Capability, BrainProvider, ProviderError};
use crate::context::RuntimeSio;
use crate::outcome::SioOutcome;

pub struct CandleBrain;

impl CandleBrain {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CandleBrain {
    fn default() -> Self {
        Self::new()
    }
}

impl BrainProvider for CandleBrain {
    fn descriptor(&self) -> &ProviderDescriptor {
        // Return static descriptor
        static DESC: ProviderDescriptor = ProviderDescriptor {
            id: alloc::string::String::from("candle-brain"),
            provider_type: ProviderType::Candle,
            capabilities: vec![Capability::Inference],
            priority: 100,
            enabled: true,
        };
        &DESC
    }

    fn initialize(&mut self) -> Result<(), ProviderError> {
        Ok(())
    }

    fn execute(&self, sio: &RuntimeSio) -> Result<SioOutcome, ProviderError> {
        let output = if let Some(prompt) = &sio.prompt {
            alloc::format!("executed: {}", prompt)
        } else {
            alloc::string::String::from("no_prompt")
        };
        
        Ok(SioOutcome {
            status: alloc::string::String::from("executed"),
            result: output.into_bytes(),
            answer: output.clone(),
            confidence: 0.5,
            reasoning_hash: String::new(),
            evidence: Vec::new(),
            recommendations: Vec::new(),
            commands: Vec::new(),
            telemetry: Default::default(),
        })
    }
}