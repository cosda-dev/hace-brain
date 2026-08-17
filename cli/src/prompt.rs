// brain/cli/src/prompt.rs
// Xu ly subcommand: hace brain prompt <TEXT>
// Output: tokens hoac decoded text tuy --raw-tokens flag.
// E4: AlgoParticle (deterministic) la default CE khi chua co HacedleBrain.

use crate::command::PromptArgs;
use crate::brain::BrainCliError;

pub fn run_prompt(text: &str, args: &PromptArgs) -> Result<(), BrainCliError> {
    let ce = args.ce.as_deref().unwrap_or("algo");

    match ce {
        "algo" => run_algo(text, args),
        "hacedle" => run_hacedle(text, args),
        "llama"   => run_llama(text, args),
        _         => {
            eprintln!("unknown CE: {ce}. valid: algo | hacedle | llama | remote");
            Err(BrainCliError::MissingArg("valid --ce value"))
        }
    }
}

// â”€â”€ CE.Algo â€” deterministic, no model needed â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

fn run_algo(text: &str, args: &PromptArgs) -> Result<(), BrainCliError> {
    // ASCII byte encoding â€” explicit E4 placeholder, not "inference"
    let tokens: Vec<u32> = text.bytes().map(|b| b as u32).collect();
    let used = tokens.len().min(args.max_tokens as usize);
    let out = &tokens[..used];

    if args.raw_tokens {
        println!("ce: algo");
        println!("tokens: {:?}", out);
    } else {
        // Decode back to string (passthrough for algo CE)
        let decoded: String = out.iter()
            .filter_map(|&t| char::from_u32(t))
            .collect();
        println!("ce: algo");
        println!("output: {decoded}");
        println!("tokens_used: {}", out.len());
    }
    Ok(())
}

// â”€â”€ CE.Hacedle â€” edge LLM (requires model path) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

fn run_hacedle(text: &str, args: &PromptArgs) -> Result<(), BrainCliError> {
    let model_path = args.model.as_deref().ok_or_else(|| {
        eprintln!("error: --model <PATH> required for ce=hacedle");
        BrainCliError::MissingArg("--model")
    })?;

    // Verify file exists before attempting load
    if !std::path::Path::new(model_path).exists() {
        eprintln!("error: model not found: {model_path}");
        return Err(BrainCliError::IoError(format!("not found: {model_path}")));
    }

    // E4: HacedleBrain not yet wired â€” emit clear stub notice, not silent default
    eprintln!("warn: CE.Hacedle not yet wired (GAP-04). Falling back to CE.Algo.");
    eprintln!("      model path recorded: {model_path}");
    run_algo(text, args)
}

// â”€â”€ CE.Llama â€” llama.cpp backend â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

fn run_llama(text: &str, args: &PromptArgs) -> Result<(), BrainCliError> {
    let model_path = args.model.as_deref().ok_or_else(|| {
        eprintln!("error: --model <PATH> required for ce=llama");
        BrainCliError::MissingArg("--model")
    })?;

    if !std::path::Path::new(model_path).exists() {
        eprintln!("error: model not found: {model_path}");
        return Err(BrainCliError::IoError(format!("not found: {model_path}")));
    }

    eprintln!("warn: CE.Llama not yet wired (GAP-10). Falling back to CE.Algo.");
    eprintln!("      model path recorded: {model_path}");
    run_algo(text, args)
}
