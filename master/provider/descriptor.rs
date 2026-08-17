use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderType {
    Candle,
    Llama,
    Onnx,
    Mlx,
    Vllm,
    Hacetral,
    External,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability {
    Inference,
    Embedding,
    Vision,
    Audio,
    Workflow,
    Scheduling,
    Routing,
    Orchestration,
}

impl Capability {
    pub fn as_str(&self) -> &'static str {
        match self {
            Capability::Inference => "inference",
            Capability::Embedding => "embedding",
            Capability::Vision => "vision",
            Capability::Audio => "audio",
            Capability::Workflow => "workflow",
            Capability::Scheduling => "scheduling",
            Capability::Routing => "routing",
            Capability::Orchestration => "orchestration",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProviderDescriptor {
    pub id: String,
    pub provider_type: ProviderType,
    pub capabilities: Vec<Capability>,
    pub priority: u32,
    pub enabled: bool,
}

impl ProviderDescriptor {
    pub fn new(id: &str, provider_type: ProviderType) -> Self {
        Self {
            id: id.to_string(),
            provider_type,
            capabilities: Vec::new(),
            priority: 100,
            enabled: true,
        }
    }
}

impl Default for ProviderDescriptor {
    fn default() -> Self {
        Self::new("default", ProviderType::Candle)
    }
}