//! Round trips through the JSON representation with types written the way Sidex
//! generates them.

mod common;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::marker::PhantomData;

use common::Device;
use common::Fleet;
use common::Indexes;
use common::Located;
use common::Point;
use common::Scalars;
use common::record;
use sidex::Decode;
use sidex::DecodeError;
use sidex::Decoder;
use sidex::Encode;
use sidex::JsonValue;
use sidex::Shape;
use sidex::Shaped;
use sidex::View;
use sidex::encode::VariantView;
use sidex::formats::json;
use sidex::shape::CaseShape;
use sidex::shape::OpaqueShape;
use sidex::shape::ShapeKind;
use sidex::shape::VariantShape;
use sidex::shape::shape_of;
use sidex::types::Bytes;
use sidex::types::Uuid;

/// Parses JSON text into a JSON value.
fn parse(text: &str) -> JsonValue {
    sidex::formats::json::from_str(text).unwrap()
}

/// Asserts that `value` encodes as `expected` and decodes from it unchanged.
fn assert_roundtrip<T: Encode + Decode + PartialEq + std::fmt::Debug>(value: &T, expected: &str) {
    let encoded = json::to_value(value).unwrap();
    assert_eq!(encoded, parse(expected));
    assert_eq!(&json::from_value::<T>(&encoded).unwrap(), value);
}

/// Every scalar uses its specified representation and round-trips exactly.
#[test]
fn scalars() {
    let scalars = Scalars {
        flag: Some(true),
        small: Some(-7),
        signed: Some(i64::MIN),
        unsigned: Some(u64::MAX),
        index: Some(3),
        single: Some(0.1),
        double: Some(f64::NEG_INFINITY),
        text: Some("text".to_owned()),
        bytes: Some(Bytes::from(vec![0, 1, 255])),
        uuid: Some("67e55044-10b1-426f-9247-bb680e5fe0c8".parse().unwrap()),
        timestamp: Some("2026-10-12T08:30:00.25Z".parse().unwrap()),
        date: Some("2026-10-12".parse().unwrap()),
    };
    assert_roundtrip(
        &scalars,
        r#"{
            "flag": true,
            "small": -7,
            "signed": "-9223372036854775808",
            "unsigned": "18446744073709551615",
            "index": 3,
            "single": 0.1,
            "double": "-Infinity",
            "text": "text",
            "bytes": "AAH/",
            "uuid": "67e55044-10b1-426f-9247-bb680e5fe0c8",
            "timestamp": "2026-10-12T08:30:00.25Z",
            "date": "2026-10-12"
        }"#,
    );
}

/// 64-bit integers are numbers up to a magnitude of 2^53 - 1 and decimal
/// strings beyond.
#[test]
fn int64_safe_range() {
    const MAX_SAFE: i64 = (1 << 53) - 1;
    assert_roundtrip(&MAX_SAFE, "9007199254740991");
    assert_roundtrip(&-MAX_SAFE, "-9007199254740991");
    assert_roundtrip(&(MAX_SAFE + 1), r#""9007199254740992""#);
    assert_roundtrip(&(-MAX_SAFE - 1), r#""-9007199254740992""#);
    assert_roundtrip(&(MAX_SAFE as u64), "9007199254740991");
    assert_roundtrip(&(MAX_SAFE as u64 + 1), r#""9007199254740992""#);
}

/// Decoding accepts the alternative inputs listed in the representation.
#[test]
fn lenient_inputs() {
    let decoded: Scalars = json::from_value(&parse(
        r#"{
            "signed": "-5",
            "unsigned": "5",
            "single": 1,
            "double": "+Infinity",
            "bytes": "AAE",
            "uuid": "67E55044-10B1-426F-9247-BB680E5FE0C8",
            "timestamp": "2026-10-12T10:30:00+02:00"
        }"#,
    ))
    .unwrap();
    assert_eq!(decoded.signed, Some(-5));
    assert_eq!(decoded.unsigned, Some(5));
    assert_eq!(decoded.single, Some(1.0));
    assert_eq!(decoded.double, Some(f64::INFINITY));
    assert_eq!(decoded.bytes, Some(Bytes::from(vec![0, 1])));
    assert_eq!(
        decoded.uuid.unwrap().to_string(),
        "67e55044-10b1-426f-9247-bb680e5fe0c8"
    );
    assert_eq!(
        decoded.timestamp.unwrap().to_string(),
        "2026-10-12T08:30:00Z"
    );
}

/// Optional fields are omitted when absent, accept `null`, and unknown fields
/// are ignored.
#[test]
fn optional_and_unknown_fields() {
    assert_roundtrip(
        &Device {
            id: Some(1),
            display_name: None,
        },
        r#"{"id":1}"#,
    );
    let decoded: Device =
        json::from_value(&parse(r#"{"id":1,"displayName":null,"color":"red"}"#)).unwrap();
    assert_eq!(
        decoded,
        Device {
            id: Some(1),
            display_name: None
        }
    );
}

/// Errors name the path to the offending item.
#[test]
fn error_paths() {
    let error =
        json::from_value::<Fleet>(&parse(r#"{"devices":[{"id":"1"},{"id":true}]}"#)).unwrap_err();
    assert_eq!(
        error.to_string(),
        "devices[1].id: expected a 64-bit unsigned integer, found a boolean"
    );
    let error = json::from_value::<Fleet>(&parse(r#"{"devices":[{}]}"#)).unwrap_err();
    assert_eq!(error.to_string(), "devices[0]: missing field `id`");
}

/// Inline fields merge into the enclosing object.
#[test]
fn inline_fields() {
    assert_roundtrip(
        &Located {
            name: Some("home".to_owned()),
            position: Some(Point {
                x: Some(1),
                y: Some(2),
            }),
        },
        r#"{"name":"home","x":1,"y":2}"#,
    );
}

/// Strict decoding rejects unknown fields, also of inline records and of
/// variant payloads, but accepts the tag of internally tagged variants.
#[test]
fn strict_decoding() {
    let error =
        json::from_value_strict::<Fleet>(&parse(r#"{"devices":[{"id":"1","color":"red"}]}"#))
            .unwrap_err();
    assert_eq!(error.to_string(), "devices[0].color: unknown field `color`");
    assert!(error.is_unknown_field());
    let error =
        json::from_value_strict::<Fleet>(&parse(r#"{"devices":[{"id":"x"}]}"#)).unwrap_err();
    assert!(!error.is_unknown_field());

    json::from_value_strict::<Located>(&parse(r#"{"name":"home","x":1,"y":2}"#)).unwrap();
    let error = json::from_value_strict::<Located>(&parse(r#"{"name":"home","x":1,"y":2,"z":3}"#))
        .unwrap_err();
    assert_eq!(error.to_string(), "z: unknown field `z`");

    json::from_value_strict::<Position<Internal>>(&parse(r#"{"tag":"Coordinates","x":1,"y":2}"#))
        .unwrap();
    let error = json::from_value_strict::<Position<Adjacent>>(&parse(
        r#"{"tag":"Coordinates","content":{"x":1,"y":2,"z":3}}"#,
    ))
    .unwrap_err();
    assert_eq!(error.to_string(), "content.z: unknown field `z`");
}

/// Schemas describe records with their JSON names and merged inline fields,
/// and variants with their taggings.
#[test]
fn schemas() {
    let schema = json::schema(<Located as Shaped>::SHAPE);
    assert_eq!(
        sidex::formats::json::to_string(&schema).unwrap(),
        r#"{"type":"object","properties":{"name":{"type":"string"},"x":{"type":"integer","format":"int32","minimum":-2147483648,"maximum":2147483647},"y":{"type":"integer","format":"int32","minimum":-2147483648,"maximum":2147483647}},"required":["name","x","y"]}"#
    );
    let schema = json::schema(<Position<Internal> as Shaped>::SHAPE);
    let alternatives = schema.get("oneOf").and_then(JsonValue::as_seq).unwrap();
    assert_eq!(
        sidex::formats::json::to_string(&alternatives[1]).unwrap(),
        r#"{"type":"object","properties":{"tag":{"const":"Named"},"content":{"type":"string"}},"required":["tag","content"]}"#
    );
}

/// Map keys use the text representation of their type.
#[test]
fn map_keys() {
    let uuid: Uuid = "67e55044-10b1-426f-9247-bb680e5fe0c8".parse().unwrap();
    assert_roundtrip(
        &Indexes {
            by_id: Some(BTreeMap::from([(u64::MAX, "max".to_owned())])),
            by_uuid: Some(BTreeMap::from([(uuid, 1)])),
        },
        r#"{
            "byId": {"18446744073709551615": "max"},
            "byUuid": {"67e55044-10b1-426f-9247-bb680e5fe0c8": 1}
        }"#,
    );
}

/// JSON values are represented as themselves, also integers beyond 2^53.
#[test]
fn json_values() {
    let value = JsonValue::from_iter([
        ("int", JsonValue::Int(i128::from(u64::MAX))),
        ("none", JsonValue::Null),
    ]);
    assert_eq!(
        json::to_string(&value).unwrap(),
        r#"{"int":18446744073709551615,"none":null}"#
    );
    assert!(json::to_value(&JsonValue::Float(f64::NAN)).is_err());
}

/// Selects the JSON representation of [`Position`]'s cases.
trait Tagging: 'static {
    const VARIANT: VariantShape;
}

/// Variant `Position { Unknown, Named: string, Coordinates: Point }` with the
/// tagging selected by `T`.
#[derive(Debug, PartialEq)]
enum Position<T> {
    Unknown,
    Named(String),
    Coordinates(Point),
    #[allow(dead_code)]
    Tagging(PhantomData<T>),
}

impl<T: Tagging> Shaped for Position<T> {
    const SHAPE: &'static Shape = &Shape::new("::tests::Position", ShapeKind::Variant(T::VARIANT));
}

impl<T: Tagging> Encode for Position<T> {
    fn encode(&self) -> View<'_> {
        View::Variant(self)
    }
}

impl<T: Tagging> VariantView for Position<T> {
    fn shape(&self) -> &'static Shape {
        Self::SHAPE
    }

    fn case(&self) -> usize {
        match self {
            Position::Unknown => 0,
            Position::Named(_) => 1,
            Position::Coordinates(_) => 2,
            Position::Tagging(_) => unreachable!(),
        }
    }

    fn payload(&self) -> Option<View<'_>> {
        match self {
            Position::Named(name) => Some(name.encode()),
            Position::Coordinates(point) => Some(point.encode()),
            _ => None,
        }
    }
}

impl<T: Tagging> Decode for Position<T> {
    fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError> {
        let mut out = None;
        decoder.variant(Self::SHAPE, &mut |case, payload| {
            out = Some(match case {
                0 => Position::Unknown,
                1 => Position::Named(String::decode(payload)?),
                _ => Position::Coordinates(Point::decode(payload)?),
            });
            Ok(())
        })?;
        out.ok_or_else(|| DecodeError::new("no case decoded"))
    }
}

/// Cases of [`Position`].
const CASES: &[CaseShape] = &[
    CaseShape::new("Unknown", None),
    CaseShape::new("Named", Some(shape_of::<String>)),
    CaseShape::new("Coordinates", Some(shape_of::<Point>)),
];

#[derive(Debug, PartialEq)]
struct Internal;

impl Tagging for Internal {
    const VARIANT: VariantShape = VariantShape::new(CASES);
}

#[derive(Debug, PartialEq)]
struct Adjacent;

impl Tagging for Adjacent {
    const VARIANT: VariantShape = VariantShape::new(CASES).json_tagging(json::Tagging::Adjacent {
        tag: "tag",
        content: "content",
    });
}

#[derive(Debug, PartialEq)]
struct External;

impl Tagging for External {
    const VARIANT: VariantShape = VariantShape::new(CASES).json_tagging(json::Tagging::External);
}

#[derive(Debug, PartialEq)]
struct Implicit;

impl Tagging for Implicit {
    const VARIANT: VariantShape = VariantShape::new(CASES).json_tagging(json::Tagging::Implicit);
}

/// Asserts the representations of [`Position`]'s cases under tagging `T`.
fn assert_tagging<T: Tagging + std::fmt::Debug + PartialEq>(
    unknown: &str,
    named: &str,
    coordinates: &str,
) {
    assert_roundtrip(&Position::<T>::Unknown, unknown);
    assert_roundtrip(&Position::<T>::Named("home".to_owned()), named);
    assert_roundtrip(
        &Position::<T>::Coordinates(Point {
            x: Some(1),
            y: Some(2),
        }),
        coordinates,
    );
}

/// The four taggings produce the representations from the format's
/// documentation.
#[test]
fn taggings() {
    assert_tagging::<Internal>(
        r#"{"tag":"Unknown"}"#,
        r#"{"tag":"Named","content":"home"}"#,
        r#"{"tag":"Coordinates","x":1,"y":2}"#,
    );
    assert_tagging::<Adjacent>(
        r#"{"tag":"Unknown"}"#,
        r#"{"tag":"Named","content":"home"}"#,
        r#"{"tag":"Coordinates","content":{"x":1,"y":2}}"#,
    );
    assert_tagging::<External>(
        r#""Unknown""#,
        r#"{"Named":"home"}"#,
        r#"{"Coordinates":{"x":1,"y":2}}"#,
    );
    assert_tagging::<Implicit>(r#""Unknown""#, r#""home""#, r#"{"x":1,"y":2}"#);
}

/// Internal tags may follow the payload's fields, and unknown cases are named.
#[test]
fn internal_tag_position_and_unknown_cases() {
    let decoded: Position<Internal> =
        json::from_value(&parse(r#"{"x":1,"y":2,"tag":"Coordinates"}"#)).unwrap();
    assert_eq!(
        decoded,
        Position::Coordinates(Point {
            x: Some(1),
            y: Some(2)
        })
    );
    let error =
        json::from_value::<Position<Internal>>(&parse(r#"{"tag":"Elsewhere"}"#)).unwrap_err();
    assert_eq!(
        error.to_string(),
        "tag: unknown case `Elsewhere` of `::tests::Position`"
    );
}

/// Implicit tagging selects the first case that matches, in declaration order.
#[test]
fn implicit_tagging_selects_first_match() {
    /// Variant `Either { Number: u64, Text: string }` with implicit tagging.
    #[derive(Debug, PartialEq)]
    enum Either {
        Number(u64),
        Text(String),
    }

    impl Shaped for Either {
        const SHAPE: &'static Shape = &Shape::new(
            "::tests::Either",
            ShapeKind::Variant(
                VariantShape::new(&[
                    CaseShape::new("Number", Some(shape_of::<u64>)),
                    CaseShape::new("Text", Some(shape_of::<String>)),
                ])
                .json_tagging(json::Tagging::Implicit),
            ),
        );
    }

    impl Decode for Either {
        fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError> {
            let mut out = None;
            decoder.variant(Self::SHAPE, &mut |case, payload| {
                out = Some(match case {
                    0 => Either::Number(u64::decode(payload)?),
                    _ => Either::Text(String::decode(payload)?),
                });
                Ok(())
            })?;
            out.ok_or_else(|| DecodeError::new("no case decoded"))
        }
    }

    let decode = |text| json::from_value::<Either>(&parse(text));
    assert_eq!(decode("42").unwrap(), Either::Number(42));
    assert_eq!(decode(r#""42""#).unwrap(), Either::Number(42));
    assert_eq!(
        decode(r#""forty-two""#).unwrap(),
        Either::Text("forty-two".to_owned())
    );
    assert_eq!(
        decode("true").unwrap_err().to_string(),
        "no case of `::tests::Either` matches a boolean \
         (`Number`: expected a 64-bit unsigned integer, found a boolean; \
         `Text`: expected a string, found a boolean)"
    );
}

/// Opaque type represented by a string or `null`, depending on `NULLABLE`.
#[derive(Debug, PartialEq)]
pub struct Label<const NULLABLE: bool>(Option<String>);

impl<const NULLABLE: bool> Shaped for Label<NULLABLE> {
    const SHAPE: &'static Shape = &Shape::new(
        "::tests::Label",
        ShapeKind::Opaque(OpaqueShape::new().json_types(if NULLABLE {
            &[json::Type::String, json::Type::Null]
        } else {
            &[json::Type::String]
        })),
    );
}

impl<const NULLABLE: bool> Encode for Label<NULLABLE> {
    fn encode(&self) -> View<'_> {
        match &self.0 {
            Some(label) => View::String(label),
            None => View::Json(Cow::Owned(JsonValue::Null)),
        }
    }
}

impl<const NULLABLE: bool> Decode for Label<NULLABLE> {
    fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError> {
        match decoder.json()? {
            JsonValue::Null => Ok(Self(None)),
            JsonValue::String(label) => Ok(Self(Some(label.into()))),
            _ => Err(DecodeError::new("expected a string or null")),
        }
    }
}

record!(Labels {
    strict: Label<false> => .optional();
    nullable: Label<true> => .optional();
});

/// Optional fields treat `null` as absent unless the opaque type admits it.
#[test]
fn null_for_optional_opaque_fields() {
    let labels: Labels = json::from_value(&parse(r#"{"strict":null,"nullable":null}"#)).unwrap();
    assert_eq!(labels.strict, None);
    assert_eq!(labels.nullable, Some(Label(None)));
}

record!(Extension {
    data: JsonValue => .optional();
});

/// Optional JSON value fields treat `null` as absent, like most languages do.
#[test]
fn null_for_optional_value_fields() {
    let extension: Extension = json::from_value(&parse(r#"{"data":null}"#)).unwrap();
    assert_eq!(extension.data, None);
    let extension: Extension = json::from_value(&parse(r#"{"data":{}}"#)).unwrap();
    assert_eq!(extension.data, Some(JsonValue::Map(Default::default())));
}

/// JSON values display as compact JSON text.
#[test]
fn value_display() {
    let value = JsonValue::from_iter([("a", JsonValue::from(1)), ("b", JsonValue::from("x"))]);
    assert_eq!(value.to_string(), r#"{"a":1,"b":"x"}"#);
}
