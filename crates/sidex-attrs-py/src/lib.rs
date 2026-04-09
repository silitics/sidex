//! Sidex attributes for Python code generation.

use sidex_gen::{
    attrs::{AttrConvertExt, TryApplyAttr, TryFromAttr, accept, reject},
    diagnostics, ir,
};

/// `type = "<PYTHON-TYPE>"`
#[derive(Debug, Clone)]
pub struct PyTypeAttr {
    pub path: String,
}

impl TryFromAttr for PyTypeAttr {
    fn try_from_attr(attr: &ir::Attr) -> diagnostics::Result<Self> {
        Ok(Self {
            path: attr
                .expect_assign_with("type")?
                .value
                .expect_string_literal()?
                .to_owned(),
        })
    }
}

/// Python attributes of an opaque type.
#[derive(Debug, Clone, Default)]
pub struct PyOpaqueTypeAttrs {
    pub typ: Option<PyTypeAttr>,
}

impl TryApplyAttr for PyOpaqueTypeAttrs {
    fn try_apply_attr(&mut self, attr: &ir::Attr) -> diagnostics::Result<()> {
        if let Ok(list) = attr.expect_list_with("py") {
            for arg in &list.args {
                if let Ok(typ) = arg.convert() {
                    self.typ = Some(typ);
                } else {
                    reject!(arg, "Expected a Python opaque type attribute.")
                }
            }
        }
        Ok(())
    }
}
