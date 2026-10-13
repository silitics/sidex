//! Writing and reading values in concrete formats.

mod common;

use common::Device;
use common::Scalars;
use sidex::JsonValue;
use sidex::formats::json;
use sidex::formats::toml;

/// JSON text follows the JSON representation and round-trips.
#[test]
fn json_text() {
    let device = Device {
        id: Some(1),
        display_name: Some("Hub".to_owned()),
    };
    let text = json::to_string(&device).unwrap();
    assert_eq!(text, r#"{"id":1,"displayName":"Hub"}"#);
    assert_eq!(json::from_str::<Device>(&text).unwrap(), device);
    assert!(json::from_str::<Device>("{").is_err());
}

/// Strict JSON reading rejects unknown fields.
#[test]
fn json_strict() {
    let text = r#"{"id":1,"color":"red"}"#;
    assert!(json::from_str::<Device>(text).is_ok());
    let error = json::from_str_strict::<Device>(text).unwrap_err();
    assert!(error.is_unknown_field());
    assert_eq!(error.path().to_string(), "color");
}

/// TOML text follows the JSON representation and round-trips.
#[test]
fn toml_text() {
    let device = Device {
        id: Some(1),
        display_name: Some("Hub".to_owned()),
    };
    let text = toml::to_string(&device).unwrap();
    assert_eq!(text, "id = 1\ndisplayName = \"Hub\"\n");
    assert_eq!(toml::from_str::<Device>(&text).unwrap(), device);
}

/// Strict TOML reading rejects unknown fields.
#[test]
fn toml_strict() {
    let text = "id = 1\ncolor = \"red\"\n";
    assert!(toml::from_str::<Device>(text).is_ok());
    let error = toml::from_str_strict::<Device>(text).unwrap_err();
    assert_eq!(error.to_string(), "color: unknown field `color`");
}

/// Reading text with unknown fields accepts them and returns their paths.
#[test]
fn unknown_fields() {
    let text = r#"{"id":1,"color":"red"}"#;
    let (device, paths) = json::from_str_with_unknown::<Device>(text).unwrap();
    assert_eq!(device.id, Some(1));
    assert_eq!(paths[0].to_string(), "color");
    let (_, paths) = toml::from_str_with_unknown::<Device>("id = 1\ncolor = \"red\"\n").unwrap();
    assert_eq!(paths[0].to_string(), "color");
}

/// TOML accepts native date-times for timestamps.
#[test]
fn toml_timestamps() {
    let scalars: Scalars = toml::from_str("timestamp = 2026-10-12T10:30:00+02:00").unwrap();
    assert_eq!(
        scalars.timestamp.unwrap().to_string(),
        "2026-10-12T08:30:00Z"
    );
}

/// TOML omits map entries whose value is `null`, cannot represent `null` in
/// arrays, and writes integers beyond its 64-bit range as decimal strings.
#[test]
fn toml_json_values() {
    let value = JsonValue::from_iter([
        ("big", JsonValue::from(u64::MAX)),
        ("none", JsonValue::Null),
    ]);
    assert_eq!(
        toml::to_string(&value).unwrap(),
        "big = \"18446744073709551615\"\n"
    );
    let array = JsonValue::from_iter([("items", JsonValue::from(vec![JsonValue::Null]))]);
    assert_eq!(
        toml::to_string(&array).unwrap_err().to_string(),
        "TOML has no representation of `null` in arrays"
    );
}
