//! Plugin generating implementations of [`serde::Serialize`] and [`serde::Deserialize`]
//! for Sidex types.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use sidex_attrs_json::{
    JsonFieldAttrs, JsonRecordTypeAttrs, JsonVariantAttrs, JsonVariantTypeAttrs,
};
use sidex_gen::{attrs::TryFromAttrs, diagnostics::Result, ir};

use super::Plugin;
use crate::context::{RustField, RustVariant, SchemaCtx};

mod identifier_enum;
mod record_type;
mod variant_type;
mod wrapper_type;

pub(crate) struct SerdeField<'ir> {
    pub field: &'ir ir::Field,
    pub name: String,
    pub rust_field: RustField,
    pub json_attrs: JsonFieldAttrs,
}

pub(crate) struct SerdeVariant<'ir> {
    pub variant: &'ir ir::Variant,
    pub name: String,
    pub rust_variant: RustVariant,
    pub json_attrs: JsonVariantAttrs,
}

/// The [`Serde`] plugin.
pub struct Serde;

impl Plugin for Serde {
    fn visit_def(&self, ctx: &SchemaCtx, def: &ir::Def) -> Result<TokenStream> {
        let ty = ctx.ty(def);
        let (serialize_body, deserialize_body) = match &def.kind {
            ir::DefKind::RecordType(typ_def) => {
                let ty_json_attrs = JsonRecordTypeAttrs::try_from_attrs(&def.attrs)?;
                let fields = typ_def
                    .fields
                    .iter()
                    .map(|field| {
                        let json_attrs = JsonFieldAttrs::try_from_attrs(&field.attrs)?;
                        Ok(SerdeField {
                            field,
                            name: ty_json_attrs.field_name(field, &json_attrs),
                            rust_field: ctx.field(def, field),
                            json_attrs,
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;

                (
                    record_type::ser::gen_serialize_body(&ty, &fields),
                    record_type::de::gen_deserialize_body(&ty, &ty_json_attrs, &fields),
                )
            }
            ir::DefKind::VariantType(typ_def) => {
                let ty_json_attrs = JsonVariantTypeAttrs::try_from_attrs(&def.attrs)?;
                let variants = typ_def
                    .variants
                    .iter()
                    .map(|variant| {
                        let rust_variant = ctx.variant(def, variant);
                        let json_attrs = JsonVariantAttrs::try_from_attrs(&variant.attrs)?;
                        Ok(SerdeVariant {
                            variant,
                            name: ty_json_attrs.variant_name(variant, &json_attrs),
                            rust_variant,
                            json_attrs,
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;
                (
                    variant_type::ser::gen_serialize_body(ctx, &ty, &ty_json_attrs, &variants)?,
                    quote! {
                        todo!()
                    },
                )
            }
            ir::DefKind::WrapperType(_) => {
                (
                    quote! {
                        self.0.serialize(__serializer)
                    },
                    quote! {
                        todo!()
                    },
                )
            }
            _ => {
                // Opaque types are translated to aliases and, hence, we do not need to
                // generate any implementations for them. The same is true for type
                // aliases. Services also do not need any Serde implementations.
                return Ok(TokenStream::default());
            }
        };

        let ty_ident = &ty.ident;

        let vars = ctx.generic_type_vars(def);
        Ok(quote! {
            #[automatically_derived]
            impl #vars ::serde::Serialize for #ty_ident #vars {
                fn serialize<__S: ::serde::Serializer>(&self, __serializer: __S) -> ::std::result::Result<__S::Ok, __S::Error> {
                    #serialize_body
                }
            }
            #[automatically_derived]
            impl<'de> ::serde::Deserialize<'de> for #ty_ident #vars {
                fn deserialize<__D: ::serde::Deserializer<'de>>(__deserializer: __D) -> ::std::result::Result<Self, __D::Error> {
                    #deserialize_body
                }
            }
        })
    }
}
