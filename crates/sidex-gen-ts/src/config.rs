use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub plugins: Vec<String>,
    #[serde(default)]
    pub types: TypesConfig,
    #[serde(default)]
    pub plugin: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub external: HashMap<String, String>,
    /// How opaque types are lowered into TypeScript types.
    #[serde(default)]
    pub opaque_lowering: OpaqueLowering,
}

/// Strategy for lowering opaque definitions in the generated TypeScript code.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum OpaqueLowering {
    /// Use the opaque's `#[json(type = ...)]` attribute as-is, the historical
    /// behaviour. Without a JSON attribute the opaque resolves to `unknown`.
    #[default]
    Native,
    /// Same as [`Self::Native`] in TypeScript today, since TS opaques don't
    /// have a per-target type override — kept as a distinct variant so the
    /// flag is present on every codegen and conformance test drivers can be
    /// configured uniformly.
    Json,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TypesConfig {
    pub table: HashMap<String, String>,
}

impl Default for TypesConfig {
    fn default() -> Self {
        let mut cfg = Self {
            table: Default::default(),
        };
        cfg.populate_table_with_builtins();
        cfg
    }
}

impl TypesConfig {
    fn populate_table_with_builtins(&mut self) {
        macro_rules! populate_table {
            ($( $sidex_path:literal => ( $($type_script_type:tt)* ) ,)*) => {
                $(
                    if !self.table.contains_key($sidex_path) {
                        self.table.insert(
                            $sidex_path.to_owned(), stringify!($($type_script_type)*).to_owned()
                        );
                    }
                )*
            };
        }
        populate_table! {
            "::core::builtins::string" => (__sidex_types.builtins.String),
            "::core::builtins::bytes" => (__sidex_types.builtins.Bytes),
            "::core::builtins::i8" => (__sidex_types.builtins.I8),
            "::core::builtins::i16" => (__sidex_types.builtins.I16),
            "::core::builtins::i32" => (__sidex_types.builtins.I32),
            "::core::builtins::i64" => (__sidex_types.builtins.I64),
            "::core::builtins::u8" => (__sidex_types.builtins.U8),
            "::core::builtins::u16" => (__sidex_types.builtins.U16),
            "::core::builtins::u32" => (__sidex_types.builtins.U32),
            "::core::builtins::u64" => (__sidex_types.builtins.U64),
            "::core::builtins::idx" => (__sidex_types.builtins.Idx),
            "::core::builtins::f32" => (__sidex_types.builtins.F32),
            "::core::builtins::f64" => (__sidex_types.builtins.F64),
            "::core::builtins::bool" => (__sidex_types.builtins.Bool),
            "::core::builtins::unit" => (__sidex_types.builtins.Unit),
            "::core::builtins::Sequence" => (__sidex_types.builtins.Sequence),
            "::core::builtins::Map" => (__sidex_types.builtins.ObjectMap),
            "::meta::ir::Value" => (unknown),
        };
    }
}
