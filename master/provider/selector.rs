use alloc::string::String;

use crate::context::RuntimeSio;
use super::descriptor::{ProviderType, Capability};

pub struct ProviderSelector;

impl ProviderSelector {
    pub fn select(sio: &RuntimeSio) -> ProviderType {
        let runtime = &sio.runtime;
        match runtime.provider.as_str() {
            "candle" => ProviderType::Candle,
            "llama" => ProviderType::Llama,
            "onnx" => ProviderType::Onnx,
            "mlx" => ProviderType::Mlx,
            "vllm" => ProviderType::Vllm,
            "hacetral" => ProviderType::Hacetral,
            _ => ProviderType::Candle,
        }
    }

    pub fn select_by_capability(capability: &str) -> Capability {
        match capability {
            "inference" => Capability::Inference,
            "embedding" => Capability::Embedding,
            "workflow" => Capability::Workflow,
            "orchestration" => Capability::Orchestration,
            _ => Capability::Inference,
        }
    }
}