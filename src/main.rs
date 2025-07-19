// src/main.rs
/*
 * Main executable for SecureMindset
 */

use clap::Parser;
use securemindset::{Result, run};

#[derive(Parser)]
#[command(version, about = "SecureMindset - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
