---
title: Bundles and Imports
description: Organize schemas and reuse types across files and projects.
order: 1
---

A _bundle_ is the unit Sidex loads and generates. It has a manifest and a flat directory of schemas:

```text
models/
├── sidex.toml
└── schemas/
    ├── users.sidex
    └── projects.sidex
```

A schema's filename gives it a name within the bundle. The bundle name, schema name, and definition name identify a type, such as `models::users::User`.

## Define a Bundle

```toml title="sidex.toml"
[bundle]
name = "models"
version = "0.1.0"
```

Commands such as `sidex check` and `sidex generate` locate `sidex.toml` by searching the current directory and its parents. Rust, TypeScript, and Python generation handle the selected bundle. Generate their dependency bundles separately. JSON Schema generation also walks loaded noninternal dependencies.

## Import from Another Schema

In `schemas/users.sidex`:

```sidex
wrapper UserId: string

record User {
    id: UserId,
    display_name: string,
}
```

In `schemas/projects.sidex`:

```sidex
import users::{User, UserId}

record Project {
    owner_id: UserId,
    members: [User],
}
```

Imports without a leading `::` are relative to the current bundle. Group imports share a prefix; `import users::*` imports all definitions from a schema. Prefer explicit names when they make dependencies easier to see.

## Depend on Another Bundle

Add a path dependency relative to the bundle directory:

```toml
[dependencies.identity]
path = "../identity"
```

An absolute import begins with `::`:

```sidex
import ::identity::users::UserId

record Project {
    owner_id: UserId,
}
```

Dependencies are local paths. Sidex does not resolve them from a package registry. The dependency key should match the imported bundle name.

Each generator may need a mapping from the dependency's bundle name to its generated language module:

```toml
[backend.rust.external]
identity = "::identity_models"

[backend.ts.external]
identity = "@example/identity-models"

[backend.py.external]
identity = "identity_models"
```

Generate the dependency separately and make its output available to the consuming application. See the [manifest reference](/sidex/docs/reference/manifest/) for all backend settings.

## Document Schemas

```sidex
//! Shared user identities.

/// A user's public profile.
record Profile {
    /// Name shown to other users.
    display_name: string,
}
```

`//!` comments document a schema. `///` comments document the next definition, field, or variant case. Ordinary `//` comments stay in the source and do not become generated documentation.
