---
sidebar_position: 1
---

# JSON

[JSON](https://en.wikipedia.org/wiki/JSON) (_JavaScript Object Notion_) is a popular text-based format for representing data – especially on the web.

## JSON Types

JSON itself specifies six different types of data:

- `number`: A decimal number with an optional fractional part.
- `boolean`: A boolean being either `true` or `false`.
- `string`: A sequence of Unicode characters.
- `null`: A special `null` value.
- `object`: A collection of name-value pairs where values can be any type and names must be of type `string`.
- `array`: A variable-length sequence of values of any type.

When working with JSON, there are some subtile limitations when it comes to representing numbers. First, the text format itself allows representing arbitrary precision numbers but, second, neither IEEE-754 `NaN` nor positive and negative infinities. Most implementations do not support arbitrary precision numbers and quietly truncate them to fit into a certain native number type.

## Type Definitions

Let's first specify how type definitions are mapped. Note that wrapper types are represented exactly as the wrapped type.

### Opaque Types

Recall that for opaque types an explicit mapping has to be specified.

To this end, opaque types can be annotated with a JSON `type` attribute:

```sidex
#[json(type = "<JSON-TYPE-EXPR>")]
```

Here, `<JSON-TYPE-EXPRESSION>` is a `|` separated list of JSON types or `any` denoting an unknown JSON type.

For example, we may define a <code className="type-name">Value</code> type which is either a JSON string or number:

```sidex
#[json(type = "string | number")]
opaque Value
```

If no type is specified, the type is implicitly `any`.

For interoperability with JSON schema, a path to a JSON schema can be specified using the `schema` attribute:

```sidex
#[json(schema = "<PATH>")]
```

Here, `<PATH>` must be a filesystem path relative to the directory of the bundle the opaque type is defined in.

Note that none of the official code generators support JSON schemata.

The [TypeScript code generator](../languages/typescript) uses the `type` attribute for generating type definitions.

The mapping of the builtin opaque types is described bellow.

### Record Types

Records are mapped to JSON objects using field names as names in the object.

Optional fields are present in the JSON object, only if they do have a value.

The mapping can be customized with the following attributes:

```sidex
#[json(rename_all = "…")]
```

Renames all fields according to a renaming schema specified by `…`. Possible renaming schemas are `none`, `lowercase`, `uppercase`, `PascalCase`, `camelCase`, `snake_case`, `SCREAMING_SNAKE_CASE`, `kebab-case`, `SCREAMING-KEBAB-CASE`.

Unless `rename_all = "none"` fields are renamed to `camelCase` by default (usual JavaScript convention).

Individual fields can be renamed with the `name` attribute:

```sidex
#[json(name = "…")]
```

Here, `…` is not a renaming schema but the actual name of the field.

### Variant Types

Variant types can be represented in JSON in four different ways.

In the following, `...` is the data associated with a variant.

- The _externally tagged_ representation will result in

  ```json
  { "VariantName": ... }
  ```

  and

  ```json
  "VariantName"
  ```

  in case there is no associated data.

- The _adjacently tagged_ representation will result in

  ```json
  { "tag": "VariantName", "content": ... }
  ```

  where the `content` field is not present if there is no associated data.

- The _internally tagged_ representation will result in

  ```json
  { "tag": "VariantName", ... }
  ```

  where `...` does not extend the object if there is no associated data.

Internally tagged representations are limited to variants where the associated type is a record type without a field with the same name as the tag (here `tag`) enabling the embedding into the JSON object. When choosing the internally tagged representation, variants with associated types which are not record types are encoded as adjacently tagged. The same is true for variants where the associated type is a type variable.

The kind of representation can be configured with the following attributes:

```sidex
#[json(tagged=adjacently)]
#[json(tagged=externally)]
#[json(tagged=internally)]
```

The internally tagged representation is the default.

The name of the tag field for the adjacently and internally tagged representations can be configured the `tag` attribute:

```sidex
#[json(tag = "…")]
```

The name of the value field for the adjacently tagged representation can be configured with the `value` attribute:

```sidex
#[json(value = "xyz")]
```

The value attribute can be used both on the variant type itself and on individual variants.

All variants can be renamed using a renaming scheme (see record types) with the `rename_all` attribute:

```sidex
#[json(rename_all = "…")]
```

Individual variants can be renamed with the `name` attribute on the specific variant:

```sidex
#[json(name = "…")]
```

## Builtin Types

### String Type

The Sidex type <code className="builtin-type">string</code> is mapped to the JSON `string` type – no surprises here.

### Bytes Type

The Sidex type <code className="builtin-type">bytes</code> is mapped to the JSON `string` type using the [Base64 encoding](https://datatracker.ietf.org/doc/html/rfc4648).

This representation of bytes reduces the size of the JSON representation and can be easily processed.

### Integer Types

Note that the JSON standard (ECMA-404) says nothing about various bit widths of integers.

In practice, many implementations (and in particular those in web browsers) can represent integers up to 32-bit but not 64-bit.

Hence, integer types up to 32-bit are directly mapped to the JSON `number` type.

The 64-bit integer types are mapped to `string` using the usual decimal encoding.

This representation prevents precision losses when round-tripping through implementations supporting only 32-bit integers and also avoids surprises when dealing with 64-bit integers in a web browser.[^1]

### Floating Point Types

Sidex supports IEEE-754 floating point numbers.

The JSON `number` type can neither be used to represent `NaN` nor the infinities.

Floating point numbers are encoded as `number` or `string` depending on their value:

```json
number | "+Infinity" | "-Infinity" | "NaN"
```

### Bool Type

The Sidex type <code className="builtin-type">bool</code> is mapped to the JSON `boolean` type – no surprises here.

### Unit Type

The Sidex type <code className="builtin-type">unit</code> is mapped to the JSON `null` type.

### Sequence Types

Sequence types are mapped to JSON arrays.

### Map Types

Note that the JSON `object` type can only be used to represent maps where the keys are strings, however, Sidex map types do not have this limitation. To be able to represent any map, we define two different representations:

- `entries`: A map is represented as an `array` of key-value pairs `[key, value]` (an `array` with two elements).
- `object`: A map is represented by an `object` mapping keys to values.

Which representation is chosen is decided based on the key type of the map type. If the key type itself is represented as `string`, then the `object` representation is chosen. Otherwise and if the key type is a type variable, then the `entries` representation is chosen.

[^1]: At Silitics, we already encountered bugs with the representation of numbers in JSON. Our goal is that going through JSON is lossless and does not come with surprises for any of the corner cases regarding numbers.
