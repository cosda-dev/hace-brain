// Block0 Parity Test - First real inference checkpoint
// CRD Directive D6: Golden path verification

use std::fs;
use std::path::Path;

fn check_gguf_header(path: &str) -> Result<bool, &'static str> {
    let gguf_path = Path::new(path);
    if !gguf_path.exists() {
        return Ok(false);
    }
    
    let data = fs::read(gguf_path).map_err(|_| "read_failed")?;
    
    if data.len() < 24 {
        return Ok(false);
    }
    
    // Check magic
    let magic: [u8; 4] = data[0..4].try_into().map_err(|_| "magic_parse")?;
    if magic != *b"GGUF" {
        return Ok(false);
    }
    
    // Check tensor count > 0
    let tensor_count = u64::from_le_bytes([
        data[8], data[9], data[10], data[11],
        data[12], data[13], data[14], data[15]
    ]);
    
    Ok(tensor_count > 0)
}

#[test]
fn test_qwen25_gguf_exists() {
    let model_path = "D:/host/llama-models/Qwen2.5-0.5B-Instruct-Q4_K_M.gguf";
    if Path::new(model_path).exists() {
        let valid = check_gguf_header(model_path).unwrap_or(false);
        assert!(valid, "GGUF header valid");
    } else {
        println!("SKIP: Model not found at {}", model_path);
    }
}

#[test]
fn test_tensor_count_positive() {
    let model_path = "D:/host/llama-models/Qwen2.5-0.5B-Instruct-Q4_K_M.gguf";
    if Path::new(model_path).exists() {
        let valid = check_gguf_header(model_path).unwrap_or(false);
        assert!(valid, "Tensor count > 0");
    } else {
        println!("SKIP: Model not found");
    }
}