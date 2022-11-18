use quote::{format_ident, quote};
use sidex_gen::ir;

use super::Plugin;
use crate::context::TypeInfo;

pub struct Builder;

impl Plugin for Builder {
    fn visit_def(
        &self,
        ctx: &crate::context::SchemaCtx,
        def: &sidex_gen::ir::Def,
    ) -> Result<proc_macro2::TokenStream, ()> {
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
                    // Methods for direct field access, mostly for convenience.
                    let set_doc = format!("Sets the value of `{}`.", name);
                    let set_method = format_ident!("set_{}", name);
                    let with_doc = format!("Sets the value of `{}`.", name);
                    let with_method = format_ident!("with_{}", name);
                    methods.push(quote! {
                        #[doc = #set_doc]
                        #vis fn #set_method(&mut self, #name: #typ) -> &mut Self {
                            self.#name = #name;
                            self
                        }
                        #[doc = #with_doc]
                        #vis fn #with_method(mut self, #name: #typ) -> Self {
                            self.#name = #name;
                            self
                        }
                    });

                    let is_container = match &field.typ.kind {
                        ir::TypeKind::Instance(instance) => {
                            match ctx.fully_qualified_type_name(instance).as_str() {
                                "::std::builtins::Sequence" | "::std::builtins::Map" => true,
                                _ => false,
                            }
                        }
                        _ => false,
                    };

                    if !field.is_optional && !is_container {
                        constructor_fields.push(info);
                    } else {
                        default_fields.push(info);
                    }
                }

                let constructor_args = constructor_fields
                    .iter()
                    .map(|info| {
                        let name = &info.name;
                        let typ = &info.typ;
                        quote! { #name: #typ }
                    })
                    .collect::<Vec<_>>();

                let new_doc = format!("Creates a new [`{}`].", &def.name);

                let construct = constructor_fields
                    .iter()
                    .map(|info| {
                        let name = &info.name;
                        quote! { #name }
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

                Ok(quote! {
                    impl #generics #ident #generics {
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
