// src/main.rs
/*
 * Main executable for ArtificialEaselStudio
 */

use clap::Parser;
use artificialeaselstudio::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialEaselStudio - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
