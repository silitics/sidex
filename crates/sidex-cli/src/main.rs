#![doc = include_str!("../README.md")]

use clap::{Parser, Subcommand};

pub mod commands;
pub mod utils;

/// Arguments of the CLI.
#[derive(Parser, Debug)]
#[clap(
    name = "sidex",
    version,
    about = "A format- and language-agnostic data modeling and API definition framework.",
    long_about = concat!(
        "A format- and language-agnostic data modeling and API definition framework.\n\n",
        "For more information visit: https://oss.silitics.com/sidex/."
    ),
)]
pub struct Args {
    /// The verbosity of the CLI.
    // #[clap(global = true, long = "verbose", short = 'v', action = clap::ArgAction::Count)]
    // pub verbosity: u8,
    /// The CLI command.
    #[clap(subcommand)]
    pub command: Command,
}

/// CLI commands.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Create a new definition.
    New(commands::new::NewArgs),
    /// Check the current definition for validity.
    Check,
    /// Generate code.
    Generate(commands::generate::GenerateArgs),
}

/// Entrypoint of the `sidex` executable.
pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    match &args.command {
        Command::New(new_args) => commands::new::exec(new_args),
        Command::Check => commands::check::exec(),
        Command::Generate(generate_args) => commands::generate::exec(generate_args),
    }
}
