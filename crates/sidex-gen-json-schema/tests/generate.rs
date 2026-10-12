//! Tests for the generated JSON Schemas.

use std::path::PathBuf;
use std::sync::OnceLock;

use sidex_core::transformer::Transformer;
use sidex_gen::Generator;
use sidex_gen_json_schema::JsonSchemaGenerator;

/// Generates the schemas of the fixture bundle and returns the definitions.
fn generate() -> &'static serde_json::Value {
    static DEFS: OnceLock<serde_json::Value> = OnceLock::new();
    DEFS.get_or_init(generate_defs)
}

fn generate_defs() -> serde_json::Value {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let output = std::env::temp_dir().join(format!("sidex-gen-json-schema-{}", std::process::id()));
    std::fs::create_dir_all(&output).unwrap();
    let ctx = sidex_diagnostics::DiagnosticCtx::new();
    ctx.exec(|| {
        let mut transformer = Transformer::new();
        let bundle = transformer.load_bundle_recursive(&fixture).unwrap();
        let unit = transformer.transform(bundle);
        JsonSchemaGenerator
            .generate(sidex_gen::Job {
                unit: &unit,
                bundle,
                output: &output,
                config: &serde_json::Value::Null,
            })
            .unwrap();
    });
    let defs = std::fs::read_to_string(output.join("schema-defs.json")).unwrap();
    std::fs::remove_dir_all(&output).unwrap();
    serde_json::from_str(&defs).unwrap()
}

/// Aliases have the schema of the aliased type and their own docs.
#[test]
fn aliases() {
    let defs = generate();
    assert_eq!(
        defs["json_schema_fixture.types.Instant"],
        serde_json::json!({
            "type": "string",
            "format": "date-time",
            "description": "Instant at which something happened.",
        })
    );
}

/// Opaque types have the JSON types of their attribute.
#[test]
fn opaque_types() {
    let defs = generate();
    assert_eq!(
        defs["json_schema_fixture.types.Identifier"]["type"],
        "string"
    );
    assert_eq!(
        defs["json_schema_fixture.types.Scalar"]["type"],
        serde_json::json!(["string", "number"])
    );
}
