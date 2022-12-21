use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use sidex_attrs_json::{
    atoms::JsonTaggedAttr, JsonFieldAttrs, JsonRecordTypeAttrs, JsonVariantAttrs,
    JsonVariantTypeAttrs, OpaqueTypeAttrs,
};
use sidex_gen::{
    attrs::TryFromAttrs,
    diagnostics::{self, Result},
    ir,
};

use super::Plugin;
use crate::context::{SchemaCtx, TypeExpr};

pub struct Types;

impl Plugin for Types {
    fn visit_def(&self, ctx: &SchemaCtx, def: &ir::Def) -> diagnostics::Result<TokenStream> {
        let name = format_ident!("{}", def.name.as_str());
        let qualified_name = format!(
            "::{}::{}::{}",
            ctx.bundle_ctx.bundle.metadata.name,
            ctx.schema.name,
            def.name.as_str()
        );
        let mut is_nominal = true;
        let mut type_expr = match &def.kind {
            ir::DefKind::TypeAlias(typ) => {
                is_nominal = false;
                ctx.resolve_type(def, &typ.aliased)
            }
            ir::DefKind::OpaqueType(_) => {
                let ty_json_attrs = OpaqueTypeAttrs::try_from_attrs(&def.attrs)?;
                ty_json_attrs.typ.map_or_else(
                    || TypeExpr::any(),
                    |typ_attr| TypeExpr::union(typ_attr.typ.types.iter().map(TypeExpr::from)),
                )
            }
            ir::DefKind::RecordType(typ) => {
                is_nominal = false;
                let ty_json_attrs = JsonRecordTypeAttrs::try_from_attrs(&def.attrs)?;
                let names = typ
                    .fields
                    .iter()
                    .map(|field| {
                        let json_attrs = JsonFieldAttrs::try_from_attrs(&field.attrs)?;
                        Ok(format_ident!(
                            "{}",
                            ty_json_attrs.field_name(field, &json_attrs)
                        ))
                    })
                    .collect::<Result<Vec<_>>>()?;
                let types = typ
                    .fields
                    .iter()
                    .map(|field| ctx.resolve_type(def, &field.typ))
                    .collect::<Vec<_>>();
                let optional = typ.fields.iter().map(|field| {
                    if field.is_optional {
                        quote! { ? }
                    } else {
                        TokenStream::default()
                    }
                });
                TypeExpr(quote! {
                    { #( #names #optional : #types ),* }
                })
            }
            ir::DefKind::VariantType(typ) => {
                is_nominal = false;
                let ty_json_attrs = JsonVariantTypeAttrs::try_from_attrs(&def.attrs)?;
                let ty_tag_field = format_ident!("{}", ty_json_attrs.tag_field_name());
                let mut variant_ts_types = Vec::new();
                for variant in &typ.variants {
                    let json_attrs = JsonVariantAttrs::try_from_attrs(&variant.attrs)?;
                    let name = ty_json_attrs.variant_name(variant, &json_attrs);

                    if let Some(typ) = &variant.typ {
                        let resolved = ctx.bundle_ctx.unit.resolve_aliases(typ);
                        let inner = ctx.resolve_type(def, typ);

                        match ty_json_attrs.tagged {
                            JsonTaggedAttr::Externally => {
                                variant_ts_types.push(TypeExpr(quote! {
                                    { #name : #inner }
                                }))
                            }
                            _ => {
                                match ctx.bundle_ctx.unit.record_type(&resolved) {
                                    Some(_)
                                        if json_attrs.content.is_none()
                                            && matches!(
                                                ty_json_attrs.tagged,
                                                JsonTaggedAttr::Internally
                                            ) =>
                                    {
                                        variant_ts_types.push(TypeExpr(quote! {
                                            ({ #ty_tag_field : #name } & #inner)
                                        }))
                                    }
                                    _ => {
                                        let content_name = format_ident!(
                                            "{}",
                                            ty_json_attrs.content_field_name(&json_attrs)
                                        );
                                        variant_ts_types.push(TypeExpr(quote! {
                                            { #ty_tag_field : #name , #content_name : #inner }
                                        }))
                                    }
                                }
                            }
                        }
                    } else {
                        variant_ts_types.push(match ty_json_attrs.tagged {
                            JsonTaggedAttr::Externally => TypeExpr(quote! { #name }),
                            _ => {
                                TypeExpr(quote! {
                                    { #ty_tag_field : #name }
                                })
                            }
                        })
                    }
                }
                TypeExpr::union(variant_ts_types.into_iter())
            }
            ir::DefKind::WrapperType(typ) => ctx.resolve_type(def, &typ.wrapped),
            ir::DefKind::Service(_) => {
                // Service definitions are handled separately.
                return Ok(TokenStream::default());
            }
        };

        if is_nominal {
            type_expr = TypeExpr(quote! { __sidex_types.Nominal< #type_expr , #qualified_name > });
        }

        let vars = if def.vars.is_empty() {
            TokenStream::default()
        } else {
            let vars = def
                .vars
                .iter()
                .map(|var| format_ident!("{}", var.name.as_str()));
            quote! { < #(#vars),* > }
        };

        Ok(quote! {
            export type #name #vars = #type_expr;
        })
    }

    fn visit_schema(&self, ctx: &SchemaCtx) -> diagnostics::Result<TokenStream> {
        let schema_imports = ctx.bundle_ctx.bundle.schemas.iter().map(|schema| {
            let mut path = String::with_capacity(schema.name.len() + 2);
            path.push_str("./");
            path.push_str(schema.name.as_str());
            let local_name = format_ident!("__schema_{}", schema.name);
            quote! {
                import * as #local_name from #path;
            }
        });
        Ok(quote! {
            import * as __sidex_types from "@sidex/types";

            #(#schema_imports)*
        })
    }

    fn visit_bundle(&self, ctx: &crate::context::BundleCtx) -> diagnostics::Result<TokenStream> {
        let schema_exports = ctx.bundle.schemas.iter().map(|schema| {
            let mut path = String::with_capacity(schema.name.len() + 2);
            path.push_str("./");
            path.push_str(schema.name.as_str());
            let name = format_ident!("{}", schema.name);
            quote! {
                export * as #name from #path;
            }
        });
        Ok(quote! {
            #(#schema_exports)*
        })
    }
}
