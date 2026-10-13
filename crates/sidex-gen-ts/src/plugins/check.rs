//! Structural JSON checks emitted by the always-on types plugin.

use sidex_attrs_json::JsonTaggedAttr;
use sidex_attrs_json::field_attrs;
use sidex_attrs_json::opaque_type_attrs;
use sidex_attrs_json::record_type_attrs;
use sidex_attrs_json::types::JsonType;
use sidex_attrs_json::variant_attrs;
use sidex_attrs_json::variant_type_attrs;
use sidex_codegen::Code;
use sidex_codegen::quote;
use sidex_gen::diagnostics::Result;
use sidex_gen::ir;

use crate::context::SchemaCtx;
use crate::plugins::types::ts_string_literal;

/// Emits object members for a type's structural checker.
pub fn emit(ctx: &SchemaCtx, def: &ir::Def) -> Result<Code> {
    let name = def.name.as_str();
    let check = definition(ctx, def)?;
    if def.vars.is_empty() {
        return Ok(quote!(
            "...__sidex_types.checker<@name>((): __sidex_types.Check => @check),"
        ));
    }
    let vars: Vec<Code> = def
        .vars
        .iter()
        .map(|var| Code::from(var.name.as_str()))
        .collect();
    let params: Vec<Code> = def
        .vars
        .iter()
        .map(|var| {
            let name = var.name.as_str();
            let param = format!("__check_{name}");
            quote!("@param: __sidex_types.Checker<@name>")
        })
        .collect();
    let args: Vec<Code> = def
        .vars
        .iter()
        .map(|var| Code::from(format!("__check_{}", var.name.as_str())))
        .collect();
    Ok(quote!(
        r#"
        checker<@(@vars), +>(@(@params), +): __sidex_types.Checker<@name<@(@vars), +>> {
            return __sidex_types.checker(() => @check);
        },
        is<@(@vars), +>(value: unknown, @(@params), +): value is @name<@(@vars), +> {
            return @name.checker(@(@args), +).is(value);
        },
        check<@(@vars), +>(value: unknown, @(@params), +): __sidex_types.CheckReport {
            return @name.checker(@(@args), +).check(value);
        },
    "#
    ))
}

/// Lowers one definition to a runtime check expression.
fn definition(ctx: &SchemaCtx, def: &ir::Def) -> Result<Code> {
    match &def.kind {
        ir::DefKind::TypeAlias(alias) => type_check(ctx, def, &alias.aliased),
        ir::DefKind::WrapperType(wrapper) => type_check(ctx, def, &wrapper.wrapped),
        ir::DefKind::OpaqueType(_) => {
            let attrs = opaque_type_attrs(def)?;
            let shapes = attrs
                .typ
                .map(|typ| {
                    typ.types_sorted()
                        .iter()
                        .map(|typ| {
                            ts_string_literal(match typ {
                                JsonType::Null => "null",
                                JsonType::Boolean => "boolean",
                                JsonType::Number => "number",
                                JsonType::String => "string",
                                JsonType::Object => "object",
                                JsonType::Array => "array",
                                JsonType::Any => "any",
                            })
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_else(|| vec![ts_string_literal("any")]);
            Ok(quote!("__sidex_types.jsonShape([@(@shapes), +])"))
        }
        ir::DefKind::RecordType(record) => {
            let attrs = record_type_attrs(def)?;
            let mut fields = Vec::new();
            let mut inline = Vec::new();
            for field in &record.fields {
                let field_attrs = field_attrs(field)?;
                let check = type_check(ctx, def, &field.typ)?;
                if field_attrs.inline {
                    inline.push(check);
                } else {
                    let name = ts_string_literal(&attrs.field_name(field, &field_attrs));
                    let optional = if field.is_optional { "true" } else { "false" };
                    fields.push(quote!("[@name, @check, @optional]"));
                }
            }
            Ok(quote!(
                "__sidex_types.record([@(@fields), +], [@(@inline), +])"
            ))
        }
        ir::DefKind::VariantType(variant) => {
            let attrs = variant_type_attrs(def)?;
            let tag = ts_string_literal(&attrs.tag_field_name());
            let mode = ts_string_literal(match attrs.tagged {
                JsonTaggedAttr::Externally => "external",
                JsonTaggedAttr::Internally => "internal",
                JsonTaggedAttr::Adjacently => "adjacent",
                JsonTaggedAttr::Implicitly => "implicit",
            });
            let mut cases = Vec::new();
            for case in &variant.variants {
                let case_attrs = variant_attrs(case)?;
                let name = ts_string_literal(&attrs.variant_name(case, &case_attrs));
                let (payload, content) = if let Some(typ) = &case.typ {
                    let content = if matches!(attrs.tagged, JsonTaggedAttr::Internally)
                        && case_attrs.content.is_none()
                        && is_record(ctx.bundle_ctx.unit, typ)
                    {
                        "null".to_owned()
                    } else {
                        let content = ts_string_literal(&attrs.content_field_name(&case_attrs));
                        if matches!(attrs.tagged, JsonTaggedAttr::Internally)
                            && case_attrs.content.is_none()
                        {
                            format!("[{content}]")
                        } else {
                            content
                        }
                    };
                    (type_check(ctx, def, typ)?, content)
                } else {
                    (Code::from("null"), "null".to_owned())
                };
                cases.push(quote!("[@name, @payload, @content]"));
            }
            Ok(quote!("__sidex_types.variant(@mode, @tag, [@(@cases), +])"))
        }
    }
}

/// Resolves a reference without using TypeScript's arbitrary type mapping table.
fn type_check(ctx: &SchemaCtx, def: &ir::Def, typ: &ir::Type) -> Result<Code> {
    match &typ.kind {
        ir::TypeKind::TypeVar(var) => {
            let name = format!("__check_{}", def[var.idx].name.as_str());
            Ok(quote!(
                "__sidex_types.lazy((): __sidex_types.Checker<unknown> => @name)"
            ))
        }
        ir::TypeKind::Instance(instance) => {
            let unit = ctx.bundle_ctx.unit;
            let bundle = &unit[instance.def.bundle];
            let schema = &unit[instance.def.schema];
            let target = &unit[instance.def];
            if bundle.metadata.name == "core" && schema.name == "builtins" {
                let name = target.name.as_str();
                return match name {
                    "Sequence" => {
                        let element = type_check(ctx, def, &instance.subst[0])?;
                        Ok(quote!("__sidex_types.sequence(@element)"))
                    }
                    "Map" => {
                        let key = type_check(ctx, def, &instance.subst[0])?;
                        let value = type_check(ctx, def, &instance.subst[1])?;
                        Ok(quote!("__sidex_types.map(@key, @value)"))
                    }
                    _ => {
                        let name = ts_string_literal(name);
                        Ok(quote!("__sidex_types.builtin(@name)"))
                    }
                };
            }
            let name = if instance.def.bundle != ctx.bundle_ctx.bundle_idx {
                format!(
                    "__bundle_{}.{}.{}",
                    bundle.metadata.name,
                    schema.name,
                    target.name.as_str()
                )
            } else if instance.def.schema != ctx.schema_idx {
                format!("__schema_{}.{}", schema.name, target.name.as_str())
            } else {
                target.name.as_str().to_owned()
            };
            if instance.subst.is_empty() {
                Ok(quote!(
                    "__sidex_types.lazy((): __sidex_types.Checker<unknown> => @name)"
                ))
            } else {
                let args: Vec<Code> = instance
                    .subst
                    .iter()
                    .map(|typ| {
                        let check = type_check(ctx, def, typ)?;
                        let typ = ctx.resolve_type(def, typ);
                        Ok(quote!(
                            "__sidex_types.checker<@typ>((): __sidex_types.Check => @check)"
                        ))
                    })
                    .collect::<Result<_>>()?;
                Ok(quote!(
                    "__sidex_types.lazy((): __sidex_types.Checker<unknown> => @name.checker(@(@args), +))"
                ))
            }
        }
    }
}

/// Wrappers retain their inner shape in the Rust reader.
pub fn is_record(unit: &ir::Ir, typ: &ir::Type) -> bool {
    let typ = unit.resolve_aliases(typ);
    let ir::TypeKind::Instance(instance) = &typ.kind else {
        return false;
    };
    match &unit[instance.def].kind {
        ir::DefKind::RecordType(_) => true,
        ir::DefKind::WrapperType(wrapper) => {
            is_record(unit, &unit.apply_subst(&wrapper.wrapped, &instance.subst))
        }
        _ => false,
    }
}
