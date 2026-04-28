//! Integration tests for the typescript `opaque_lowering` config flag.
//!
//! TypeScript has no per-target opaque type override today, so the
//! `opaque_lowering` flag is structural rather than behavioural — its job is
//! to keep the configuration surface symmetric across codegens. The tests
//! below assert that both modes produce the same JSON-shape lowering, while
//! verifying the lowering itself is correct.

use std::path::PathBuf;

use sidex_core::ir;
use sidex_core::transformer::Transformer;
use sidex_gen::Generator;
use sidex_gen::Job;
use sidex_gen_ts::TsGenerator;

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

fn render(config: serde_json::Value) -> String {
    let (unit, bundle) = load_fixture();
    let out_dir = tempfile::tempdir().expect("tempdir");
    let job = Job {
        unit: &unit,
        bundle,
        output: out_dir.path(),
        config: &config,
    };
    TsGenerator::new()
        .generate(job)
        .expect("ts generation succeeds");
    std::fs::read_to_string(out_dir.path().join("data.ts")).expect("data.ts present")
}

#[test]
fn json_mode_matches_native_mode_today() {
    // No per-target opaque override exists in TS, so both modes produce the
    // same output. The flag is wired so test drivers can be configured
    // uniformly across codegens.
    let native = render(serde_json::json!({}));
    let json_mode = render(serde_json::json!({ "opaque_lowering": "json" }));
    assert_eq!(native, json_mode);
}

// TS wraps every opaque in `__sidex_types.Nominal<INNER, "TAG">`. The tests
// below match against the `INNER` portion within that wrapping.

#[test]
fn opaque_without_attr_is_any() {
    let out = render(serde_json::json!({ "opaque_lowering": "json" }));
    assert!(
        out.contains("Nominal<any,"),
        "Uuid (no JSON attr) should wrap `any`, got:\n{out}"
    );
}

#[test]
fn json_string_lowers_to_string() {
    let out = render(serde_json::json!({ "opaque_lowering": "json" }));
    assert!(
        out.contains("Nominal<(string),"),
        "Email should wrap `string`, got:\n{out}"
    );
}

#[test]
fn json_nullable_string_lowers_to_union() {
    let out = render(serde_json::json!({ "opaque_lowering": "json" }));
    // Union members are sorted: Null, Boolean, Number, String, ...
    assert!(
        out.contains("Nominal<(null | string),"),
        "NullableTimestamp should wrap `null | string` (sorted), got:\n{out}"
    );
}

#[test]
fn json_any_lowers_to_any() {
    let out = render(serde_json::json!({ "opaque_lowering": "json" }));
    assert!(
        out.contains("Nominal<(any),"),
        "Anything should wrap `any`, got:\n{out}"
    );
}

#[test]
fn opaque_lowering_flag_is_accepted() {
    // Smoke test — passing the flag should not be rejected by the config
    // deserializer even though TS doesn't change its behaviour.
    render(serde_json::json!({ "opaque_lowering": "native" }));
    render(serde_json::json!({ "opaque_lowering": "json" }));
}
