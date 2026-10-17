use sidex_gen::diagnostics::Result;
use sidex_gen::ir::Def;

use super::Plugin;
use crate::codec;
use crate::context::BundleCtx;
use crate::context::SchemaCtx;
use crate::group_traits;
use crate::rstyir::rs_type_from_def;
use crate::rstyir::rs_type_to_rs_def;

pub struct Types;

impl Plugin for Types {
    fn visit_def(&self, ctx: &SchemaCtx, def: &Def) -> Result<proc_macro2::TokenStream> {
        let Some(rs_type) = rs_type_from_def(ctx, def)? else {
            return Ok(Default::default());
        };
        println!("Generating type definition for {}.", def.name.as_str());
        let mut tokens = rs_type_to_rs_def(&rs_type);
        tokens.extend(codec::impl_traits(ctx, def, &rs_type)?);
        tokens.extend(group_traits::impl_traits(ctx, def, &rs_type)?);
        Ok(tokens)
    }

    fn visit_bundle(&self, ctx: &BundleCtx) -> Result<proc_macro2::TokenStream> {
        group_traits::define_traits(ctx)
    }
}
