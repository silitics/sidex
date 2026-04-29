//! The `gen` command.

use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;
use sidex_gen::Generator;
use sidex_gen_ir::IrGenerator;
use sidex_gen_json_schema::JsonSchemaGenerator;
use sidex_gen_py::PyGenerator;
use sidex_gen_rs::RustGenerator;
use sidex_gen_ts::TsGenerator;

use crate::utils::load_unit_and_bundle;

#[derive(Parser, Debug)]
pub struct GenerateArgs {
    pub backend: String,
    pub output: PathBuf,
    /// Override a backend-config field without editing `sidex.toml`.
    ///
    /// Format: `key.path=value`, repeatable. The `key.path` is a dot-separated
    /// JSON path into the backend's configuration object; `value` is parsed as
    /// JSON when possible, otherwise treated as a string. Examples:
    ///
    ///   `--config-override opaque_lowering=json`
    ///   `--config-override types.table.\\:\\:core\\:\\:builtins\\:\\:i64=u64`
    ///
    /// Paths use `\.` to escape dots inside a key segment.
    #[clap(long = "config-override", value_name = "KEY=VALUE")]
    pub config_overrides: Vec<String>,
}

struct GeneratorRegistry {
    registry: HashMap<String, Box<dyn Generator>>,
}

impl GeneratorRegistry {
    pub fn new() -> Self {
        let mut registry: HashMap<_, Box<dyn Generator>> = HashMap::new();
        registry.insert("ir".to_owned(), Box::new(IrGenerator));
        registry.insert("rust".to_owned(), Box::new(RustGenerator::new()));
        registry.insert("ts".to_owned(), Box::new(TsGenerator::new()));
        registry.insert("json-schema".to_owned(), Box::new(JsonSchemaGenerator));
        registry.insert("py".to_owned(), Box::new(PyGenerator::new()));
        Self { registry }
    }
}

pub fn exec(args: &GenerateArgs) -> eyre::Result<()> {
    let (unit, bundle, transformer) = load_unit_and_bundle(None)?;

    let null = serde_json::Value::Null;
    let mut config = transformer
        .get_bundle_manifest(bundle)
        .backend
        .get(&args.backend)
        .unwrap_or(&null)
        .clone();

    for override_str in &args.config_overrides {
        apply_config_override(&mut config, override_str)?;
    }

    let job = sidex_gen::Job {
        unit: &unit,
        bundle,
        output: &args.output,
        config: &config,
    };

    std::fs::create_dir_all(&args.output)?;

    GeneratorRegistry::new()
        .registry
        .get(&args.backend)
        .unwrap()
        .generate(job)
        .map_err(|err| eyre::eyre!(format!("{err:?}")))?;

    Ok(())
}

/// Apply one `key.path=value` override to the backend config in place.
///
/// The value is parsed as JSON when possible (booleans, numbers, arrays,
/// objects, quoted strings); otherwise it's stored as a JSON string. Path
/// segments are dot-separated; literal dots inside a segment can be escaped
/// with `\.`.
fn apply_config_override(config: &mut serde_json::Value, spec: &str) -> eyre::Result<()> {
    let (key, value_str) = spec
        .split_once('=')
        .ok_or_else(|| eyre::eyre!("--config-override `{spec}` must be `key=value`"))?;
    let segments = parse_path_segments(key);
    if segments.is_empty() {
        eyre::bail!("--config-override key cannot be empty (in `{spec}`)");
    }

    let value = serde_json::from_str(value_str)
        .unwrap_or_else(|_| serde_json::Value::String(value_str.to_owned()));

    if !config.is_object() {
        *config = serde_json::Value::Object(Default::default());
    }
    let mut cursor = config;
    for segment in &segments[..segments.len() - 1] {
        let map = cursor.as_object_mut().ok_or_else(|| {
            eyre::eyre!("--config-override `{spec}` traverses non-object at segment `{segment}`")
        })?;
        cursor = map
            .entry(segment.clone())
            .or_insert_with(|| serde_json::Value::Object(Default::default()));
        if !cursor.is_object() {
            *cursor = serde_json::Value::Object(Default::default());
        }
    }
    let last = &segments[segments.len() - 1];
    cursor
        .as_object_mut()
        .ok_or_else(|| eyre::eyre!("--config-override `{spec}` ends in a non-object container"))?
        .insert(last.clone(), value);
    Ok(())
}

/// Split a dotted path, honoring `\.` as an escape for a literal dot.
fn parse_path_segments(path: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut chars = path.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            '.' => {
                out.push(std::mem::take(&mut current));
            }
            _ => current.push(c),
        }
    }
    out.push(current);
    out
}
