---
title: Types
description: Express structured data with records, variants, wrappers, aliases, and opaque types.
order: 0
---

A Sidex type describes a set of values. Fields are required unless explicitly marked optional, and types do not include a null value implicitly.

Sidex has five definition forms. Definitions and variant cases conventionally use `PascalCase`; record fields use `snake_case`.

## Records

A record groups named fields:

```sidex
record User {
    display_name: string,
    active: bool,
    nickname?: string,
}
```

A value of `User` must contain `display_name` and `active`. `nickname` may be absent. An optional field is independent of a type that explicitly represents a nullable or optional value.

## Variants

A variant is a choice between named cases. Each case has either no payload or one payload type:

```sidex
record Position {
    latitude: f64,
    longitude: f64,
}

variant Location {
    Unknown,
    Named: string,
    Coordinates: Position,
}
```

Each `Location` value is exactly one of these cases. In Rust this becomes an enum; in TypeScript it becomes a union matching the chosen [JSON tagging](/sidex/docs/guides/json/#variant-tagging).

Use variants for states and outcomes instead of strings whose permitted values are only documented informally.

## Wrappers and Aliases

A wrapper gives an existing representation a distinct identity:

```sidex
wrapper UserId: string
wrapper ProjectId: string
```

A `UserId` and a `ProjectId` both contain a string but are separate types. Rust emits newtypes. TypeScript uses nominal type brands. Python emits Pydantic root models.

An alias gives a type another name without creating a new identity:

```sidex
alias Labels: [string: string]
alias UserIds: [UserId]
```

Use a wrapper when exchanging two values of the same underlying type would be a mistake. Use an alias to name a repeated type expression.

## Opaque Types

An opaque type delegates its representation to generators. It is useful for a type already implemented by a library:

```sidex
#[json(type = "string")]
#[rust(type = "::uuid::Uuid")]
#[py(typ = "uuid.UUID")]
opaque UserUuid
```

The JSON contract is a string. Rust and Python use their native UUID implementations; your application must provide the Rust dependency. TypeScript derives the representation from `json(type = ...)`.

Opaque types can also represent free-form JSON:

```sidex
#[json(type = "any")]
opaque JsonValue
```

See [opaque lowering](/sidex/docs/reference/manifest/#opaque-lowering) for generators that need raw JSON values instead of native mappings.

## Built-in Types

| Type                      | Meaning                                  |
| ------------------------- | ---------------------------------------- |
| `bool`                    | Boolean                                  |
| `string`                  | Unicode string                           |
| `bytes`                   | Sequence of bytes                        |
| `i8`, `i16`, `i32`, `i64` | Signed integers of the indicated width   |
| `u8`, `u16`, `u32`, `u64` | Unsigned integers of the indicated width |
| `idx`                     | Index type, mapped to `usize` in Rust    |
| `f32`, `f64`              | Floating-point numbers                   |
| `unit`                    | A value carrying no data                 |

`[T]` is a sequence of values of type `T`. `[K: V]` is a map from `K` to `V`:

```sidex
record Inventory {
    names: [string],
    quantities: [string: u32],
}
```

For interoperable JSON objects, use string keys. The [JSON mapping](/sidex/docs/guides/json/#built-in-types) lists the representation of each built-in type.

## Generics

Generic definitions take type parameters:

```sidex
record Page<T> {
    items: [T],
    next_cursor?: string,
}

alias UserPage: Page<User>
```

Here `Page<User>` substitutes `User` for `T`. JSON Schema cannot represent an unbound type parameter, so use a nongeneric definition or alias when you need a concrete schema document.

## Recursive Types

Types can refer to themselves:

```sidex
record Tree {
    label: string,
    children: [Tree],
}
```

A sequence supplies indirection in Rust. For a recursive record field stored directly, add a Rust wrapper such as `box`:

```sidex
record Node {
    value: string,
    #[rust(box)]
    next?: Node,
}
```

This makes the Rust type finite in size without changing its JSON representation.

## Standard Option and Result

The standard bundle provides explicit optional values and results:

```sidex
import ::std::option::Option
import ::std::result::Result

alias OptionalName: Option<string>
alias LookupResult: Result<string, string>
```

These are variants with `None`/`Some` and `Ok`/`Err` cases. They differ from an optional record field: the field's presence and the value stored in it are separate choices. When defining a domain-specific result, a custom variant often communicates more than `Result<T, E>`.
