---
title: Rust
description: Generate Rust types, constructors, Serde implementations, and validators.
order: 0
---

The Rust generator emits structs for records, enums for variants, newtypes for wrappers, and aliases for type aliases. It writes a `mod.rs` containing one module per schema.

## Generate a Module

For the [task bundle](/sidex/docs/getting-started/), enable additional plugins in `sidex.toml`:

```toml
[backend.rust]
derive = ["Debug", "Clone"]
plugins = ["builder", "serde"]
```

```sh
sidex generate rust generated/rust
```

The `types` plugin is built in. Add `builder` for constructors, `serde` for serialization, and `validate` for [validation rules](/sidex/docs/guides/validation/). Do not add `types` again.

## Generate during a Cargo Build

Place the task bundle in a `models` directory next to your application's `Cargo.toml`. Add:

```toml title="Cargo.toml"
[dependencies]
sidex = "0.1"
serde = "1"
serde_json = "1"

[build-dependencies]
sidex-build-rs = "0.1"
```

The `sidex` crate re-exports the serialization runtime used by generated code. `sidex-build-rs` loads bundles during the build and places generated modules in Cargo's output directory.

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
sidex::include_bundle!(pub tasks);

fn main() -> Result<(), serde_json::Error> {
    let task = tasks::task::Task::new("Write a schema".into(), false);
    let json = serde_json::to_string(&task)?;
    let decoded: tasks::task::Task = serde_json::from_str(&json)?;
    assert_eq!(decoded.title, task.title);
    println!("{json}");
    Ok(())
}
```

The build helper emits Cargo rerun directives for relevant files and formats output with Rustfmt when available. Use matching Sidex crate versions for the features your bundle requires.

## Constructors and Field Visibility

Generated record fields are public by default. Restrict them with `#[rust(private)]`, `#[rust(pub(crate))]`, or `#[rust(pub(super))]`. A field's generated setter and `with_...` method use the same visibility.

The `builder` plugin adds `new`, `set_<field>`, and `with_<field>` methods. Optional fields are defaulted in `new`. A required field with `#[default]` is also initialized with Rust's `Default::default()`:

```sidex
record Settings {
    name: string,
    #[default]
    enabled: bool,
    description?: string,
}
```

```rust
let settings = Settings::new("demo".into())
    .with_enabled(true)
    .with_description(Some("Development settings".into()));
```

A constructor supplies defaults; it does not validate constraints. Call the generated validator explicitly when using `validate`.

## Native Types and Derives

```sidex
#[rust(derive(PartialEq, Eq))]
wrapper UserId: string

#[json(type = "string")]
#[rust(type = "::uuid::Uuid")]
opaque UserUuid
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

Map external bundles with `backend.rust.external` and override resolved types with `backend.rust.types.table`. See the [manifest reference](/sidex/docs/reference/manifest/#rust-backend) for syntax and runtime path overrides.
