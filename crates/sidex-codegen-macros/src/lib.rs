extern crate proc_macro;

mod expand;

use proc_macro::TokenStream;
use syn::LitStr;
use syn::parse_macro_input;

/// Indentation-aware code generation macro.
///
/// Takes a string literal template and produces a `Code` value at runtime.
/// Variables in scope are interpolated with `@var`. Iteration uses `@(...)*`
/// (vertical) or `@(...)+` (horizontal).
#[proc_macro]
pub fn quote(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitStr);
    let template_str = lit.value();

    let template = match sidex_codegen_syntax::parse(&template_str) {
        Ok(t) => t,
        Err(e) => {
            return syn::Error::new(lit.span(), e.message)
                .to_compile_error()
                .into();
        }
    };

    let expanded = expand::expand(&template);
    expanded.into()
}
