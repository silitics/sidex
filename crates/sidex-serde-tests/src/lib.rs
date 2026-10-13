sidex::include_bundle!(json_test_bundle as generated);

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
    }

    #[test]
    fn test_record_types() {
        test!(RecordFieldNamesDefault);
        test!(RecordFieldNamesScreamingSnake);
        test!(JsRoundtrip);
    }
}
