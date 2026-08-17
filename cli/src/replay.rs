// brain/cli/src/replay.rs
// Xu ly session replay: save / load / list
// SessionRecord: serialize interactions to .sio JSON file.

use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::brain::BrainCliError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    pub role:      String,  // "user" | "brain"
    pub content:   String,
    pub ce:        String,
    pub tokens:    u32,
    pub latency_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecord {
    pub session_id:   String,
    pub model_path:   Option<String>,
    pub ce:           String,
    pub created_at:   u64,
    pub interactions: Vec<Interaction>,
}

impl SessionRecord {
    pub fn new(ce: &str) -> Self {
        Self {
            session_id:   uuid_simple(),
            model_path:   None,
            ce:           ce.to_string(),
            created_at:   unix_now(),
            interactions: Vec::new(),
        }
    }

    pub fn push(&mut self, role: &str, content: &str, tokens: u32, latency_ms: u64) {
        self.interactions.push(Interaction {
            role:      role.to_string(),
            content:   content.to_string(),
            ce:        self.ce.clone(),
            tokens,
            latency_ms,
        });
    }
}

pub struct SessionStore;

impl SessionStore {
    /// Save a new empty session record as placeholder.
    /// Real session data populated by prompt handler after inference.
    pub fn save_current(path: &str) -> Result<(), String> {
        let record = SessionRecord::new("algo");
        let json = serde_json::to_string_pretty(&record)
            .map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())
    }

    pub fn load(path: &str) -> Result<SessionRecord, String> {
        if !Path::new(path).exists() {
            return Err(format!("file not found: {path}"));
        }
        let raw = fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&raw).map_err(|e| format!("parse error: {e}"))
    }
}

pub fn run_replay(path: &str) -> Result<(), BrainCliError> {
    SessionStore::save_current(path)
        .map_err(BrainCliError::IoError)?;
    println!("saved: {path}");
    Ok(())
}

// â”€â”€ helpers â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn uuid_simple() -> String {
    // deterministic from timestamp â€” no uuid dep needed in E4
    format!("sess-{}", unix_now())
}
