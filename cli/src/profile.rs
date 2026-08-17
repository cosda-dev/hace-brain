// brain/cli/src/profile.rs
// Hien thi / set brain profile: algo | coder | architect | auditor | legal

use crate::command::ProfileArgs;
use crate::brain::BrainCliError;

pub fn run_profile(args: &ProfileArgs) -> Result<(), BrainCliError> {
    if let Some(name) = &args.set {
        match name.as_str() {
            "algo" | "coder" | "architect" | "auditor" | "legal" => {
                println!("profile: {name}");
                println!("status: set (in-memory, session scope only in E4)");
            }
            _ => {
                eprintln!("error: unknown profile '{name}'");
                eprintln!("valid: algo | coder | architect | auditor | legal");
                return Err(BrainCliError::MissingArg("valid profile name"));
            }
        }
    } else {
        // Default: show current
        println!("profile: algo  (default â€” E4)");
        println!("ce:      AlgoParticle");
        println!("status:  HacedleBrain pending GAP-04");
    }
    Ok(())
}
