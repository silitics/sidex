//! Integration tests for the `validate` plugin: load a small fixture
//! bundle, run the Rust generator with `plugins = ["validate"]`, and
//! assert the emitted text contains the expected runtime calls.

use std::path::PathBuf;

use sidex_core::ir;
use sidex_core::transformer::Transformer;
use sidex_gen_rs::RustGenerator;
use sidex_gen_rs::config::Config;

fn load_fixture() -> (ir::Ir, ir::BundleIdx) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/validate_fixture");
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

fn render() -> String {
    let (unit, bundle) = load_fixture();
    let mut cfg = Config::default();
    cfg.plugins.push("validate".to_owned());
    // Also enable types so the bundle compiles to something inspectable.
    let tokens = RustGenerator::new()
        .generate_macro(&cfg, &unit, bundle)
        .expect("generation succeeds");
    tokens.to_string()
}

fn normalize(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn record_with_string_field_emits_size_helpers() {
    let out = normalize(&render());
    // `1 <= _.size <= 254` on a string decomposes into a min_inclusive +
    // max_inclusive call against char_count, both pointing at the `email`
    // field path. Codes are uniform `min_size` / `max_size` regardless of
    // the underlying accessor.
    assert!(
        out.contains(":: sidex_validate :: min_inclusive"),
        "expected min_inclusive call, got:\n{out}"
    );
    assert!(
        out.contains(":: sidex_validate :: max_inclusive"),
        "expected max_inclusive call, got:\n{out}"
    );
    assert!(
        out.contains(":: sidex_validate :: char_count"),
        "expected char_count accessor, got:\n{out}"
    );
    assert!(
        out.contains("\"min_size\""),
        "expected min_size code, got:\n{out}"
    );
    assert!(
        out.contains("\"max_size\""),
        "expected max_size code, got:\n{out}"
    );
}

#[test]
fn message_and_code_overrides_thread_through() {
    let out = render();
    assert!(
        out.contains("\"Bad email.\""),
        "expected user-supplied message in generated code"
    );
    assert!(
        out.contains("\"format:email\""),
        "expected user-supplied code in generated code"
    );
}

#[test]
fn regex_rule_emits_static_oncelock() {
    let out = render();
    assert!(
        out.contains("OnceLock"),
        "expected OnceLock for regex caching"
    );
    assert!(
        out.contains("regex_match"),
        "expected regex_match call, got:\n{out}"
    );
}

#[test]
fn wrapper_with_rules_emits_try_new() {
    let out = normalize(&render());
    // `Slug` has a size-range rule; codegen emits both a `Validate` impl
    // and a `try_new` constructor.
    assert!(
        out.contains("impl Slug"),
        "expected Slug impl block, got:\n{out}"
    );
    assert!(
        out.contains("pub fn try_new"),
        "expected Slug::try_new, got:\n{out}"
    );
    assert!(
        out.contains("impl :: sidex_validate :: Validate for Slug"),
        "expected Validate impl for Slug, got:\n{out}"
    );
}

#[test]
fn optional_field_validates_only_when_present() {
    let out = render();
    // `nickname?: string` with a min_size rule generates an
    // `if let Some(...)` guard so absent values bypass validation.
    assert!(
        out.contains("if let :: std :: option :: Option :: Some"),
        "expected optional-field guard, got snippet:\n{out}"
    );
}

#[test]
fn every_record_emits_a_validate_impl() {
    let out = normalize(&render());
    // Once `validate` is enabled, every record in the bundle gets a
    // `Validate` impl — types without rules get a no-op body so generic
    // call sites (`fn run<T: Validate>(t: T)`) can rely on the bound
    // uniformly.
    assert!(out.contains("impl :: sidex_validate :: Validate for User"));
    assert!(out.contains("impl :: sidex_validate :: Validate for Range"));
    assert!(out.contains("impl :: sidex_validate :: Validate for Slug"));
    assert!(out.contains("impl :: sidex_validate :: Validate for Empty"));
}

/// Verify the generated code is syntactically valid Rust so a compile
/// failure will surface here instead of at the user's `cargo build`.
#[test]
fn generated_output_is_syntactically_valid_rust() {
    let out = render();
    // The generator emits code intended to live inside a `pub mod` body;
    // wrap it so syn can parse it as a top-level file.
    let wrapped = format!("mod __probe {{ {out} }}");
    if let Err(err) = syn::parse_file(&wrapped) {
        panic!(
            "generated validate code is not syntactically valid Rust:\n  {err}\n\n\
             snippet:\n{out}"
        );
    }
}
