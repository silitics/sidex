use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub derive: Vec<String>,
    #[serde(default)]
    pub plugins: Vec<String>,
    #[serde(default)]
    pub types: TypesConfig,
    #[serde(default)]
    pub plugin: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TypesConfig {
    pub table: HashMap<String, String>,
}

impl TypesConfig {
    pub fn populate_table_with_builtins(&mut self) {
        macro_rules! populate_table {
            ($( $sidex_path:literal => $rust_path:ty ,)*) => {
                $(
                    if !self.table.contains_key($sidex_path) {
                        self.table.insert(
                            $sidex_path.to_owned(), stringify!($rust_path).to_owned()
                        );
                    }
                )*
            };
        }
        populate_table! {
            "::core::builtins::string" => ::std::string::String,
            "::core::builtins::bytes" => ::std::vec::Vec<u8>,
            "::core::builtins::i8" => i8,
            "::core::builtins::i16" => i16,
            "::core::builtins::i32" => i32,
            "::core::builtins::i64" => i64,
            "::core::builtins::u8" => u8,
            "::core::builtins::u16" => u16,
            "::core::builtins::u32" => u32,
            "::core::builtins::u64" => u64,
            "::core::builtins::idx" => usize,
            "::core::builtins::f32" => f32,
            "::core::builtins::f64" => f64,
            "::core::builtins::bool" => bool,
            "::core::builtins::unit" => (),
            "::core::builtins::Sequence" => ::std::vec::Vec,
            "::core::builtins::Map" => ::std::collections::HashMap,
        };
    }
}
