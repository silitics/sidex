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
            "::std::builtins::string" => ::std::string::String,
            "::std::builtins::bytes" => ::std::vec::Vec<u8>,
            "::std::builtins::i8" => i8,
            "::std::builtins::i16" => i16,
            "::std::builtins::i32" => i32,
            "::std::builtins::i64" => i64,
            "::std::builtins::u8" => u8,
            "::std::builtins::u16" => u16,
            "::std::builtins::u32" => u32,
            "::std::builtins::u64" => u64,
            "::std::builtins::idx" => usize,
            "::std::builtins::f32" => f32,
            "::std::builtins::f64" => f64,
            "::std::builtins::bool" => bool,
            "::std::builtins::unit" => (),
            "::std::builtins::Sequence" => ::std::vec::Vec,
            "::std::builtins::Map" => ::std::collections::HashMap,
        };
    }
}
