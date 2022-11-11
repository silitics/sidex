---
sidebar_position: 2
---

# TypeScript

The goal of this backend is not only to generate TypeScript type declarations compatible with how we represent data in [JSON](../formats/json) but also to provide a fully type-safe and sound approach to serialization and interaction with other languages.

Let us start with a simple observation. Consider the Sidex type <code className="builtin-type">i8</code>. How do we map this type to TypeScript? A naive approach would simply map <code className="builtin-type">i8</code> to the TypeScript `number` type, however, while `3.1415` is a `number` it is certainly not <code className="builtin-type">i8</code>. Hence, if we were to map <code className="builtin-type">i8</code> to `number`, we could, for example, pass `3.1415` to a method which expects <code className="builtin-type">i8</code> without TypeScript complaining — but, this is not type-safe and also not what we want. We want a mapping where we can use <code className="builtin-type">i8</code> as a `number` (e.g., when it is returned by a method) while not being able to use just any `number` as <code className="builtin-type">i8</code>. The language mapping provided here, enables precisely that.

More general, the language mapping is such that for any Sidex type `T` and JSON representation `"..."` of data of type `T`

```js
JSON.parse("...")
```

has type `ts(T)`, i.e., the TypeScript language mapping of type `T`. Furthermore, for any data `...` of type `ts(T)`

```js
JSON.stringify(...)
```

yields a JSON representation of type `T`. This ensures that we can use the standard functions for JSON.

## Runtime Library

TypeScript's type system is structural, while Sidex's type system is nominal.

A central part of the runtime library is the `Nominal` TS type definition:

```typescript
declare const SIDEX_PATH_SYMBOL: unique symbol

/**
 * A nominal type based on `T` and identified by it's Sidex path `P`.
 */
export type Nominal<T, P extends string> = T & { [SIDEX_PATH_SYMBOL]: P }
```

With that, we define types:

```typescript
type I8 = Nominal<number, "i8">
```

Any value of type `I8` is a `number`, however, not any `number` is also of type `i8`.

We can use `I8` in computations and it will just behave like `number`.

## Type Mapping

The `type` attribute can be used on any type definition to explicitly define a TS type:

```sidex
#[typescript(type = "<TYPE-EXPR>")]
```

Whether a type is nominal or not can be controlled with the following attributes:

```sidex
#[typescript(nominal)]
#[typescript(non_nominal)]
```

Opaque types are automatically made nominal if they have no `type = ...` attribute.

### JSON Types

If no `type` attribute is present, the JSON type attribute is used.

For instance,

```sidex
#[json(type = "string")]
opaque Uuid
```

will result in:

```typescript
type Uuid = Nominal<string, "::bundle_name::schema_name::Uuid">
```

Helper functions for converting between types will also be generated, e.g.:

```typescript
function asUuid(x: string): Uuid {
  return x as Uuid
}
```

### Services

**🚧 TODO:** Think about how services will be translated.

```typescript
interface Api {
  (arg1: I8): Promise<string>
}
```

## Supported Extensions

### Extension: Diff and Path

The TypeScript backend has support for the _Diff and Patch_ extension. It supports applying patches to objects and generating patches with the help of [Immer](https://immerjs.github.io/immer/).
