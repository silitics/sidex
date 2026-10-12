//! Writing and reading values in concrete formats.

mod common;

use common::Device;
use common::Scalars;
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
    assert_eq!(text, r#"{"id":"1","displayName":"Hub"}"#);
    assert_eq!(json::from_str::<Device>(&text).unwrap(), device);
    assert!(json::from_str::<Device>("{").is_err());
}

/// TOML text follows the JSON representation and round-trips.
#[test]
fn toml_text() {
    let device = Device {
        id: Some(1),
        display_name: Some("Hub".to_owned()),
    };
    let text = toml::to_string(&device).unwrap();
    assert_eq!(text, "id = \"1\"\ndisplayName = \"Hub\"\n");
    assert_eq!(toml::from_str::<Device>(&text).unwrap(), device);
}

/// TOML accepts native date-times for timestamps and cannot represent `null`.
#[test]
fn toml_specifics() {
    let scalars: Scalars = toml::from_str("timestamp = 2026-10-12T10:30:00+02:00").unwrap();
    assert_eq!(
        scalars.timestamp.unwrap().to_string(),
        "2026-10-12T08:30:00Z"
    );
    let unit = Scalars {
        unit: Some(()),
        ..Scalars::default()
    };
    assert!(toml::to_string(&unit).is_err());
}
