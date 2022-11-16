#![doc = include_str!("../README.md")]

use std::path::PathBuf;

use proc_macro::TokenStream;
use sidex_core::utils::load_unit_and_bundle;
use sidex_gen_rs::{config::Config, RustGenerator};
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
pub fn include_bundle(tokens: TokenStream) -> TokenStream {
    let args = parse_macro_input!(tokens as IncludeSidexArgs);

    let (unit, bundle) = load_unit_and_bundle(&args.path).unwrap();

    let generator = RustGenerator::new();

    let cfg = Config::default();

    let inner = generator
        .generate_bundle_inner(&cfg, &unit, bundle)
        .unwrap();

    inner.into()
}
