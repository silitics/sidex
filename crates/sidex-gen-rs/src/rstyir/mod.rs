//! Rust Type Intermediate Representation (RsTyIR)

use std::str::FromStr;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use sidex_attrs_json::JsonFieldAttrs;
use sidex_attrs_json::JsonRecordTypeAttrs;
use sidex_attrs_json::JsonVariantAttrs;
use sidex_attrs_json::JsonVariantTypeAttrs;
use sidex_attrs_json::field_attrs as json_field_attrs;
use sidex_attrs_json::opaque_type_attrs as json_opaque_type_attrs;
use sidex_attrs_json::record_type_attrs as json_record_type_attrs;
use sidex_attrs_json::types::JsonShape;
use sidex_attrs_json::types::JsonType;
use sidex_attrs_json::variant_attrs as json_variant_attrs;
use sidex_attrs_json::variant_type_attrs as json_variant_type_attrs;
use sidex_attrs_rust::FieldAttrs;
use sidex_attrs_rust::Visibility;
use sidex_attrs_rust::field_attrs as rust_field_attrs;
use sidex_attrs_rust::type_attrs as rust_type_attrs;
use sidex_gen::diagnostics::Result;
use sidex_gen::ir::Def;
use sidex_gen::ir::DefKind;

use crate::context::SchemaCtx;

pub mod serde;

pub fn rs_type_to_rs_def(rs_type: &RsType) -> TokenStream {
    let RsType {
        ident,
        visibility,
        docs,
        meta,
        kind,
        ..
    } = &rs_type;

    let type_generics = rs_type.type_generics();

    match kind {
        RsTypeKind::Alias(rs_type_alias) => {
            let RsTypeAlias { aliased } = rs_type_alias;
            quote! {
                #[doc = #docs]
                #visibility type #ident <#type_generics> = #aliased;
            }
        }
        RsTypeKind::Wrapper(rs_type_wrapper) => {
            let RsTypeWrapper { wrapped, .. } = rs_type_wrapper;
            quote! {
                #[doc = #docs]
                #meta
                #visibility struct #ident <#type_generics> (pub(crate) #wrapped);

                impl <#type_generics> ::std::convert::From<#ident <#type_generics>> for #wrapped {
                    fn from(wrapped: #ident) -> Self {
                        wrapped.0
                    }
                }
            }
        }
        RsTypeKind::Record(rs_type_record) => {
            let RsTypeRecord { fields, .. } = rs_type_record;
            let fields = fields.iter().map(
                |RsField {
                     ident,
                     docs,
                     visibility,
                     ty,
                     ..
                 }| {
                    quote! {
                       #[doc = #docs]
                       #visibility #ident: #ty,
                    }
                },
            );
            quote! {
                #[doc = #docs]
                #meta
                #visibility struct #ident <#type_generics> {
                    #(#fields)*
                }
            }
        }
        RsTypeKind::Variant(rs_type_variant) => {
            let RsTypeVariant { variants, .. } = rs_type_variant;
            let variants = variants
                .iter()
                .map(
                    |RsVariant {
                         ident, docs, ty, ..
                     }| {
                        if let Some(ty) = &ty {
                            quote! {
                                #[doc = #docs]
                                #ident(#ty),
                            }
                        } else {
                            quote! {
                                #[doc = #docs]
                                #ident,
                            }
                        }
                    },
                )
                .collect::<Vec<_>>();
            quote! {
                #[doc = #docs]
                #meta
                #visibility enum #ident <#type_generics> {
                    #(#variants)*
                }
            }
        }
    }
}

pub fn rs_type_from_def(ctx: &SchemaCtx, def: &Def) -> Result<Option<RsType>> {
    let vars = def
        .vars
        .iter()
        .map(|var| format_ident!("{}", var.name.as_str()))
        .collect();
    let docs = def
        .docs
        .as_ref()
        .map(|docs| docs.as_str())
        .unwrap_or_default()
        .to_owned();
    let attrs = rust_type_attrs(def)?;
    let mut derive = ctx
        .bundle_ctx
        .cfg
        .derive
        .iter()
        .map(|path| TokenStream::from_str(path).unwrap())
        .collect::<Vec<_>>();
    // In plain-JSON lowering, per-type `#[rust(derive(...))]` attributes are
    // skipped: they're tuned to whatever native opaque types the user picked,
    // e.g. `Copy` works on `uuid::Uuid` but not on the `serde_json::Value`
    // alias the JSON lowering substitutes.
    let strip_type_derives = matches!(
        ctx.bundle_ctx.cfg.opaque_lowering,
        crate::config::OpaqueLowering::Json,
    );
    if !strip_type_derives {
        for derive_trait in &attrs.derive.positive {
            derive.push(derive_trait.clone());
        }
    }
    let attr_iter: &[TokenStream] = if strip_type_derives {
        &[]
    } else {
        &attrs.attrs
    };
    let meta = quote! {
        #(#[derive(#derive)])*
        #(#[#attr_iter])*
    };
    let kind = match &def.kind {
        DefKind::WrapperType(wrapper_type_def) => {
            let wrapped = ctx.resolve_type_old(def, &wrapper_type_def.wrapped, false);
            let wrapped_encoding = ctx.resolve_encoding(def, &wrapper_type_def.wrapped);
            RsTypeKind::Wrapper(RsTypeWrapper {
                wrapped,
                wrapped_encoding,
            })
        }
        DefKind::TypeAlias(alias) => {
            let aliased = ctx.resolve_type_old(def, &alias.aliased, false);
            RsTypeKind::Alias(RsTypeAlias { aliased })
        }
        DefKind::OpaqueType(_) => {
            // In `json` lowering mode the per-target `#[rust(typ = ...)]` is
            // ignored — opaques become a transparent alias to the JSON-shape
            // primitive. This is what test drivers use to round-trip values
            // by their canonical JSON shape rather than a strict native type.
            let use_json = matches!(
                ctx.bundle_ctx.cfg.opaque_lowering,
                crate::config::OpaqueLowering::Json,
            );
            let aliased = if use_json {
                opaque_json_lowering(def)?
            } else {
                match &attrs.typ {
                    Some(typ) => TokenStream::from_str(&typ.path).unwrap(),
                    None => todo!(),
                }
            };
            RsTypeKind::Alias(RsTypeAlias { aliased })
        }
        DefKind::RecordType(typ) => {
            let ty_json_attrs = json_record_type_attrs(def)?;
            let fields = typ
                .fields
                .iter()
                .map(|field| {
                    let attrs = rust_field_attrs(field)?;

                    let name = format_ident!(
                        "{}",
                        attrs.name.as_deref().unwrap_or_else(|| field.name.as_str())
                    );
                    let docs = field
                        .docs
                        .as_ref()
                        .map(|docs| docs.as_str())
                        .unwrap_or_default()
                        .to_owned();
                    let mut inner_ty = ctx.resolve_type_old(def, &field.typ, false);
                    let mut inner_encoding = ctx.resolve_encoding(def, &field.typ);
                    for wrapper in attrs.wrappers {
                        let wrapper_path = TokenStream::from_str(&wrapper.wrapper).unwrap();
                        inner_encoding =
                            wrap_encoding(&wrapper.wrapper, &wrapper_path, &inner_encoding);
                        inner_ty = quote! { #wrapper_path < #inner_ty > };
                    }
                    let (ty, encoding) = if field.is_optional {
                        (
                            quote! { ::std::option::Option< #inner_ty > },
                            quote! { ::std::option::Option< #inner_encoding > },
                        )
                    } else {
                        (inner_ty.clone(), inner_encoding.clone())
                    };
                    let json_attrs = json_field_attrs(field)?;
                    Ok(RsField {
                        name: field.name.name.clone(),
                        ident: name,
                        docs,
                        visibility: attrs.visibility.clone(),
                        is_optional: field.is_optional,
                        json_name: ty_json_attrs.field_name(field, &json_attrs),
                        json_attrs,
                        ty,
                        encoding,
                        inner_ty,
                        inner_encoding,
                    })
                })
                .collect::<Result<Vec<_>>>()?;

            RsTypeKind::Record(RsTypeRecord {
                fields,
                json_attrs: ty_json_attrs,
            })
        }
        DefKind::VariantType(typ) => {
            let ty_json_attrs = json_variant_type_attrs(def)?;
            let variants = typ
                .variants
                .iter()
                .map(|variant| {
                    let name = format_ident!("{}", &variant.name.as_str());
                    let docs = variant
                        .docs
                        .as_ref()
                        .map(|docs| docs.as_str())
                        .unwrap_or_default()
                        .to_owned();
                    let ty = if let Some(typ) = &variant.typ {
                        Some(ctx.resolve_type_old(def, typ, false))
                    } else {
                        None
                    };
                    let encoding = variant
                        .typ
                        .as_ref()
                        .map(|typ| ctx.resolve_encoding(def, typ));
                    let json_attrs = json_variant_attrs(variant)?;
                    Ok(RsVariant {
                        name: variant.name.name.clone(),
                        docs,
                        ident: name,
                        json_name: ty_json_attrs.variant_name(variant, &json_attrs),
                        json_attrs,
                        is_record: if let Some(typ) = &variant.typ {
                            let resolved = ctx.bundle_ctx.unit.resolve_aliases(typ);
                            ctx.bundle_ctx.unit.record_type(&resolved).is_some()
                        } else {
                            false
                        },
                        ty,
                        encoding,
                    })
                })
                .collect::<Result<Vec<_>>>()?;

            RsTypeKind::Variant(RsTypeVariant {
                variants,
                json_attrs: ty_json_attrs,
            })
        }
        _ => {
            // Service definitions and derived types are handled separately.
            return Ok(None);
        }
    };
    Ok(Some(RsType {
        name: def.name.name.clone(),
        ident: format_ident!("{}", def.name.as_str()),
        visibility: Visibility::Pub,
        vars,
        docs,
        meta,
        kind,
    }))
}

/// Wrap an inner [`SerializeAs`]/[`DeserializeAs`] encoding in a user-supplied
/// field wrapper.
///
/// For the standard library's transparent shared-pointer wrappers (`Box`,
/// `Arc`, `Rc`) Sidex provides blanket encoding propagation, so the inner
/// encoding bubbles up through the wrapper. For other wrappers we don't know
/// their `SerializeAs` impl shape, so we collapse to [`AsSelf`] over the
/// wrapped type — string-encoded `i64`/`u64`/floats inside an arbitrary
/// user wrapper degrade to their native serde encoding.
///
/// [`SerializeAs`]: sidex_serde::SerializeAs
/// [`DeserializeAs`]: sidex_serde::DeserializeAs
/// [`AsSelf`]: sidex_serde::AsSelf
fn wrap_encoding(
    wrapper_path: &str,
    wrapper_tokens: &TokenStream,
    inner_encoding: &TokenStream,
) -> TokenStream {
    let propagates = matches!(
        wrapper_path,
        "::std::boxed::Box" | "::std::sync::Arc" | "::std::rc::Rc" | "Box" | "Arc" | "Rc"
    );
    if propagates {
        quote! { #wrapper_tokens < #inner_encoding > }
    } else {
        quote! { __sidex_serde::AsSelf }
    }
}

/// Lower an opaque definition to a Rust type token stream that mirrors its
/// `#[json(type = ...)]`-declared JSON shape, defaulting to
/// `serde_json::Value` when no JSON attribute is set.
fn opaque_json_lowering(def: &Def) -> Result<TokenStream> {
    let union = match json_opaque_type_attrs(def)?.typ {
        Some(union) => union,
        None => return Ok(quote! { ::serde_json::Value }),
    };
    Ok(match union.classify() {
        JsonShape::Single(t) => json_type_to_rs(t),
        JsonShape::NullableSingle(t) => {
            let inner = json_type_to_rs(t);
            quote! { ::std::option::Option<#inner> }
        }
        JsonShape::Other => quote! { ::serde_json::Value },
    })
}

fn json_type_to_rs(ty: JsonType) -> TokenStream {
    match ty {
        JsonType::String => quote! { ::std::string::String },
        JsonType::Number => quote! { f64 },
        JsonType::Boolean => quote! { bool },
        JsonType::Null => quote! { () },
        JsonType::Array => quote! { ::std::vec::Vec<::serde_json::Value> },
        JsonType::Object => {
            quote! { ::serde_json::Map<::std::string::String, ::serde_json::Value> }
        }
        JsonType::Any => quote! { ::serde_json::Value },
    }
}

pub type RsTypePath = TokenStream;

#[derive(Debug, Clone)]
pub struct RsType {
    pub name: String,
    pub ident: Ident,
    pub visibility: Visibility,
    pub vars: Vec<Ident>,
    pub docs: String,
    pub meta: TokenStream,
    pub kind: RsTypeKind,
}

impl RsType {
    pub fn type_generics(&self) -> TokenStream {
        let vars = &self.vars;
        quote! { #(#vars,)* }
    }

    pub fn type_generics_with_bounds(&self, bounds: &TokenStream) -> TokenStream {
        let vars = &self.vars;
        quote! { #(#vars: #bounds , )* }
    }
}

#[derive(Debug, Clone)]
pub enum RsTypeKind {
    Wrapper(RsTypeWrapper),
    Alias(RsTypeAlias),
    Record(RsTypeRecord),
    Variant(RsTypeVariant),
}

#[derive(Debug, Clone)]
pub struct RsTypeWrapper {
    pub wrapped: RsTypePath,
    pub wrapped_encoding: TokenStream,
}

#[derive(Debug, Clone)]
pub struct RsTypeAlias {
    pub aliased: RsTypePath,
}

#[derive(Debug, Clone)]
pub struct RsTypeRecord {
    pub fields: Vec<RsField>,
    pub json_attrs: JsonRecordTypeAttrs,
}

#[derive(Debug, Clone)]
pub struct RsField {
    pub name: String,
    pub ident: Ident,
    pub docs: String,
    pub visibility: Visibility,
    pub is_optional: bool,
    pub json_attrs: JsonFieldAttrs,
    pub json_name: String,
    /// Field type as written in the generated struct. For optional fields,
    /// this is `Option<inner_ty>`.
    pub ty: RsTypePath,
    /// `SerializeAs` / `DeserializeAs` encoding mirroring [`Self::ty`],
    /// substituting Sidex-specific wire forms at the leaves.
    pub encoding: TokenStream,
    /// `ty` without the optional `Option<...>` wrap. Equal to `ty` when the
    /// field is non-optional.
    pub inner_ty: RsTypePath,
    /// `encoding` without the optional `Option<...>` wrap. Equal to
    /// `encoding` when the field is non-optional.
    pub inner_encoding: TokenStream,
}

#[derive(Debug, Clone)]
pub struct RsTypeVariant {
    pub variants: Vec<RsVariant>,
    pub json_attrs: JsonVariantTypeAttrs,
}

#[derive(Debug, Clone)]
pub struct RsVariant {
    pub name: String,
    pub ident: Ident,
    pub docs: String,
    pub json_attrs: JsonVariantAttrs,
    pub json_name: String,
    pub is_record: bool,
    pub ty: Option<RsTypePath>,
    /// Marker mirroring `ty`. `None` iff `ty` is `None`.
    pub encoding: Option<TokenStream>,
}
