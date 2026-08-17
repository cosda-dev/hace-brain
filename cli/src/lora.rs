// Brain CLI - LoRA Commands
// Commands: lora attach, lora detach, lora list, lora verify

use std::path::Path;

/// LoRA command handler
pub struct LoraCommand;

impl LoraCommand {
    pub fn handle(&self, action: &str, lora_path: Option<&str>) -> Result<String, &'static str> {
        let executor = crate::executor::LoraExecutor;
        
        match action {
            "attach" => {
                let path = Path::new(lora_path.unwrap_or(""));
                executor.attach(path).map_err(|_| "attach_failed")?;
                Ok("attached".to_string())
            }
            "detach" => {
                let path = Path::new(lora_path.unwrap_or(""));
                executor.detach(path).map_err(|_| "detach_failed")?;
                Ok("detached".to_string())
            }
            "list" => {
                let adapters = executor.list().map_err(|_| "list_failed")?;
                Ok(adapters.join("\n"))
            }
            "verify" => {
                let path = Path::new(lora_path.unwrap_or(""));
                let valid = executor.verify(path).map_err(|_| "verify_failed")?;
                Ok(if valid { "VALID" } else { "INVALID" }.to_string())
            }
            _ => Err("unknown_action"),
        }
    }
}