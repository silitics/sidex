//! Tests for the generated JSON Schemas.

use std::path::PathBuf;
use std::sync::OnceLock;

use sidex::Shaped;
use sidex::formats::json;
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
    let mut expected = json::schema(sidex::types::Timestamp::SHAPE);
    if let sidex::JsonValue::Map(entries) = &mut expected {
        entries.insert(
            "description".into(),
            "Instant at which something happened.".into(),
        );
    }
    assert_eq!(
        defs["json_schema_fixture.types.Instant"],
        serde_json::Value::from(expected),
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

/// 64-bit integers are integers or, beyond 2^53 - 1, decimal strings.
#[test]
fn integers() {
    let properties = &generate()["json_schema_fixture.types.Event"]["properties"];
    assert_eq!(
        properties["sequence"],
        serde_json::Value::from(json::schema(u64::SHAPE)),
    );
}

/// Runtime and generated schemas agree for every built-in and nested containers.
#[test]
fn builtin_schema_parity() {
    let properties = &generate()["json_schema_fixture.types.Builtins"]["properties"];
    for (name, shape) in [
        ("boolean", bool::SHAPE),
        ("signed8", i8::SHAPE),
        ("signed16", i16::SHAPE),
        ("signed32", i32::SHAPE),
        ("signed64", i64::SHAPE),
        ("unsigned8", u8::SHAPE),
        ("unsigned16", u16::SHAPE),
        ("unsigned32", u32::SHAPE),
        ("unsigned64", u64::SHAPE),
        ("index", usize::SHAPE),
        ("float32", f32::SHAPE),
        ("float64", f64::SHAPE),
        ("text", String::SHAPE),
        ("binary", sidex::types::Bytes::SHAPE),
        ("identifier", sidex::types::Uuid::SHAPE),
        ("instant", sidex::types::Timestamp::SHAPE),
        ("calendar", sidex::types::Date::SHAPE),
        ("arbitrary", sidex::JsonValue::SHAPE),
        ("sequence", Vec::<u64>::SHAPE),
        ("map", sidex::types::Map::<String, i64>::SHAPE),
        ("integerKeys", sidex::types::Map::<u64, String>::SHAPE),
        ("signedIntegerKeys", sidex::types::Map::<i64, String>::SHAPE),
        ("aliasedKeys", sidex::types::Map::<u8, String>::SHAPE),
        ("wrappedKeys", sidex::types::Map::<u16, String>::SHAPE),
        (
            "nested",
            sidex::types::Map::<String, Vec<sidex::JsonValue>>::SHAPE,
        ),
    ] {
        assert_eq!(
            properties[name],
            serde_json::Value::from(json::schema(shape)),
            "schema of {name}",
        );
    }
    for (name, format) in [
        ("signed32", "int32"),
        ("unsigned32", "uint32"),
        ("signed64", "int64"),
        ("unsigned64", "uint64"),
        ("identifier", "uuid"),
        ("instant", "date-time"),
        ("calendar", "date"),
    ] {
        assert_eq!(properties[name]["format"], format, "format of {name}");
    }
}
