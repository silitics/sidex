---
sidebar_position: 1
---

# Rust

The runtime of the Sidex backend is provided by the [`sidex`](https://crates.io/crates/sidex) crate. Behind the `gen` feature gate, this crate also provides procedural macros for generating Rust code from a bundle. This makes it particularly easy to use Sidex with Rust.

## Type Mapping

Opaque type definitions can be annotated with the `type` attribute to specify their Rust counterpart:

```sidex
#[rust(type = "<PATH>")]
```

Record, variant, and wrapper type definitions can be annotated with the `derive` attribute:

```sidex
#[rust(derive(...))]
```

In addition, type definitions, fields, and variants can be annotated with any Rust attribute via the `attr` attribute:

```sidex
#[rust(attr(...))]
```

For instance, the `non_exhaustive` attribute can be applied with:

```sidex
#[rust(attr(non_exhaustive))]
```

On fields and variants with associated data, the `wrap` attribute can be used to wrap the Rust type of the field in another generic type:

```sidex
#[rust(wrap = "<PATH>")]
```

### Recursive Types and Boxes

Sidex allows defining recursive data types which requires boxing in Rust.

To this end, a shortcut for wrapping field or variant data in a box is available:

```sidex
#[rust(box)] ⇒ #[rust(wrap = "::std::boxed::Box")]
```

In the future, this backend may use analysis techniques to break recursion automatically.

### Reference-Counted Smart Pointers

Similar to boxing, there are shortcuts for wrapping field and variant types in `Arc` and `Rc`:

```sidex
#[rust(arc)] ⇒ #[rust(wrap = "::std::sync::Arc")]
#[rust(rc)] ⇒ #[rust(wrap = "::std::rc::Rc")]
```

### Visibility

By default everything will be `pub`.

The following attributes can be used to modify visibility:

```sidex
#[rust(pub)]
#[rust(pub(crate))]
#[rust(pub(super))]
#[rust(private)]
```

## Interoperability with Serde

[Serde](https://serde.rs) is the de-facto standard for serialization and deserialization in Rust.
Sidex is fully compatible with the Serde ecosystem.
With the `serde` plugin of the Rust backend, code for the `Serialize` and `Deserialize` traits can be generated.

Note that there are a few differences to Serde's own derive macros:

- Optional fields of record types are always skipped if they are `None`.
- A tagged representation of variant types is only generated for human-readable formats. To this end, the JSON `tagged` attribute is used. By default, variant types are internally tagged instead of externally tagged.

When serializing with `serde_json` the representation is the standard JSON representation.

In particular, this means that out-of-the-box you can use any Serde supported interchange format with Sidex.

**🚧 TODO:** Explain how Sidex plays together with Serde.

## Configuration

```toml
# Derive `Clone` and `Debug` by default for all types
# without an explicit `derive` attribute.
derive = ["Clone", "Debug"]

[mapping]
"::sidex::builtins::Sequence" = "::std::vec::Vec"
"::sidex::builtins::Map" = "::std::hash_map::HashMap"
"::sidex::builtins::string" = "::std::string::String"
"::sidex::builtins::bytes" = "::std::vec::Vec<u8>"
```
