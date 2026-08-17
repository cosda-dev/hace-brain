//! hace-brain-base — Reasoning kernel (Zeus brain interface)
//! canon: SMF://hace.agem.executor-mesh.v1 (brain=zeus component)
//!
//! BrainKernel   = universal reasoning interface (route + reason)
//! BrainRuntime  = CE (Cluster Execute) contract — impl per backend
//!
//! CE map:
//!   CE.Algo     = AlgoParticle    (deterministic, WIC only)
//!   CE.Hacedle  = HacedleBrain   (edge LLM, Candle/no_std)
//!   CE.Hacetral = HacetralBrain  (Mistral + Authority DSL)
//!   CE.Llama    = LlamaBrain     (llama.cpp)
//!   CE.Remote   = RemoteBrain    (cloud RACEX)
//!
//! Zeus does NOT know Candle/ONNX/llama — only calls BrainRuntime trait.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ── Reasoning context ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonCtx {
    pub intent_id: String,
    pub action:    String,
    pub payload:   serde_json::Value,
    pub memory:    Vec<MemoryItem>,    // injected from soul/memory
    pub domain:    Option<String>,     // expert domain hint from SoulProfile
    pub soul_id:   Option<String>,     // active soul://architect etc.
    pub brain_profile: Option<String>, // zeus://coder.v1 etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryItem {
    pub key:       String,
    pub value:     serde_json::Value,
    pub relevance: f32,  // 0.0-1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonResult {
    pub output:      serde_json::Value,
    pub confidence:  f32,
    pub tokens_used: u32,
    pub model_id:    String,   // "algo" | "ce.hacedle" | "ce.llama" | "ce.remote:*"
    pub plan:        Option<Vec<String>>,
    pub latency_ms:  u64,
}

#[derive(Debug, Error)]
pub enum BrainError {
    #[error("reasoning failed: {msg}")]
    ReasonFailed { msg: String },
    #[error("model unavailable: {model}")]
    ModelUnavailable { model: String },
    #[error("context too large: {tokens} tokens")]
    ContextTooLarge { tokens: u32 },
    #[error("timeout after {ms}ms")]
    Timeout { ms: u64 },
    #[error("runtime not loaded")]
    RuntimeNotLoaded,
}

// ── BrainKernel trait (Zeus routing interface) ────────────────────────────────

#[async_trait]
pub trait BrainKernel: Send + Sync {
    fn model_id(&self) -> &str;
    fn is_local(&self) -> bool;
    fn max_context_tokens(&self) -> u32 { 4096 }

    async fn reason(&self, ctx: ReasonCtx) -> Result<ReasonResult, BrainError>;

    /// AEP cost estimate: WIC for algo, TIC for LLM
    fn estimate_aep(&self, ctx: &ReasonCtx) -> u64 {
        let _ = ctx;
        if self.is_local() { 10_000 } else { 5_000_000 }
    }
}

// ── BrainRuntime trait (CE contract) ─────────────────────────────────────────
//
// Every CE backend (HacedleBrain, LlamaBrain, RemoteBrain) implements this.
// Zeus only calls BrainRuntime — never knows Candle/ONNX underneath.

#[async_trait]
pub trait BrainRuntime: BrainKernel {
    /// CE identity tag — "ce.hacedle" | "ce.llama" | "ce.remote" | "ce.algo"
    fn ce_id(&self) -> &str;

    /// Load model artifact into CE runtime
    async fn load(&self, artifact: BrainArtifact) -> Result<(), BrainError>;

    /// Embed text to vector (optional — not all CEs support this)
    async fn embed(&self, text: &str) -> Result<Embedding, BrainError> {
        let _ = text;
        Err(BrainError::ReasonFailed { msg: format!("{} does not support embed()", self.ce_id()) })
    }

    /// Unload model — free runtime resources
    async fn unload(&self) -> Result<(), BrainError>;

    /// Health check — is this CE ready?
    fn is_ready(&self) -> bool;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainArtifact {
    pub id:      String,
    pub path:    Option<String>,
    pub bytes:   Option<Vec<u8>>,
    pub format:  ArtifactFormat,
    pub quant:   Option<String>,  // "q4", "q8", "f16", "f32"
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactFormat {
    Gguf,
    Safetensors,
    Onnx,
    Wasm,
    Raw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    pub dims:   usize,
    pub values: Vec<f32>,
    pub model:  String,
}

// ── BrainProfile — Zeus expert configuration ──────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainProfile {
    pub id:           String,   // "zeus://coder.v1"
    pub models:       Vec<String>,
    pub capabilities: Vec<String>,
    pub preferred_ce: Option<String>, // "ce.hacedle" | "ce.llama"
}

// ── AlgoParticle — deterministic CE (CE.Algo) ────────────────────────────────

pub struct AlgoParticle {
    pub rules: Vec<AlgoRule>,
}

#[derive(Debug, Clone)]
pub struct AlgoRule {
    pub pattern: String,
    pub output:  serde_json::Value,
}

impl AlgoParticle {
    pub fn new() -> Self { Self { rules: Vec::new() } }
    pub fn add_rule(&mut self, pattern: &str, output: serde_json::Value) {
        self.rules.push(AlgoRule { pattern: pattern.into(), output });
    }
}

impl Default for AlgoParticle { fn default() -> Self { Self::new() } }

#[async_trait]
impl BrainKernel for AlgoParticle {
    fn model_id(&self) -> &str { "algo" }
    fn is_local(&self) -> bool { true }

    async fn reason(&self, ctx: ReasonCtx) -> Result<ReasonResult, BrainError> {
        let t0 = std::time::Instant::now();
        for rule in &self.rules {
            if ctx.action.contains(&rule.pattern) {
                return Ok(ReasonResult {
                    output:      rule.output.clone(),
                    confidence:  1.0,
                    tokens_used: 0,
                    model_id:    "algo".into(),
                    plan:        None,
                    latency_ms:  t0.elapsed().as_millis() as u64,
                });
            }
        }
        Ok(ReasonResult {
            output:      ctx.payload,
            confidence:  0.5,
            tokens_used: 0,
            model_id:    "algo".into(),
            plan:        None,
            latency_ms:  t0.elapsed().as_millis() as u64,
        })
    }
}

// ── BrainMode ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrainMode {
    AlgoOnly,
    LocalLlm,
    RemoteLlm,
    Hybrid,
}

// ── Complexity routing ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Complexity {
    Simple,   // CE.Algo
    Medium,   // CE.Hacedle
    Complex,  // CE.Hacetral | CE.Llama | CE.Remote
}

pub fn classify_complexity(action: &str, payload_size: usize) -> Complexity {
    if payload_size < 256 && matches!(action, "compute" | "route" | "validate" | "ping") {
        return Complexity::Simple;
    }
    if payload_size < 2048 { Complexity::Medium } else { Complexity::Complex }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(action: &str) -> ReasonCtx {
        ReasonCtx {
            intent_id: "i1".into(), action: action.into(),
            payload: serde_json::json!({"x": 1}),
            memory: vec![], domain: None, soul_id: None, brain_profile: None,
        }
    }

    #[tokio::test]
    async fn algo_rule_match() {
        let mut b = AlgoParticle::new();
        b.add_rule("compute", serde_json::json!({"result": 42}));
        let r = b.reason(ctx("compute")).await.unwrap();
        assert_eq!(r.output["result"], 42);
        assert_eq!(r.tokens_used, 0);
    }

    #[tokio::test]
    async fn algo_echo_fallback() {
        let b = AlgoParticle::new();
        let r = b.reason(ctx("unknown")).await.unwrap();
        assert_eq!(r.output["x"], 1);
    }

    #[test]
    fn complexity_routing() {
        assert_eq!(classify_complexity("compute", 100), Complexity::Simple);
        assert_eq!(classify_complexity("infer",   100), Complexity::Medium);
        assert_eq!(classify_complexity("plan",   3000), Complexity::Complex);
    }

    #[test]
    fn reason_ctx_has_soul_fields() {
        let c = ReasonCtx {
            intent_id: "i1".into(), action: "code".into(),
            payload: serde_json::json!({}), memory: vec![],
            domain: Some("rust".into()),
            soul_id: Some("soul://coder".into()),
            brain_profile: Some("zeus://coder.v1".into()),
        };
        assert_eq!(c.soul_id.as_deref(), Some("soul://coder"));
    }
}
