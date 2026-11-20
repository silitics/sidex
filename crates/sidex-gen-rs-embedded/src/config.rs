//! Configuration of the backend.

use std::collections::HashMap;

use serde::Deserialize;

/// Configuration of the `rust-embedded` backend in `sidex.toml`.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Traits to derive for generated structs and enums, in addition to
    /// `Clone` and `Copy`.
    #[serde(default)]
    pub derive: Vec<String>,
    /// Groups whose definitions to generate. If empty, all definitions are
    /// generated.
    #[serde(default)]
    pub groups: Vec<String>,
    /// Rust types replacing Sidex types.
    #[serde(default)]
    pub types: TypesConfig,
    /// Rust paths of the modules generated for other bundles, by bundle name.
    #[serde(default)]
    pub external: HashMap<String, String>,
}

/// Rust types replacing Sidex types.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypesConfig {
    /// Rust type by fully qualified Sidex path, for instance,
    /// `::bundle::schema::Type`. Types that borrow use the lifetime `'a`.
    #[serde(default)]
    pub table: HashMap<String, String>,
}
