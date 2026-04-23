use proc_macro2::TokenStream;
use quote::quote;
use sidex_codegen_syntax::ast::{Fragment, IterMode};

/// Expands a parsed template into a TokenStream that builds a `Code` value.
pub fn expand(template: &sidex_codegen_syntax::Template) -> TokenStream {
    let body = expand_fragments(&template.fragments);
    quote! {
        {
            let mut __code = ::sidex_codegen::Code::new();
            #body
            __code
        }
    }
}

fn expand_fragments(fragments: &[Fragment]) -> TokenStream {
    let mut stmts = TokenStream::new();
    for fragment in fragments {
        stmts.extend(expand_fragment(fragment));
    }
    stmts
}

fn expand_fragment(fragment: &Fragment) -> TokenStream {
    match fragment {
        Fragment::Literal(text) => {
            quote! { __code.push_static(#text); }
        }
        Fragment::Newline => {
            quote! { __code.push_newline(); }
        }
        Fragment::Interpolation { var, column } => {
            let var_ident = syn::Ident::new(var, proc_macro2::Span::call_site());
            quote! {
                __code.interpolate(
                    ::sidex_codegen::ToCode::to_code(&#var_ident),
                    #column,
                );
            }
        }
        Fragment::Iteration {
            body,
            mode,
            separator,
            column,
        } => {
            let vars = collect_vars(body);
            if vars.is_empty() {
                return TokenStream::new();
            }

            let inner_body = expand_iteration_body(body);

            let iter_expr = build_iter_expr(&vars);
            let destructure = build_destructure(&vars);

            match mode {
                IterMode::Vertical => {
                    quote! {
                        __code.join_vertical(
                            #iter_expr.map(|#destructure| {
                                let mut __code = ::sidex_codegen::Code::new();
                                #inner_body
                                __code
                            }),
                            #column,
                            #separator,
                        );
                    }
                }
                IterMode::Horizontal => {
                    quote! {
                        __code.join_horizontal(
                            #iter_expr.map(|#destructure| {
                                let mut __code = ::sidex_codegen::Code::new();
                                #inner_body
                                __code
                            }),
                            #separator,
                        );
                    }
                }
            }
        }
    }
}

/// Expands iteration body fragments — same as expand_fragments but uses the
/// local `__code` (shadowed inside the closure).
fn expand_iteration_body(fragments: &[Fragment]) -> TokenStream {
    expand_fragments(fragments)
}

/// Collects all unique variable names referenced in a fragment list.
fn collect_vars(fragments: &[Fragment]) -> Vec<String> {
    let mut vars = Vec::new();
    collect_vars_inner(fragments, &mut vars);
    vars
}

fn collect_vars_inner(fragments: &[Fragment], vars: &mut Vec<String>) {
    for fragment in fragments {
        match fragment {
            Fragment::Interpolation { var, .. } => {
                if !vars.contains(var) {
                    vars.push(var.clone());
                }
            }
            Fragment::Iteration { body, .. } => {
                collect_vars_inner(body, vars);
            }
            Fragment::Literal(_) | Fragment::Newline => {}
        }
    }
}

/// Builds the chained iterator expression for one or more variables.
fn build_iter_expr(vars: &[String]) -> TokenStream {
    assert!(!vars.is_empty());
    let first = syn::Ident::new(&vars[0], proc_macro2::Span::call_site());
    let mut expr = quote! { ::std::iter::IntoIterator::into_iter(&#first) };
    for var in &vars[1..] {
        let ident = syn::Ident::new(var, proc_macro2::Span::call_site());
        expr = quote! { #expr.zip(::std::iter::IntoIterator::into_iter(&#ident)) };
    }
    expr
}

/// Builds the destructuring pattern for the `.map()` closure.
/// zip produces nested tuples: (a, b) for 2 vars, ((a, b), c) for 3, etc.
fn build_destructure(vars: &[String]) -> TokenStream {
    assert!(!vars.is_empty());
    let idents: Vec<syn::Ident> = vars
        .iter()
        .map(|v| syn::Ident::new(v, proc_macro2::Span::call_site()))
        .collect();

    if idents.len() == 1 {
        let id = &idents[0];
        quote! { #id }
    } else {
        build_nested_destructure(&idents)
    }
}

/// Builds a nested tuple destructure pattern matching zip's nesting.
/// For [a, b, c]: ((a, b), c)
/// For [a, b, c, d]: (((a, b), c), d)
fn build_nested_destructure(idents: &[syn::Ident]) -> TokenStream {
    assert!(idents.len() >= 2);
    let first = &idents[0];
    let second = &idents[1];
    let mut pattern = quote! { (#first, #second) };
    for ident in &idents[2..] {
        pattern = quote! { (#pattern, #ident) };
    }
    pattern
}
