use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use sidex_gen::{
    diagnostics::Result,
    ir::{Def, DefKind},
    rename::{to_camel_case, to_pascal_case, to_snake_case},
};

use super::Plugin;
use crate::context::SchemaCtx;

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
        let mod_name = format_ident!("{}_rpc", to_snake_case(def.name.as_str()));
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
                let param_name = format_ident!("{}", to_pascal_case(method.name.as_str()));

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
                    fn #name(&self, request: __sidex_rpc_core::request::Request<#mod_name :: #param_name>) #returns;
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let request_types = interface_def.methods.iter().map(|method| {
            let name = format_ident!("{}", to_pascal_case(method.name.as_str()));
            let parameters = method.parameters.iter().map(|param| {
                let name = format_ident!("{}", param.name.as_str());
                let mut typ = ctx.resolve_type_old(def, &param.typ, true);
                if param.is_optional {
                    typ = quote! { ::std::option::Option< #typ > };
                }
                quote! { #name: #typ }
            });
            let doc = format!(
                "Method type for [`{}::{}`].",
                def.name.as_str(),
                method.name.as_str()
            );
            let returns = method
                .returns
                .as_ref()
                .map(|returns| {
                    let typ = ctx.resolve_type_old(def, returns, true);
                    quote! { #typ }
                })
                .unwrap_or_else(|| quote! { () });

            quote! {
                #[doc = #doc]
                pub struct #name {
                    #(#parameters,)*
                }
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

        let mod_doc = format!("Request and response types for [`{}`].", def.name.as_str());

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
            let name = format_ident!("{}", to_pascal_case(method.name.as_str()));
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

                #(#request_types)*
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
