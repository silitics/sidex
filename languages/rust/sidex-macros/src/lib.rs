#![doc = include_str!("../README.md")]

use std::path::PathBuf;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Lit};

struct IncludeSidexArgs {
    pub path: PathBuf,
}

impl Parse for IncludeSidexArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = match input.parse::<Lit>()? {
            Lit::Str(path) => path.value().into(),
            _ => todo!(),
        };
        Ok(Self { path })
    }
}

/// Procedural macro for including a Sidex definition.
#[proc_macro]
pub fn include_definition(tokens: TokenStream) -> TokenStream {
    let args = parse_macro_input!(tokens as IncludeSidexArgs);

    let manifest_path = sidex_core::model::try_locate_manifest(&args.path).unwrap();
    // let (unit, root_id) = match sidex_core::elaborate::load(manifest_path) {
    //     Ok(loaded) => loaded,
    //     Err(err) => {
    //         let message = err.to_string();
    //         return quote! { compile_error!(#message); }.into();
    //     }
    // };

    // let code = sidex_gen_rs::generate_code(&unit, root_id);

    // TokenStream::from(code)
    Default::default()
}
