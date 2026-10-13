use sidex_gen::diagnostics;
use sidex_gen::Generator;

pub struct IrGenerator;

impl Generator for IrGenerator {
    fn generate(&self, job: sidex_gen::Job) -> diagnostics::Result<()> {
        let ir_file = job.output.join("ir.json");

        let json = sidex::formats::json::to_string(job.unit).map_err(|error| {
            Box::new(diagnostics::Diagnostic::error(format!(
                "Unable to encode IR: {error}"
            )))
        })?;
        std::fs::write(ir_file, json)?;

        Ok(())
    }
}
