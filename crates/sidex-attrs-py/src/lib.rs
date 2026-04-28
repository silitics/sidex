//! Sidex attributes for Python code generation.

use sidex_gen::attrs::AttrConvertExt;
use sidex_gen::attrs::TryApplyAttr;
use sidex_gen::attrs::TryFromAttr;
use sidex_gen::attrs::TryFromAttrValue;
use sidex_gen::attrs::reject;
use sidex_gen::diagnostics;
use sidex_gen::ir;

/// `type = "<PYTHON-TYPE>"`
#[derive(Debug, Clone)]
pub struct PyTypeAttr {
    pub path: String,
}

impl TryFromAttr for PyTypeAttr {
    fn try_from_attr(attr: &ir::Attr) -> diagnostics::Result<Self> {
        let assign = attr.expect_assign_with("type")?;
        Ok(Self {
            path: String::try_from_attr_value(&assign.value, attr)?,
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
