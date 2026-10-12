use quote::format_ident;
use quote::quote;
use sidex_gen::diagnostics::Result;
use sidex_gen::ir;

use super::Plugin;
use crate::context::TypeInfo;

pub struct Builder;

impl Plugin for Builder {
    fn visit_def(
        &self,
        ctx: &crate::context::SchemaCtx,
        def: &sidex_gen::ir::Def,
    ) -> Result<proc_macro2::TokenStream> {
        match &def.kind {
            sidex_gen::ir::DefKind::RecordType(record) => {
                let TypeInfo {
                    ident, generics, ..
                } = ctx.type_info(def);
                let mut methods = Vec::new();

                let mut constructor_fields = Vec::new();
                let mut default_fields = Vec::new();
                for field in &record.fields {
                    let info = ctx.field_info(def, field);
                    let name = &info.name;
                    let typ = &info.typ;
                    let vis = &info.vis;
                    // Parameters accept anything convertible into the field's type,
                    // except for numbers, whose literals would be ambiguous, and
                    // optional fields, whose `None` would be ambiguous.
                    let (param, value) = if field.is_optional || is_number(ctx, &field.typ) {
                        (quote! { #typ }, quote! { #name })
                    } else {
                        (
                            quote! { impl ::std::convert::Into<#typ> },
                            quote! { #name.into() },
                        )
                    };
                    // Methods for direct field access, mostly for convenience.
                    let set_doc = format!("Sets the value of `{}`.", name);
                    let set_method = format_ident!("set_{}", name);
                    let with_doc = format!("Sets the value of `{}`.", name);
                    let with_method = format_ident!("with_{}", name);
                    methods.push(quote! {
                        #[doc = #set_doc]
                        #vis fn #set_method(&mut self, #name: #param) -> &mut Self {
                            self.#name = #value;
                            self
                        }
                        #[doc = #with_doc]
                        #vis fn #with_method(mut self, #name: #param) -> Self {
                            self.#name = #value;
                            self
                        }
                    });

                    let is_container = match &field.typ.kind {
                        ir::TypeKind::Instance(instance) => {
                            matches!(
                                ctx.fully_qualified_type_name(instance).as_str(),
                                "::std::builtins::Sequence" | "::std::builtins::Map"
                            )
                        }
                        _ => false,
                    };

                    let is_default_field = field.attrs.iter().any(|attr| {
                        if let ir::AttrKind::Path(path) = &attr.kind {
                            path.as_str() == "default"
                        } else {
                            false
                        }
                    });

                    if !field.is_optional && !is_container && !is_default_field {
                        constructor_fields.push((info, param, value));
                    } else {
                        default_fields.push(info);
                    }
                }

                let constructor_args = constructor_fields
                    .iter()
                    .map(|(info, param, _)| {
                        let name = &info.name;
                        quote! { #name: #param }
                    })
                    .collect::<Vec<_>>();

                let new_doc = format!("Creates a new [`{}`].", def.name.as_str());

                let construct = constructor_fields
                    .iter()
                    .map(|(info, _, value)| {
                        let name = &info.name;
                        quote! { #name: #value }
                    })
                    .chain(default_fields.iter().map(|info| {
                        let name = &info.name;
                        quote! { #name: ::std::default::Default::default() }
                    }));

                let default_impl = if constructor_args.is_empty() {
                    quote! {
                        impl #generics ::std::default::Default for #ident #generics {
                            fn default() -> Self {
                                Self::new()
                            }
                        }
                    }
                } else {
                    quote! {}
                };

                let generics = ctx.generics(def);

                let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

                Ok(quote! {
                    impl #impl_generics #ident #ty_generics #where_clause {
                        #[doc = #new_doc]
                        pub fn new(#(#constructor_args),*) -> Self {
                            Self {
                                #(#construct),*
                            }
                        }

                        #(#methods)*
                    }

                    #default_impl
                })
            }
            _ => {
                // We only care about record types.
                Ok(quote! {})
            }
        }
    }
}

/// Indicates whether the type is a built-in number type.
fn is_number(ctx: &crate::context::SchemaCtx, typ: &ir::Type) -> bool {
    let resolved = ctx.bundle_ctx.unit.resolve_aliases(typ);
    let ir::TypeKind::Instance(instance) = &resolved.kind else {
        return false;
    };
    matches!(
        ctx.fully_qualified_type_name(instance).as_str(),
        "::core::builtins::i8"
            | "::core::builtins::i16"
            | "::core::builtins::i32"
            | "::core::builtins::i64"
            | "::core::builtins::u8"
            | "::core::builtins::u16"
            | "::core::builtins::u32"
            | "::core::builtins::u64"
            | "::core::builtins::idx"
            | "::core::builtins::f32"
            | "::core::builtins::f64"
    )
}
