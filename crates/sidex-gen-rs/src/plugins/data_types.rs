use quote::{format_ident, quote};
use sidex_gen::ir::{Def, DefKind};

use super::Plugin;
use crate::context::SchemaCtx;

pub struct Types;

impl Plugin for Types {
    fn visit_def(&self, ctx: &SchemaCtx, def: &Def) -> Result<proc_macro2::TokenStream, ()> {
        let name = format_ident!("{}", def.name);
        let vars = ctx.generic_type_vars(def);
        match &def.kind {
            DefKind::TypeAlias(_) => Ok(quote! { type #name #vars = (); }),
            DefKind::OpaqueType(_) => Ok(quote! { type #name #vars = (); }),
            DefKind::RecordType(_) => {
                Ok(quote! {
                    struct #name #vars {
                        // Fields ...
                    }
                })
            }
            DefKind::VariantType(_) => {
                Ok(quote! {
                    enum #name #vars {
                        // Fields ...
                    }
                })
            }
            DefKind::WrapperType(_) => {
                Ok(quote! {
                    struct #name #vars (());
                })
            }
            DefKind::Service(_) => {
                // Service definitions are handled by a separate plugin.
                Ok(Default::default())
            }
        }
    }
}
