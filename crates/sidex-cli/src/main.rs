#![doc = include_str!("../README.md")]

use clap::{Parser, Subcommand};

pub mod commands;

/// CLI arguments.
#[derive(Parser, Debug)]
#[clap(
    name = "sidex",
    version,
    about = "A format- and language-agnostic data modeling and API definition language."
)]
pub struct Args {
    /// CLI subcommand.
    #[clap(subcommand)]
    pub command: Command,
}

/// CLI subcommands.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Create a new definition.
    New(commands::new::NewArgs),
    /// Check the current definition for validity.
    Check(commands::check::CheckArgs),
    /// Generate code.
    Generate(commands::generate::GenerateArgs),
}

/// Entrypoint of the `sidex` executable.
pub fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    match &args.command {
        Command::New(new_args) => commands::new::exec(new_args),
        Command::Check(check_args) => commands::check::exec(check_args),
        Command::Generate(generate_args) => commands::generate::exec(generate_args),
    }
}
