use sidex_ir as ir;

pub trait ApplyAttr {
    fn apply(&mut self, attr: &ir::Attr);
}
