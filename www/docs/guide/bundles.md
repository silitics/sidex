---
sidebar_position: 2
---

# Bundles and Schemas

Most programming languages have some concept of _packages_, _namespaces_, or _modules_ such that related things can be grouped. Sidex is no exception to this rule. At the top level, similar to Rust's _crates_ or Python's _packages_, Sidex has _bundles_.

Each bundle gets its own _bundle directory_ with a _bundle manifest_:

```toml title="sidex.toml"
[bundle]
name = "my_bundle"
version = "0.1.0"
```

The bundle manifest specifies the `name` of the bundle and its `version`.

Every bundle consists of a flat collection of schemas (in the `schemas` directory) with type and service definitions. As bundles are versioned, you should use them to group schemas that are expected to evolve together.

Note that the decision to have only two levels of nesting (bundles and schemas) has been intentional. This decision simplifies code generation, and these two levels should provide enough structural flexibility for any practical use cases.

Check out the [getting started](../getting-started.md) instructions for how to create a bundle and an example schema.

## Importing Definitions

To import a type or service definition from another schema in the same bundle, use an <code className="keyword">import</code> directive. For instance,

```sidex
import schema_name::TypeDef1
```

imports the type definition <code className="type-name">TypeDef1</code> from the schema `schema_name`.

Multiple and wildcard imports are also allowed using a Rust-inspired syntax:

```sidex
import schema_name::{TypeDef1, TypeDef2}
import other_schema::*  // Import all definitions.
```

## Dependencies and Imports

To use types and services defined in other bundles, you must first add the other bundles as _dependencies_ to the bundle where you want to use these definitions. To this end, use the `dependencies` section of the bundle manifest.

For example, to introduce a dependency on `other_bundle`:

```toml
[dependencies]
other_bundle = { path = "../other_bundle" }
```

Here, `path` is a path to the dependency relative to the bundle manifest.

Definitions can then be imported with the <code className="keyword">import</code> directive. For example:

```sidex
import ::other_bundle::some_schema::{TypeDefX, TypeDefY}
import ::other_bundle::other_schema::*
```

Note the proceeding `::` indicating an absolute path.
