/// Runtime Context hierarchy for Brain Master
/// Includes: RuntimeContext, SessionContext, PromptContext, InferenceContext, ReplayContext, DynamicBrain

use alloc::string::String;
use alloc::vec::Vec;
use crate::session;

/// Runtime SIO - input to Brain Master
#[derive(Debug, Clone, Default)]
pub struct RuntimeSio {
    pub prompt: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub runtime: RuntimeConfig,
}

/// Runtime configuration
#[derive(Debug, Clone, Default)]
pub struct RuntimeConfig {
    pub provider: String,
    pub model: String,
}

/// Inference context for LLM parameters
#[derive(Debug, Clone, Default)]
pub struct InferenceContext {
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
}

/// Prompt context for working with prompts
#[derive(Debug, Clone, Default)]
pub struct PromptContext {
    pub prompt: String,
    pub tokens: Vec<u32>,
}

/// Runtime Context - main context for inference sessions
#[derive(Debug, Clone)]
pub struct RuntimeContext {
    pub model_id: String,
    pub sio: RuntimeSio,
    pub session_context: session::context::SessionContext,
    pub inference_context: InferenceContext,
    pub prompt_context: PromptContext,
}

impl RuntimeContext {
    pub fn new(model_id: &str) -> Self {
        Self {
            model_id: model_id.to_string(),
            sio: RuntimeSio::default(),
            session_context: session::context::SessionContext::new(model_id),
            inference_context: InferenceContext::default(),
            prompt_context: PromptContext::default(),
        }
    }

    pub fn set_prompt(&mut self, prompt: String) {
        self.sio.prompt = Some(prompt.clone());
        self.prompt_context.prompt = prompt;
    }

    pub fn get_prompt(&self) -> Option<String> {
        self.sio.prompt.clone()
    }
}

impl Default for RuntimeContext {
    fn default() -> Self {
        Self::new("default")
    }
}

/// Single turn
#[derive(Debug, Clone)]
pub struct Turn {
    pub prompt: String,
    pub response: String,
}

/// Turn History
pub struct TurnHistory {
    pub turns: Vec<Turn>,
}

impl TurnHistory {
    pub fn new() -> Self {
        Self { turns: Vec::new() }
    }

    pub fn add(&mut self, prompt: String, response: String) {
        self.turns.push(Turn { prompt, response });
    }

    pub fn get_turns(&self) -> Vec<(String, String)> {
        self.turns.iter().map(|t| (t.prompt.clone(), t.response.clone())).collect()
    }
}

impl Default for TurnHistory {
    fn default() -> Self {
        Self::new()
    }
}

/// Replay context for session replay and SKB injection
#[derive(Debug, Clone)]
pub struct ReplayContext {
    pub session_id: String,
    pub model_id: String,
    pub replay_enabled: bool,
    pub replay_data: ReplayData,
    pub skb_injection: Option<SkbInjection>,
}

#[derive(Debug, Clone)]
pub struct ReplayData {
    pub prompt_history: Vec<String>,
    pub response_history: Vec<String>,
    pub token_history: Vec<Vec<u32>>,
    pub embedding_history: Vec<Vec<f32>>,
    pub logits_history: Vec<Vec<f32>>,
}

#[derive(Debug, Clone)]
pub struct SkbInjection {
    pub manifest: SkbManifest,
    pub semantic_graph: SemanticGraph,
    pub payload_views: Vec<PayloadView>,
}

impl ReplayContext {
    pub fn new(session_id: &str, model_id: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            model_id: model_id.to_string(),
            replay_enabled: false,
            replay_data: ReplayData::new(),
            skb_injection: None,
        }
    }
    
    pub fn enable_replay(&mut self) {
        self.replay_enabled = true;
    }
    
    pub fn disable_replay(&mut self) {
        self.replay_enabled = false;
    }
    
    pub fn record_prompt(&mut self, prompt: &str) {
        if self.replay_enabled {
            self.replay_data.prompt_history.push(prompt.to_string());
        }
    }
    
    pub fn record_response(&mut self, response: &str) {
        if self.replay_enabled {
            self.replay_data.response_history.push(response.to_string());
        }
    }
    
    pub fn record_tokens(&mut self, tokens: &[u32]) {
        if self.replay_enabled {
            self.replay_data.token_history.push(tokens.to_vec());
        }
    }
    
    pub fn record_embeddings(&mut self, embeddings: &[f32]) {
        if self.replay_enabled {
            self.replay_data.embedding_history.push(embeddings.to_vec());
        }
    }
    
    pub fn record_logits(&mut self, logits: &[f32]) {
        if self.replay_enabled {
            self.replay_data.logits_history.push(logits.to_vec());
        }
    }
    
    pub fn inject_skb(&mut self, skb: SkbInjection) {
        self.skb_injection = Some(skb);
    }
    
    pub fn get_replay_summary(&self) -> String {
        format!(
            "Replay: {} prompts, {} responses, {} token sequences",
            self.replay_data.prompt_history.len(),
            self.replay_data.response_history.len(),
            self.replay_data.token_history.len()
        )
    }
}

impl Default for ReplayContext {
    fn default() -> Self {
        Self::new("default-session", "default-model")
    }
}

impl ReplayData {
    pub fn new() -> Self {
        Self {
            prompt_history: Vec::new(),
            response_history: Vec::new(),
            token_history: Vec::new(),
            embedding_history: Vec::new(),
            logits_history: Vec::new(),
        }
    }
    
    pub fn clear(&mut self) {
        self.prompt_history.clear();
        self.response_history.clear();
        self.token_history.clear();
        self.embedding_history.clear();
        self.logits_history.clear();
    }
}

#[derive(Debug, Clone)]
pub struct SkbManifest {
    pub title: String,
    pub category: String,
    pub language: String,
    pub domain: String,
}

#[derive(Debug, Clone)]
pub struct SemanticGraph {
    pub nodes: Vec<String>,
    pub edges: Vec<(String, String, String)>,
}

#[derive(Debug, Clone)]
pub struct PayloadView {
    pub id: String,
    pub data: Vec<u8>,
    pub metadata: Vec<(String, String)>,
}

impl SkbManifest {
    pub fn new(title: &str, category: &str, language: &str, domain: &str) -> Self {
        Self {
            title: title.to_string(),
            category: category.to_string(),
            language: language.to_string(),
            domain: domain.to_string(),
        }
    }
}

impl SemanticGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    
    pub fn add_node(&mut self, node: &str) {
        self.nodes.push(node.to_string());
    }
    
    pub fn add_edge(&mut self, from: &str, relation: &str, to: &str) {
        self.edges.push((from.to_string(), relation.to_string(), to.to_string()));
    }
}

impl PayloadView {
    pub fn new(id: &str, data: Vec<u8>) -> Self {
        Self {
            id: id.to_string(),
            data,
            metadata: Vec::new(),
        }
    }
    
    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.push((key.to_string(), value.to_string()));
    }
}

/// Dynamic brain capacity orchestration - manages compute resources adaptively
#[derive(Debug, Clone)]
pub struct DynamicBrain {
    pub capacity_profile: CapacityProfile,
    pub current_load: LoadMetrics,
    pub scaling_policy: ScalingPolicy,
    pub replay_context: Option<ReplayContext>,
}

#[derive(Debug, Clone)]
pub struct CapacityProfile {
    pub max_context_length: usize,
    pub max_batch_size: usize,
    pub max_memory_mb: usize,
    pub compute_units: usize,
}

#[derive(Debug, Clone)]
pub struct LoadMetrics {
    pub current_context_length: usize,
    pub current_batch_size: usize,
    pub current_memory_mb: usize,
    pub queue_length: usize,
    pub avg_latency_ms: f32,
}

#[derive(Debug, Clone)]
pub struct ScalingPolicy {
    pub scale_up_threshold: f32,
    pub scale_down_threshold: f32,
    pub scale_up_factor: f32,
    pub scale_down_factor: f32,
    pub min_capacity: CapacityProfile,
    pub max_capacity: CapacityProfile,
}

impl DynamicBrain {
    pub fn new() -> Self {
        Self {
            capacity_profile: CapacityProfile::default(),
            current_load: LoadMetrics::default(),
            scaling_policy: ScalingPolicy::default(),
            replay_context: None,
        }
    }

    /// Attach a replay context for session replay capabilities
    pub fn attach_replay_context(&mut self, replay_context: ReplayContext) {
        self.replay_context = Some(replay_context);
    }

    /// Evaluate current load and suggest scaling actions
    pub fn evaluate_load(&self) -> ScalingAction {
        let load_ratio = self.calculate_load_ratio();
        
        if load_ratio > self.scaling_policy.scale_up_threshold {
            ScalingAction::ScaleUp
        } else if load_ratio < self.scaling_policy.scale_down_threshold {
            ScalingAction::ScaleDown
        } else {
            ScalingAction::Maintain
        }
    }
    
    /// Calculate overall load ratio (0.0 to 1.0+)
    fn calculate_load_ratio(&self) -> f32 {
        let context_ratio = self.current_load.current_context_length as f32 
            / self.capacity_profile.max_context_length as f32;
        let memory_ratio = self.current_load.current_memory_mb as f32 
            / self.capacity_profile.max_memory_mb as f32;
        let queue_ratio = self.current_load.queue_length as f32 
            / 10.0;
        
        context_ratio.max(memory_ratio).max(queue_ratio)
    }
    
    /// Apply scaling decision to adjust capacity
    pub fn apply_scaling(&mut self, action: ScalingAction) {
        match action {
            ScalingAction::ScaleUp => self.scale_up(),
            ScalingAction::ScaleDown => self.scale_down(),
            ScalingAction::Maintain => {}
        }
    }
    
    fn scale_up(&mut self) {
        let new_context = (self.capacity_profile.max_context_length as f32 
            * self.scaling_policy.scale_up_factor) as usize;
        let new_memory = (self.capacity_profile.max_memory_mb as f32 
            * self.scaling_policy.scale_up_factor) as usize;
        let new_batch = (self.capacity_profile.max_batch_size as f32 
            * self.scaling_policy.scale_up_factor) as usize;
        
        self.capacity_profile.max_context_length = 
            new_context.min(self.scaling_policy.max_capacity.max_context_length);
        self.capacity_profile.max_memory_mb = 
            new_memory.min(self.scaling_policy.max_capacity.max_memory_mb);
        self.capacity_profile.max_batch_size = 
            new_batch.min(self.scaling_policy.max_capacity.max_batch_size);
    }
    
    fn scale_down(&mut self) {
        let new_context = (self.capacity_profile.max_context_length as f32 
            * self.scaling_policy.scale_down_factor) as usize;
        let new_memory = (self.capacity_profile.max_memory_mb as f32 
            * self.scaling_policy.scale_down_factor) as usize;
        let new_batch = (self.capacity_profile.max_batch_size as f32 
            * self.scaling_policy.scale_down_factor) as usize;
        
        self.capacity_profile.max_context_length = 
            new_context.max(self.scaling_policy.min_capacity.max_context_length);
        self.capacity_profile.max_memory_mb = 
            new_memory.max(self.scaling_policy.min_capacity.max_memory_mb);
        self.capacity_profile.max_batch_size = 
            new_batch.max(self.scaling_policy.min_capacity.max_batch_size);
    }
    
    /// Optimize context length for a given model and prompt
    pub fn optimize_context_length(
        &mut self, 
        model_info: &super::model_registry::ModelInfo, 
        prompt_length: usize
    ) -> usize {
        let max_response = 128;
        let available_context = model_info.verification.context_length as usize - max_response;
        
        available_context
            .min(self.capacity_profile.max_context_length)
            .max(prompt_length)
    }
}

impl Default for DynamicBrain {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CapacityProfile {
    fn default() -> Self {
        Self {
            max_context_length: 4096,
            max_batch_size: 1,
            max_memory_mb: 2048,
            compute_units: 1,
        }
    }
}

impl Default for LoadMetrics {
    fn default() -> Self {
        Self {
            current_context_length: 0,
            current_batch_size: 0,
            current_memory_mb: 0,
            queue_length: 0,
            avg_latency_ms: 0.0,
        }
    }
}

impl Default for ScalingPolicy {
    fn default() -> Self {
        Self {
            scale_up_threshold: 0.8,
            scale_down_threshold: 0.3,
            scale_up_factor: 1.5,
            scale_down_factor: 0.7,
            min_capacity: CapacityProfile {
                max_context_length: 512,
                max_batch_size: 1,
                max_memory_mb: 256,
                compute_units: 1,
            },
            max_capacity: CapacityProfile {
                max_context_length: 32768,
                max_batch_size: 8,
                max_memory_mb: 8192,
                compute_units: 4,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalingAction {
    ScaleUp,
    ScaleDown,
    Maintain,
}