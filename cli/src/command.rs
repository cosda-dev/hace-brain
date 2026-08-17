// brain/cli/src/command.rs
// Dinh nghia CLI commands cho `hace brain` subcommand tree.
// Moi subcommand co OwnerArgs rieng â€” khong share struct.

use clap::{Parser, Subcommand, Args};

/// `hace brain` â€” Brain CE CLI
#[derive(Parser, Debug)]
#[command(name = "brain", about = "Brain CE operations")]
pub struct BrainCli {
    #[command(subcommand)]
    pub cmd: BrainCommand,
}

#[derive(Subcommand, Debug)]
pub enum BrainCommand {
    /// Run inference on a prompt
    Prompt(PromptArgs),
    /// Model operations (verify, inspect, load)
    Model(ModelArgs),
    /// Session replay operations
    Replay(ReplayArgs),
    /// Show active brain profile
    Profile(ProfileArgs),
    /// Benchmark CE performance
    Bench(BenchArgs),
}

// â”€â”€ prompt subcommand â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

#[derive(Args, Debug)]
pub struct PromptArgs {
    /// Prompt text (positional) OR via --prompt flag
    #[arg(index = 1)]
    pub text: Option<String>,

    /// Prompt text (flag form)
    #[arg(long, value_name = "TEXT")]
    pub prompt: Option<String>,

    /// Model file path (.gguf)
    #[arg(long, short = 'm', value_name = "PATH")]
    pub model: Option<String>,

    /// Max tokens to generate
    #[arg(long, default_value = "64")]
    pub max_tokens: u32,

    /// Temperature (0.0 â€“ 2.0)
    #[arg(long, default_value = "0.7")]
    pub temperature: f32,

    /// Output raw token IDs (debug)
    #[arg(long)]
    pub raw_tokens: bool,

    /// Force CE backend: algo | hacedle | llama | remote
    #[arg(long, value_name = "CE")]
    pub ce: Option<String>,
}

impl PromptArgs {
    /// Resolve prompt text: positional arg > --prompt flag
    pub fn resolve_text(&self) -> Option<&str> {
        self.text.as_deref().or(self.prompt.as_deref())
    }
}

// â”€â”€ model subcommand â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

#[derive(Args, Debug)]
pub struct ModelArgs {
    #[command(subcommand)]
    pub action: ModelAction,
}

#[derive(Subcommand, Debug)]
pub enum ModelAction {
    /// Verify GGUF integrity (metadata + tensor count)
    Verify {
        #[arg(value_name = "PATH")]
        path: String,
        /// Show full tensor list
        #[arg(long)]
        tensors: bool,
    },
    /// Inspect model metadata
    Inspect {
        #[arg(value_name = "PATH")]
        path: String,
    },
}

// â”€â”€ replay subcommand â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

#[derive(Args, Debug)]
pub struct ReplayArgs {
    #[command(subcommand)]
    pub action: ReplayAction,
}

#[derive(Subcommand, Debug)]
pub enum ReplayAction {
    /// Save current session to file
    Save {
        #[arg(value_name = "PATH")]
        path: String,
    },
    /// Load and replay a saved session
    Load {
        #[arg(value_name = "PATH")]
        path: String,
        /// Re-run inference instead of printing stored output
        #[arg(long)]
        rerun: bool,
    },
    /// List saved sessions
    List {
        #[arg(long, default_value = ".")]
        dir: String,
    },
}

// â”€â”€ profile subcommand â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

#[derive(Args, Debug)]
pub struct ProfileArgs {
    /// Set active profile: algo | coder | architect | auditor | legal
    #[arg(long, value_name = "NAME")]
    pub set: Option<String>,
    /// Show current profile config
    #[arg(long)]
    pub show: bool,
}

// â”€â”€ bench subcommand â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

#[derive(Args, Debug)]
pub struct BenchArgs {
    #[arg(long, value_name = "PATH")]
    pub model: Option<String>,
    #[arg(long, default_value = "10")]
    pub iterations: u32,
    #[arg(long, default_value = "32")]
    pub prompt_tokens: u32,
}
