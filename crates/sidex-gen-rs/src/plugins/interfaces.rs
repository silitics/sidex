use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use sidex_attrs_json::JsonFieldAttrs;
use sidex_attrs_rust::Visibility;
use sidex_gen::{
    diagnostics::Result,
    ir::{Def, DefKind},
    rename::{to_camel_case, to_pascal_case, to_snake_case},
};
use syn::Ident;

use super::Plugin;
use crate::{
    context::SchemaCtx,
    rstyir::{
        RsField, RsType, RsTypeKind, RsTypeRecord, rs_type_to_rs_def, serde::rs_type_impl_serde,
    },
};

pub struct Interfaces;

impl Plugin for Interfaces {
    fn visit_def(&self, ctx: &SchemaCtx, def: &Def) -> Result<proc_macro2::TokenStream> {
        let DefKind::Interface(interface_def) = &def.kind else {
            // We only generate code for interface definitions.
            return Ok(Default::default());
        };
        println!("Generating interface definition for {}.", def.name.as_str());
        let trait_name = format_ident!("{}Handler", def.name.as_str());
        let service_name = format_ident!("{}Service", def.name.as_str());
        let mod_name = format_ident!("{}", to_snake_case(def.name.as_str()));
        let docs = def
            .docs
            .as_ref()
            .map(|docs| docs.as_str())
            .unwrap_or_default();
        let vars = ctx.generic_type_vars(def);
        let vis = quote! { pub };

        let methods = interface_def
            .methods
            .iter()
            .map(|method| {
                let name = format_ident!("{}", &method.name.as_str());
                let docs = method
                    .docs
                    .as_ref()
                    .map(|docs| docs.as_str())
                    .unwrap_or_default();
                let args_name = format_ident!("{}Args", to_pascal_case(method.name.as_str()));

                let returns = method
                    .returns
                    .as_ref()
                    .map(|returns| {
                        let typ = ctx.resolve_type_old(def, returns, false);
                        quote! { -> impl ::std::future::Future<Output = __sidex_rpc_core::response::Response<#typ>> + Send }
                    })
                    .unwrap_or_else(|| quote! { -> impl ::std::future::Future<Output = __sidex_rpc_core::response::Response<()>> + Send });

                Ok(quote! {
                    #[doc = #docs]
                    fn #name(&self, request: __sidex_rpc_core::request::Request<#mod_name :: #args_name>) #returns;
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let arg_types = interface_def.methods.iter().map(|method| {
            let name = format!("{}Args", to_pascal_case(method.name.as_str()));
            let ident = format_ident!("{}", name);
            let docs = format!(
                "Arguments type for `{}::{}`.",
                def.name.as_str(),
                method.name.as_str()
            );
            let ty = RsType {
                name,
                ident,
                visibility: Visibility::Pub,
                vars: Default::default(),
                docs,
                meta: quote! {
                    #[derive(Debug, Clone)]
                },
                kind: RsTypeKind::Record(RsTypeRecord {
                    fields: method
                        .parameters
                        .iter()
                        .map(|param| {
                            let ident = format_ident!("{}", param.name.as_str());
                            let mut ty = ctx.resolve_type_old(def, &param.typ, true);
                            if param.is_optional {
                                ty = quote! { ::std::option::Option< #ty > };
                            }
                            let docs = format!("Argument `{}`.", param.name.as_str());
                            RsField {
                                name: param.name.identifier.clone(),
                                ident,
                                docs,
                                visibility: Visibility::Pub,
                                is_optional: param.is_optional,
                                json_attrs: JsonFieldAttrs::default(),
                                json_name: to_camel_case(param.name.as_str()),
                                ty,
                            }
                        })
                        .collect(),
                    json_attrs: Default::default(),
                }),
            };
            let rs_def = rs_type_to_rs_def(&ty);
            let serde_impl = rs_type_impl_serde(&ty);
            quote! {
                #rs_def
                #serde_impl
            }
        });

        // let response_name = format_ident!("{}Response", def.name.as_str());

        // let response_variants = interface_def.methods.iter().map(|method| {
        //     let name = format_ident!("{}", to_pascal_case(method.name.as_str()));
        //     let returns = method
        //         .returns
        //         .as_ref()
        //         .map(|returns| {
        //             let typ = ctx.resolve_type_old(def, returns);
        //             quote! { #typ }
        //         })
        //         .unwrap_or_else(|| quote! { () });

        //     quote! {
        //         #name ( #returns )
        //     }
        // });

        // let service_name = format_ident!("{}Service", def.name.as_str());

        let mod_doc = format!("Types for the `{}` interface.", def.name.as_str());

        let handle_match_arms = interface_def.methods.iter().map(|method| {
            let name = to_camel_case(method.name.as_str());
            let method_name = format_ident!("{}", method.name.as_str());
            quote! {
                #name => {
                    let Ok(x) = request.decode() else {
                        todo!("handle decode error")
                    };
                    ::std::boxed::Box::pin(async move {
                        let Ok(response) = handler.#method_name(x).await.encode() else {
                            todo!("handle encode error")
                        };
                        response
                    })
                }
            }
        });

        let service_bounds = interface_def.methods.iter().map(|method| {
            let name = format_ident!("{}Args", to_pascal_case(method.name.as_str()));
            let returns = method
                .returns
                .as_ref()
                .map(|returns| {
                    let typ = ctx.resolve_type_old(def, returns, false);
                    quote! { #typ }
                })
                .unwrap_or_else(|| quote! { () });
            quote! {
                __I: __sidex_rpc_core::codec::TryDecodeInto<#mod_name::#name>,
                __O: __sidex_rpc_core::codec::TryEncodeFrom<#returns>
            }
        });

        Ok(quote! {
            #[doc = #docs]
            #vis trait #trait_name {
                #(#methods)*
            }

            #[derive(Debug, Clone)]
            pub struct #service_name<__H> {
                handler: ::std::sync::Arc<__H>,
            }

            impl<__H> #service_name<__H> {
                pub fn new(handler: __H) -> Self {
                    Self {
                        handler: ::std::sync::Arc::new(handler),
                    }
                }
            }

            impl<__H: 'static + Send + Sync + #trait_name, __I: 'static + Send + Sync, __O: 'static + Send + Sync> __sidex_rpc_core::AsyncCall<__I, __O> for #service_name<__H> where #(#service_bounds),* {
                type Future = __sidex_rpc_core::BoxedCallFuture<__O>;

                fn call(&self, request: __sidex_rpc_core::request::Request<__I>) -> Self::Future {
                    let handler = self.handler.clone();
                    match request.parts.method.as_str() {
                        #(#handle_match_arms),*
                        _ => todo!("unknown method")
                    }
                }
            }

            // #vis mod #mod_name {
            //     use super::#trait_name;

            //     pub struct Server<T: #trait_name> {}
            // }

            #[doc = #mod_doc]
            #vis mod #mod_name {
                use super::#trait_name;

                use ::serde as __serde;
                use ::sidex_serde as __sidex_serde;

                #(#arg_types)*
            }



            // #vis enum #response_name < #vars > {
            //     #(#response_variants,)*
            // }

            //pub struct #service_name<__H: #trait_name>(__H);

            // #(#tower_impls)*
        })
    }

    fn visit_schema(&self, _: &SchemaCtx) -> Result<TokenStream> {
        Ok(quote! {
            #[allow(unused)]
            use ::sidex_rpc_core as __sidex_rpc_core;
        })
    }
}
