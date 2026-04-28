use sidex_gen::attrs::AttrConvertExt;
use sidex_gen::attrs::TryFromAttr;
use sidex_gen::attrs::accept;
use sidex_gen::attrs::new_assign_attr;
use sidex_gen::attrs::reject;
use sidex_gen::diagnostics;
use sidex_gen::ir;
use sidex_gen::rename::RenameFunction;

use crate::types;

/// An attribute of the form `type = "<JSON-TYPE-EXPR>"`.
#[derive(Debug, Clone)]
pub struct TypeAttr {
    pub typ: types::JsonUnionType,
}

impl TryFromAttr for TypeAttr {
    fn try_from_attr(attr: &ir::Attr) -> diagnostics::Result<Self> {
        let assign = attr.expect_assign_with("type")?;
        let s = assign
            .value
            .as_string()
            .ok_or_else(|| {
                diagnostics::Diagnostic::error("Expected a string value.")
                    .with_span(attr.span.clone())
            })?;
        Ok(Self {
            typ: s.parse()?,
        })
    }
}

/// An attribute of the form `schema = "<JSON-SCHEMA-PATH>"`.
#[derive(Debug, Clone)]
pub struct SchemaAttr {
    pub schema: String,
}

impl TryFromAttr for SchemaAttr {
    fn try_from_attr(attr: &ir::Attr) -> diagnostics::Result<Self> {
        let assign = attr.expect_assign_with("schema")?;
        let s = assign.value.as_string().ok_or_else(|| {
            diagnostics::Diagnostic::error("Expected a string value.")
                .with_span(attr.span.clone())
        })?;
        Ok(Self {
            schema: s.to_owned(),
        })
    }
}

/// An attribute of the form `rename_all = "<RENAME-FUNCTION>"`.
#[derive(Debug, Clone)]
pub struct RenameAllAttr {
    pub function: RenameFunction,
}

impl From<RenameFunction> for RenameAllAttr {
    fn from(function: RenameFunction) -> Self {
        Self { function }
    }
}

impl TryFromAttr for RenameAllAttr {
    fn try_from_attr(attr: &ir::Attr) -> diagnostics::Result<Self> {
        let assign = attr.expect_assign_with("rename_all")?;
        let s = assign.value.as_string().ok_or_else(|| {
            diagnostics::Diagnostic::error("Expected a string value.")
                .with_span(attr.span.clone())
        })?;
        Ok(Self {
            function: s.parse()?,
        })
    }
}

/// An attribute of the form `rename = "<RENAME-FUNCTION>"`.
#[derive(Debug, Clone)]
pub struct RenameAttr {
    pub function: RenameFunction,
}

impl TryFromAttr for RenameAttr {
    fn try_from_attr(attr: &ir::Attr) -> diagnostics::Result<Self> {
        let assign = attr.expect_assign_with("rename")?;
        let s = assign.value.as_string().ok_or_else(|| {
            diagnostics::Diagnostic::error("Expected a string value.")
                .with_span(attr.span.clone())
        })?;
        Ok(Self {
            function: s.parse()?,
        })
    }
}

new_assign_attr! {
    /// An attribute of the form `name = "<NAME>"`.
    pub struct NameAttr["name"](pub String)
}

new_assign_attr! {
    /// An attribute of the form `tag = "<TAG-FIELD-NAME>"`.
    pub struct TagAttr["tag"](pub String)
}

new_assign_attr! {
    /// An attribute of the form `content = "<CONTENT-FIELD-NAME>"`.
    pub struct ContentAttr["content"](pub String)
}

/// An attribute of the form `tagged = <TAG-KIND>`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum JsonTaggedAttr {
    Adjacently,
    Externally,
    #[default]
    Internally,
    Implicitly,
}

impl TryFromAttr for JsonTaggedAttr {
    fn try_from_attr(attr: &ir::Attr) -> sidex_gen::diagnostics::Result<Self> {
        let assign = attr.expect_assign_with("tagged")?;
        let path = assign.value.as_path().ok_or_else(|| {
            diagnostics::Diagnostic::error("Expected a path value.")
                .with_span(attr.span.clone())
        })?;
        match path {
            "adjacently" => accept!(Self::Adjacently),
            "externally" => accept!(Self::Externally),
            "internally" => accept!(Self::Internally),
            "implicitly" => accept!(Self::Implicitly),
            _ => reject!(attr, "Expected a tag"),
        }
    }
}
