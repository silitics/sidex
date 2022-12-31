---
sidebar_position: 4
---

# Type Classes

_Type classes_ enable a loose aggregation of types with common properties or interfaces.
For instance, the <code class="type-name">Equatable</code> type class is _instantiated_ by types the data of which can be compared for equality.
Note that type classes are instantiated by types, not by data.

```sidex
class PartiallyEquatable<T>

class Equatable<T> implies PartiallyEquatable<T>
```

```sidex
class Hashable<T>
```

For opaque types:

```sidex
opaque i32 is Equatable + Hashable
```

Will result in instances:

```sidex
instance Equatable<i32>
instance Hashable<i32>
```

Only works if the type classes do not contain any members and have only one type parameter.

What do we do with wrapper, record, and variant types?

## Associated Types

Type classes can have _associated types_:

```sidex
class Mutable<T> {
    type Mutation
}
```
