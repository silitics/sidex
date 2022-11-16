//! Various helper functions for dealing with bundles.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use sidex_syntax::source::SourceId;
use thiserror::Error;

use crate::ir;

pub const MANIFEST_NAME: &'static str = "sidex.toml";
pub const SCHEMAS_DIR: &'static str = "schemas";
pub const SCHEMA_EXTENSION: &'static str = ".sidex";

/// A model manifest.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct Manifest {
    /// Metadata of the schema.
    #[serde(rename = "bundle")]
    pub metadata: ir::Metadata,
    /// Dependencies of the schema.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    dependencies: HashMap<String, Dependency>,
    /// Generator-specific configurations.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub backend: HashMap<String, serde_json::Value>,
}

impl Manifest {
    /// Create a new manifest with the provided metadata.
    pub fn new(metadata: ir::Metadata) -> Self {
        Self {
            metadata,
            dependencies: Default::default(),
            backend: Default::default(),
        }
    }

    /// The dependencies of the schema.
    pub fn dependencies(&self) -> impl Iterator<Item = (&str, &Dependency)> {
        self.dependencies
            .iter()
            .map(|(name, dependency)| (name.as_str(), dependency))
    }

    /// Extract generator-specific configurations.
    ///
    /// Returns the default configuration in case no configuration has been specified.
    pub fn get_generator_config<C: Default + for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<C, ConfigParseError> {
        self.backend.get(key).map_or_else(
            || Ok(C::default()),
            |value| serde_json::from_value(value.clone()).map_err(ConfigParseError),
        )
    }
}

/// Error parsing a generator-specific configuration.
#[derive(Error, Debug)]
#[error("Error parsing generator-specific configuration.")]
pub struct ConfigParseError(#[source] serde_json::Error);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dependency {
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct BundleSource {
    pub manifest: Manifest,
    pub schemas: HashMap<String, SourceId>,
    pub path: Option<PathBuf>,
}

/// Tries to locate a bundle in the given path and its parents.
///
/// Returns [`None`] in case a bundle cannot be located.
///
/// Otherwise, the path to the bundle is returned.
pub fn try_locate_bundle(path: &Path) -> std::io::Result<Option<PathBuf>> {
    let canonical = std::fs::canonicalize(path)?;
    let mut current = Some(canonical.as_path());
    while let Some(path) = current {
        if path.join(MANIFEST_NAME).is_file() {
            return Ok(Some(path.to_owned()));
        }
        current = path.parent();
    }
    Ok(None)
}

/// Error parsing a Sidex manifest.
#[derive(Error, Debug)]
#[error("Error parsing Sidex manifest.")]
pub struct ManifestParseError {
    #[source]
    source: toml::de::Error,
}

/// Tries to parse a manifest from the provided string.
pub fn try_parse_manifest(src: &str) -> Result<Manifest, ManifestParseError> {
    toml::from_str(src).map_err(|source| ManifestParseError { source })
}

/// Error loading a Sidex manifest.
#[derive(Error, Debug)]
pub enum ManifestLoadError {
    /// Error communicating with the filesystem.
    #[error("Error communicating with the filesystem.")]
    Io(#[from] std::io::Error),
    /// Error parsing the manifest file.
    #[error("Error parsing the manifest file.")]
    Parse(#[from] ManifestParseError),
}

/// Tries to load a manifest from the provided path.
pub fn try_load_manifest(bundle_path: &Path) -> Result<Manifest, ManifestLoadError> {
    Ok(try_parse_manifest(&std::fs::read_to_string(
        bundle_path.join(MANIFEST_NAME),
    )?)?)
}

/// Iterator over the schemas based on the provided manifest path.
pub fn iter_schemas(
    bundle_path: &Path,
) -> std::io::Result<impl Iterator<Item = Result<PathBuf, std::io::Error>>> {
    let schemas_dir = bundle_path.join(SCHEMAS_DIR);
    let dir = std::fs::read_dir(&schemas_dir)?;
    Ok(dir.into_iter().filter_map(|entry| {
        entry
            .and_then(|entry| {
                let file_type = entry.file_type()?;
                let file_name = entry.file_name();
                Ok(
                    if file_type.is_file()
                        && file_name.to_string_lossy().ends_with(SCHEMA_EXTENSION)
                    {
                        Some(entry.path())
                    } else {
                        None
                    },
                )
            })
            .transpose()
    }))
}
