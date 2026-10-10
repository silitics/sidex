---
title: JSON Mapping
description: Choose a wire representation that each application can read and write.
order: 0
---

A _wire representation_ is the JSON shape applications exchange. Sidex's type structure does not depend on JSON, but its generators use `json` attributes to agree on field names, variant tags, and opaque representations.

## Records and Optional Fields

Record fields default to camelCase:

```sidex
record User {
    display_name: string,
    nickname?: string,
}
```

```json
{ "displayName": "Ada" }
```

An absent optional field is omitted. Its underlying type does not automatically include `null`. Some language runtimes accept additional input forms, so use the generated JSON Schema to define the shape you require at a boundary.

Change names at the record or field level:

```sidex
#[json(rename_all = "kebab-case")]
record Settings {
    update_interval: u32,
    #[json(name = "label")]
    display_name: string,
}
```

```json
{ "update-interval": 60, "label": "Workshop" }
```

`name` sets an explicit name. `rename` applies a naming convention to one field. `rename_all` changes the default for a record's fields or a variant's cases. Supported naming conventions are listed in the [attribute reference](/sidex/docs/reference/attributes/#json).

## Variant Tagging

A _tag_ identifies a variant case. Sidex supports internal, adjacent, external, and implicit tagging.

Consider:

```sidex
record Point {
    x: i32,
    y: i32,
}

variant Position {
    Unknown,
    Named: string,
    Coordinates: Point,
}
```

Internal tagging is the default, using `tag` as the tag key. Case names retain their spelling.

| Case            | Default JSON                        |
| --------------- | ----------------------------------- |
| `Unknown`       | `{"tag":"Unknown"}`                 |
| `Named: "home"` | `{"tag":"Named","content":"home"}`  |
| `Coordinates`   | `{"tag":"Coordinates","x":1,"y":2}` |

Internal tagging merges a record payload's fields into the outer object. Nonrecord payloads use a content field.

For a consistent payload field, choose adjacent tagging:

```sidex
#[json(tagged = adjacently, tag = "kind", content = "value")]
variant Position {
    Unknown,
    Named: string,
    Coordinates: Point,
}
```

```json
{ "kind": "Coordinates", "value": { "x": 1, "y": 2 } }
```

The same payload under external tagging is `{"Coordinates":{"x":1,"y":2}}`; a case without a payload is just `"Unknown"`.

Implicit tagging omits the tag. A payload is encoded directly and a case without a payload becomes its name. This can make cases ambiguous. Prefer explicit tags when multiple cases could accept the same JSON shape.

## Numbers and Bytes

JSON numbers do not encode integer width. JavaScript numbers cannot exactly represent all 64-bit integers. Sidex's TypeScript runtime uses nominal numeric types, including a string representation for `I64` and a string-or-number representation for `U64`. Rust and Python have different native numeric representations.

Choose and test a representation across all participating applications. A TypeScript type mapping changes the static type only; it does not make Rust or Python serialize integers as strings.

The JSON sample generator exposes `--integers-as-strings` and `--floats-as-strings` for testing alternate representations. These flags affect samples; they do not reconfigure your application serializers.

`bytes` are represented as Base64 strings in the JSON contract. Nonfinite floating-point values need an explicitly agreed representation because JSON cannot represent them as numbers.

## Wrappers and Opaque Values

Wrappers retain their inner JSON representation. A `wrapper UserId: string` is a JSON string even though target languages treat it as a distinct type.

An opaque type can declare its JSON shape:

```sidex
#[json(type = "string|null")]
opaque NullableName
```

Accepted JSON shape names are `number`, `boolean`, `string`, `null`, `object`, `array`, and `any`. Join multiple shapes with `|`.

## Keep the Contract Consistent

Renaming a wire field or case, changing tagging, or changing a payload shape affects readers and writers. Regenerate each application's code after a model change and test representative JSON values in every language you use.

[Schema evolution](/sidex/docs/guides/api-lifecycle/#evolve-a-shared-contract) describes compatibility considerations. [Working with schemas](/sidex/docs/guides/working-with-schemas/) shows CLI checks and sample generation for this workflow.
