//! Walks the Sidex IR to produce a JSON value matching a type's canonical
//! JSON shape.
//!
//! The walker mirrors the layout decisions of `sidex-gen-json-schema`: variant
//! tagging modes, record field renaming, opaque-type unions, and the builtin
//! type roster. Anything that has a JSON schema also has a generator here.

use serde_json::Map;
use serde_json::Value;
use sidex_attrs_json::JsonRecordTypeAttrs;
use sidex_attrs_json::JsonTaggedAttr;
use sidex_attrs_json::JsonVariantTypeAttrs;
use sidex_attrs_json::field_attrs;
use sidex_attrs_json::opaque_type_attrs;
use sidex_attrs_json::record_type_attrs;
use sidex_attrs_json::types::JsonType;
use sidex_attrs_json::types::JsonUnionType;
use sidex_attrs_json::variant_attrs;
use sidex_attrs_json::variant_type_attrs;
use sidex_diagnostics::Diagnostic;
use sidex_diagnostics::Result;
use sidex_ir as ir;

use crate::config::Config;
use crate::source::Source;

/// Generate a JSON value satisfying `typ` using the given source and config.
pub fn generate(
    ir: &ir::Ir,
    typ: &ir::Type,
    source: &mut Source,
    config: &Config,
) -> Result<Value> {
    let mut walker = Walker {
        ir,
        config,
        depth: 0,
    };
    walker.gen_value(typ, source)
}

struct Walker<'a> {
    ir: &'a ir::Ir,
    config: &'a Config,
    depth: usize,
}

impl<'a> Walker<'a> {
    fn gen_value(&mut self, typ: &ir::Type, source: &mut Source) -> Result<Value> {
        let typ = self.ir.resolve_aliases(typ);
        match &typ.kind {
            ir::TypeKind::TypeVar(_) => {
                // Free type variable: emit a value drawn from the configured fallback
                // JSON union. This lets users generate samples for generic types
                // without having to specify substitutions explicitly.
                let union = self.config.unbound_var_default.clone();
                self.gen_from_union(&union, source)
            }
            ir::TypeKind::Instance(instance) => self.gen_instance(instance, source),
        }
    }

    fn gen_instance(&mut self, instance: &ir::InstanceType, source: &mut Source) -> Result<Value> {
        let def = &self.ir[instance.def];
        match &def.kind {
            ir::DefKind::TypeAlias(_) => {
                // Aliases are resolved in `gen`, so this branch is unreachable.
                unreachable!("alias should have been resolved")
            }
            ir::DefKind::OpaqueType(_) => self.gen_opaque(instance, def, source),
            ir::DefKind::RecordType(record) => self.gen_record(def, record, instance, source),
            ir::DefKind::VariantType(variant) => self.gen_variant(def, variant, instance, source),
            ir::DefKind::WrapperType(wrapper) => {
                let wrapped = self.ir.apply_subst(&wrapper.wrapped, &instance.subst);
                self.gen_value(&wrapped, source)
            }
        }
    }

    fn gen_opaque(
        &mut self,
        instance: &ir::InstanceType,
        def: &ir::Def,
        source: &mut Source,
    ) -> Result<Value> {
        // Built-in container and primitive types are recognized by qualified path,
        // matching `sidex-gen-json-schema`.
        let qualified = qualified_path(self.ir, instance);
        match qualified.as_str() {
            "::core::builtins::Sequence" => return self.gen_sequence(instance, source),
            "::core::builtins::Map" => return self.gen_map(instance, source),
            "::core::builtins::unit" => return Ok(Value::Null),
            "::core::builtins::bool" => return Ok(pick_bool(source)),
            "::core::builtins::string" | "::core::builtins::bytes" => {
                return Ok(pick_string(source));
            }
            "::core::builtins::i8" => return Ok(pick_signed(source, I8_TABLE)),
            "::core::builtins::i16" => return Ok(pick_signed(source, I16_TABLE)),
            "::core::builtins::i32" => return Ok(pick_signed(source, I32_TABLE)),
            "::core::builtins::i64" => return Ok(self.draw_i64(source)),
            "::core::builtins::u8" => return Ok(pick_unsigned(source, U8_TABLE)),
            "::core::builtins::u16" => return Ok(pick_unsigned(source, U16_TABLE)),
            "::core::builtins::u32" => return Ok(pick_unsigned(source, U32_TABLE)),
            "::core::builtins::u64" | "::core::builtins::idx" => {
                return Ok(self.draw_u64(source));
            }
            "::core::builtins::f32" | "::core::builtins::f64" => {
                return Ok(self.draw_float(source));
            }
            _ => {}
        }
        // User-defined opaque: the contract is the `json` attribute's union type.
        let attrs = opaque_type_attrs(def)?;
        let union = attrs.typ.unwrap_or_else(|| {
            JsonUnionType {
                types: std::iter::once(JsonType::Any).collect(),
            }
        });
        self.gen_from_union(&union, source)
    }

    /// Draw an `i64` from the curated table, encoding as a JSON string when
    /// `integers_as_strings` is set since values past 2^53 lose precision in JS.
    fn draw_i64(&self, source: &mut Source) -> Value {
        let v = I64_TABLE[source.draw_choice(I64_TABLE.len())];
        if self.config.integers_as_strings {
            Value::String(v.to_string())
        } else {
            Value::Number(v.into())
        }
    }

    fn draw_u64(&self, source: &mut Source) -> Value {
        let v = U64_TABLE[source.draw_choice(U64_TABLE.len())];
        if self.config.integers_as_strings {
            Value::String(v.to_string())
        } else {
            Value::Number(v.into())
        }
    }

    /// Draw a float. The finite table is always available; the special-value
    /// table extends it only when `floats_as_strings` is set, and those entries
    /// encode as the JSON strings `"NaN"`, `"Infinity"`, `"-Infinity"`.
    fn draw_float(&self, source: &mut Source) -> Value {
        let len = FLOAT_FINITE_TABLE.len()
            + if self.config.floats_as_strings {
                FLOAT_SPECIAL_TABLE.len()
            } else {
                0
            };
        let idx = source.draw_choice(len);
        if idx < FLOAT_FINITE_TABLE.len() {
            json_f64(FLOAT_FINITE_TABLE[idx])
        } else {
            Value::String(FLOAT_SPECIAL_TABLE[idx - FLOAT_FINITE_TABLE.len()].to_owned())
        }
    }

    fn gen_from_union(&mut self, union: &JsonUnionType, source: &mut Source) -> Result<Value> {
        // Pick one member of the declared union, then generate a value of that
        // JSON type. For `any`, fall through to a depth-budgeted free draw.
        let members: Vec<JsonType> = union.types.iter().copied().collect();
        let chosen = members[source.draw_choice(members.len())];
        Ok(match chosen {
            JsonType::Number => self.draw_float(source),
            JsonType::Boolean => pick_bool(source),
            JsonType::String => pick_string(source),
            JsonType::Null => Value::Null,
            JsonType::Object => {
                let len = source.draw_len(self.config.max_collection_len);
                let mut map = Map::new();
                for _ in 0..len {
                    // Object keys are themselves drawn from the string table, so the
                    // empty key, escape-heavy keys, and non-ASCII keys all show up.
                    let key = pick_string_raw(source).to_owned();
                    map.insert(key, self.gen_any_value(source)?);
                }
                Value::Object(map)
            }
            JsonType::Array => {
                let len = source.draw_len(self.config.max_collection_len);
                let mut arr = Vec::with_capacity(len);
                for _ in 0..len {
                    arr.push(self.gen_any_value(source)?);
                }
                Value::Array(arr)
            }
            JsonType::Any => self.gen_any_value(source)?,
        })
    }

    fn gen_any_value(&mut self, source: &mut Source) -> Result<Value> {
        // At depth limit, prefer leaf values to keep the structure bounded.
        let leaf_only = self.depth >= self.config.max_depth;
        let kinds: &[JsonType] = if leaf_only {
            &[
                JsonType::Null,
                JsonType::Boolean,
                JsonType::Number,
                JsonType::String,
            ]
        } else {
            &[
                JsonType::Null,
                JsonType::Boolean,
                JsonType::Number,
                JsonType::String,
                JsonType::Array,
                JsonType::Object,
            ]
        };
        let pick = kinds[source.draw_choice(kinds.len())];
        let union = JsonUnionType {
            types: std::iter::once(pick).collect(),
        };
        self.depth += 1;
        let value = self.gen_from_union(&union, source);
        self.depth -= 1;
        value
    }

    fn gen_sequence(&mut self, instance: &ir::InstanceType, source: &mut Source) -> Result<Value> {
        let element = &instance.subst[0];
        let len = source.draw_len(self.config.max_collection_len);
        let mut out = Vec::with_capacity(len);
        self.depth += 1;
        for _ in 0..len {
            out.push(self.gen_value(element, source)?);
        }
        self.depth -= 1;
        Ok(Value::Array(out))
    }

    fn gen_map(&mut self, instance: &ir::InstanceType, source: &mut Source) -> Result<Value> {
        let value_type = &instance.subst[1];
        let len = source.draw_len(self.config.max_collection_len);
        let mut map = Map::new();
        self.depth += 1;
        for _ in 0..len {
            let key_len = source.draw_len(self.config.max_string_len);
            let key = source.draw_ascii_string(key_len);
            map.insert(key, self.gen_value(value_type, source)?);
        }
        self.depth -= 1;
        Ok(Value::Object(map))
    }

    fn gen_record(
        &mut self,
        def: &ir::Def,
        record: &ir::RecordTypeDef,
        instance: &ir::InstanceType,
        source: &mut Source,
    ) -> Result<Value> {
        let attrs = record_type_attrs(def)?;
        let mut map = Map::new();
        self.fill_record(&attrs, record, instance, source, &mut map)?;
        Ok(Value::Object(map))
    }

    fn fill_record(
        &mut self,
        attrs: &JsonRecordTypeAttrs,
        record: &ir::RecordTypeDef,
        instance: &ir::InstanceType,
        source: &mut Source,
        out: &mut Map<String, Value>,
    ) -> Result<()> {
        self.depth += 1;
        for field in &record.fields {
            let field_attrs = field_attrs(field)?;
            let field_typ = self.ir.apply_subst(&field.typ, &instance.subst);
            if field_attrs.inline {
                // Inline fields merge the field's object contents into the parent.
                // Only valid when the field is itself a record-shaped type.
                self.inline_record_into(&field_typ, source, out)?;
                continue;
            }
            if field.is_optional && !source.draw_bool() {
                // Drop the field for this sample; matches the `Undefined` repr.
                continue;
            }
            let name = attrs.field_name(field, &field_attrs);
            let value = self.gen_value(&field_typ, source)?;
            out.insert(name, value);
        }
        self.depth -= 1;
        Ok(())
    }

    fn inline_record_into(
        &mut self,
        typ: &ir::Type,
        source: &mut Source,
        out: &mut Map<String, Value>,
    ) -> Result<()> {
        let resolved = self.ir.resolve_aliases(typ);
        let ir::TypeKind::Instance(instance) = &resolved.kind else {
            return Err(Diagnostic::error(
                "cannot inline a type variable as a field",
            ));
        };
        let def = &self.ir[instance.def];
        match &def.kind {
            ir::DefKind::RecordType(record) => {
                let attrs = record_type_attrs(def)?;
                self.fill_record(&attrs, record, instance, source, out)
            }
            ir::DefKind::WrapperType(wrapper) => {
                let inner = self.ir.apply_subst(&wrapper.wrapped, &instance.subst);
                self.inline_record_into(&inner, source, out)
            }
            _ => {
                Err(Diagnostic::error(format!(
                    "cannot inline non-record type `{}`",
                    def.name.as_str()
                )))
            }
        }
    }

    fn gen_variant(
        &mut self,
        def: &ir::Def,
        variant: &ir::VariantTypeDef,
        instance: &ir::InstanceType,
        source: &mut Source,
    ) -> Result<Value> {
        if variant.variants.is_empty() {
            return Err(Diagnostic::error(format!(
                "variant type `{}` has no cases",
                def.name.as_str()
            )));
        }
        let attrs = variant_type_attrs(def)?;
        // At depth limit, prefer a unit case if any exists; otherwise fall through.
        let pick = self.pick_variant_idx(variant, source);
        let chosen = &variant.variants[pick];
        let case_attrs = variant_attrs(chosen)?;
        let case_name = attrs.variant_name(chosen, &case_attrs);
        self.depth += 1;
        let value =
            self.encode_variant(&attrs, &case_attrs, &case_name, chosen, instance, source)?;
        self.depth -= 1;
        Ok(value)
    }

    fn pick_variant_idx(&self, variant: &ir::VariantTypeDef, source: &mut Source) -> usize {
        if self.depth >= self.config.max_depth {
            // Find a unit case to terminate recursion if one exists.
            if let Some(idx) = variant.variants.iter().position(|v| v.typ.is_none()) {
                return idx;
            }
        }
        source.draw_choice(variant.variants.len())
    }

    fn encode_variant(
        &mut self,
        attrs: &JsonVariantTypeAttrs,
        case_attrs: &sidex_attrs_json::JsonVariantAttrs,
        case_name: &str,
        case: &ir::Variant,
        instance: &ir::InstanceType,
        source: &mut Source,
    ) -> Result<Value> {
        let payload_type = case
            .typ
            .as_ref()
            .map(|t| self.ir.apply_subst(t, &instance.subst));
        match attrs.tagged {
            JsonTaggedAttr::Externally => {
                match payload_type {
                    None => Ok(Value::String(case_name.to_owned())),
                    Some(typ) => {
                        let inner = self.gen_value(&typ, source)?;
                        let mut map = Map::new();
                        map.insert(case_name.to_owned(), inner);
                        Ok(Value::Object(map))
                    }
                }
            }
            JsonTaggedAttr::Adjacently => {
                let mut map = Map::new();
                map.insert(attrs.tag_field_name(), Value::String(case_name.to_owned()));
                if let Some(typ) = payload_type {
                    let content = self.gen_value(&typ, source)?;
                    map.insert(attrs.content_field_name(case_attrs), content);
                }
                Ok(Value::Object(map))
            }
            JsonTaggedAttr::Internally => {
                let mut map = Map::new();
                map.insert(attrs.tag_field_name(), Value::String(case_name.to_owned()));
                if let Some(typ) = payload_type {
                    // Inline the payload's record fields next to the tag if it's a
                    // record; otherwise fall back to adjacent encoding.
                    if self.ir.record_type(&typ).is_some() {
                        self.inline_record_into(&typ, source, &mut map)?;
                    } else {
                        let content = self.gen_value(&typ, source)?;
                        map.insert(attrs.content_field_name(case_attrs), content);
                    }
                }
                Ok(Value::Object(map))
            }
            JsonTaggedAttr::Implicitly => {
                match payload_type {
                    None => Ok(Value::String(case_name.to_owned())),
                    Some(typ) => self.gen_value(&typ, source),
                }
            }
        }
    }
}

fn qualified_path(ir: &ir::Ir, instance: &ir::InstanceType) -> String {
    let bundle = &ir[instance.def.bundle];
    let schema = &ir[instance.def.schema];
    let def = &ir[instance.def];
    format!(
        "::{}::{}::{}",
        bundle.metadata.name,
        schema.name,
        def.name.as_str()
    )
}

// --- Curated edge-case tables --------------------------------------------
//
// Each table is ordered so that smaller indices are "simpler" (and the
// shrinker, when it lands, will pull toward index 0). The first entry is
// always the canonical zero/empty value; extremes follow.

const BOOL_TABLE: &[bool] = &[false, true];

const I8_TABLE: &[i8] = &[0, 1, -1, i8::MAX, i8::MIN, i8::MAX - 1, i8::MIN + 1, 2];
const I16_TABLE: &[i16] = &[
    0,
    1,
    -1,
    i16::MAX,
    i16::MIN,
    i16::MAX - 1,
    i16::MIN + 1,
    127,
    -128,
];
const I32_TABLE: &[i32] = &[
    0,
    1,
    -1,
    i32::MAX,
    i32::MIN,
    i32::MAX - 1,
    i32::MIN + 1,
    65_535,
    -65_536,
];

/// `i64` table — includes the JS safe-integer boundary (±2^53) so we exercise
/// the precision cliff that motivates `integers_as_strings`.
const I64_TABLE: &[i64] = &[
    0,
    1,
    -1,
    i64::MAX,
    i64::MIN,
    i64::MAX - 1,
    i64::MIN + 1,
    1 << 53,
    -(1 << 53),
    (1 << 53) - 1,
    (1 << 53) + 1,
];

const U8_TABLE: &[u8] = &[0, 1, u8::MAX, u8::MAX - 1, 2, 127, 128];
const U16_TABLE: &[u16] = &[0, 1, u16::MAX, u16::MAX - 1, 255, 256];
const U32_TABLE: &[u32] = &[0, 1, u32::MAX, u32::MAX - 1, u16::MAX as u32, 1 << 16];

const U64_TABLE: &[u64] = &[
    0,
    1,
    u64::MAX,
    u64::MAX - 1,
    1 << 53,
    (1 << 53) - 1,
    (1 << 53) + 1,
    u32::MAX as u64,
    (u32::MAX as u64) + 1,
];

const FLOAT_FINITE_TABLE: &[f64] = &[
    0.0,
    1.0,
    -1.0,
    -0.0,
    f64::MAX,
    f64::MIN,
    f64::MIN_POSITIVE,
    f64::EPSILON,
    2.0,
    -2.0,
    0.5,
    -0.5,
];

/// Float values that aren't valid JSON numbers — only emitted (as JSON
/// strings) when [`Config::floats_as_strings`] is set.
const FLOAT_SPECIAL_TABLE: &[&str] = &["NaN", "Infinity", "-Infinity"];

const STRING_TABLE: &[&str] = &[
    "",
    "a",
    "ASCII text",
    "héllo",
    "\"\\\n\t",
    "\u{0}embedded",
    "🦀",
    "  leading and trailing  ",
];

fn pick_bool(source: &mut Source) -> Value {
    Value::Bool(BOOL_TABLE[source.draw_choice(BOOL_TABLE.len())])
}

fn pick_string(source: &mut Source) -> Value {
    Value::String(pick_string_raw(source).to_owned())
}

fn pick_string_raw<'a>(source: &mut Source) -> &'a str {
    STRING_TABLE[source.draw_choice(STRING_TABLE.len())]
}

fn pick_signed<T: Copy + Into<i64>>(source: &mut Source, table: &[T]) -> Value {
    let value: i64 = table[source.draw_choice(table.len())].into();
    Value::Number(value.into())
}

fn pick_unsigned<T: Copy + Into<u64>>(source: &mut Source, table: &[T]) -> Value {
    let value: u64 = table[source.draw_choice(table.len())].into();
    Value::Number(value.into())
}

fn json_f64(value: f64) -> Value {
    serde_json::Number::from_f64(value)
        .map(Value::Number)
        .unwrap_or(Value::Null)
}
