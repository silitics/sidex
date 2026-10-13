//! Runtime descriptions of Sidex types.
//!
//! A [`Shape`] describes a Sidex type at runtime: its path, documentation, raw
//! attributes, stability (see [`stability`]), validation rules (see
//! [`validation`]), and structure. Every Sidex type implements
//! [`Shaped`], which provides its shape as an associated constant. Shapes are static
//! data, so inspecting them never allocates.
//!
//! Shapes also carry format-specific metadata, such as the JSON names of fields
//! and the tagging of variants (see [`json`]). Formats read this metadata
//! instead of interpreting attributes at runtime.
//!
//! [`validation`]: crate::validation
//!
//! Generated code constructs shapes with `const` builder methods. Unless set
//! otherwise, JSON names are the schema names and variants are internally
//! tagged with `tag` as the tag field:
//!
//! ```
//! use sidex::shape::{FieldShape, RecordShape, Shape, ShapeKind, shape_of};
//!
//! const FIELDS: &[FieldShape] = &[
//!     FieldShape::new("id", shape_of::<u64>),
//!     FieldShape::new("display_name", shape_of::<String>)
//!         .optional()
//!         .json_name("displayName"),
//! ];
//!
//! const DEVICE: &Shape = &Shape::new(
//!     "::example::devices::Device",
//!     ShapeKind::Record(RecordShape::new(FIELDS)),
//! )
//! .with_docs("A device.");
//!
//! let record = DEVICE.as_record().unwrap();
//! assert_eq!(record.fields[1].json.name, "displayName");
//! assert!((record.fields[1].shape)().path.ends_with("string"));
//! ```

use std::fmt;

pub use self::attr::Attr;
pub use self::attr::AttrValue;
use self::stability::Stability;
use crate::formats::json;
use crate::validation::Rule;

mod attr;
pub mod stability;

/// Types with a [`Shape`].
pub trait Shaped {
    /// The shape of the type.
    const SHAPE: &'static Shape;
}

/// Lazy reference to a shape.
///
/// References are functions, so that recursive types do not create cycles
/// between constants.
pub type ShapeRef = fn() -> &'static Shape;

/// Returns the shape of `T`.
///
/// Use `shape_of::<T>` as a [`ShapeRef`].
pub fn shape_of<T: Shaped + ?Sized>() -> &'static Shape {
    T::SHAPE
}

/// Description of a Sidex type.
#[derive(Debug)]
#[non_exhaustive]
pub struct Shape {
    /// Fully qualified Sidex path of the type's definition, for instance,
    /// `::core::builtins::u64`.
    pub path: &'static str,
    /// Shapes of the type arguments of a generic definition.
    pub args: &'static [ShapeRef],
    /// Documentation of the definition.
    pub docs: Option<&'static str>,
    /// Raw attributes of the definition.
    pub attrs: &'static [Attr],
    /// Stability of the definition.
    pub stability: Stability,
    /// Validation rules of the definition.
    ///
    /// The rules apply to values of the type. For records, they can compare
    /// fields, for instance, `_.start <= _.end`. For wrappers and opaque
    /// types, they apply to the wrapped value.
    pub rules: &'static [Rule],
    /// Structure of the type.
    pub kind: ShapeKind,
}

impl Shape {
    /// Creates a stable shape without type arguments, documentation, attributes,
    /// or rules.
    pub const fn new(path: &'static str, kind: ShapeKind) -> Self {
        Self {
            path,
            args: &[],
            docs: None,
            attrs: &[],
            stability: Stability::new(),
            rules: &[],
            kind,
        }
    }

    /// Sets the shapes of the type arguments.
    pub const fn with_args(self, args: &'static [ShapeRef]) -> Self {
        Self { args, ..self }
    }

    /// Sets the documentation.
    pub const fn with_docs(self, docs: &'static str) -> Self {
        Self {
            docs: Some(docs),
            ..self
        }
    }

    /// Sets the raw attributes.
    pub const fn with_attrs(self, attrs: &'static [Attr]) -> Self {
        Self { attrs, ..self }
    }

    /// Sets the stability.
    pub const fn with_stability(self, stability: Stability) -> Self {
        Self { stability, ..self }
    }

    /// Sets the validation rules.
    pub const fn with_rules(self, rules: &'static [Rule]) -> Self {
        Self { rules, ..self }
    }

    /// Returns the record structure if the shape describes a record.
    pub fn as_record(&self) -> Option<&RecordShape> {
        match &self.kind {
            ShapeKind::Record(record) => Some(record),
            _ => None,
        }
    }

    /// Returns the variant structure if the shape describes a variant.
    pub fn as_variant(&self) -> Option<&VariantShape> {
        match &self.kind {
            ShapeKind::Variant(variant) => Some(variant),
            _ => None,
        }
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.path)?;
        if let Some((first, rest)) = self.args.split_first() {
            write!(f, "<{}", first())?;
            for arg in rest {
                write!(f, ", {}", arg())?;
            }
            f.write_str(">")?;
        }
        Ok(())
    }
}

/// Structure of a type.
#[derive(Debug)]
pub enum ShapeKind {
    /// Built-in scalar type.
    Scalar(Scalar),
    /// Sequence with elements of the given shape.
    Sequence(ShapeRef),
    /// Map with keys and values of the given shapes.
    Map(ShapeRef, ShapeRef),
    /// Record.
    Record(RecordShape),
    /// Variant.
    Variant(VariantShape),
    /// Wrapper around a type of the given shape.
    Wrapper(ShapeRef),
    /// Opaque type whose representation is provided by its implementation.
    Opaque(OpaqueShape),
    /// Freeform value.
    Value,
}

/// Built-in scalar types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scalar {
    /// The `unit` type.
    Unit,
    /// The `bool` type.
    Bool,
    /// The `i8` type.
    I8,
    /// The `i16` type.
    I16,
    /// The `i32` type.
    I32,
    /// The `i64` type.
    I64,
    /// The `u8` type.
    U8,
    /// The `u16` type.
    U16,
    /// The `u32` type.
    U32,
    /// The `u64` type.
    U64,
    /// The `idx` type.
    Idx,
    /// The `f32` type.
    F32,
    /// The `f64` type.
    F64,
    /// The `string` type.
    String,
    /// The `bytes` type.
    Bytes,
    /// The `uuid` type.
    Uuid,
    /// The `timestamp` type.
    Timestamp,
    /// The `date` type.
    Date,
}

/// Structure of an opaque type.
#[derive(Debug)]
#[non_exhaustive]
pub struct OpaqueShape {
    /// JSON representation of the opaque type.
    pub json: json::Opaque,
}

impl OpaqueShape {
    /// Creates the structure of an opaque type represented by any JSON value.
    pub const fn new() -> Self {
        Self {
            json: json::Opaque::ANY,
        }
    }

    /// Restricts the JSON representation to the given types.
    pub const fn json_types(self, types: &'static [json::Type]) -> Self {
        Self {
            json: json::Opaque { types: Some(types) },
        }
    }
}

impl Default for OpaqueShape {
    fn default() -> Self {
        Self::new()
    }
}

/// Structure of a record.
///
/// Rules involving multiple fields belong to the record's [`Shape`] (see
/// [`Shape::rules`]). Rules of individual fields belong to the fields.
#[derive(Debug)]
#[non_exhaustive]
pub struct RecordShape {
    /// Fields in declaration order.
    pub fields: &'static [FieldShape],
}

impl RecordShape {
    /// Creates a record structure.
    pub const fn new(fields: &'static [FieldShape]) -> Self {
        Self { fields }
    }
}

/// Field of a record.
#[derive(Debug)]
#[non_exhaustive]
pub struct FieldShape {
    /// Name of the field in the schema.
    pub name: &'static str,
    /// Documentation of the field.
    pub docs: Option<&'static str>,
    /// Raw attributes of the field.
    pub attrs: &'static [Attr],
    /// Stability of the field.
    pub stability: Stability,
    /// Validation rules of the field, which apply if the field is present.
    pub rules: &'static [Rule],
    /// Indicates whether the field is optional.
    pub optional: bool,
    /// Shape of the field's type.
    pub shape: ShapeRef,
    /// JSON representation of the field.
    pub json: json::Field,
}

impl FieldShape {
    /// Creates a stable, required field without documentation, attributes, or
    /// rules.
    ///
    /// The field's JSON name is its schema name.
    pub const fn new(name: &'static str, shape: ShapeRef) -> Self {
        Self {
            name,
            docs: None,
            attrs: &[],
            stability: Stability::new(),
            rules: &[],
            optional: false,
            shape,
            json: json::Field::new(name),
        }
    }

    /// Marks the field as optional.
    pub const fn optional(self) -> Self {
        Self {
            optional: true,
            ..self
        }
    }

    /// Sets the documentation.
    pub const fn with_docs(self, docs: &'static str) -> Self {
        Self {
            docs: Some(docs),
            ..self
        }
    }

    /// Sets the raw attributes.
    pub const fn with_attrs(self, attrs: &'static [Attr]) -> Self {
        Self { attrs, ..self }
    }

    /// Sets the stability.
    pub const fn with_stability(self, stability: Stability) -> Self {
        Self { stability, ..self }
    }

    /// Sets the validation rules.
    pub const fn with_rules(self, rules: &'static [Rule]) -> Self {
        Self { rules, ..self }
    }

    /// Sets the JSON name.
    pub const fn json_name(self, name: &'static str) -> Self {
        Self {
            json: json::Field { name, ..self.json },
            ..self
        }
    }

    /// Merges the fields of the field's record into the enclosing JSON map.
    pub const fn json_inline(self) -> Self {
        Self {
            json: json::Field {
                inline: true,
                ..self.json
            },
            ..self
        }
    }

    /// Represents the field's 64-bit integer as a JSON number instead of a
    /// string.
    ///
    /// Formats defined by others, such as the claims of JSON Web Tokens, may
    /// require numbers. Many JSON implementations lose the precision of
    /// integers beyond 2^53.
    pub const fn json_number(self) -> Self {
        Self {
            json: json::Field {
                number: true,
                ..self.json
            },
            ..self
        }
    }
}

/// Structure of a variant.
#[derive(Debug)]
#[non_exhaustive]
pub struct VariantShape {
    /// Cases in declaration order.
    pub cases: &'static [CaseShape],
    /// JSON tagging of the variant.
    pub json: json::Tagging,
}

impl VariantShape {
    /// Creates a variant structure with internal JSON tagging using `tag` as
    /// the tag field.
    pub const fn new(cases: &'static [CaseShape]) -> Self {
        Self {
            cases,
            json: json::Tagging::Internal {
                tag: "tag",
                content: "content",
            },
        }
    }

    /// Sets the JSON tagging.
    pub const fn json_tagging(self, json: json::Tagging) -> Self {
        Self { json, ..self }
    }
}

/// Case of a variant.
#[derive(Debug)]
#[non_exhaustive]
pub struct CaseShape {
    /// Name of the case in the schema.
    pub name: &'static str,
    /// Documentation of the case.
    pub docs: Option<&'static str>,
    /// Raw attributes of the case.
    pub attrs: &'static [Attr],
    /// Stability of the case.
    pub stability: Stability,
    /// Shape of the payload, if the case has one.
    pub payload: Option<ShapeRef>,
    /// JSON representation of the case.
    pub json: json::Case,
}

impl CaseShape {
    /// Creates a stable case without documentation or attributes.
    ///
    /// The case's JSON name is its schema name.
    pub const fn new(name: &'static str, payload: Option<ShapeRef>) -> Self {
        Self {
            name,
            docs: None,
            attrs: &[],
            stability: Stability::new(),
            payload,
            json: json::Case::new(name),
        }
    }

    /// Sets the documentation.
    pub const fn with_docs(self, docs: &'static str) -> Self {
        Self {
            docs: Some(docs),
            ..self
        }
    }

    /// Sets the raw attributes.
    pub const fn with_attrs(self, attrs: &'static [Attr]) -> Self {
        Self { attrs, ..self }
    }

    /// Sets the stability.
    pub const fn with_stability(self, stability: Stability) -> Self {
        Self { stability, ..self }
    }

    /// Sets the JSON name.
    pub const fn json_name(self, name: &'static str) -> Self {
        Self {
            json: json::Case { name, ..self.json },
            ..self
        }
    }

    /// Sets the JSON field holding the payload.
    pub const fn json_content(self, content: &'static str) -> Self {
        Self {
            json: json::Case {
                content: Some(content),
                ..self.json
            },
            ..self
        }
    }

    /// Represents the 64-bit integer payload of an implicitly tagged case as a
    /// JSON number instead of a string.
    ///
    /// Generated code uses this for cases whose declared payload type is a
    /// 64-bit integer, so that they remain distinct from string cases.
    /// Decoding such a case accepts numbers only.
    pub const fn json_number(self) -> Self {
        Self {
            json: json::Case {
                number: true,
                ..self.json
            },
            ..self
        }
    }
}
