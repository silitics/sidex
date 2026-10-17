---
title: Extending Sidex
description: Build tools from the resolved model and define typed attribute plugins.
order: 6
---

Sidex separates parsing and resolution from code generation. The compiler builds an intermediate representation, or _IR_, containing bundles, schemas, definitions, references, documentation, and attributes. Generators consume that model.

This separation lets a tool use the same schema as an application without reparsing the source or independently resolving imports.

## Start with Exported IR

```sh
sidex generate ir generated/ir
```

Read `generated/ir/ir.json` in a standalone tool. Its types are defined in the repository's `lib/meta` bundle and exposed by the `sidex-ir` crate. The JSON IR includes resolved references and `typedAttrs` entries for built-in plugins (`typed_attrs` in Rust).

For Rust tools that work directly with compiler output, `sidex-gen::Generator` accepts a generation job containing the IR, selected bundle, output path, and backend configuration. Existing generator crates provide examples.

IR is a tool integration surface under active development. Pin matching versions when consuming its structure.

## Define Typed Metadata

An attribute plugin can describe its accepted arguments with Sidex itself. For example, a custom UI tool might accept:

```sidex
#[ui(label = "Display Name")]
record Profile {
    display_name: string,
}
```

The attribute's schema can be defined as:

```sidex
import ::core::attrs::*

#[attrs(plugin = "ui", target = record)]
record RecordUi {
    label: string,
}
```

`attrs` describes the plugin name and target. The compiler validates an attribute against the registered schema and places the result in typed metadata. `#[attr(name = "...")]` can rename an argument in the plugin's schema, and `repeated` supports multiple independent attributes of the same family.

The compiler discovers `attrs` definitions in loaded schemas automatically. Include the attribute schema in the current bundle or a loaded dependency. A generator must still consume the resulting metadata to give it an effect; the schema alone does not add UI generation. The built-in plugins under `crates/sidex-core/lib` provide examples.

## Extend a Generator

Rust and TypeScript generators have plugin traits that visit bundles, schemas, and definitions. Their standard type generation stays enabled while additional plugins add constructors, serialization, or validation.

A custom Rust integration can implement the generator or plugin traits and consume the resolved metadata. The stock CLI's backend registry and available generator plugins are compiled in; it does not discover an external executable from a manifest entry.

## Repository Map

| Path                             | Purpose                                                  |
| -------------------------------- | -------------------------------------------------------- |
| `crates/sidex-syntax`            | Source syntax                                            |
| `crates/sidex-core`              | Bundle loading, resolution, attribute parsing, and lints |
| `crates/sidex-ir` and `lib/meta` | IR model                                                 |
| `crates/sidex-gen-*`             | Code generation backends                                 |
| `crates/sidex-attrs-*`           | Typed attribute access                                   |
| `crates/sidex`                   | Rust runtime: shapes, formats, and validation            |
| `crates/sidex-validate-expr`     | Validation expression engine shared by the generators    |
| `languages/typescript/packages`  | TypeScript runtime packages                              |

The [source repository](https://github.com/silitics/sidex) contains the implementation and examples. Run the repository's relevant checks with `mise` when changing compiler or generator behavior.
