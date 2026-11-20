use std::path::Path;

use sidex_core::ir;
use sidex_core::transformer::Transformer;
use sidex_gen_rs_embedded::Config;
use sidex_gen_rs_embedded::EmbeddedRustGenerator;

/// Selecting a group generates its definitions and the definitions they use,
/// and skips schemas without any.
#[test]
fn test_groups() {
    let (unit, bundle) = load_bundle();
    let config = Config {
        groups: vec!["client".to_owned()],
        ..Config::default()
    };
    let code = EmbeddedRustGenerator
        .generate_bundle(&config, &unit, bundle)
        .unwrap()
        .to_string();
    for name in ["SyncChanges", "Page", "Change", "Item", "ItemId"] {
        assert!(
            code.contains(&format!("pub struct {name} "))
                || code.contains(&format!("pub enum {name} ")),
            "{name}"
        );
    }
    assert!(code.contains("pub mod groups"));
    assert!(!code.contains("pub mod data"));
}

fn load_bundle() -> (ir::Ir, ir::BundleIdx) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/json");
    let ctx = sidex_diagnostics::DiagnosticCtx::new();
    ctx.exec(|| {
        let mut transformer = Transformer::new();
        let bundle = transformer.load_bundle_recursive(&path).unwrap();
        (transformer.transform(bundle), bundle)
    })
}
