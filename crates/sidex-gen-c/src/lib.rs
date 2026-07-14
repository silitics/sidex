#![doc = include_str!("../README.md")]

use sidex_gen::Generator;
use sidex_gen::diagnostics;

/// Implements [`Generator`] for C.
///
/// The backend is still under construction. The portable runtime lives in
/// `runtime/` and is tested independently before generated code starts using it.
#[derive(Clone, Debug, Default)]
pub struct CGenerator;

impl Generator for CGenerator {
    fn generate(&self, _job: sidex_gen::Job) -> diagnostics::Result<()> {
        todo!("C code generation is not implemented yet")
    }
}
