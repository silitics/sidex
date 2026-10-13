---
title: Attribute Reference
description: Supported built-in attributes, targets, and arguments.
order: 2
---

Attributes precede the element they annotate. Use `#[...]` for definitions, fields, and cases, or `#![...]` for schema metadata. The [attribute guide](/sidex/docs/language/attributes/) explains syntax.

## JSON

| Target       | Arguments                                                              | Meaning                                                  |
| ------------ | ---------------------------------------------------------------------- | -------------------------------------------------------- |
| Opaque type  | `type = "..."`, `schema = "..."`                                       | JSON shape and optional JSON Schema reference            |
| Record       | `rename_all = "..."`                                                   | Default field naming                                     |
| Field        | `name = "..."`, `rename = "..."`, `inline`                             | Explicit name, naming convention, or inlined record      |
| Variant      | `rename_all = "..."`, `tagged = ...`, `tag = "..."`, `content = "..."` | Case naming and tagging                                  |
| Variant case | `name = "..."`, `rename = "..."`, `content = "..."`                    | Case name and content field override                     |

JSON shape names are `number`, `boolean`, `string`, `null`, `object`, `array`, and `any`. Combine them with `|`, as in `"string|null"`.

Naming conventions are `none`, `lowercase`, `UPPERCASE`, `camelCase`, `PascalCase`, `snake_case`, `SCREAMING_SNAKE_CASE`, `kebab-case`, and `SCREAMING-KEBAB-CASE`.

Tagging values are unquoted paths: `internally`, `adjacently`, `externally`, and `implicitly`. Defaults are internal tagging, `tag` for the tag key, `content` for the payload key, camelCase record fields, and unchanged case names.

An `inline` field merges the entries of its record into the surrounding object. It must be required, and its type must be a record or resolve to one through aliases and wrappers. JSON field names must be unique after renaming and flattening nested inline records, including the tag of an internally tagged variant. See [JSON mapping](/sidex/docs/guides/json/) for examples.

## Rust

| Target     | Arguments                                                                                        |
| ---------- | ------------------------------------------------------------------------------------------------ |
| Definition | `type = "..."`, `derive(...)`, `attr(...)`                                                       |
| Field      | `name = "..."`, `pub`, `pub(crate)`, `pub(super)`, `private`, `box`, `arc`, `rc`, `wrap = "..."` |

Fields are public by default. `pub` and `private` cannot both apply to a field. Indirection attributes wrap the native field type without changing JSON shape. `derive` and `attr` arguments accumulate.

```sidex
#[rust(derive(Clone, Debug), attr(non_exhaustive))]
record Profile {
    #[rust(private)]
    display_name: string,
}
```

A bare `#[default]` on a record field tells the Rust `builder` plugin to initialize it with `Default::default()` instead of adding it to the constructor arguments.

## Python

`#[py(typ = "module.Type")]` maps an opaque definition to a native Python type:

```sidex
#[json(type = "string")]
#[py(typ = "uuid.UUID")]
opaque UserId
```

The argument is named `typ`, while the Rust attribute uses `type`.

## Validation

Targets are fields, records, wrappers, and opaque types. Each attribute contains one expression and optional `message` and `code` strings:

```sidex
#[validate({ 1 <= _.size <= 64 }, message = "Use 1 to 64 characters.", code = "name:size")]
```

Repeat attributes for multiple rules. See [validation](/sidex/docs/guides/validation/) for expression syntax, generator activation, and runtime limitations.

## API Lifecycle

These attributes target definitions, fields, cases, and schemas:

| Attribute    | Arguments                                                      |
| ------------ | -------------------------------------------------------------- |
| `deprecated` | Optional `since`, `remove_in`, and `note` strings; may be bare |
| `unstable`   | Optional `feature`, `note`, and `issue` strings; may be bare   |
| `since`      | Required `version` string                                      |

See [API lifecycle](/sidex/docs/guides/api-lifecycle/) for lint behavior and migration examples.

## Requests

`request` applies to a request definition and requires:

- `name = "..."`: wire request identifier.
- `response = Type`: resolved response type reference.

```sidex
#[request(name = "tasks.CreateTask", response = CreateTaskOutput)]
record CreateTask {
    title: string,
}
```

Define or import `CreateTaskOutput` in the same bundle. [Request metadata](/sidex/docs/guides/requests/) explains how tools can consume this relationship.

## Attribute Plugin Schemas

`attrs` and `attr` describe custom typed attribute plugins:

```sidex
#[attrs(plugin = "ui", target = record, repeated)]
record UiRule {
    #[attr(name = "label")]
    display_label: string,
}
```

They are compiler extension metadata. See [extending Sidex](/sidex/docs/guides/extending-sidex/) for discovery and generator integration.
