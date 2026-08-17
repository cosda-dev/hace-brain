// brain/cli/src/brain.rs
// BrainCmd dispatcher â€” routes BrainCommand enum to correct handler.
// RULE: moi arm match goi handler rieng biet, KHONG shared executor stub.

use clap::Parser;
use crate::command::{BrainCli, BrainCommand, ModelAction, ReplayAction};
use crate::prompt::run_prompt;
use crate::replay::run_replay;

pub struct BrainCmd;

impl BrainCmd {
    pub fn run(raw_args: &[String]) -> Result<(), BrainCliError> {
        // Parse from slice â€” skip binary name if present
        let cli = match BrainCli::try_parse_from(raw_args) {
            Ok(c) => c,
            Err(e) => {
                // clap prints help/error to stderr automatically
                e.print().ok();
                return Err(BrainCliError::ParseError);
            }
        };

        match cli.cmd {
            BrainCommand::Prompt(args) => {
                let text = args.resolve_text().ok_or_else(|| {
                    eprintln!("error: prompt text required â€” use: hace brain prompt <TEXT>");
                    BrainCliError::MissingArg("prompt text")
                })?;
                run_prompt(text, &args)
            }

            BrainCommand::Model(args) => match args.action {
                ModelAction::Verify { path, tensors } => {
                    crate::model::run_verify(&path, tensors)
                }
                ModelAction::Inspect { path } => {
                    crate::model::run_inspect(&path)
                }
            },

            BrainCommand::Replay(args) => match args.action {
                ReplayAction::Save { path } => run_replay_save(&path),
                ReplayAction::Load { path, rerun } => run_replay_load(&path, rerun),
                ReplayAction::List { dir } => run_replay_list(&dir),
            },

            BrainCommand::Profile(args) => {
                crate::profile::run_profile(&args)
            }

            BrainCommand::Bench(args) => {
                crate::benchmark::run_bench(&args)
            }
        }
    }
}

// â”€â”€ replay wrappers (local to brain.rs to keep replay.rs focused) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

fn run_replay_save(path: &str) -> Result<(), BrainCliError> {
    use crate::replay::SessionStore;
    SessionStore::save_current(path).map_err(|e| BrainCliError::IoError(e))?;
    println!("saved: {path}");
    Ok(())
}

fn run_replay_load(path: &str, rerun: bool) -> Result<(), BrainCliError> {
    use crate::replay::SessionStore;
    let session = SessionStore::load(path).map_err(|e| BrainCliError::IoError(e))?;
    if rerun {
        println!("rerun: {} interactions", session.interactions.len());
        // TODO: wire to ZeusRuntime when HacedleBrain ready
    } else {
        for (i, interaction) in session.interactions.iter().enumerate() {
            println!("[{}] {}: {}", i, interaction.role, interaction.content);
        }
    }
    Ok(())
}

fn run_replay_list(dir: &str) -> Result<(), BrainCliError> {
    use std::fs;
    let entries = fs::read_dir(dir)
        .map_err(|e| BrainCliError::IoError(e.to_string()))?;
    let mut count = 0;
    for entry in entries.flatten() {
        let name = entry.file_name();
        let n = name.to_string_lossy();
        if n.ends_with(".sio") || n.ends_with(".session") {
            println!("{}", entry.path().display());
            count += 1;
        }
    }
    if count == 0 {
        println!("no sessions found in: {dir}");
    }
    Ok(())
}

// â”€â”€ error type â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

#[derive(Debug)]
pub enum BrainCliError {
    ParseError,
    MissingArg(&'static str),
    IoError(String),
    InferError(String),
}

impl std::fmt::Display for BrainCliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError          => write!(f, "argument parse error"),
            Self::MissingArg(a)      => write!(f, "missing argument: {a}"),
            Self::IoError(e)         => write!(f, "io error: {e}"),
            Self::InferError(e)      => write!(f, "inference error: {e}"),
        }
    }
}
