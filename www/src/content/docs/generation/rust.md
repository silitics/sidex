---
title: Rust
description: Generate Rust types with constructors, JSON support, and validation.
order: 0
---

The Rust generator emits structs for records, enums for variants, newtypes for wrappers, and aliases for type aliases. It writes a `mod.rs` containing one module per schema.

Generated types implement the `Shaped`, `Encode`, and `Decode` traits of the `sidex` crate. Their _shape_ describes a type's structure, documentation, attributes, and [validation rules](/sidex/docs/guides/validation/). The `sidex` crate uses shapes to read and write the [JSON representation](/sidex/docs/guides/json/) and to check rules.

## Generate a Module

For the [task bundle](/sidex/docs/getting-started/), enable additional plugins in `sidex.toml`:

```toml
[backend.rust]
derive = ["Debug", "Clone"]
plugins = ["builder"]
```

```sh
sidex generate rust generated/rust
```

The `types` plugin is built in. Add `builder` for constructors and `serde` for Serde's `Serialize` and `Deserialize`, which use the JSON representation and need the `serde-1` feature of the `sidex` crate. Do not add `types` again.

## Generate during a Cargo Build

Place the task bundle in a `models` directory next to your application's `Cargo.toml`. Add:

```toml title="Cargo.toml"
[dependencies]
sidex = "0.1"

[build-dependencies]
sidex-build-rs = "0.1"
```

The `sidex` crate provides the runtime used by generated code. `sidex-build-rs` loads bundles during the build and places generated modules in Cargo's output directory.

```rust title="build.rs"
fn main() {
    sidex_build_rs::configure()
        .with_bundle("models")
        .generate()
        .expect("Failed to generate task models.");
}
```

Include the bundle by its manifest name:

```rust title="src/main.rs"
use sidex::formats::json;

sidex::include_bundle!(pub tasks);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let task = tasks::task::Task::new("Write a schema", false);
    let text = json::to_string(&task)?;
    let decoded: tasks::task::Task = json::from_str(&text)?;
    assert_eq!(decoded.title, task.title);
    println!("{text}");
    Ok(())
}
```

`json::from_str` ignores fields that a record does not define, so older readers accept fields that later versions add. For documents written by people, such as configuration files, `json::from_str_with_unknown` also returns the paths of such fields, so that applications can warn about likely typos:

```rust
let (config, unknown) = json::from_str_with_unknown::<Config>(&text)?;
for path in unknown {
    eprintln!("warning: unknown field `{path}`");
}
```

`json::from_str_strict` rejects such fields instead. With the `toml` feature, `sidex::formats::toml` reads and writes TOML in the same way.

The build helper emits Cargo rerun directives for relevant files and formats output with Rustfmt when available. Use matching Sidex crate versions for the features your bundle requires.

## Built-in Types

Strings, sequences, maps, and bytes are `sidex::types::Str`, `Seq`, `Map`, and `Bytes`. Clones share their data, so cloning large values is cheap, and mutation copies shared data first. Maps preserve the insertion order of their entries. `uuid`, `timestamp`, and `date` are `sidex::types::Uuid`, `Timestamp`, and `Date`, and `json` is `sidex::JsonValue`.

To use other Rust types, map the built-in type in `backend.rust.types.table`. Rust's `String`, `Vec`, `HashMap`, and `BTreeMap` implement the runtime traits:

```toml
[backend.rust.types.table]
"::core::builtins::string" = "::std::string::String"
```

The `sidex` crate does not expose third-party crates. Its compatibility features, such as `uuid-1` and `jiff-02`, add conversions to and from types of other crates and implement the runtime traits for them.

## Constructors and Field Visibility

Generated record fields are public by default. Restrict them with `#[rust(private)]`, `#[rust(pub(crate))]`, or `#[rust(pub(super))]`. A field's generated setter and `with_...` method use the same visibility.

The `builder` plugin adds `new`, `set_<field>`, and `with_<field>` methods. Optional fields are defaulted in `new`. A required field with `#[default]` is also initialized with Rust's `Default::default()`. Required string and bytes parameters accept any value convertible into them:

```sidex
record Settings {
    name: string,
    #[default]
    enabled: bool,
    description?: string,
}
```

```rust
let settings = Settings::new("demo")
    .with_enabled(true)
    .with_description(Some("Development settings".into()));
```

A constructor supplies defaults; it does not check validation rules. Call `sidex::validation::validate` explicitly.

## Native Types and Derives

An opaque type needs a native Rust type, which must implement the runtime traits:

```sidex
#[rust(derive(PartialEq, Eq))]
wrapper UserId: string

#[json(type = "string")]
#[rust(type = "::uuid::Uuid")]
opaque UserUuid
```

`uuid::Uuid` implements the traits with the `uuid-1` feature. For your own types, `sidex::types::impl_text!` implements the traits for types represented as strings, using their `Display` and `FromStr` implementations. `sidex::serde::impl_opaque!` implements them for types with Serde implementations, using the representation `serde_json` produces:

```rust
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Reading {
    Value(f64),
    Fault(String),
}

sidex::serde::impl_opaque!(Reading);
```

The application must provide any dependency referenced by a native type. `derive` adds traits to the manifest-wide list. `attr(...)` adds native Rust attributes:

```sidex
#[rust(attr(non_exhaustive))]
variant Status {
    Pending,
    Complete,
}
```

## Indirection and Type Mappings

For a recursive field, use `box`, `arc`, `rc`, or `wrap`:

```sidex
record Node {
    label: string,
    #[rust(arc)]
    next?: Node,
}
```

These change the Rust representation without adding a JSON nesting level.

Map external bundles with `backend.rust.external` and override resolved types with `backend.rust.types.table`. See the [manifest reference](/sidex/docs/reference/manifest/#rust-backend) for syntax.
