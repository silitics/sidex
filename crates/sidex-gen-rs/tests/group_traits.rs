//! Integration tests for the `group_traits` config setting.

use std::path::PathBuf;

use sidex_core::ir;
use sidex_core::transformer::Transformer;
use sidex_gen_rs::RustGenerator;
use sidex_gen_rs::config::Config;

fn load_fixture() -> (ir::Ir, ir::BundleIdx) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/groups_fixture");
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

fn config(group_traits: &[&str]) -> Config {
    Config {
        group_traits: group_traits.iter().map(|name| (*name).to_owned()).collect(),
        ..Default::default()
    }
}

/// Only selected groups get a trait, and every member with a generated Rust
/// type implements it, generic definitions for all type arguments. Aliases
/// do not implement it.
#[test]
fn members_implement_selected_group_traits() {
    let (unit, bundle) = load_fixture();
    let out = RustGenerator::new()
        .generate_macro(&config(&["device"]), &unit, bundle)
        .expect("generation succeeds")
        .to_string();
    let out = out.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(out.contains("pub trait DeviceGroup { }"), "{out}");
    assert!(!out.contains("AdminGroup"), "{out}");
    let implementors: Vec<&str> = out
        .split("super :: DeviceGroup for ")
        .skip(1)
        .map(|rest| rest.split(" {").next().unwrap())
        .collect();
    assert_eq!(
        implementors,
        ["Page < T >", "Event < >", "PublishEvents < >"]
    );
    assert!(out.contains("impl < T > super :: DeviceGroup for Page < T >"));
}

/// Selecting a group that the bundle does not declare is an error.
#[test]
fn rejects_undeclared_groups() {
    let (unit, bundle) = load_fixture();
    let error = RustGenerator::new()
        .generate_macro(&config(&["mobile"]), &unit, bundle)
        .unwrap_err();
    assert_eq!(
        error.message(),
        "Rust setting `group_traits` names group `mobile`, which bundle `groups_fixture` does \
         not declare."
    );
}
