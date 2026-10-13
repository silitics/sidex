//! Integration tests for validation rules: load a small fixture bundle, run
//! the Rust generator, and check the rules in the generated shapes.

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
    let tokens = RustGenerator::new()
        .generate_macro(&Config::default(), &unit, bundle)
        .expect("generation succeeds");
    normalize(&tokens.to_string())
}

fn normalize(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn chained_comparisons_become_compare_rules() {
    let out = render();
    assert!(
        out.contains(
            "Rule :: new (\"1 <= _.size <= 254\" , :: sidex :: validation :: Expr :: Compare { \
             operands : & [:: sidex :: validation :: Operand :: Int (1i128) , \
             :: sidex :: validation :: Operand :: Member (& [\"size\"]) , \
             :: sidex :: validation :: Operand :: Int (254i128)] , \
             ops : & [:: sidex :: validation :: CmpOp :: Le , :: sidex :: validation :: CmpOp :: Le] , })"
        ),
        "expected the size rule of `email`, got:\n{out}"
    );
}

#[test]
fn matches_becomes_a_matches_rule_with_overrides() {
    let out = render();
    assert!(
        out.contains(
            ":: sidex :: validation :: Expr :: Matches { operand : \
             :: sidex :: validation :: Operand :: Value , pattern : \"^[^@]+@[^@]+$\" }) \
             . with_message (\"Bad email.\") . with_code (\"format:email\")"
        ),
        "expected the regex rule of `email`, got:\n{out}"
    );
}

#[test]
fn record_rules_compare_members() {
    let out = render();
    assert!(
        out.contains(
            "operands : & [:: sidex :: validation :: Operand :: Member (& [\"min\"]) , \
             :: sidex :: validation :: Operand :: Member (& [\"max\"])]"
        ),
        "expected the record rule of `Range`, got:\n{out}"
    );
}

#[test]
fn wrapper_rules_are_part_of_the_wrapper_shape() {
    let out = render();
    assert!(
        out.contains(
            "ShapeKind :: Wrapper (:: sidex :: shape :: shape_of :: < :: sidex :: types :: Str >)) \
             . with_attrs"
        ),
        "expected the shape of `Slug`, got:\n{out}"
    );
    assert!(
        out.contains(
            ". with_rules (& [:: sidex :: validation :: Rule :: new (\"1 <= _.size <= 64\""
        )
    );
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
        panic!("generated code is not syntactically valid Rust:\n  {err}\n\nsnippet:\n{out}");
    }
}
