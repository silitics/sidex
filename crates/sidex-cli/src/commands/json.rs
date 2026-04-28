//! The `json` command group — JSON-format helpers built on the canonical
//! Sidex JSON shape.

use std::io::Write;

use clap::Parser;
use clap::Subcommand;
use eyre::Result;
use rand::RngCore;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use sidex_fuzz::Config;
use sidex_fuzz::Source;

use crate::utils::load_unit_and_bundle;

#[derive(Parser, Debug)]
pub struct JsonArgs {
    #[clap(subcommand)]
    pub command: JsonCommand,
}

#[derive(Subcommand, Debug)]
pub enum JsonCommand {
    /// Generate random JSON instances conforming to a Sidex type.
    Generate(GenerateArgs),
}

#[derive(Parser, Debug)]
pub struct GenerateArgs {
    /// Fully qualified type to generate, in the form `bundle::schema::Name`.
    pub type_path: String,
    /// Number of samples to emit (one JSON value per line).
    #[clap(long, default_value_t = 1)]
    pub count: usize,
    /// Seed for the PRNG. Random by default; pass an explicit value for
    /// reproducible output.
    #[clap(long)]
    pub seed: Option<u64>,
    /// Pretty-print each JSON sample on its own.
    ///
    /// Without this flag samples are emitted as compact JSON, one per line, so
    /// the output is line-addressable for piping into round-trip harnesses.
    #[clap(long)]
    pub pretty: bool,
    /// Maximum recursion depth for nested types.
    #[clap(long)]
    pub max_depth: Option<usize>,
    /// Maximum length of generated lists, maps, and free-form objects.
    #[clap(long)]
    pub max_collection_len: Option<usize>,
    /// Maximum length of generated strings and free-form object keys.
    #[clap(long)]
    pub max_string_len: Option<usize>,
    /// Encode `NaN`/`±Infinity` as the JSON strings `"NaN"`/`"Infinity"`/
    /// `"-Infinity"`. Mirrors `JsonConfig::enable_floats_as_strings` on the
    /// codegens — set this when targets are built with that flag enabled.
    #[clap(long)]
    pub floats_as_strings: bool,
    /// Encode `i64`/`u64`/`idx` as decimal JSON strings instead of JSON
    /// numbers. Mirrors `JsonConfig::enable_integers_as_strings` on the
    /// codegens — set this when targets serialize past JS's 53-bit safe
    /// integer boundary as strings to avoid precision loss.
    #[clap(long)]
    pub integers_as_strings: bool,
}

pub fn exec(args: &JsonArgs) -> Result<()> {
    match &args.command {
        JsonCommand::Generate(generate_args) => exec_generate(generate_args),
    }
}

fn exec_generate(args: &GenerateArgs) -> Result<()> {
    let (unit, _bundle, _transformer) = load_unit_and_bundle(None)?;

    let typ = sidex_fuzz::lookup_type(&unit, &args.type_path).ok_or_else(|| {
        eyre::eyre!(
            "no definition matches `{}` — expected `bundle::schema::Name`",
            args.type_path
        )
    })?;

    let mut config = Config::default();
    if let Some(d) = args.max_depth {
        config.max_depth = d;
    }
    if let Some(c) = args.max_collection_len {
        config.max_collection_len = c;
    }
    if let Some(s) = args.max_string_len {
        config.max_string_len = s;
    }
    config.floats_as_strings = args.floats_as_strings;
    config.integers_as_strings = args.integers_as_strings;

    // A user-supplied seed reproduces a specific sample; otherwise we derive a
    // fresh seed per sample from a CSPRNG so successive runs differ.
    let mut seed_rng = match args.seed {
        Some(seed) => ChaCha8Rng::seed_from_u64(seed),
        None => ChaCha8Rng::from_entropy(),
    };

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    for _ in 0..args.count {
        let sample_seed = seed_rng.next_u64();
        let mut source = Source::from_seed(sample_seed);
        let value = sidex_fuzz::generate(&unit, &typ, &mut source, &config)
            .map_err(|err| eyre::eyre!(format!("{err:?}")))?;
        if args.pretty {
            serde_json::to_writer_pretty(&mut handle, &value)?;
        } else {
            serde_json::to_writer(&mut handle, &value)?;
        }
        handle.write_all(b"\n")?;
    }
    Ok(())
}
