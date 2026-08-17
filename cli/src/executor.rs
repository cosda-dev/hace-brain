// brain/cli/src/executor.rs
// BrainExecutor â€” wrapper cho ZeusRuntime::execute().
// E4: AlgoParticle la default CE. HacedleBrain wire sau khi GAP-04 done.
// KHONG return stub default â€” emit ro rang trang thai CE.

use hace_brain_base::{AlgoParticle, BrainKernel, ReasonCtx};
use hace_brain_runtime::{ZeusRuntime, ZeusConfig};

pub struct BrainExecutor {
    zeus: ZeusRuntime,
}

impl BrainExecutor {
    /// E4 default: AlgoParticle only. HacedleBrain injected sau T2.
    pub fn new_algo() -> Self {
        let config = ZeusConfig {
            mode:          hace_brain_base::BrainMode::AlgoOnly,
            max_local_ctx: 4096,
            prefer_local:  true,
        };
        Self { zeus: ZeusRuntime::new(config) }
    }

    /// Sau T2: inject HacedleBrain
    // pub fn new_hacedle(brain: HacedleBrain) -> Self { ... }

    pub async fn infer(
        &self,
        text:       &str,
        max_tokens: u32,
        temperature: f32,
    ) -> ExecutorResult {
        let ctx = ReasonCtx {
            intent_id:     "cli".to_string(),
            action:        text.to_string(),
            payload:       serde_json::json!({ "text": text }),
            memory:        vec![],
            domain:        None,
            soul_id:       None,
            brain_profile: None,
        };

        let t0 = std::time::Instant::now();
        match self.zeus.execute(ctx).await {
            Ok(result) => ExecutorResult {
                ce_id:      result.model_id.clone(),
                output:     result.output.to_string(),
                tokens:     result.tokens_used.min(max_tokens),
                latency_ms: t0.elapsed().as_millis() as u64,
                error:      None,
            },
            Err(e) => ExecutorResult {
                ce_id:      "error".to_string(),
                output:     String::new(),
                tokens:     0,
                latency_ms: t0.elapsed().as_millis() as u64,
                error:      Some(e.to_string()),
            },
        }
    }
}

#[derive(Debug)]
pub struct ExecutorResult {
    pub ce_id:      String,
    pub output:     String,
    pub tokens:     u32,
    pub latency_ms: u64,
    pub error:      Option<String>,
}
