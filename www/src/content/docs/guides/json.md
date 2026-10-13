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

An absent optional field is omitted. Its underlying type does not automatically include `null`. Readers treat `null` for an optional field as absent, unless `null` is a value of the field's type. Optional `value` fields always treat `null` as absent, because many languages cannot tell an absent value from `null`. Use the generated JSON Schema to define the shape you require at a boundary.

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

Implicit tagging omits the tag. A payload is encoded directly and a case without a payload becomes its name. Readers pick the first case in declaration order that accepts the value. This can make cases ambiguous. Prefer explicit tags when multiple cases could accept the same JSON shape. For instance, readers accept 64-bit integers as decimal strings, so an `i64` case declared before a `string` case also accepts `"3"`.

## Numbers and Bytes

Integers are JSON numbers. JavaScript represents integers exactly only up to a magnitude of 2^53 − 1, so `i64` and `u64` values beyond are decimal strings, such as `"9007199254740993"`. Readers also accept decimal strings for smaller 64-bit integers.

`f32` and `f64` are JSON numbers. JSON has no non-finite numbers, so these are the strings `"NaN"`, `"Infinity"`, and `"-Infinity"`.

`bytes` are Base64 strings.

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
