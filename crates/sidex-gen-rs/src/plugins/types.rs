use sidex_gen::{diagnostics::Result, ir::Def};

use super::Plugin;
use crate::{
    context::SchemaCtx,
    rstyir::{rs_type_from_def, rs_type_to_rs_def},
};

pub struct Types;

impl Plugin for Types {
    fn visit_def(&self, ctx: &SchemaCtx, def: &Def) -> Result<proc_macro2::TokenStream> {
        let Some(rs_type) = rs_type_from_def(ctx, def)? else {
            return Ok(Default::default());
        };
        println!("Generating type definition for {}.", def.name.as_str());
        Ok(rs_type_to_rs_def(&rs_type))
    }
}
