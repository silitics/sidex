//! Internal utility functions.

use std::{
    env, io,
    path::{Path, PathBuf},
    process,
};

/// Emits a `cargo:rerun-if-changed` directive for the provided path.
pub(crate) fn emit_rerun_if_changed<P: AsRef<Path>>(path: P) {
    #[doc(hidden)]
    fn inner(path: &Path) {
        println!("cargo:rerun-if-changed={}", path.to_string_lossy())
    }
    inner(path.as_ref())
}

/// Runs `rustfmt` on the provided path.
pub(crate) fn run_rustfmt<P: AsRef<Path>>(path: P) -> io::Result<process::ExitStatus> {
    #[doc(hidden)]
    fn inner(path: &Path) -> io::Result<process::ExitStatus> {
        process::Command::new("rustfmt").arg(path).spawn()?.wait()
    }
    inner(path.as_ref())
}

/// Returns the default output path obtained from the `OUT_DIR` environment variable.
pub(crate) fn default_output_path() -> PathBuf {
    let out_dir = env::var_os("OUT_DIR").expect("`OUT_DIR` environment variable not set.");
    PathBuf::from(out_dir).join("sidex-bundles")
}

/// Returns the default root path obtained from the `CARGO_MANIFEST_DIR` environment
/// variable.
pub(crate) fn default_root_path() -> PathBuf {
    env::var_os("CARGO_MANIFEST_DIR")
        .expect("`CARGO_MANIFEST_DIR` environment variable not set.")
        .into()
}
