---
title: Attributes
description: Add format, language, validation, and lifecycle information to a model.
order: 2
---

An attribute is metadata attached to a schema element. The type structure says what values exist; attributes tell tools how to represent, validate, or document those values.

Built-in attribute plugins parse known attributes into typed metadata. Generators consume that metadata. An attribute's effect therefore depends on the generator and any enabled generator plugins.

## Attach Attributes

Attributes precede the definition, field, or case they describe:

```sidex
#[json(rename_all = "snake_case")]
record User {
    #[json(name = "name")]
    display_name: string,

    #[validate({ 1 <= _.size <= 64 })]
    username: string,
}
```

This record uses snake_case JSON keys, overrides one key to `name`, and places a size rule on `username`. The rule produces runtime validation only when a supported validation plugin is enabled.

Schema-level attributes use `#![...]`:

```sidex
#![unstable(note = "This schema is being revised.")]
```

## Attribute Arguments

Arguments can be flags, assignments, nested lists, paths, or token bodies:

```sidex
#[json(tagged = externally)]
#[rust(derive(Clone, Debug))]
#[validate({ 0 <= _ <= 100 }, message = "Use a value from 0 to 100.")]
```

Strings are quoted. Path values such as `externally` and type references such as `User` are not. The `{ ... }` body of a validation rule is parsed by the validation expression engine.

Several attributes can apply to one element. Repeated `validate` attributes create separate rules; Rust's `derive` and `attr` arguments accumulate.

## Choose an Attribute Family

| Family                            | Purpose                                                 | Details                                                                           |
| --------------------------------- | ------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `json`                            | Wire names, variant tagging, and opaque representations | [JSON mapping](/sidex/docs/guides/json/)                                          |
| `rust`                            | Native types, derives, visibility, and indirection      | [Rust generation](/sidex/docs/generation/rust/)                                   |
| `py`                              | Native opaque type mapping                              | [Python generation](/sidex/docs/generation/python/)                               |
| `validate`                        | Runtime constraints                                     | [Validation](/sidex/docs/guides/validation/)                                      |
| `deprecated`, `unstable`, `since` | API lifecycle metadata                                  | [API lifecycle](/sidex/docs/guides/api-lifecycle/)                                |
| `request`                         | Request name and response type                          | [Requests](/sidex/docs/guides/requests/)                                          |
| `default`                         | Default a field in a generated Rust constructor         | [Rust generation](/sidex/docs/generation/rust/#constructors-and-field-visibility) |

The [attribute reference](/sidex/docs/reference/attributes/) lists accepted arguments and targets. For custom metadata, see [extending Sidex](/sidex/docs/guides/extending-sidex/).
