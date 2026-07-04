//! Headless CLI entry point for the Sea of Stars TAS.
//!
//! Runs the TAS without a GUI, emitting structured logs and a deterministic
//! exit code. See `sea_of_stars_tas::cli` for the flags and exit codes.

use clap::Parser;
use sea_of_stars_tas::cli::{self, CliArgs};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = CliArgs::parse();
    cli::run(args)
}
