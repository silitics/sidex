use sidex_gen::{diagnostics, Generator};

pub struct IrGenerator;

impl Generator for IrGenerator {
    fn generate(&self, job: sidex_gen::Job) -> diagnostics::Result<()> {
        let ir_file = job.output.join("ir.json");

        std::fs::write(ir_file, serde_json::to_vec(job.unit)?)?;

        Ok(())
    }
}
