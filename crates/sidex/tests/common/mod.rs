//! Types written the way Sidex generates them, shared by the tests.

#![allow(dead_code, unused_imports, unused_macros)]

use std::collections::BTreeMap;

use sidex::types::Bytes;
use sidex::types::Date;
use sidex::types::Timestamp;
use sidex::types::Uuid;

/// Defines a record whose fields are all `Option`s. Unlike generated code, the
/// macro relies on the decoder to report missing required fields.
///
/// Builder calls after `=>` apply to the record's shape and to fields.
macro_rules! record {
    (
        $name:ident $(=> $(.$shape_method:ident($($shape_arg:expr)?))+)? {
            $($field:ident: $ty:ty $(=> $(.$method:ident($($arg:expr)?))+)?;)*
        }
    ) => {
        #[derive(Debug, Default, PartialEq)]
        pub struct $name {
            $(pub $field: Option<$ty>,)*
        }

        impl ::sidex::Shaped for $name {
            const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
                concat!("::tests::", stringify!($name)),
                ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                    $(
                        ::sidex::shape::FieldShape::new(
                            stringify!($field),
                            ::sidex::shape::shape_of::<$ty>,
                        )
                        $($(.$method($($arg)?))+)?,
                    )*
                ])),
            )
            $($(.$shape_method($($shape_arg)?))+)?;
        }

        impl ::sidex::Encode for $name {
            fn encode(&self) -> ::sidex::View<'_> {
                ::sidex::View::Record(self)
            }
        }

        impl ::sidex::encode::RecordView for $name {
            fn shape(&self) -> &'static ::sidex::Shape {
                <Self as ::sidex::Shaped>::SHAPE
            }

            fn field(&self, index: usize) -> Option<::sidex::View<'_>> {
                let fields: &[Option<&dyn ::sidex::Encode>] =
                    &[$(self.$field.as_ref().map(|value| value as &dyn ::sidex::Encode),)*];
                fields.get(index).copied().flatten().map(::sidex::Encode::encode)
            }
        }

        impl ::sidex::Decode for $name {
            fn decode(
                decoder: &mut dyn ::sidex::Decoder,
            ) -> Result<Self, ::sidex::DecodeError> {
                let mut out = Self::default();
                decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, field| {
                    let mut position = 0;
                    $(
                        if index == position {
                            out.$field = Some(<$ty as ::sidex::Decode>::decode(field)?);
                        }
                        position += 1;
                    )*
                    let _ = position;
                    Ok(())
                })?;
                Ok(out)
            }
        }
    };
}

pub(crate) use record;

record!(Scalars {
    flag: bool => .optional();
    small: i32 => .optional();
    signed: i64 => .optional();
    unsigned: u64 => .optional();
    index: usize => .optional();
    single: f32 => .optional();
    double: f64 => .optional();
    text: String => .optional();
    bytes: Bytes => .optional();
    uuid: Uuid => .optional();
    timestamp: Timestamp => .optional();
    date: Date => .optional();
    unit: () => .optional();
});

record!(Point {
    x: i32;
    y: i32;
});

record!(Device {
    id: u64;
    display_name: String => .optional().json_name("displayName");
});

record!(Fleet {
    devices: Vec<Device>;
});

record!(Located {
    name: String;
    position: Point => .json_inline();
});

record!(Indexes {
    by_id: BTreeMap<u64, String> => .json_name("byId");
    by_uuid: BTreeMap<Uuid, u8> => .json_name("byUuid");
});
