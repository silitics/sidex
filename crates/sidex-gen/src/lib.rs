use std::path::Path;

// Re-export
pub use sidex_attrs as attrs;
pub use sidex_diagnostics as diagnostics;
// Re-export the Sidex Intermediate Representation.
pub use sidex_ir as ir;

/// A code generation job.
#[derive(Debug)]
pub struct Job<'j> {
    /// The unit to generate code for.
    pub unit: &'j ir::Collection,
    /// The bundle of the unit to generate code for.
    pub bundle: ir::BundleIdx,
    /// The output path.
    pub output: &'j Path,
    /// Configuration for the generator.
    pub config: &'j serde_json::Value,
}

/// A code generation backend.
pub trait Generator {
    /// Generates code.
    fn generate(&self, job: Job) -> Result<(), Box<dyn std::error::Error>>;
}
