//! Rust Type Intermediate Representation (RsTyIR)

use std::str::FromStr;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use sidex_attrs_json::{
    JsonFieldAttrs, JsonRecordTypeAttrs, JsonVariantAttrs, JsonVariantTypeAttrs,
};
use sidex_attrs_rust::{FieldAttrs, TypeAttrs, Visibility};
use sidex_gen::{
    attrs::TryFromAttrs,
    diagnostics::Result,
    ir::{Def, DefKind},
};

use crate::context::SchemaCtx;

pub mod serde;

pub fn rs_type_to_rs_def(rs_type: &RsType) -> TokenStream {
    let RsType {
        ident,
        visibility,
        generics,
        docs,
        meta,
        kind,
        ..
    } = &rs_type;

    let (_, vars, _) = generics.split_for_impl();

    match kind {
        RsTypeKind::Alias(rs_type_alias) => {
            let RsTypeAlias { aliased } = rs_type_alias;
            quote! {
                #[doc = #docs]
                #visibility type #ident #vars = #aliased;
            }
        }
        RsTypeKind::Wrapper(rs_type_wrapper) => {
            let RsTypeWrapper { wrapped } = rs_type_wrapper;
            quote! {
                #[doc = #docs]
                #meta
                #visibility struct #ident #vars (pub(crate) #wrapped);

                impl ::std::convert::From<#ident> for #wrapped {
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
                #visibility struct #ident #vars {
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
                #visibility enum #ident #vars {
                    #(#variants)*
                }
            }
        }
    }
}

pub fn rs_type_from_def(ctx: &SchemaCtx, def: &Def) -> Result<Option<RsType>> {
    let mut generics = syn::Generics::default();
    for var in &def.vars {
        let ident = format_ident!("{}", var.name.as_str());
        let param = syn::GenericParam::Type(ident.into());
        generics.params.push(param);
    }
    let docs = def
        .docs
        .as_ref()
        .map(|docs| docs.as_str())
        .unwrap_or_default()
        .to_owned();
    let attrs = TypeAttrs::try_from(def.attrs.as_slice())?;
    let mut derive = ctx
        .bundle_ctx
        .cfg
        .derive
        .iter()
        .map(|path| TokenStream::from_str(path).unwrap())
        .collect::<Vec<_>>();
    for derive_trait in &attrs.derive.positive {
        derive.push(derive_trait.clone());
    }
    let attr = &attrs.attrs;
    let meta = quote! {
        #(#[derive(#derive)])*
        #(#[#attr])*
    };
    let kind = match &def.kind {
        DefKind::WrapperType(wrapper_type_def) => {
            let wrapped = ctx.resolve_type_old(def, &wrapper_type_def.wrapped, false);
            RsTypeKind::Wrapper(RsTypeWrapper { wrapped })
        }
        DefKind::TypeAlias(alias) => {
            let aliased = ctx.resolve_type_old(def, &alias.aliased, false);
            RsTypeKind::Alias(RsTypeAlias { aliased })
        }
        DefKind::OpaqueType(_) => {
            match &attrs.typ {
                Some(typ) => {
                    let aliased = TokenStream::from_str(&typ.path).unwrap();
                    RsTypeKind::Alias(RsTypeAlias { aliased })
                }
                None => todo!(),
            }
        }
        DefKind::RecordType(typ) => {
            let fields = typ
                .fields
                .iter()
                .map(|field| {
                    let attrs = FieldAttrs::try_from_attrs(&field.attrs)?;

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
                    let mut typ = ctx.resolve_type_old(def, &field.typ, false);
                    for wrapper in attrs.wrappers {
                        let wrapper = TokenStream::from_str(&wrapper.wrapper).unwrap();
                        typ = quote! { #wrapper < #typ > }
                    }
                    if field.is_optional {
                        typ = quote! { ::std::option::Option< #typ > };
                    }
                    let json_attrs = JsonFieldAttrs::try_from_attrs(&field.attrs)?;
                    Ok(RsField {
                        name: field.name.identifier.clone(),
                        ident: name,
                        docs,
                        visibility: attrs.visibility.clone(),
                        json: json_attrs,
                        ty: typ,
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            let ty_json_attrs = JsonRecordTypeAttrs::try_from_attrs(&def.attrs)?;
            RsTypeKind::Record(RsTypeRecord {
                fields,
                json: ty_json_attrs,
            })
        }
        DefKind::VariantType(typ) => {
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
                    let json_attrs = JsonVariantAttrs::try_from_attrs(&variant.attrs)?;
                    Ok(RsVariant {
                        name: variant.name.identifier.clone(),
                        docs,
                        ident: name,
                        json: json_attrs,
                        ty,
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            let ty_json_attrs = JsonVariantTypeAttrs::try_from_attrs(&def.attrs)?;
            RsTypeKind::Variant(RsTypeVariant {
                variants,
                json: ty_json_attrs,
            })
        }
        _ => {
            // Service definitions and derived types are handled separately.
            return Ok(None);
        }
    };
    Ok(Some(RsType {
        name: def.name.identifier.clone(),
        ident: format_ident!("{}", def.name.as_str()),
        visibility: Visibility::Pub,
        generics,
        docs,
        meta,
        kind,
    }))
}

pub type RsTypePath = TokenStream;

#[derive(Debug, Clone)]
pub struct RsType {
    pub name: String,
    pub ident: Ident,
    pub visibility: Visibility,
    pub generics: syn::Generics,
    pub docs: String,
    pub meta: TokenStream,
    pub kind: RsTypeKind,
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
}

#[derive(Debug, Clone)]
pub struct RsTypeAlias {
    pub aliased: RsTypePath,
}

#[derive(Debug, Clone)]
pub struct RsTypeRecord {
    pub fields: Vec<RsField>,
    pub json: JsonRecordTypeAttrs,
}

#[derive(Debug, Clone)]
pub struct RsField {
    pub name: String,
    pub ident: Ident,
    pub docs: String,
    pub visibility: Visibility,
    pub json: JsonFieldAttrs,
    pub ty: RsTypePath,
}

#[derive(Debug, Clone)]
pub struct RsTypeVariant {
    pub variants: Vec<RsVariant>,
    pub json: JsonVariantTypeAttrs,
}

#[derive(Debug, Clone)]
pub struct RsVariant {
    pub name: String,
    pub ident: Ident,
    pub docs: String,
    pub json: JsonVariantAttrs,
    pub ty: Option<RsTypePath>,
}
