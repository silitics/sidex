#![doc = include_str!("../README.md")]

use std::{
    env,
    path::{Path, PathBuf},
};

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

    let ctx = sidex_core::diagnostics::DiagnosticCtx::new();

    let out = ctx.exec(|| {
        let cargo_manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();

        let bundle_path = Path::new(&cargo_manifest_dir).join(args.path);

        let (unit, bundle, transformer) = load_unit_and_bundle(&bundle_path).unwrap();

        let generator = RustGenerator::new();

        let null = serde_json::Value::Null;
        let config = transformer
            .get_bundle_manifest(bundle)
            .backend
            .get("rust")
            .unwrap_or(&null)
            .clone();

        let cfg = serde_json::from_value::<Config>(config).unwrap();

        let inner = generator.generate_macro(&cfg, &unit, bundle).unwrap();

        inner
    });

    out.into()
}
