---
sidebar_position: 3
---

import CodeBlock from "@theme/CodeBlock"

# Data Types

In order to use Sidex effectively, you need to understand it's type system. But don't worry, Sidex's type system is simple!

Data types constrain the form data may take and Sidex's type system knows four _sorts_ of data types:

<dl>
  <dt>
    <strong>Opaque Types</strong>
  </dt>
  <dd>
    An <em>opaque type</em> does not have any structure known to Sidex's type system – it is opaque. Opaque types are the primitives of the type system. Builtin types such as <code className="builtin-type">string</code> or <code className="builtin-type">i32</code> are opaque types. Sidex allows you to define your own opaque types thereby extending the type system with primitives. This makes Sidex highly flexible and extensible.
  </dd>
  <dt>
    <strong>Record Types</strong>
  </dt>
  <dd>
    A <em>record type</em> defines structures, called <a href="https://en.wikipedia.org/wiki/Record_(computer_science)" target="_blank"><em>records</em></a>, with <em>named fields</em> of different types. Record types are like C's or Rust's <code>struct</code> types. Type theoretically, they are nominal product types.
  </dd>
  <dt>
    <strong>Variant Types</strong>
  </dt>
  <dd>
    A <em>variant type</em> defines a <a href="https://en.wikipedia.org/wiki/Tagged_union" target="_blank"><em>tagged union</em></a> over <em>variants</em> of different types. Record types are like Rust's <code>enum</code> types or Haskell's <code>datatype</code> types. Type theoretically, they are nominal sum types.
  </dd>
  <dt>
    <strong>Wrapper Types</strong>
  </dt>
  <dd>
    A <em>wrapper type</em> defines a new nominal type wrapping another type. The are similar to Haskell's <a href="https://wiki.haskell.org/Newtype" target="_blank"><code>newtype</code></a>.
  </dd>
</dl>

While opaque types constitute the primitives of the type system. The three other sorts of types, record types, variant types, and wrapper types, are used to define new types by combining already existing types.

By convention, type names are required to be `CamelCase` (except for builtin types).

## Four Sorts of Types

Let us now have a closer look at each sort of type.

### Opaque Types

Opaque types are defined with the keyword <code className="keyword">opaque</code>. For example:

```sidex
opaque Uuid
```

This defines a new opaque type <code className="type-name">Uuid</code>. Data of that type should have a specific form, e.g., be a [_Universally Unique Identifier_](https://de.wikipedia.org/wiki/Universally_Unique_Identifier) (UUID). It is important to note, however, that Sidex's type system is not aware of this form and treats the type as a black box.

The representation of data of an opaque type is entirely defined by a format or language mapping. For instance, for a binary format, a UUID may be represented by 16 bytes while in JSON it may be represented as a string. For example:

```json
"9a4654f0-8fb7-40f3-975f-a230b063b75b"
```

Opaque types are also used to define the builtin types of Sidex, e.g., the different integers or <code className="keyword">string</code>. Opaque types set Sidex apart from other data modeling frameworks because opaque types enable you to define your own primitives.

Opaque types become really useful with attributes defining their representation in various formats and languages:

```sidex
#[json(type = "string")]
#[rust(type = "::uuid::Uuid")]
opaque Uuid
```

The attributes `#[json(type = "string")]` and `#[rust(type = "::uuid::Uuid")]` tell code generators that data of type <code className="type-name">Uuid</code> shall be represented as a string in JSON and as [`::uuid::Uuid`](https://docs.rs/uuid/latest/uuid/struct.Uuid.html) in Rust. As a result, they may generate:

```rust title="schema.rs"
type Uuid = ::uuid::Uuid
```

```typescript title="schema.ts"
type Uuid = string
```

### Record Types

Record types are defined with the keyword <code className="keyword">record</code>. For example:

```sidex
record Task {
  id: Uuid,
  description: string,
  completed: bool,
}
```

This defines a new record type <code className="type-name">Task</code>. Data of this type should comprise a unique task identifier `id` which must be of the earlier defined type <code className="type-name">Uuid</code>, a _description_ which must be of type <code className="keyword">string</code>, and a flag `completed` which must be of type <code className="keyword">bool</code>.

In JSON data of type <code className="type-name">Task</code> may then be represented as follows:

```json
{
  "id": "9a4654f0-8fb7-40f3-975f-a230b063b75b",
  "description": "Learn more about Sidex's type system.",
  "completed": false
}
```

Again, how records are represented depends on the format and language mapping, however, in contrast to opaque types you do not need to explicitly tell code generators how to represent data of the type. They already know how to translate record types to the specific format or language. As a result, they may generate:

```rust title="schema.rs"
struct Task {
  id: Uuid,
  description: String,
  completed: bool,
}
```

```typescript title="schema.ts"
type Task = {
  id: Uuid
  description: string
  completed: boolean
}
```

Fields in record types can be optional which is indicated by a `?` after the name of the field.

For instance, to make the <code>description</code> field of the <code className="type-name">Task</code> type optional:

```sidex
record Task {
  id: Uuid,
  description?: string,
  completed: bool,
}
```

By convention, fields are required to be `snake_case`.

### Variant Types

Variant types are defined with the keyword <code className="keyword">variant</code>. For example:

```sidex
variant Progress {
  Pending: string
  Completed
}
```

This defines a new variant type <code className="type-name">Progress</code> with two variants, <code className="type-name">Pending</code> and <code className="type-name">Completed</code>. Data of this type should either be an instance of <code className="type-name">Pending</code> with an associated <code className="keyword">string</code>, e.g., to represent a message, or an instance of <code className="type-name">Completed</code> without any associated additional data. In contrast to Rust's <code>enum</code> types, there can be at most one associated type for each variant.

The [JSON encoding](../formats/json) of variant types is configurable with attributes. A possible encoding may be:

```json
{
  "status": "Pending",
  "message": "Work in progress!"
}
```

By convention, variant names are required to be `CamelCase`.

### Wrapper Types

Wrapper types are useful to define fresh types carrying additional invariants. For instance, not every <code className="type-name">Uuid</code> is also an id of a task. However, every id of a task is an <code className="type-name">Uuid</code>. This can be captured with a wrapper type:

```sidex
wrapper TaskId: Uuid
```

This defines a new type <code className="type-name">TaskId</code> which wraps a <code className="type-name">Uuid</code> but is still a separate type.

## Types Aliases

_Type aliases_ are defined with the <code className="keyword">alias</code> keyword. In contrast to wrapper types, they do not introduce a fresh nominal type. All they do is to bind an already existing type definition to another alternative name.

## Generic Types

In Sidex, all sorts of types can be _generic_. A generic types has _type parameters_ which can be instantiated with other types.

Imagine you want a <code className="type-name">Future</code> type representing an ongoing computation which eventually yields a value of some type <code className="type-name">T</code> where <code className="type-name">T</code> is not fixed. With generics such a type may be defined as follows:

```sidex
variant Future<T> {
  Pending: string,
  Completed: T,
}
```

Here <code className="type-name">T</code> is a type parameter. This <code className="type-name">Future</code> may then be used as follows:

```sidex
record Task {
  name: string,
  future: Future<u64>,
}
```

Here the `future` field is a <code className="type-name">Future</code> which will eventually resolve to an <code className="builtin-type">u64</code> integer.

## Builtin Types

Sidex comes with some _builtin types_ which are just opaque types defined in an internal schema which is implicitly imported.

The builtin types are:

- <code className="builtin-type">string</code>: A sequence of Unicode code points.
- <code className="builtin-type">bytes</code>: A sequence of bytes.
- <code className="builtin-type">i8</code>, <code className="builtin-type">i16</code>, <code className="builtin-type">i32</code>, <code className="builtin-type">i64</code>: A signed integers of a certain bit width.
- <code className="builtin-type">u8</code>, <code className="builtin-type">u16</code>, <code className="builtin-type">u32</code>, <code className="builtin-type">u64</code>: An unsigned integers of a certain bit width.
- <code className="builtin-type">idx</code>: An index into a sequence.
- <code className="builtin-type">f32</code>, <code className="builtin-type">f64</code>: An IEEE-754 floating point number.
- <code className="builtin-type">bool</code>: A boolean.
- <code className="builtin-type">unit</code>: Indicates the absence of specific data.
- <code className="type-name">Sequence&lt;T&gt;</code>: A sequence of values of type <code>T</code>.
- <code className="type-name">Map&lt;K, V&gt;</code>: A map from <em>keys</em> of type <code>K</code> to values of type <code>V</code>.

In every schema, there is an implicit import:

```sidex
import ::std::builtins::*
```

### Syntactic Sugar

Most data modeling frameworks treat sequences or maps as special cases. In Sidex, there is syntactic sugar for them, however, they are defined using generic opaque types. For notational convenience, the following syntactic sugar is supported:

```sidex
() ⇒ ::std::builtins::unit

[T] ⇒ ::std::builtins::Sequence<T>

[K: V] ⇒ ::std::builtins::Map<K, V>
```

Hence, you can use `[T]` and `[K: V]` for sequence and map types respectively.

### Reference

The builtin types are just predefined as opaque types in the following schema:

```sidex title="builtins.sidex" showLineNumbers
//! Sidex builtin types.
//!
//! This schema is used to populate the builtin types.

/// A sequence of Unicode code points.
opaque string

/// A sequence of bytes.
opaque bytes

/// An 8-bit signed integer
opaque i8

/// A 16-bit signed integer.
opaque i16

/// A 32-bit signed integer.
opaque i32

/// A 64-bit signed integer.
opaque i64

/// An 8-bit unsigned integer.
opaque u8

/// A 16-bit unsigned integer.
opaque u16

/// A 32-bit unsigned integer.
opaque u32

/// A 64-bit unsigned integer.
opaque u64

/// Type used for indexing sequences.
///
/// This is an unsigned integer of target-specific bit width.
opaque idx

/// A 32-bit floating-point number.
opaque f32

/// A 64-bit floating-point number.
opaque f64

/// A boolean.
opaque bool

/// Unit type indicating the absence of any specific data.
opaque unit

/// A sequence type.
opaque Sequence<T>

/// A map type.
opaque Map<K, V>
```
