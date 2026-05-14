//! # grove-cli
//!
//! Command-line interface for the grove procedural tree generator.
//!
//! This tool provides a convenient way to generate trees from the command line,
//! supporting various output formats and configuration options.
//!
//! ## Usage
//!
//! ```bash
//! grove generate --species oak --seed 42 --output tree.glb
//! grove list-species
//! grove validate-preset species/oak.toml
//! ```

use clap::Parser;

/// Grove - Procedural tree generator
#[derive(Parser, Debug)]
#[command(name = "grove")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let _cli = Cli::parse();

    println!("Grove - Procedural Tree Generator");
    println!("Use --help for usage information");

    // TODO: Implement CLI commands
    // TODO: Implement CLI commands using grove_core::generate_tree()
}
