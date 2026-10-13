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
        assert!(obj.get("x").is_some_and(is_float), "x is f64");
        assert!(obj.get("y").is_some_and(is_float), "y is f64");
        assert_eq!(obj.len(), 2, "no extra fields");
    }
}

/// Returns whether the value represents a float.
fn is_float(value: &Value) -> bool {
    value.is_f64()
        || value.is_i64()
        || value.is_u64()
        || matches!(value.as_str(), Some("NaN" | "Infinity" | "-Infinity"))
}

#[test]
fn f32_field_wire_form_survives_f32_roundtrip() {
    // Regression: when the same float table was used for both `f32` and `f64`
    // fields, sample values like `f64::MAX` would land in an `f32` field, the
    // Rust driver would deserialize to `f32::INFINITY`, and the round-trip
    // would diverge from looser dynamic-type targets that kept f64 precision.
    // After the fix, every value drawn for an `f32` field is in `f32` range
    // *and* its JSON wire form is the f32-shortest decimal — critical because
    // the Rust driver re-serializes via `ryu_f32`, which would otherwise emit
    // a different (shorter) string than the source.
    let ir = load_fixture();
    for seed in 0..500 {
        let value = sample(&ir, "fuzz_fixture::data::F32Point", seed);
        let obj = value.as_object().unwrap();
        for key in ["x", "y"] {
            // Non-finite floats are strings.
            let Some(f) = obj.get(key).and_then(Value::as_f64) else {
                continue;
            };
            // The wire form must equal the value's f32-shortest decimal —
            // otherwise the Rust driver's deserialize-as-f32 / serialize-as-f32
            // path emits a different string and the conformance suite flags
            // it as a divergence.
            assert_eq!(
                f.to_string(),
                (f as f32).to_string(),
                "f32 field's JSON wire form differs from its f32-shortest decimal \
                 (seed {seed}, key {key})",
            );
        }
    }
}

#[test]
fn float_table_emits_nan_and_inf_as_strings() {
    let ir = load_fixture();
    let mut seen = std::collections::HashSet::new();
    for seed in 0..500 {
        let v = sample(&ir, "fuzz_fixture::data::Point", seed);
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
fn i64_includes_full_range() {
    let ir = load_fixture();
    let mut saw_max = false;
    for seed in 0..300 {
        let v = sample(&ir, "fuzz_fixture::data::BigInts", seed);
        if v.get("s").and_then(Value::as_str) == Some(&i64::MAX.to_string()) {
            saw_max = true;
        }
    }
    assert!(saw_max, "expected i64::MAX as a string");
}

#[test]
fn integers_with_64_bits_are_numbers_or_decimal_strings_beyond_2_53() {
    const MAX_SAFE: u128 = (1 << 53) - 1;
    let ir = load_fixture();
    for seed in 0..50 {
        let v = sample(&ir, "fuzz_fixture::data::BigInts", seed);
        for value in [v.get("s").unwrap(), v.get("u").unwrap()] {
            let integer = match value {
                Value::Number(number) => number.to_string().parse::<i128>().unwrap(),
                Value::String(text) => text.parse::<i128>().expect("decimal integer"),
                other => panic!("expected a number or a string, got: {other}"),
            };
            assert_eq!(
                value.is_string(),
                integer.unsigned_abs() > MAX_SAFE,
                "{value} uses the wrong representation"
            );
        }
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
