---
title: JSON Mapping
description: Choose a wire representation that each application can read and write.
order: 0
---

A _wire representation_ is the JSON shape applications exchange. Sidex's type structure does not depend on JSON, but its generators use `json` attributes to agree on field names, variant tags, and opaque representations.

## Built-in Types

| Type                      | JSON                                    |
| ------------------------- | --------------------------------------- |
| `bool`                    | boolean                                 |
| `i8`, `i16`, `i32`        | number                                  |
| `u8`, `u16`, `u32`, `idx` | number                                  |
| `i64`, `u64`              | number, or string beyond ±(2^53 − 1)    |
| `f32`, `f64`              | number, or string if not finite         |
| `string`                  | string                                  |
| `bytes`                   | Base64 string with padding              |
| `uuid`                    | lowercase hyphenated string             |
| `timestamp`               | RFC 3339 string in UTC                  |
| `date`                    | `YYYY-MM-DD` string                     |
| `json`                    | the value itself                        |
| `[T]`                     | array                                   |
| `[K: V]`                  | object with keys in text representation |

For example:

```sidex
record Release {
    downloads: u32,
    size: u64,
    checksum: u64,
    ratio: f64,
    signature: bytes,
    id: uuid,
    published_at: timestamp,
    released_on: date,
}
```

```json
{
  "downloads": 42,
  "size": 1791847993,
  "checksum": "18446744073709551615",
  "ratio": 0.5,
  "signature": "AAH/",
  "id": "67e55044-10b1-426f-9247-bb680e5fe0c8",
  "publishedAt": "2026-10-12T08:30:00.25Z",
  "releasedOn": "2026-10-12"
}
```

Readers also accept:

- decimal strings for all `i64` and `u64` values
- `"Infinity"` for floats
- Base64 without padding
- uppercase UUIDs
- timestamps with any offset

Readers reject other JSON types and values out of range. For instance, `1` is not a `bool`, `true` is not an integer, `"1"` is not a `u32`, and `300` is not a `u8`.

JavaScript represents integers exactly only up to a magnitude of 2^53 − 1 (`Number.MAX_SAFE_INTEGER`). Larger `i64` and `u64` values are therefore decimal strings, such as `"9007199254740993"`, so they stay exact in every language. Typical values, such as sizes, counts, and timestamps in seconds, remain plain numbers.

JSON has no non-finite numbers, so non-finite floats are the strings `"NaN"`, `"+Infinity"`, and `"-Infinity"`.

Timestamps retain up to nine fractional-second digits. Writers normalize them to UTC and remove trailing fractional zeroes. Python values read from JSON retain nanosecond precision; native Python `datetime` values have microsecond precision.

A `json` value is written as is. Its integers remain numbers beyond 2^53 − 1, so data from other systems passes through unchanged, but JavaScript may lose their precision. A `json` value cannot hold non-finite floats.

Map keys may be strings, integers, UUIDs, timestamps, and dates. A `[u64: string]` map is an object such as `{"42": "answer"}`.

Records, variants, wrappers, and opaque types follow the rules below.

## JSON Schema

The Rust runtime and the JSON Schema generator share built-in schemas for JSON Schema 2020-12. Numeric format hints (`int32`, `uint32`, `int64`, and `uint64`) help OpenAPI client generators select types. UUIDs, timestamps, and dates use the `uuid`, `date-time`, and `date` formats. Bytes carry `contentEncoding: "base64"`.

Schemas retain numeric bounds for 8-, 16-, and 32-bit integers and a minimum of zero for `u64`. The `idx` schema has a minimum of zero and no maximum, independent of the generating host. Simple patterns describe decimal strings for 64-bit integers and integer map keys, and the non-finite float strings. These patterns check spelling; Sidex readers enforce integer ranges. The schemas describe this wire mapping. Enable format validation when using a JSON Schema validator.

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

An absent optional field is omitted. Its underlying type does not automatically include `null`. Rust and Python readers treat `null` for an optional field as absent, also if `null` is a value of the field's type, because many languages cannot tell an absent value from `null`. Generated TypeScript types define the canonical data model: `is` and `check` reject a present `null` unless the field's type admits it. Unknown record fields are accepted so older readers can consume data with added fields. Use JSON Schema when a boundary requires a different policy.

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
