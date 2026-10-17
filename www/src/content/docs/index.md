---
title: Sidex Documentation
description: Define shared data models and generate code for every part of your stack.
order: 0
---

Sidex is a data modeling language. A schema describes the structure of your data independently of a programming language. Code generators turn that schema into Rust types, TypeScript types, Python models, and JSON Schema.

Use Sidex for data that crosses a boundary: API requests and responses, configuration files, persistent data, or messages shared by several applications.

## Start with a Working Model

The [getting started guide](/sidex/docs/getting-started/) takes you from a new bundle to generated code with a small task model. A _bundle_ is a directory containing a `sidex.toml` manifest and a `schemas` directory.

Already have a model? Choose your target:

| Target      | Generated Output                                                                      | Guide                                              |
| ----------- | ------------------------------------------------------------------------------------- | -------------------------------------------------- |
| Rust        | Structs, enums, newtypes, and optional Serde, builder, and validation implementations | [Rust](/sidex/docs/generation/rust/)               |
| TypeScript  | Types matching the JSON representation, nominal wrappers, and optional validators     | [TypeScript](/sidex/docs/generation/typescript/)   |
| Python      | Pydantic models, root models, and JSON adapters                                       | [Python](/sidex/docs/generation/python/)           |
| JSON Schema | Schema documents for concrete JSON representations                                    | [JSON Schema](/sidex/docs/generation/json-schema/) |

## Understand the Model

A [record](/sidex/docs/language/types/#records) groups named fields. A [variant](/sidex/docs/language/types/#variants) describes a choice between cases. Wrappers create distinct types, aliases reuse existing types, and opaque types leave their implementation to a target language.

[Attributes](/sidex/docs/language/attributes/) attach additional information to the model. They can set JSON field names, map a type to a native implementation, declare validation rules, or describe an API's lifecycle.

## Keep a Shared Contract

A schema's type structure and its JSON representation are related but separate. Read [JSON mapping](/sidex/docs/guides/json/) before exchanging data between languages. [Validation rules](/sidex/docs/guides/validation/) add application constraints such as size limits or numeric ranges.

For an existing API, [lifecycle attributes](/sidex/docs/guides/api-lifecycle/) record when items were introduced and how they should be retired. [Request metadata](/sidex/docs/guides/requests/) pairs a request with its response type for custom tooling. [API groups](/sidex/docs/guides/api-groups/) document which definitions share a compatibility promise, such as definitions used by clients you cannot update.

## Find a Specific Setting

The reference covers [CLI commands](/sidex/docs/reference/cli/), the [bundle manifest](/sidex/docs/reference/manifest/), and [built-in attributes](/sidex/docs/reference/attributes/). To build your own generator or attribute plugin, see [extending Sidex](/sidex/docs/guides/extending-sidex/).

:::note[Documentation Version]
These docs describe Sidex's main branch. Sidex is under active development; a published CLI package may lag behind features described here. [Build from source](/sidex/docs/installation/#build-from-source) to use the matching implementation.
:::
