//! Integration tests for the `group_traits` and `group_trait_names` config
//! settings.

use std::collections::HashMap;
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

fn config(group_traits: &[&str], group_trait_names: &[(&str, &str)]) -> Config {
    Config {
        group_traits: group_traits.iter().map(|name| (*name).to_owned()).collect(),
        group_trait_names: group_trait_names
            .iter()
            .map(|(group, name)| ((*group).to_owned(), (*name).to_owned()))
            .collect::<HashMap<_, _>>(),
        ..Default::default()
    }
}

/// Generates the fixture with whitespace normalized, or returns the error
/// message.
fn generate(cfg: &Config) -> Result<String, String> {
    let (unit, bundle) = load_fixture();
    RustGenerator::new()
        .generate_macro(cfg, &unit, bundle)
        .map(|tokens| {
            tokens
                .to_string()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .map_err(|error| error.message().to_owned())
}

/// Only selected groups get a trait, and every member with a generated Rust
/// type implements it, generic definitions for all type arguments. Aliases
/// do not implement it.
#[test]
fn members_implement_selected_group_traits() {
    let out = generate(&config(&["device"], &[])).unwrap();
    assert!(out.contains("pub trait DeviceGroupMarker { }"), "{out}");
    assert!(!out.contains("AdminGroupMarker"), "{out}");
    let implementors: Vec<&str> = out
        .split("super :: DeviceGroupMarker for ")
        .skip(1)
        .map(|rest| rest.split(" {").next().unwrap())
        .collect();
    assert_eq!(
        implementors,
        ["Page < T >", "Event < >", "PublishEvents < >"]
    );
    assert!(out.contains("impl < T > super :: DeviceGroupMarker for Page < T >"));
}

/// `group_trait_names` replaces the default trait name.
#[test]
fn trait_names_can_be_overridden() {
    let out = generate(&config(&["device"], &[("device", "DeviceApi")])).unwrap();
    assert!(out.contains("pub trait DeviceApi { }"), "{out}");
    assert!(
        out.contains("super :: DeviceApi for PublishEvents"),
        "{out}"
    );
    assert!(!out.contains("DeviceGroupMarker"), "{out}");
}

/// Selecting an undeclared group or naming the trait of an unselected group is
/// an error.
#[test]
fn rejects_invalid_settings() {
    assert_eq!(
        generate(&config(&["mobile"], &[])).unwrap_err(),
        "Rust setting `group_traits` names group `mobile`, which bundle `groups_fixture` does \
         not declare."
    );
    assert_eq!(
        generate(&config(&["device"], &[("admin", "AdminApi")])).unwrap_err(),
        "Rust setting `group_trait_names` names group `admin`, which `group_traits` does not \
         select."
    );
}
