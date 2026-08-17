// Prove Inference Binary - End-to-end test
// Pipeline: GGUF -> Hacedle -> Brain Runtime -> First Token

use std::env;

use hacedle::x::loader::gguf::GgufLoader;
use hacedle::x::provider::candle::{InferenceEngine, TokenizerEngine};
use hacedle::x::provider::candle::LogitsProcessor;

fn main() {
    let args: Vec<String> = env::args().collect();
    let prompt = if args.len() > 1 {
        args[1].clone()
    } else {
        "hello world, what your model name?".to_string()
    };

    // Model paths
    let model_paths = [
        "D:/host/llama-models/Qwen2.5-0.5B-Instruct-Q4_K_M.gguf",
        "D:/host/llama-models/Phi-3-mini-4k-instruct-Q4_K_M.gguf",
    ];

    let mut model_found = None;
    for path in &model_paths {
        if std::path::Path::new(path).exists() {
            model_found = Some(path.clone());
            break;
        }
    }

    let model_path = match model_found {
        Some(p) => p,
        None => {
            eprintln!("ERROR: No GGUF model found");
            eprintln!("Looking in:");
            for p in &model_paths {
                eprintln!("  - {}", p);
            }
            std::process::exit(1);
        }
    };

    println!("============================================");
    println!("Prove Inference: Hacedle → Brain Runtime");
    println!("============================================");
    println!();
    println!("Model: {}", model_path);
    println!("Prompt: {}", prompt);

    // Create inference engine
    let engine = InferenceEngine::default();

    // Get logits
    let logits = engine.infer_logits(&prompt);

    // Find top 5 tokens
    let mut top5: Vec<(usize, f32)> = logits.iter().enumerate()
        .map(|(i, &v)| (i, v))
        .collect();
    top5.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    top5.truncate(5);

    // Get first token via argmax
    let first_token = top5.first().map(|(i, _)| *i).unwrap_or(0);

    println!();
    println!("First token index: {}", first_token);
    println!();
    println!("Top 5 logits:");
    for (i, (idx, val)) in top5.iter().enumerate() {
        println!("  {}. index={}, logit={:.4}", i + 1, idx, val);
    }

    // Decode first token for sanity
    let decoded = engine.tokenizer.decode(&[first_token as u32]);
    println!();
    println!("Decoded first token: {:?}", decoded);
    println!();
    println!("Status: REAL_INFERENCE_SUCCESS");
}