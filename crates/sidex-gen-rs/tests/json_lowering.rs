//! Integration tests for the `opaque_lowering` config flag.

use std::path::PathBuf;

use sidex_core::ir;
use sidex_core::transformer::Transformer;
use sidex_gen_rs::RustGenerator;
use sidex_gen_rs::config::Config;
use sidex_gen_rs::config::OpaqueLowering;

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

fn render(cfg: Config) -> String {
    let (unit, bundle) = load_fixture();
    let tokens = RustGenerator::new()
        .generate_macro(&cfg, &unit, bundle)
        .expect("generation succeeds");
    tokens.to_string()
}

/// Strip whitespace differences so token-stream-formatted output is comparable
/// to the patterns we care about.
fn normalize(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn render_normalized(cfg: Config) -> String {
    normalize(&render(cfg))
}

#[test]
fn native_mode_uses_rust_typ_attribute() {
    let out = render_normalized(Config::default());
    // The `#[rust(typ = "uuid::Uuid")]` attribute drives the lowering.
    assert!(
        out.contains("type Uuid < > = uuid :: Uuid"),
        "expected uuid::Uuid alias, got:\n{out}"
    );
}

#[test]
fn json_mode_overrides_rust_typ_attribute() {
    // With opaque_lowering = "json", the `#[rust(typ)]` is ignored. `Uuid`
    // has no JSON attribute, so it falls back to `serde_json::Value`.
    let mut cfg = Config::default();
    cfg.opaque_lowering = OpaqueLowering::Json;
    let out = render_normalized(cfg);
    assert!(
        !out.contains("uuid :: Uuid"),
        "rust.typ should be ignored in JSON mode, but uuid::Uuid appeared:\n{out}"
    );
    assert!(
        out.contains("type Uuid < > = :: serde_json :: Value"),
        "Uuid should alias to serde_json::Value, got:\n{out}"
    );
}

#[test]
fn json_mode_lowers_string_to_native_string() {
    let mut cfg = Config::default();
    cfg.opaque_lowering = OpaqueLowering::Json;
    let out = render_normalized(cfg);
    assert!(
        out.contains("type Email < > = :: std :: string :: String"),
        "Email should alias to String, got:\n{out}"
    );
}

#[test]
fn json_mode_lowers_nullable_string_to_option() {
    let mut cfg = Config::default();
    cfg.opaque_lowering = OpaqueLowering::Json;
    let out = render_normalized(cfg);
    assert!(
        out.contains(
            "type NullableTimestamp < > = :: std :: option :: Option < :: std :: string :: String >"
        ),
        "NullableTimestamp should alias to Option<String>, got:\n{out}"
    );
}

#[test]
fn json_mode_lowers_any_to_value() {
    let mut cfg = Config::default();
    cfg.opaque_lowering = OpaqueLowering::Json;
    let out = render_normalized(cfg);
    assert!(
        out.contains("type Anything < > = :: serde_json :: Value"),
        "Anything should alias to serde_json::Value, got:\n{out}"
    );
}

#[test]
fn json_mode_lowers_multi_union_to_value() {
    let mut cfg = Config::default();
    cfg.opaque_lowering = OpaqueLowering::Json;
    let out = render_normalized(cfg);
    assert!(
        out.contains("type StringOrNumber < > = :: serde_json :: Value"),
        "StringOrNumber should fall back to serde_json::Value, got:\n{out}"
    );
}
