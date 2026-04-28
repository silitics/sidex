use sidex_attrs_json::JsonFieldAttrs;
use sidex_attrs_json::JsonOpaqueTypeAttrs;
use sidex_attrs_json::JsonRecordTypeAttrs;
use sidex_attrs_json::JsonVariantAttrs;
use sidex_attrs_json::JsonVariantTypeAttrs;
use sidex_attrs_json::atoms::JsonTaggedAttr;
use sidex_codegen::Code;
use sidex_codegen::quote;
use sidex_gen::attrs::TryFromAttrs;
use sidex_gen::diagnostics;
use sidex_gen::ir;

use super::Plugin;
use crate::context::BundleCtx;
use crate::context::SchemaCtx;
use crate::context::TypeExpr;

pub struct Types;

impl Plugin for Types {
    fn visit_def(&self, ctx: &SchemaCtx, def: &ir::Def) -> diagnostics::Result<Code> {
        let name = def.name.as_str().to_owned();
        let qualified_name = ts_string_literal(&format!(
            "::{}::{}::{}",
            ctx.bundle_ctx.bundle.metadata.name,
            ctx.schema.name,
            def.name.as_str()
        ));
        let mut is_nominal = true;
        let mut type_expr = match &def.kind {
            ir::DefKind::TypeAlias(typ) => {
                is_nominal = false;
                ctx.resolve_type(def, &typ.aliased)
            }
            ir::DefKind::OpaqueType(_) => {
                let ty_json_attrs = JsonOpaqueTypeAttrs::try_from_attrs(&def.attrs)?;
                ty_json_attrs.typ.map_or_else(TypeExpr::any, |typ_attr| {
                    TypeExpr::union(typ_attr.typ.types.iter().map(TypeExpr::from))
                })
            }
            ir::DefKind::RecordType(typ) => {
                is_nominal = false;
                let ty_json_attrs = JsonRecordTypeAttrs::try_from_attrs(&def.attrs)?;
                if typ.fields.is_empty() {
                    TypeExpr(Code::from("Record<string, never>"))
                } else {
                    let mut field_names: Vec<Code> = Vec::with_capacity(typ.fields.len());
                    let mut field_opts: Vec<Code> = Vec::with_capacity(typ.fields.len());
                    let mut field_types: Vec<Code> = Vec::with_capacity(typ.fields.len());
                    for field in &typ.fields {
                        let json_attrs = JsonFieldAttrs::try_from_attrs(&field.attrs)?;
                        let field_name = ty_json_attrs.field_name(field, &json_attrs);
                        field_names.push(Code::from(ts_string_literal(&field_name)));
                        field_opts.push(Code::from(if field.is_optional { "?" } else { "" }));
                        field_types.push(ctx.resolve_type(def, &field.typ).0);
                    }
                    TypeExpr(quote!("{ @(@field_names@field_opts: @field_types), + }"))
                }
            }
            ir::DefKind::VariantType(typ) => {
                is_nominal = false;
                let ty_json_attrs = JsonVariantTypeAttrs::try_from_attrs(&def.attrs)?;
                let tag_field = ts_string_literal(&ty_json_attrs.tag_field_name());
                let mut variant_ts_types: Vec<TypeExpr> = Vec::new();
                for variant in &typ.variants {
                    let json_attrs = JsonVariantAttrs::try_from_attrs(&variant.attrs)?;
                    let variant_name =
                        ts_string_literal(&ty_json_attrs.variant_name(variant, &json_attrs));

                    if let Some(typ_) = &variant.typ {
                        let resolved = ctx.bundle_ctx.unit.resolve_aliases(typ_);
                        let inner = ctx.resolve_type(def, typ_);

                        match ty_json_attrs.tagged {
                            JsonTaggedAttr::Externally => {
                                variant_ts_types
                                    .push(TypeExpr(quote!("{ @variant_name: @inner }")));
                            }
                            JsonTaggedAttr::Implicitly => {
                                variant_ts_types.push(inner);
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
                                        variant_ts_types.push(TypeExpr(quote!(
                                            "({ @tag_field: @variant_name } & @inner)"
                                        )));
                                    }
                                    _ => {
                                        let content_field = ts_string_literal(
                                            &ty_json_attrs.content_field_name(&json_attrs),
                                        );
                                        variant_ts_types.push(TypeExpr(quote!(
                                            "{ @tag_field: @variant_name, @content_field: @inner }"
                                        )));
                                    }
                                }
                            }
                        }
                    } else {
                        variant_ts_types.push(match ty_json_attrs.tagged {
                            JsonTaggedAttr::Externally | JsonTaggedAttr::Implicitly => {
                                TypeExpr(Code::from(variant_name))
                            }
                            _ => TypeExpr(quote!("{ @tag_field: @variant_name }")),
                        });
                    }
                }
                TypeExpr::union(variant_ts_types.into_iter())
            }
            ir::DefKind::WrapperType(typ) => ctx.resolve_type(def, &typ.wrapped),
        };

        if is_nominal {
            type_expr = TypeExpr(quote!("__sidex_types.Nominal<@type_expr, @qualified_name>"));
        }

        let vars: Vec<Code> = def
            .vars
            .iter()
            .map(|var| Code::from(var.name.as_str()))
            .collect();
        let vars_clause = if vars.is_empty() {
            Code::new()
        } else {
            quote!("<@(@vars), +>")
        };

        Ok(quote!("export type @name@vars_clause = @type_expr;"))
    }

    fn visit_schema(&self, ctx: &SchemaCtx) -> diagnostics::Result<Code> {
        let mut schemas: Vec<&ir::Schema> = ctx
            .bundle_ctx
            .unit
            .schemas_of(ctx.bundle_ctx.bundle_idx)
            .map(|(_, s)| s)
            .collect();
        schemas.sort_by(|a, b| a.name.cmp(&b.name));
        let schema_imports: Vec<Code> = schemas
            .iter()
            .map(|schema| {
                let alias = format!("__schema_{}", schema.name);
                let path = ts_string_literal(&format!("./{}", schema.name));
                quote!("import * as @alias from @path;")
            })
            .collect();

        let mut external: Vec<(&String, &String)> = ctx.bundle_ctx.cfg.external.iter().collect();
        external.sort_by(|(a, _), (b, _)| a.cmp(b));
        let external_imports: Vec<Code> = external
            .iter()
            .map(|(name, path)| {
                let alias = format!("__bundle_{}", name);
                let path = ts_string_literal(path);
                quote!("import * as @alias from @path;")
            })
            .collect();

        Ok(quote!(
            r#"
            import * as __sidex_types from "@@sidex/types";
            @(@schema_imports)*
            @(@external_imports)*
            "#
        ))
    }

    fn visit_bundle(&self, ctx: &BundleCtx) -> diagnostics::Result<Code> {
        let mut schemas: Vec<&ir::Schema> = ctx
            .unit
            .schemas_of(ctx.bundle_idx)
            .map(|(_, s)| s)
            .collect();
        schemas.sort_by(|a, b| a.name.cmp(&b.name));
        let exports: Vec<Code> = schemas
            .iter()
            .map(|schema| {
                let name = schema.name.as_str().to_owned();
                let path = ts_string_literal(&format!("./{}", schema.name));
                quote!("export * as @name from @path;")
            })
            .collect();
        Ok(quote!("@(@exports)*"))
    }
}

fn ts_string_literal(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}
