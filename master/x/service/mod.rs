mod memory_service;
mod telemetry_service;
mod audit_service;

pub use memory_service::MemoryService;
pub use telemetry_service::TelemetryService;
pub use audit_service::AuditService;

pub trait BrainService {
    fn start(&self);
    fn stop(&self);
}

use alloc::vec::Vec;
use alloc::string::String;

pub struct MemoryService;

impl BrainService for MemoryService {
    fn start(&self) {}
    fn stop(&self) {}
}

pub struct TelemetryService;

impl BrainService for TelemetryService {
    fn start(&self) {}
    fn stop(&self) {}
}

pub struct AuditService;

impl BrainService for AuditService {
    fn start(&self) {}
    fn stop(&self) {}
}