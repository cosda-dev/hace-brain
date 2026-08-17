mod hacedle_bridge;
mod hacetral_bridge;

pub use hacedle_bridge::HacedleBridge;
pub use hacetral_bridge::HacetralBridge;

use alloc::string::String;
use alloc::vec::Vec;

pub trait RuntimeBridge {
    fn invoke(&self, _req: RuntimeRequest) -> RuntimeResponse;
}

pub struct RuntimeRequest {
    pub action: String,
}

pub struct RuntimeResponse {
    pub status: String,
    pub result: Vec<u8>,
}

pub struct HacedleBridge;

impl RuntimeBridge for HacedleBridge {
    fn invoke(&self, _req: RuntimeRequest) -> RuntimeResponse {
        RuntimeResponse { status: String::from("ok"), result: Vec::new() }
    }
}

pub struct HacetralBridge;

impl RuntimeBridge for HacetralBridge {
    fn invoke(&self, _req: RuntimeRequest) -> RuntimeResponse {
        RuntimeResponse { status: String::from("ok"), result: Vec::new() }
    }
}