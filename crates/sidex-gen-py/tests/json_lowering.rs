//! Integration tests for the python `opaque_lowering` config flag.

use std::path::PathBuf;

use sidex_core::ir;
use sidex_core::transformer::Transformer;
use sidex_gen::Generator;
use sidex_gen::Job;
use sidex_gen_py::PyGenerator;

fn load_fixture() -> (ir::Ir, ir::BundleIdx) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let ctx = sidex_diagnostics::DiagnosticCtx::new();
    ctx.exec(|| {
        let mut transformer = Transformer::new();
        let bundle = transformer
            .load_bundle_recursive(&path)
            .expect("fixture bundle loads");
        let unit = transformer.transform(bundle);
        (unit, bundle)
    })
}

/// Runs the Python generator with the supplied JSON config object and returns
/// the contents of `data.py`.
fn render(config: serde_json::Value) -> String {
    let (unit, bundle) = load_fixture();
    let out_dir = tempfile::tempdir().expect("tempdir");
    let job = Job {
        unit: &unit,
        bundle,
        output: out_dir.path(),
        config: &config,
    };
    PyGenerator::new()
        .generate(job)
        .expect("py generation succeeds");
    std::fs::read_to_string(out_dir.path().join("data.py")).expect("data.py present")
}

#[test]
fn native_mode_uses_py_typ_attribute() {
    let out = render(serde_json::json!({}));
    // Default mode subclasses the Python type from `#[py(typ)]`.
    assert!(
        out.contains("class Uuid(uuid.UUID)"),
        "expected `class Uuid(uuid.UUID)`, got:\n{out}"
    );
}

#[test]
fn json_mode_overrides_py_typ_attribute() {
    // With opaque_lowering = "json", `#[py(typ)]` is ignored. `Uuid` has no
    // JSON attribute, so it falls back to `pydantic.JsonValue`.
    let out = render(serde_json::json!({ "opaque_lowering": "json" }));
    // The doc comment for `Uuid` mentions `uuid.UUID`, so we can't simply
    // assert the string is absent — instead check the actual emitted types.
    assert!(
        !out.contains("class Uuid(uuid.UUID)"),
        "py.typ should be ignored in JSON mode, but `class Uuid(uuid.UUID)` appeared:\n{out}"
    );
    assert!(
        !out.contains("type Uuid = uuid.UUID"),
        "py.typ should be ignored in JSON mode, but `type Uuid = uuid.UUID` appeared:\n{out}"
    );
    assert!(
        out.contains("type Uuid = pydantic.JsonValue"),
        "Uuid should alias to pydantic.JsonValue, got:\n{out}"
    );
}

#[test]
fn json_mode_lowers_string_to_str() {
    let out = render(serde_json::json!({ "opaque_lowering": "json" }));
    // `Email` becomes a `str` subclass (the existing JSON-derived lowering
    // emits Wrapper-style classes for primitive single types).
    assert!(
        out.contains("class Email(str)"),
        "Email should subclass str, got:\n{out}"
    );
}

#[test]
fn json_mode_lowers_nullable_string_to_union() {
    let out = render(serde_json::json!({ "opaque_lowering": "json" }));
    // The `string|null` union produces a `str | None` alias.
    assert!(
        out.contains("type NullableTimestamp = None | str")
            || out.contains("type NullableTimestamp = str | None"),
        "NullableTimestamp should alias to `str | None`, got:\n{out}"
    );
}

#[test]
fn json_mode_lowers_any_to_jsonvalue() {
    let out = render(serde_json::json!({ "opaque_lowering": "json" }));
    assert!(
        out.contains("type Anything = pydantic.JsonValue"),
        "Anything should alias to pydantic.JsonValue, got:\n{out}"
    );
}

#[test]
fn json_mode_lowers_multi_union_to_str_or_float() {
    let out = render(serde_json::json!({ "opaque_lowering": "json" }));
    // String + Number lowers to `str | float` (the codegen emits the union
    // verbatim — its order is whatever `types_sorted` returns).
    assert!(
        out.contains("type StringOrNumber = float | str")
            || out.contains("type StringOrNumber = str | float"),
        "StringOrNumber should alias to `str | float`, got:\n{out}"
    );
}
