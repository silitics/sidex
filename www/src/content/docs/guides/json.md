---
title: JSON Mapping
description: Choose a wire representation that each application can read and write.
order: 0
---

A _wire representation_ is the JSON shape applications exchange. Sidex's type structure does not depend on JSON, but its generators use `json` attributes to agree on field names, variant tags, and opaque representations.

## Built-in Types

Writers produce the canonical representation. Readers also accept the listed alternatives.

| Type                      | Written as                                                                    | Also read       |
| ------------------------- | ----------------------------------------------------------------------------- | --------------- |
| `bool`                    | `true` or `false`                                                             |                 |
| `i8`, `i16`, `i32`        | number                                                                        |                 |
| `u8`, `u16`, `u32`, `idx` | number                                                                        |                 |
| `i64`, `u64`              | number, or decimal string beyond ±(2^53 − 1)                                  | decimal string  |
| `f32`, `f64`              | number, or `"NaN"`, `"Infinity"`, and `"-Infinity"`                           | `"+Infinity"`   |
| `string`                  | string                                                                        |                 |
| `bytes`                   | Base64 string with padding                                                    | missing padding |
| `uuid`                    | lowercase hyphenated string, such as `"67e55044-10b1-426f-9247-bb680e5fe0c8"` | uppercase       |
| `timestamp`               | RFC 3339 string in UTC, such as `"2026-10-12T08:30:00.25Z"`                   | any offset      |
| `date`                    | `YYYY-MM-DD` string, such as `"2026-10-12"`                                   |                 |
| `json`                    | the JSON value itself                                                         |                 |
| `[T]`                     | array                                                                         |                 |
| `[K: V]`                  | object with keys in their text representation                                 |                 |

Readers reject other JSON types and values out of range. For instance, `1` is not a `bool`, `true` is not an integer, `"1"` is not a `u32`, and `300` is not a `u8`.

JavaScript represents integers exactly only up to a magnitude of 2^53 − 1 (`Number.MAX_SAFE_INTEGER`). Larger `i64` and `u64` values are therefore decimal strings, such as `"9007199254740993"`, so they stay exact in every language. Typical values, such as sizes, counts, and timestamps in seconds, remain plain numbers.

JSON has no non-finite numbers, so non-finite floats are strings.

A `json` value is written as is. Its integers remain numbers beyond 2^53 − 1, so data from other systems passes through unchanged, but JavaScript may lose their precision. A `json` value cannot hold non-finite floats.

Map keys may be strings, integers, UUIDs, timestamps, and dates. A `[u64: string]` map is an object such as `{"42": "answer"}`.

Records, variants, wrappers, and opaque types follow the rules below.

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

An absent optional field is omitted. Its underlying type does not automatically include `null`. Readers treat `null` for an optional field as absent, unless `null` is a value of the field's type. Optional `json` fields always treat `null` as absent, because many languages cannot tell an absent value from `null`. Use the generated JSON Schema to define the shape you require at a boundary.

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

## Wrappers and Opaque Values

Wrappers retain their inner JSON representation. A `wrapper UserId: string` is a JSON string even though target languages treat it as a distinct type.

An opaque type can declare its JSON shape:

```sidex
#[json(type = "string|null")]
opaque NullableName
```

Accepted JSON shape names are `number`, `boolean`, `string`, `null`, `object`, `array`, and `any`. Join multiple shapes with `|`.

## TOML

TOML documents, such as configuration files, use the same representation. TOML has no `null` and only signed 64-bit integers, so writers adapt three cases:

- Map entries whose value is `null` are omitted, like absent optional fields.
- `null` in arrays cannot be written, so writing fails.
- Integers beyond the 64-bit range of TOML are decimal strings.

Readers also accept TOML date-times for timestamps.

## Keep the Contract Consistent

Renaming a wire field or case, changing tagging, or changing a payload shape affects readers and writers. Regenerate each application's code after a model change and test representative JSON values in every language you use.

[Schema evolution](/sidex/docs/guides/api-lifecycle/#evolve-a-shared-contract) describes compatibility considerations. [Working with schemas](/sidex/docs/guides/working-with-schemas/) shows CLI checks and sample generation for this workflow.
