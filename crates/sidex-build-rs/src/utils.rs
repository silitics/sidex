//! Utility functions.

use std::path::Path;

/// Emits a `cargo:rerun-if-changed` directive for the provided path.
pub(crate) fn emit_cargo_rerun_if_changed<P: AsRef<Path>>(path: P) {
    println!("cargo:rerun-if-changed={}", path.as_ref().to_string_lossy())
}
