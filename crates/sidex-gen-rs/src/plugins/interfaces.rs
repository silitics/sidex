use quote::{format_ident, quote};
use sidex_gen::{
    diagnostics::Result,
    ir::{Def, DefKind},
};

use super::Plugin;
use crate::context::SchemaCtx;

pub struct Interfaces;

impl Plugin for Interfaces {
    fn visit_def(&self, ctx: &SchemaCtx, def: &Def) -> Result<proc_macro2::TokenStream> {
        let DefKind::Service(interface_def) = &def.kind else {
            // We only generate code for interface definitions.
            return Ok(Default::default())
        };
        println!("Generating interface definition for {}.", def.name.as_str());
        let trait_name = format_ident!("{}", def.name.as_str());
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
                let parameters = method.parameters.iter().map(|param| {
                    let name = format_ident!("{}", param.name.as_str());
                    let mut typ = ctx.resolve_type_old(def, &param.typ);
                    if param.is_optional {
                        typ = quote! { ::std::option::Option< #typ > };
                    }
                    quote! { #name: #typ }
                });

                let returns = method
                    .returns
                    .as_ref()
                    .map(|returns| {
                        let typ = ctx.resolve_type_old(def, returns);
                        quote! { -> crate::rpc::BoxedFuture<'_, #typ> }
                    })
                    .unwrap_or_else(|| quote! { -> crate::rpc::BoxedFuture<'_, ()> });

                Ok(quote! {
                    #[doc = #docs]
                    fn #name(&self, #(#parameters,)* ) #returns;
                })
            })
            .collect::<Result<Vec<_>>>()?;

        // let request_types = interface_def.methods.iter().map(|method| {
        //     let name = format_ident!("{}Request", to_pascal_case(method.name.as_str()));
        //     let parameters = method.parameters.iter().map(|param| {
        //         let name = format_ident!("{}", param.name.as_str());
        //         let mut typ = ctx.resolve_type_old(def, &param.typ);
        //         if param.is_optional {
        //             typ = quote! { ::std::option::Option< #typ > };
        //         }
        //         quote! { #name: #typ }
        //     });
        //     let doc = format!(
        //         "Request type for [`{}::{}`].",
        //         def.name.as_str(),
        //         method.name.as_str()
        //     );
        //     let returns = method
        //         .returns
        //         .as_ref()
        //         .map(|returns| {
        //             let typ = ctx.resolve_type_old(def, returns);
        //             quote! { #typ }
        //         })
        //         .unwrap_or_else(|| quote! { () });

        //     quote! {
        //         #[doc = #doc]
        //         pub struct #name {
        //             #(#parameters,)*
        //         }

        //         impl ::sidex::rpc::Request for #name {
        //             type Response = #returns;
        //         }
        //     }
        // });

        // let request_name = format_ident!("{}Request", def.name.as_str());

        // let request_variants = interface_def.methods.iter().map(|method| {
        //     let name = format_ident!("{}", to_pascal_case(method.name.as_str()));
        //     let request_name = format_ident!("{}Request",
        // to_pascal_case(method.name.as_str()));     quote! {
        //         #name ( #request_name )
        //     }
        // });

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

        // let mod_name = format_ident!("{}", to_snake_case(def.name.as_str()));

        // let service_name = format_ident!("{}Service", def.name.as_str());

        // let mod_doc = format!("Request and response types for [`{}`].", def.name.as_str());

        // let handle_match_arms = interface_def.methods.iter().map(|method| {
        //     let fun_name = format_ident!("{}", method.name.as_str());
        //     let name = format_ident!("{}", to_pascal_case(method.name.as_str()));
        //     let args = method.parameters.iter().map(|param| {
        //         let name = format_ident!("{}", param.name.as_str());
        //         quote! { __request . #name }
        //     });

        //     quote! {
        //         #request_name :: #name ( __request ) => {
        //             #response_name :: #name ( self . #fun_name ( #(#args,)* ).await )
        //         }
        //     }
        // });

        Ok(quote! {
            #[doc = #docs]
            #vis trait #trait_name < #vars > {
                #(#methods)*
            }

            // #vis mod #mod_name {
            //     use super::#trait_name;

            //     pub struct Server<T: #trait_name> {}
            // }

            // #(#request_types)*

            // #vis enum #request_name < #vars > {
            //     #(#request_variants,)*
            // }

            // #vis enum #response_name < #vars > {
            //     #(#response_variants,)*
            // }

            //pub struct #service_name<__H: #trait_name>(__H);

            // #(#tower_impls)*



        })
    }
}
