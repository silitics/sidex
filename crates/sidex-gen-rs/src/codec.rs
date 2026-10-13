//! Generation of the runtime traits of the `sidex` crate.
//!
//! For every record, variant, and wrapper, the generator implements `Shaped`,
//! `Encode`, and `Decode`. Shapes carry the documentation, raw attributes,
//! stability, validation rules, and JSON representation of a type.

use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use sidex_attrs_api::Stability;
use sidex_attrs_api::stability_of_def;
use sidex_attrs_api::stability_of_field;
use sidex_attrs_api::stability_of_variant;
use sidex_attrs_json::JsonTaggedAttr;
use sidex_attrs_validate as validate_attrs;
use sidex_gen::diagnostics::Diagnostic;
use sidex_gen::diagnostics::Result;
use sidex_gen::ir;
use sidex_validate_expr as expr;

use crate::context::SchemaCtx;
use crate::rstyir::RsType;
use crate::rstyir::RsTypeKind;

/// Generates the implementations of `Shaped`, `Encode`, and `Decode`.
pub fn impl_traits(ctx: &SchemaCtx, def: &ir::Def, ty: &RsType) -> Result<TokenStream> {
    let ident = &ty.ident;
    let vars = &ty.vars;
    let impl_header = |bounds: TokenStream| {
        quote! { impl<#(#vars: #bounds),*> }
    };
    let shaped_header = impl_header(quote!(::sidex::Shaped));
    let encode_header = impl_header(quote!(::sidex::Shaped + ::sidex::Encode));
    let decode_header = impl_header(quote!(::sidex::Shaped + ::sidex::Decode));
    let self_ty = quote! { #ident<#(#vars),*> };

    let path = format!(
        "::{}::{}::{}",
        ctx.bundle_ctx.bundle.metadata.name,
        ctx.schema.name,
        def.name.as_str()
    );
    let mut builders = shape_builders(
        def.docs.as_ref().map(|docs| docs.as_str()),
        &def.attrs,
        &stability_of_def(def),
    );
    if !vars.is_empty() {
        builders.extend(quote! { .with_args(&[#(::sidex::shape::shape_of::<#vars>),*]) });
    }

    let (kind, encode, decode) = match (&ty.kind, &def.kind) {
        (RsTypeKind::Wrapper(wrapper), ir::DefKind::WrapperType(_)) => {
            builders.extend(rules(&validate_attrs::wrapper_rules(def)?)?);
            let wrapped = &wrapper.wrapped;
            (
                quote! { ::sidex::shape::ShapeKind::Wrapper(::sidex::shape::shape_of::<#wrapped>) },
                quote! {
                    #[automatically_derived]
                    #encode_header ::sidex::Encode for #self_ty {
                        fn encode(&self) -> ::sidex::View<'_> {
                            ::sidex::Encode::encode(&self.0)
                        }
                    }
                },
                quote! {
                    #[automatically_derived]
                    #decode_header ::sidex::Decode for #self_ty {
                        fn decode(
                            decoder: &mut dyn ::sidex::Decoder,
                        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
                            ::std::result::Result::Ok(Self(
                                <#wrapped as ::sidex::Decode>::decode(decoder)?,
                            ))
                        }
                    }
                },
            )
        }
        (RsTypeKind::Record(record), ir::DefKind::RecordType(record_def)) => {
            builders.extend(rules(&validate_attrs::record_rules(def)?)?);
            let mut fields = Vec::new();
            let mut encode_arms = Vec::new();
            let mut decode_vars = Vec::new();
            let mut decode_arms = Vec::new();
            let mut construct = Vec::new();
            for (index, (field, field_def)) in
                record.fields.iter().zip(&record_def.fields).enumerate()
            {
                let name = &field.name;
                let field_ident = &field.ident;
                let inner_ty = &field.inner_ty;
                let mut field_builders = shape_builders(
                    field_def.docs.as_ref().map(|docs| docs.as_str()),
                    &field_def.attrs,
                    &stability_of_field(field_def),
                );
                field_builders.extend(rules(&validate_attrs::field_rules(field_def)?)?);
                if field.is_optional {
                    field_builders.extend(quote! { .optional() });
                }
                if &field.json_name != name {
                    let json_name = &field.json_name;
                    field_builders.extend(quote! { .json_name(#json_name) });
                }
                if field.json_attrs.inline {
                    field_builders.extend(quote! { .json_inline() });
                }
                fields.push(quote! {
                    ::sidex::shape::FieldShape::new(
                        #name,
                        ::sidex::shape::shape_of::<#inner_ty>,
                    ) #field_builders
                });

                let var = format_ident!("__field_{}", index);
                encode_arms.push(if field.is_optional {
                    quote! {
                        #index => self.#field_ident.as_ref().map(|value| ::sidex::Encode::encode(value)),
                    }
                } else {
                    quote! {
                        #index => ::std::option::Option::Some(::sidex::Encode::encode(&self.#field_ident)),
                    }
                });
                decode_vars.push(quote! {
                    let mut #var = ::std::option::Option::<#inner_ty>::None;
                });
                decode_arms.push(quote! {
                    #index => #var = ::std::option::Option::Some(
                        <#inner_ty as ::sidex::Decode>::decode(decoder)?,
                    ),
                });
                construct.push(if field.is_optional {
                    quote! { #field_ident: #var, }
                } else {
                    let missing = format!("missing field `{}`", field.json_name);
                    quote! {
                        #field_ident: #var.ok_or_else(|| ::sidex::DecodeError::new(#missing))?,
                    }
                });
            }
            (
                quote! {
                    ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                        #(#fields),*
                    ]))
                },
                quote! {
                    #[automatically_derived]
                    #encode_header ::sidex::Encode for #self_ty {
                        fn encode(&self) -> ::sidex::View<'_> {
                            ::sidex::View::Record(self)
                        }
                    }

                    #[automatically_derived]
                    #[allow(deprecated)]
                    #encode_header ::sidex::encode::RecordView for #self_ty {
                        fn shape(&self) -> &'static ::sidex::Shape {
                            <Self as ::sidex::Shaped>::SHAPE
                        }

                        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
                            match index {
                                #(#encode_arms)*
                                _ => ::std::option::Option::None,
                            }
                        }
                    }
                },
                quote! {
                    #[automatically_derived]
                    #[allow(deprecated)]
                    #decode_header ::sidex::Decode for #self_ty {
                        fn decode(
                            decoder: &mut dyn ::sidex::Decoder,
                        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
                            #(#decode_vars)*
                            decoder.record(
                                <Self as ::sidex::Shaped>::SHAPE,
                                &mut |index, decoder| {
                                    match index {
                                        #(#decode_arms)*
                                        _ => {}
                                    }
                                    ::std::result::Result::Ok(())
                                },
                            )?;
                            ::std::result::Result::Ok(Self { #(#construct)* })
                        }
                    }
                },
            )
        }
        (RsTypeKind::Variant(variant), ir::DefKind::VariantType(variant_def)) => {
            builders.extend(rules(&validate_attrs::record_rules(def)?)?);
            let mut cases = Vec::new();
            let mut case_arms = Vec::new();
            let mut payload_arms = Vec::new();
            let mut decode_arms = Vec::new();
            for (index, (case, case_def)) in variant
                .variants
                .iter()
                .zip(&variant_def.variants)
                .enumerate()
            {
                let name = &case.name;
                let case_ident = &case.ident;
                let mut case_builders = shape_builders(
                    case_def.docs.as_ref().map(|docs| docs.as_str()),
                    &case_def.attrs,
                    &stability_of_variant(case_def),
                );
                if &case.json_name != name {
                    let json_name = &case.json_name;
                    case_builders.extend(quote! { .json_name(#json_name) });
                }
                if let Some(content) = &case.json_attrs.content {
                    case_builders.extend(quote! { .json_content(#content) });
                }
                match &case.ty {
                    Some(payload_ty) => {
                        cases.push(quote! {
                            ::sidex::shape::CaseShape::new(
                                #name,
                                ::std::option::Option::Some(::sidex::shape::shape_of::<#payload_ty>),
                            ) #case_builders
                        });
                        case_arms.push(quote! { Self::#case_ident(_) => #index, });
                        payload_arms.push(quote! {
                            Self::#case_ident(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                        });
                        decode_arms.push(quote! {
                            #index => Self::#case_ident(<#payload_ty as ::sidex::Decode>::decode(decoder)?),
                        });
                    }
                    None => {
                        cases.push(quote! {
                            ::sidex::shape::CaseShape::new(#name, ::std::option::Option::None)
                                #case_builders
                        });
                        case_arms.push(quote! { Self::#case_ident => #index, });
                        payload_arms.push(quote! {
                            Self::#case_ident => ::std::option::Option::None,
                        });
                        decode_arms.push(quote! { #index => Self::#case_ident, });
                    }
                }
            }
            let tag = &variant.json_attrs.tag;
            let content = &variant.json_attrs.content;
            let tagging = match variant.json_attrs.tagged {
                JsonTaggedAttr::Internally if tag == "tag" && content == "content" => None,
                JsonTaggedAttr::Internally => {
                    Some(quote! {
                        ::sidex::formats::json::Tagging::Internal { tag: #tag, content: #content }
                    })
                }
                JsonTaggedAttr::Adjacently => {
                    Some(quote! {
                        ::sidex::formats::json::Tagging::Adjacent { tag: #tag, content: #content }
                    })
                }
                JsonTaggedAttr::Externally => {
                    Some(quote! { ::sidex::formats::json::Tagging::External })
                }
                JsonTaggedAttr::Implicitly => {
                    Some(quote! { ::sidex::formats::json::Tagging::Implicit })
                }
            };
            let tagging = tagging.map(|tagging| quote! { .json_tagging(#tagging) });
            let decode_body = if cases.is_empty() {
                quote! {
                    let _ = decoder;
                    ::std::result::Result::Err(::sidex::DecodeError::new("variant without cases"))
                }
            } else {
                quote! {
                    let mut out = ::std::option::Option::<Self>::None;
                    decoder.variant(
                        <Self as ::sidex::Shaped>::SHAPE,
                        &mut |case, decoder| {
                            out = ::std::option::Option::Some(match case {
                                #(#decode_arms)*
                                _ => {
                                    return ::std::result::Result::Err(
                                        ::sidex::DecodeError::new("unknown case"),
                                    );
                                }
                            });
                            ::std::result::Result::Ok(())
                        },
                    )?;
                    out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
                }
            };
            (
                quote! {
                    ::sidex::shape::ShapeKind::Variant(
                        ::sidex::shape::VariantShape::new(&[#(#cases),*]) #tagging
                    )
                },
                quote! {
                    #[automatically_derived]
                    #encode_header ::sidex::Encode for #self_ty {
                        fn encode(&self) -> ::sidex::View<'_> {
                            ::sidex::View::Variant(self)
                        }
                    }

                    #[automatically_derived]
                    #[allow(deprecated)]
                    #encode_header ::sidex::encode::VariantView for #self_ty {
                        fn shape(&self) -> &'static ::sidex::Shape {
                            <Self as ::sidex::Shaped>::SHAPE
                        }

                        fn case(&self) -> usize {
                            match *self {
                                #(#case_arms)*
                            }
                        }

                        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
                            match self {
                                #(#payload_arms)*
                            }
                        }
                    }
                },
                quote! {
                    #[automatically_derived]
                    #[allow(deprecated)]
                    #decode_header ::sidex::Decode for #self_ty {
                        fn decode(
                            decoder: &mut dyn ::sidex::Decoder,
                        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
                            #decode_body
                        }
                    }
                },
            )
        }
        _ => return Ok(TokenStream::new()),
    };

    Ok(quote! {
        #[automatically_derived]
        #shaped_header ::sidex::Shaped for #self_ty {
            const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(#path, #kind) #builders;
        }

        #encode

        #decode
    })
}

/// Generates the builder calls for documentation, attributes, and stability.
fn shape_builders(docs: Option<&str>, attrs: &[ir::Attr], stability: &Stability) -> TokenStream {
    let mut builders = TokenStream::new();
    if let Some(docs) = docs {
        builders.extend(quote! { .with_docs(#docs) });
    }
    if !attrs.is_empty() {
        let attrs = attrs.iter().map(attr);
        builders.extend(quote! { .with_attrs(&[#(#attrs),*]) });
    }
    if !stability.is_empty() {
        let mut stability_builders = TokenStream::new();
        if let Some(since) = &stability.since {
            let version = &since.version;
            stability_builders.extend(quote! { .since(#version) });
        }
        if let Some(deprecated) = &stability.deprecated {
            let mut deprecated_builders = TokenStream::new();
            if let Some(since) = &deprecated.since {
                deprecated_builders.extend(quote! { .since(#since) });
            }
            if let Some(remove_in) = &deprecated.remove_in {
                deprecated_builders.extend(quote! { .remove_in(#remove_in) });
            }
            if let Some(note) = &deprecated.note {
                deprecated_builders.extend(quote! { .note(#note) });
            }
            stability_builders.extend(quote! {
                .deprecated(::sidex::shape::stability::Deprecated::new() #deprecated_builders)
            });
        }
        if let Some(unstable) = &stability.unstable {
            let mut unstable_builders = TokenStream::new();
            if let Some(feature) = &unstable.feature {
                unstable_builders.extend(quote! { .feature(#feature) });
            }
            if let Some(note) = &unstable.note {
                unstable_builders.extend(quote! { .note(#note) });
            }
            if let Some(issue) = &unstable.issue {
                unstable_builders.extend(quote! { .issue(#issue) });
            }
            stability_builders.extend(quote! {
                .unstable(::sidex::shape::stability::Unstable::new() #unstable_builders)
            });
        }
        builders.extend(quote! {
            .with_stability(::sidex::shape::stability::Stability::new() #stability_builders)
        });
    }
    builders
}

/// Generates a raw attribute.
fn attr(attr: &ir::Attr) -> TokenStream {
    match &attr.kind {
        ir::AttrKind::Path(path) => quote! { ::sidex::shape::Attr::Path(#path) },
        ir::AttrKind::List(list) => {
            let path = &list.path;
            let args = list.args.iter().map(self::attr);
            quote! { ::sidex::shape::Attr::List { path: #path, args: &[#(#args),*] } }
        }
        ir::AttrKind::Assign(assign) => {
            let path = &assign.path;
            let value = attr_value(&assign.value);
            quote! { ::sidex::shape::Attr::Assign { path: #path, value: #value } }
        }
        ir::AttrKind::Value(value) => {
            let value = attr_value(value);
            quote! { ::sidex::shape::Attr::Value(#value) }
        }
    }
}

/// Generates the value of a raw attribute.
fn attr_value(value: &ir::AttrValue) -> TokenStream {
    match value {
        ir::AttrValue::Bool(value) => quote! { ::sidex::shape::AttrValue::Bool(#value) },
        ir::AttrValue::Number(value) => quote! { ::sidex::shape::AttrValue::Number(#value) },
        ir::AttrValue::String(value) => quote! { ::sidex::shape::AttrValue::String(#value) },
        ir::AttrValue::Path(value) => quote! { ::sidex::shape::AttrValue::Path(#value) },
        ir::AttrValue::Tokens(tokens) => {
            let text = &tokens.text;
            quote! { ::sidex::shape::AttrValue::Tokens(#text) }
        }
    }
}

/// Validation rule as written in a `#[validate(...)]` attribute.
pub(crate) trait SourceRule {
    fn source(&self) -> &str;
    fn message(&self) -> Option<&str>;
    fn code(&self) -> Option<&str>;
}

macro_rules! impl_source_rule {
    ($($ty:ty),*) => {
        $(
            impl SourceRule for $ty {
                fn source(&self) -> &str { &self.expr.text }
                fn message(&self) -> Option<&str> { self.message.as_deref() }
                fn code(&self) -> Option<&str> { self.code.as_deref() }
            }
        )*
    };
}

impl_source_rule!(
    validate_attrs::raw::FieldRule,
    validate_attrs::raw::RecordRule,
    validate_attrs::raw::WrapperRule,
    validate_attrs::raw::OpaqueRule
);

/// Generates the `with_rules` builder call for the given rules.
fn rules<R: SourceRule>(rules: &[R]) -> Result<TokenStream> {
    if rules.is_empty() {
        return Ok(TokenStream::new());
    }
    let rules = rules.iter().map(rule).collect::<Result<Vec<_>>>()?;
    Ok(quote! { .with_rules(&[#(#rules),*]) })
}

/// Generates a validation rule.
fn rule<R: SourceRule>(rule: &R) -> Result<TokenStream> {
    let source = rule.source().trim();
    let invalid = |message: &str| {
        Box::new(Diagnostic::error(format!(
            "Invalid `#[validate]` rule `{source}`: {message}"
        )))
    };
    let parsed = expr::parse(source).map_err(|error| invalid(&error.message))?;
    let recognized = expr::recognize(&parsed, source, expr::Target::Other);
    if let Some(error) = recognized.errors.first() {
        return Err(invalid(&error.message));
    }
    let expr = match &parsed {
        expr::Expr::Compare { operands, ops } => {
            let operands = operands
                .iter()
                .map(|operand| self::operand(operand).ok_or_else(|| invalid("invalid operand")))
                .collect::<Result<Vec<_>>>()?;
            let ops = ops.iter().map(|op| {
                match op {
                    expr::CmpOp::Lt => quote! { ::sidex::validation::CmpOp::Lt },
                    expr::CmpOp::Le => quote! { ::sidex::validation::CmpOp::Le },
                    expr::CmpOp::Eq => quote! { ::sidex::validation::CmpOp::Eq },
                    expr::CmpOp::Ne => quote! { ::sidex::validation::CmpOp::Ne },
                    expr::CmpOp::Ge => quote! { ::sidex::validation::CmpOp::Ge },
                    expr::CmpOp::Gt => quote! { ::sidex::validation::CmpOp::Gt },
                }
            });
            quote! {
                ::sidex::validation::Expr::Compare {
                    operands: &[#(#operands),*],
                    ops: &[#(#ops),*],
                }
            }
        }
        expr::Expr::Call { name, args } if name == "matches" => {
            let [operand, expr::Expr::Lit(expr::Literal::String(pattern))] = args.as_slice() else {
                return Err(invalid("expected `matches(<operand>, \"<pattern>\")`"));
            };
            let operand = self::operand(operand).ok_or_else(|| invalid("invalid operand"))?;
            quote! {
                ::sidex::validation::Expr::Matches { operand: #operand, pattern: #pattern }
            }
        }
        _ => return Err(invalid("expected a comparison or `matches`")),
    };
    let source = render_expr(&parsed);
    let mut builders = TokenStream::new();
    if let Some(message) = rule.message() {
        builders.extend(quote! { .with_message(#message) });
    }
    if let Some(code) = rule.code() {
        builders.extend(quote! { .with_code(#code) });
    }
    Ok(quote! { ::sidex::validation::Rule::new(#source, #expr) #builders })
}

/// Renders an expression as canonical source text.
fn render_expr(expr: &expr::Expr) -> String {
    match expr {
        expr::Expr::Underscore => "_".to_owned(),
        expr::Expr::Member { path } => format!("_.{}", path.join(".")),
        expr::Expr::Lit(expr::Literal::Number(number)) => number.clone(),
        expr::Expr::Lit(expr::Literal::String(string)) => format!("{string:?}"),
        expr::Expr::Call { name, args } => {
            let args = args.iter().map(render_expr).collect::<Vec<_>>();
            format!("{name}({})", args.join(", "))
        }
        expr::Expr::Compare { operands, ops } => {
            let mut out = render_expr(&operands[0]);
            for (op, operand) in ops.iter().zip(&operands[1..]) {
                out.push_str(&format!(" {op} {}", render_expr(operand)));
            }
            out
        }
    }
}

/// Generates an operand of a validation rule.
fn operand(operand: &expr::Expr) -> Option<TokenStream> {
    Some(match operand {
        expr::Expr::Underscore => quote! { ::sidex::validation::Operand::Value },
        expr::Expr::Member { path } => {
            quote! { ::sidex::validation::Operand::Member(&[#(#path),*]) }
        }
        expr::Expr::Lit(expr::Literal::String(value)) => {
            quote! { ::sidex::validation::Operand::String(#value) }
        }
        expr::Expr::Lit(expr::Literal::Number(number)) => {
            if let Ok(value) = number.parse::<i128>() {
                let value = Literal::i128_suffixed(value);
                quote! { ::sidex::validation::Operand::Int(#value) }
            } else {
                let value = Literal::f64_suffixed(number.parse::<f64>().ok()?);
                quote! { ::sidex::validation::Operand::Float(#value) }
            }
        }
        _ => return None,
    })
}
