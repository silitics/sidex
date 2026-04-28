//! Integration tests for the sidex-fuzz walker.
//!
//! These exercise the walker against the fixture bundle in `tests/fixtures/`,
//! loaded through the same path as the Sidex CLI. They are aimed at the
//! shape-correctness of generated values for each Sidex construct, not at
//! the statistical distribution of draws.

use std::path::PathBuf;

use serde_json::Value;
use sidex_core::ir;
use sidex_core::transformer::Transformer;
use sidex_fuzz::Config;
use sidex_fuzz::Source;
use sidex_fuzz::generate;
use sidex_fuzz::lookup_type;

fn load_fixture() -> ir::Ir {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut transformer = Transformer::new();
    let bundle = transformer
        .load_bundle_recursive(&path)
        .expect("fixture bundle loads");
    transformer.transform(bundle)
}

fn sample(ir: &ir::Ir, path: &str, seed: u64) -> Value {
    let typ = lookup_type(ir, path).unwrap_or_else(|| panic!("type `{path}` not found"));
    let mut source = Source::from_seed(seed);
    generate(ir, &typ, &mut source, &Config::default())
        .unwrap_or_else(|err| panic!("generation failed for `{path}`: {err:?}"))
}

#[test]
fn deterministic_for_same_seed() {
    let ir = load_fixture();
    let a = sample(&ir, "fuzz_fixture::data::Point", 42);
    let b = sample(&ir, "fuzz_fixture::data::Point", 42);
    assert_eq!(a, b);
}

#[test]
fn point_has_two_floats() {
    let ir = load_fixture();
    for seed in 0..10 {
        let value = sample(&ir, "fuzz_fixture::data::Point", seed);
        let obj = value.as_object().expect("Point is an object");
        assert!(obj.get("x").and_then(Value::as_f64).is_some(), "x is f64");
        assert!(obj.get("y").and_then(Value::as_f64).is_some(), "y is f64");
        assert_eq!(obj.len(), 2, "no extra fields");
    }
}

#[test]
fn float_table_finite_only_by_default() {
    // Without `floats_as_strings`, every Point coordinate is a finite number.
    let ir = load_fixture();
    for seed in 0..200 {
        let value = sample(&ir, "fuzz_fixture::data::Point", seed);
        let obj = value.as_object().unwrap();
        for key in ["x", "y"] {
            let f = obj.get(key).and_then(Value::as_f64).unwrap();
            assert!(
                f.is_finite(),
                "default mode emits only finite floats, got {f}"
            );
        }
    }
}

#[test]
fn float_table_emits_nan_and_inf_as_strings_when_enabled() {
    // With `floats_as_strings`, ±Infinity and NaN appear as JSON strings.
    let ir = load_fixture();
    let typ = sidex_fuzz::lookup_type(&ir, "fuzz_fixture::data::Point").unwrap();
    let mut config = sidex_fuzz::Config::default();
    config.floats_as_strings = true;
    let mut seen = std::collections::HashSet::new();
    for seed in 0..500 {
        let mut source = sidex_fuzz::Source::from_seed(seed);
        let v = sidex_fuzz::generate(&ir, &typ, &mut source, &config).unwrap();
        for key in ["x", "y"] {
            if let Some(s) = v.get(key).and_then(Value::as_str) {
                seen.insert(s.to_owned());
            }
        }
    }
    assert!(seen.contains("NaN"), "expected NaN string, saw {seen:?}");
    assert!(seen.contains("Infinity"), "expected Infinity, saw {seen:?}");
    assert!(
        seen.contains("-Infinity"),
        "expected -Infinity, saw {seen:?}"
    );
}

#[test]
fn optional_fields_sometimes_missing() {
    let ir = load_fixture();
    let mut saw_missing = false;
    let mut saw_present = false;
    for seed in 0..50 {
        let value = sample(&ir, "fuzz_fixture::data::OptFields", seed);
        let obj = value.as_object().unwrap();
        assert!(
            obj.contains_key("required"),
            "required field always present"
        );
        if obj.contains_key("maybe") {
            saw_present = true;
        } else {
            saw_missing = true;
        }
    }
    assert!(saw_missing, "expected at least one sample without `maybe`");
    assert!(saw_present, "expected at least one sample with `maybe`");
}

#[test]
fn externally_tagged_unit_is_string() {
    let ir = load_fixture();
    let mut saw_unit_string = false;
    let mut saw_data_object = false;
    for seed in 0..30 {
        let value = sample(&ir, "fuzz_fixture::data::ExternallyTagged", seed);
        match value {
            Value::String(s) => {
                assert_eq!(s, "Unit");
                saw_unit_string = true;
            }
            Value::Object(map) => {
                assert!(map.contains_key("Data"));
                assert_eq!(map.len(), 1);
                saw_data_object = true;
            }
            other => panic!("unexpected variant shape: {other}"),
        }
    }
    assert!(saw_unit_string && saw_data_object);
}

#[test]
fn adjacently_tagged_uses_custom_field_names() {
    let ir = load_fixture();
    for seed in 0..30 {
        let value = sample(&ir, "fuzz_fixture::data::AdjacentlyTagged", seed);
        let obj = value.as_object().unwrap();
        let tag = obj.get("kind").and_then(Value::as_str).unwrap();
        assert!(tag == "Unit" || tag == "Data");
        if tag == "Data" {
            assert!(obj.contains_key("data"));
        }
    }
}

#[test]
fn internally_tagged_inlines_record_payload() {
    let ir = load_fixture();
    let mut saw_record_inline = false;
    for seed in 0..30 {
        let value = sample(&ir, "fuzz_fixture::data::InternallyTagged", seed);
        let obj = value.as_object().unwrap();
        let tag = obj.get("kind").and_then(Value::as_str).unwrap();
        if tag == "AsRecord" {
            assert!(obj.contains_key("x"), "x inlined alongside tag");
            assert!(obj.contains_key("y"), "y inlined alongside tag");
            saw_record_inline = true;
        }
    }
    assert!(saw_record_inline);
}

#[test]
fn implicitly_tagged_emits_just_payload() {
    let ir = load_fixture();
    let mut saw_string = false;
    let mut saw_bool = false;
    for seed in 0..30 {
        let value = sample(&ir, "fuzz_fixture::data::ImplicitlyTagged", seed);
        match value {
            Value::String(_) => saw_string = true,
            Value::Bool(_) => saw_bool = true,
            other => panic!("expected string or bool, got: {other}"),
        }
    }
    assert!(saw_string && saw_bool);
}

#[test]
fn i64_default_mode_stays_in_safe_range() {
    // Without `integers_as_strings`, the table is restricted to JS-safe
    // values so all targets (including JS) round-trip without precision loss.
    // `±2^53` is the boundary and must appear; `i64::MAX` must NOT.
    let ir = load_fixture();
    let mut saw_boundary = false;
    for seed in 0..300 {
        let value = sample(&ir, "fuzz_fixture::data::BigInts", seed);
        let s = value.get("s").and_then(Value::as_i64).unwrap();
        assert!(
            s.abs() <= 1i64 << 53,
            "default-mode i64 must stay in JS-safe range, got {s}"
        );
        if s == 1i64 << 53 || s == -(1i64 << 53) {
            saw_boundary = true;
        }
    }
    assert!(saw_boundary, "expected ±2^53 to appear in i64 draws");
}

#[test]
fn i64_strings_mode_includes_full_range() {
    // With `integers_as_strings`, `i64::MAX` and friends DO appear, encoded
    // as JSON strings rather than numbers.
    let ir = load_fixture();
    let typ = sidex_fuzz::lookup_type(&ir, "fuzz_fixture::data::BigInts").unwrap();
    let mut config = sidex_fuzz::Config::default();
    config.integers_as_strings = true;
    let mut saw_max = false;
    for seed in 0..300 {
        let mut source = sidex_fuzz::Source::from_seed(seed);
        let v = sidex_fuzz::generate(&ir, &typ, &mut source, &config).unwrap();
        let s_str = v.get("s").and_then(Value::as_str).unwrap();
        if s_str == i64::MAX.to_string() {
            saw_max = true;
        }
    }
    assert!(saw_max, "expected i64::MAX as a string in strings mode");
}

#[test]
fn integers_as_strings_emits_decimal_strings() {
    let ir = load_fixture();
    let typ = sidex_fuzz::lookup_type(&ir, "fuzz_fixture::data::BigInts").unwrap();
    let mut config = sidex_fuzz::Config::default();
    config.integers_as_strings = true;
    for seed in 0..50 {
        let mut source = sidex_fuzz::Source::from_seed(seed);
        let v = sidex_fuzz::generate(&ir, &typ, &mut source, &config).unwrap();
        let s = v.get("s").unwrap();
        let u = v.get("u").unwrap();
        assert!(s.is_string(), "i64 field should be a string, got {s}");
        assert!(u.is_string(), "u64 field should be a string, got {u}");
        // String must parse back as a signed/unsigned integer.
        s.as_str().unwrap().parse::<i64>().expect("decimal i64");
        u.as_str().unwrap().parse::<u64>().expect("decimal u64");
    }
}

#[test]
fn opaque_with_json_attr_uses_declared_type() {
    let ir = load_fixture();
    for seed in 0..20 {
        let value = sample(&ir, "fuzz_fixture::data::UuidString", seed);
        assert!(value.is_string(), "UuidString declared as string-typed");
    }
}

#[test]
fn opaque_without_json_attr_falls_back_to_any() {
    // Without a `#[json(type = ...)]` attribute, the opaque accepts any JSON
    // value, so the walker should emit a mix of shapes across seeds.
    let ir = load_fixture();
    let mut shapes = std::collections::HashSet::new();
    for seed in 0..50 {
        let value = sample(&ir, "fuzz_fixture::data::Uuid", seed);
        shapes.insert(std::mem::discriminant(&value));
    }
    assert!(
        shapes.len() >= 2,
        "free-form opaque should produce multiple JSON shapes"
    );
}
