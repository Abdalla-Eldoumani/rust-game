//! CLI entry for the Rust learning game.

mod engine;
mod exercise;
mod grader;
mod persistence;
mod util;

use anyhow::Result;
use clap::{Parser, Subcommand};
use engine::Command as EngineCommand;

#[derive(Parser)]
#[command(name = "rust-game")]
#[command(version, about = "Learn Rust by solving real exercises")]
struct Cli {
    #[arg(long)]
    lessons: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    List,
    Start { id: String },
    Open { id: String },
    Check { id: String, #[arg(long)] timeout: Option<u64> },
    Hint { id: String },
    Progress,
    Reset { id: String },
    CheckAll,
    Validate,
    Solution { id: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let lessons_root = cli.lessons.unwrap_or_else(|| "lessons".to_string());
    let cmd = match cli.command {
        Commands::List => EngineCommand::List,
        Commands::Start { id } => EngineCommand::Start { id },
        Commands::Open { id } => EngineCommand::Open { id },
        Commands::Check { id, timeout } => EngineCommand::Check { id, timeout },
        Commands::Hint { id } => EngineCommand::Hint { id },
        Commands::Progress => EngineCommand::Progress,
        Commands::Reset { id } => EngineCommand::Reset { id },
        Commands::CheckAll => EngineCommand::CheckAll,
        Commands::Validate => EngineCommand::Validate,
        Commands::Solution { id } => EngineCommand::Solution { id },
    };
    engine::run(lessons_root, cmd)
}