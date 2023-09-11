sidex::include_bundle!(json_test_bundle as generated);

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use serde::{Deserialize, Serialize};

    use super::generated::data::*;

    fn expected_dir() -> &'static Path {
        Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/json/expected"
        ))
    }

    fn test<T: Serialize + for<'de> Deserialize<'de>>(type_name: &str) {
        for entry in fs::read_dir(expected_dir().join(type_name)).unwrap() {
            let path = entry.unwrap().path();

            println!("{}", path.to_string_lossy());

            let json = fs::read_to_string(&path).unwrap();

            let expected = serde_json::from_str::<serde_json::Value>(&json).unwrap();
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
    }

    #[test]
    fn test_record_types() {
        test!(RecordFieldNamesDefault);
        test!(RecordFieldNamesScreamingSnake);
    }
}
