---
title: Manifest Reference
description: Bundle metadata, local dependencies, and backend configuration in sidex.toml.
order: 1
---

A `sidex.toml` file identifies a bundle and configures its generators. Schema files live in a flat `schemas` directory next to the manifest.

## Bundle Metadata

```toml
[bundle]
name = "models"
version = "0.1.0"
```

`name` identifies the bundle in imports and fully qualified type paths. `version` is metadata used by lifecycle checks when it is a semantic version.

## Dependencies

```toml
[dependencies.identity]
path = "../identity"
```

Each dependency has a local `path`, resolved relative to the manifest's directory. Use the bundle name as the dependency key and import its types with an absolute path such as `::identity::users::UserId`.

## Rust Backend

```toml
[backend.rust]
derive = ["Debug", "Clone"]
plugins = ["builder", "serde"]
opaque_lowering = "native"

[backend.rust.external]
identity = "::identity_models"

[backend.rust.types.table]
"::core::builtins::Map" = "::std::collections::BTreeMap"
```

| Key               | Default           | Meaning                                 |
| ----------------- | ----------------- | --------------------------------------- |
| `derive`          | `[]`              | Native derives on generated types       |
| `plugins`         | `[]`              | Additional plugins: `builder`, `serde`  |
| `external`        | `{}`              | Bundle name to Rust module path         |
| `types.table`     | Built-in mappings | Fully qualified Sidex path to Rust type |
| `opaque_lowering` | `native`          | Opaque representation strategy          |

Generated types always carry their validation rules, so Rust has no `validate` plugin.

The `types` plugin is automatically enabled. User mappings take precedence over built-in defaults.

## TypeScript Backend

```toml
[backend.ts]
plugins = ["validate"]
opaque_lowering = "native"

[backend.ts.external]
identity = "@example/identity-models"

[backend.ts.types.table]
"::core::builtins::timestamp" = "string"
```

| Key               | Default           | Meaning                                         |
| ----------------- | ----------------- | ----------------------------------------------- |
| `plugins`         | `[]`              | Additional plugins; currently `validate`        |
| `external`        | `{}`              | Bundle name to module import                    |
| `types.table`     | Built-in mappings | Fully qualified Sidex path to type expression   |
| `opaque_lowering` | `native`          | Same JSON-based lowering for both current modes |

The `types` plugin is automatically enabled. If you supply a `types.table`, include all mappings your model needs: the TypeScript configuration deserializes that table as supplied instead of merging it with the default table.

## Python Backend

```toml
[backend.py]
opaque_lowering = "native"

[backend.py.external]
identity = "identity_models"

[backend.py.types.table]
"::core::builtins::string" = "str"
"::core::builtins::bool" = "bool"
```

| Key               | Default           | Meaning                                              |
| ----------------- | ----------------- | ---------------------------------------------------- |
| `external`        | `{}`              | Bundle name to importable Python module              |
| `types.table`     | Built-in mappings | Fully qualified Sidex path to Python type expression |
| `opaque_lowering` | `native`          | Opaque representation strategy                       |

When providing `types.table`, supply the mappings needed by the model. A supplied Python table also replaces the default table. Python generation has no configurable generator plugin list.

## Opaque Lowering

An opaque type can have a target-specific native mapping and a JSON shape. The `opaque_lowering` setting chooses between them:

| Backend    | `native`                                                         | `json`                                                  |
| ---------- | ---------------------------------------------------------------- | ------------------------------------------------------- |
| Rust       | Use `#[rust(type = "...")]`; a native mapping is required        | Ignore the native mapping and lower from the JSON shape |
| Python     | Use `#[py(typ = "...")]` when provided; otherwise use JSON shape | Ignore the native mapping and use JSON shape            |
| TypeScript | Use JSON shape                                                   | Currently the same as `native`                          |

JSON lowering is useful for raw-value conformance tests. When no JSON shape is given, the fallback is `sidex::JsonValue` in Rust, `pydantic.JsonValue` in Python, and `unknown` in TypeScript.

## IR and JSON Schema

The IR backend has no manifest configuration. The current JSON Schema CLI generator uses its built-in configuration and does not consume a manifest `id_prefix` setting. Use `json` attributes to configure supported wire representations.

## Command Overrides

```sh
sidex generate py out --config-override opaque_lowering=json
```

Overrides change configuration for a single command. See the [CLI reference](/sidex/docs/reference/cli/#generate) for value parsing and key escaping.
