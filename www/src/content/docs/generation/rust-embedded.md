---
title: Rust for Embedded Targets
description: Generate allocation-free Rust types for microcontrollers and other no_std targets.
order: 1
---

The `rust-embedded` generator emits Rust types for `no_std` targets without an allocator, such as microcontrollers. Like the [Rust generator](/sidex/docs/generation/rust/), it emits structs for records, enums for variants, newtypes for wrappers, and aliases for type aliases, with one module per schema. The generated types implement the `Decode` and `Encode` traits of the `sidex-embedded` crate, which reads and writes the [JSON representation](/sidex/docs/guides/json/). The crate has no dependencies and never allocates.

Use it for firmware that exchanges JSON with applications built from the same bundle, for example, a sensor that reports readings to a service. Where an allocator is available, the Rust generator offers more, such as constructors, validation, and Serde support.

## How Decoded Values Borrow from the Input

Decoding reads a complete JSON document from a byte slice, for example, the body of an HTTP response. Decoded values do not copy strings and collections into memory of their own. They borrow them from the slice instead. Generated types that contain strings, sequences, maps, bytes, or JSON values therefore take a lifetime parameter `'a`, and the input must outlive the decoded values.

Decoding checks the entire input against the type, so errors in the input surface when decoding. Strings, sequences, maps, and bytes keep the part of the input that holds them and decode it again on access. Because the input has already been checked, iterating over a sequence or map cannot fail. Iteration decodes the items one by one, which needs no allocation and keeps the size of decoded values independent of the input's size. In return, reading items repeatedly decodes them repeatedly. `Seq::get` decodes the items before the requested one, and `Map::get` decodes all entries because the last entry with a key counts.

Strings with escape sequences, such as `\n`, stay escaped in the input. `Str::as_str` returns the text of strings without escape sequences. `Str::chars` and `Str::unescape_into` replace escape sequences while reading.

To encode values, applications build them from data they own, for example, a sequence from a slice with `Seq::new`. Encoding writes to a byte slice or to a `Sink`, a trait for destinations such as network connections.

## Generate during a Cargo Build

Place the [task bundle](/sidex/docs/getting-started/) in a `models` directory next to your firmware's `Cargo.toml` and configure the generator in its `sidex.toml`:

```toml title="sidex.toml"
[backend.rust-embedded]
derive = ["Debug"]
```

Generated structs and enums derive `Clone` and `Copy`. `derive` adds further traits. Add the runtime and the build helper to your firmware's `Cargo.toml`:

```toml title="Cargo.toml"
[dependencies]
sidex-embedded = "0.6"

[build-dependencies]
sidex-build-rs = "0.6"
```

With the `embedded` option, `sidex-build-rs` uses the `rust-embedded` generator:

```rust title="build.rs"
fn main() {
    sidex_build_rs::configure()
        .embedded()
        .with_bundle("models")
        .generate()
        .expect("Failed to generate task models.");
}
```

For the `Task` record, the generator emits:

```rust
#[derive(Clone, Copy, Debug)]
pub struct Task<'a> {
    pub title: ::sidex_embedded::types::Str<'a>,
    pub completed: bool,
    pub description: ::core::option::Option<::sidex_embedded::types::Str<'a>>,
}
```

Include the bundle by its manifest name and use the `json` module to read and write tasks:

```rust title="src/lib.rs"
#![no_std]

use sidex_embedded::json;
use sidex_embedded::types::Str;

sidex_embedded::include_bundle!(pub tasks);

use tasks::task::Task;

/// Reads a task from `input`, marks it as complete, and writes it to `output`.
pub fn complete(input: &[u8], output: &mut [u8]) -> Option<usize> {
    let task: Task<'_> = json::from_slice(input).ok()?;
    json::to_slice(&Task { completed: true, ..task }, output).ok()
}

/// Writes a new task to `output`.
pub fn create(title: &str, output: &mut [u8]) -> Option<usize> {
    let task = Task {
        title: Str::new(title),
        completed: false,
        description: None,
    };
    json::to_slice(&task, output).ok()
}
```

The output buffer must hold the entire document. `json::encoded_len` returns its length without writing it, for example, to set the length of an HTTP request.

`sidex generate rust-embedded OUTPUT` writes the same module to a directory instead.

### Generate Only a Group

An API often serves several kinds of clients, while a device uses only a small part of it. With `groups`, the generator emits only the definitions of the selected [groups](/sidex/docs/guides/api-groups/) and the definitions they use:

```toml title="sidex.toml"
[backend.rust-embedded]
groups = ["device"]
```

## Built-in Types

| Sidex type                         | Rust type                                         |
| ---------------------------------- | ------------------------------------------------- |
| `bool`, `f32`, `f64`, integers     | `bool`, `f32`, `f64`, integers; `idx` is `usize` |
| `string`                           | `sidex_embedded::types::Str<'a>`                  |
| `bytes`                            | `sidex_embedded::types::Bytes<'a>`                |
| `[T]`                              | `sidex_embedded::types::Seq<'a, T>`               |
| `[K: V]`                           | `sidex_embedded::types::Map<'a, K, V>`            |
| `json`                             | `sidex_embedded::types::Json<'a>`                 |
| `uuid`, `timestamp`, `date`        | `sidex_embedded::types::Uuid`, `Timestamp`, `Date` |

Keys of maps may be strings, integers, UUIDs, timestamps, dates, and wrappers of them. Maps keep entries with duplicate keys, and `Map::get` returns the value of the last one.

`Json` holds the text of decoded JSON values. `Json::decode` decodes the text as a type, and `Json::from_value` creates a JSON value from any value implementing `Encode`.

## Opaque Types and Type Mappings

For the Rust generator, opaque types name a native type with `#[rust(type = "...")]`. The `rust-embedded` generator ignores this attribute and emits a newtype of the opaque type's JSON representation instead: `Str<'a>` for `#[json(type = "string")]`, `bool` for `#[json(type = "boolean")]`, and `Json<'a>` otherwise. Generic opaque types have no such newtype and need a mapping.

To use a type of your own, map the Sidex type to it in `types.table`. Write the lifetime `'a` if the type borrows from the input:

```toml title="sidex.toml"
[backend.rust-embedded.types.table]
"::tasks::task::TaskId" = "crate::TaskId<'a>"
```

The type must be `Copy` and implement `Decode<'a>` and `Encode`, and `MapKey<'a>` to be the key of a map. Wrappers of mapped types cannot be keys of maps. The `sidex-embedded` crate documents how to implement the traits.

## Differences from the Rust Generator

- Generated types do not check [validation rules](/sidex/docs/guides/validation/).
- There are no plugins, such as `builder` and `serde`, and no shapes.
- Fields are always public. Of the `rust` attributes, only `#[rust(name = "...")]` takes effect. `type`, `derive`, `attr`, and the visibility attributes are ignored.
- Recursive types need a sequence or map between a type and itself. The indirection attributes `box`, `arc`, `rc`, and `wrap` are errors.
- Decoding always ignores unknown fields.
- Errors report the byte offset of the offending value instead of its path.
