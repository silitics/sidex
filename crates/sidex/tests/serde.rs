//! Interoperability with Serde.

mod common;

use common::Device;
use common::Scalars;
use serde::Deserialize;
use serde::Deserializer;
use sidex::Decode;
use sidex::Shaped;

/// Deserializes a Sidex type with Serde.
#[derive(Debug, PartialEq)]
struct Via<T>(T);

impl<'de, T: Shaped + Decode> Deserialize<'de> for Via<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        sidex::serde::deserialize(deserializer).map(Via)
    }
}

/// Deserialization uses shapes as hints, so that formats representing
/// everything as strings can parse scalars.
#[test]
fn deserializers_get_hints_from_shapes() {
    let Via(scalars) = serde_urlencoded::from_str::<Via<Scalars>>(
        "flag=true&small=-3&signed=-9223372036854775808&unsigned=18446744073709551615&index=7&text=hi",
    )
    .unwrap();
    assert_eq!(
        scalars,
        Scalars {
            flag: Some(true),
            small: Some(-3),
            signed: Some(i64::MIN),
            unsigned: Some(u64::MAX),
            index: Some(7),
            text: Some("hi".to_owned()),
            ..Scalars::default()
        }
    );
}

/// Serialization and deserialization use the JSON representation.
#[test]
fn json_representation() {
    let device = Device {
        id: Some(1 << 60),
        display_name: Some("Lamp".to_owned()),
    };
    let mut out = Vec::new();
    sidex::serde::serialize(&device, &mut serde_json::Serializer::new(&mut out)).unwrap();
    let text = String::from_utf8(out).unwrap();
    assert_eq!(text, r#"{"id":"1152921504606846976","displayName":"Lamp"}"#);
    let Via(decoded) = serde_json::from_str::<Via<Device>>(&text).unwrap();
    assert_eq!(decoded, device);
    // Absent optional fields may be `null`.
    let Via(decoded) =
        serde_json::from_str::<Via<Device>>(r#"{"id":"1","displayName":null}"#).unwrap();
    assert_eq!(decoded.display_name, None);
}

/// Byte size that, like `byte_calc::NumBytes`, requests an integer from the
/// deserializer, but also accepts strings.
#[derive(Debug, PartialEq)]
struct Size(u64);

impl<'de> Deserialize<'de> for Size {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = Size;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a byte size")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Size, E> {
                Ok(Size(value))
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Size, E> {
                let kibibytes = value
                    .strip_suffix("KiB")
                    .and_then(|n| n.parse::<u64>().ok());
                kibibytes
                    .map(|n| Size(n * 1024))
                    .ok_or_else(|| E::custom("invalid byte size"))
            }
        }

        deserializer.deserialize_u64(Visitor)
    }
}

/// Serde types used as Sidex types receive values like from a self-describing
/// format, so that a type requesting an integer also receives strings.
#[test]
fn serde_types_receive_any_value() {
    use sidex::serde::Serde;

    let size: Serde<Size> = sidex::formats::json::from_str(r#""4KiB""#).unwrap();
    assert_eq!(size.0, Size(4096));
    let size: Serde<Size> = sidex::formats::json::from_str("4096").unwrap();
    assert_eq!(size.0, Size(4096));
    let map: Serde<std::collections::BTreeMap<String, Option<u8>>> =
        sidex::formats::json::from_str(r#"{"a":null,"b":1}"#).unwrap();
    assert_eq!(map.0["a"], None);
    assert_eq!(map.0["b"], Some(1));
}
