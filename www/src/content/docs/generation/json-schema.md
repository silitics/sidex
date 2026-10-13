---
title: JSON Schema
description: Generate schema documents for the JSON representation of your models.
order: 3
---

JSON Schema describes the structure of JSON values. Sidex's `json-schema` backend generates Draft 2020-12 documents from your model and its `json` attributes.

## Generate Schemas

From the [task bundle](/sidex/docs/getting-started/):

```sh
sidex generate json-schema generated/json-schema
```

The output includes:

```text
generated/json-schema/
├── tasks.task.Task.schema.json
└── schema-defs.json
```

Type schema filenames contain the bundle, schema, and definition names separated by dots. A root schema embeds transitively referenced definitions in `$defs`, so it can describe nested data without loading every unrelated model.

The generator walks loaded noninternal bundles, including dependencies. It also emits `schema-defs.json`, a definitions map whose references use `#/components/schemas/` for integration with an API description. That file is a component map, not a complete OpenAPI document.

## Validate Incoming JSON

Load the schema document for the type you expect and pass it to a validator supporting Draft 2020-12. For the task example, the schema distinguishes:

- Required `title` and `completed` fields.
- An optional `description` field.
- A string description when the field is present.
- JSON field names and variant tagging chosen with attributes.

Validation of shape is separate from application constraints. The backend does not currently translate `#[validate(...)]` rules into JSON Schema.

In TypeScript, the generated [`is` and `check` methods](/sidex/docs/generation/typescript/#check-incoming-data) check incoming JSON without a JSON Schema validator.

## Opaque Types

Declare an opaque type's JSON representation:

```sidex
#[json(type = "string")]
opaque UserId
```

To reference an independently maintained schema, add `schema`:

```sidex
#[json(type = "string", schema = "https://example.com/schemas/user-id.json")]
opaque UserId
```

Your JSON Schema validator must be configured to resolve that reference.

## Generic Types

JSON Schema has no Sidex type parameters. Generate and use concrete instantiations, such as a nongeneric alias:

```sidex
record Page<T> {
    items: [T],
}

alias Names: Page<string>
```

The current generator can also emit documents for definitions with unbound parameters, but those contain placeholder references. Use schemas for concrete instances as validation contracts.

## Current Configuration Surface

The CLI backend uses its built-in JSON Schema settings. Fields such as `id_prefix` and the Rust library's `JsonSchemaConfig` are not wired through `sidex.toml` in the current generator. Use `json` attributes for the supported schema-level customization.
