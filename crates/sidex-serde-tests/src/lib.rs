//! Shared JSON fixtures and Rust representation tests.

sidex::include_bundle!(pub json_test_bundle as generated);

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use serde::Deserialize;
    use serde::Serialize;
    use sidex::Decode;
    use sidex::Encode;
    use sidex::formats::json;

    use super::generated::data::*;

    fn expected_dir() -> &'static Path {
        Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/json/expected"
        ))
    }

    /// Checks that every fixture decodes and encodes to itself, both with the
    /// JSON format and with Serde.
    fn test<T>(type_name: &str)
    where
        T: Encode + Decode + Serialize + for<'de> Deserialize<'de>,
    {
        for entry in fs::read_dir(expected_dir().join(type_name)).unwrap() {
            let path = entry.unwrap().path();
            println!("{}", path.to_string_lossy());
            let text = fs::read_to_string(&path).unwrap();
            let expected = serde_json::from_str::<serde_json::Value>(&text).unwrap();

            let decoded = json::from_str::<T>(&text).unwrap();
            let encoded =
                serde_json::from_str::<serde_json::Value>(&json::to_string(&decoded).unwrap())
                    .unwrap();
            assert_eq!(expected, encoded);

            let deserialized = serde_json::from_value::<T>(expected.clone()).unwrap();
            let serialized = serde_json::to_value(&deserialized).unwrap();
            assert_eq!(expected, serialized);
        }
    }

    macro_rules! test {
        ($type:ty) => {
            test::<$type>(stringify!($type));
        };
    }

    #[test]
    fn test_variant_types() {
        test::<VariantInternallyTagged<i32>>("VariantInternallyTagged");
        test::<VariantExternallyTagged<i32>>("VariantExternallyTagged");
        test::<VariantAdjacentlyTagged<i32>>("VariantAdjacentlyTagged");
        test::<VariantImplicitlyTagged<bool>>("VariantImplicitlyTagged");
        test::<MaybeRef<u64>>("MaybeRefInteger");
        test::<MaybeRef<Point>>("MaybeRefPoint");
        test::<Any>("Any");
        test!(FallbackInternallyTagged);
        test!(FallbackExternallyTagged);
        test!(FallbackAdjacentlyTagged);
        test!(FallbackImplicitlyTagged);
    }

    /// Known cases take precedence over fallback cases, which also round-trip
    /// the values of known cases. Values that select no case only decode as
    /// fallback cases, so the fixtures cover them.
    #[test]
    fn test_fallback_cases() {
        use sidex::types::JsonValue;

        assert!(matches!(
            json::from_str(r#"{"kind":"B","content":42}"#).unwrap(),
            FallbackInternallyTagged::B(42)
        ));
        assert!(matches!(
            json::from_str("42").unwrap(),
            FallbackImplicitlyTagged::Number(42)
        ));

        // Known cases with invalid payloads do not fall back.
        let error = json::from_str::<FallbackInternallyTagged>(r#"{"kind":"B","content":"x"}"#)
            .unwrap_err();
        assert_eq!(
            error.to_string(),
            "content: expected a 64-bit unsigned integer, found `x`"
        );
        // Values that the fallback's payload does not admit fail with both reasons.
        let error = json::from_str::<FallbackExternallyTagged>(r#"{"C":1}"#).unwrap_err();
        assert_eq!(
            error.to_string(),
            "unknown case `C` of `::json_test_bundle::data::FallbackExternallyTagged`, and \
             fallback case `Other` does not match: expected a string, found a map"
        );
        // The tag of a record fallback is not an unknown field.
        let error = json::from_str_strict::<FallbackAdjacentlyTagged>(
            r#"{"tag":"C","content":1,"extra":2}"#,
        )
        .unwrap_err();
        assert!(
            error.to_string().ends_with("unknown field `extra`"),
            "{error}"
        );

        // Schemas offer the fallback's payload as an alternative to the other cases.
        let schema = json::schema(<FallbackExternallyTagged as sidex::Shaped>::SHAPE);
        let alternatives = schema.get("anyOf").and_then(JsonValue::as_seq).unwrap();
        let known = alternatives[0].get("oneOf").and_then(JsonValue::as_seq);
        assert_eq!(known.map(|known| known.len()), Some(2));
        assert_eq!(
            alternatives[1],
            json::from_str::<JsonValue>(r#"{"type":"string"}"#).unwrap()
        );
    }

    #[test]
    fn test_record_types() {
        test!(RecordFieldNamesDefault);
        test!(RecordFieldNamesScreamingSnake);
        test!(JsRoundtrip);
    }
}
