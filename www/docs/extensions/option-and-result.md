---
sidebar_position: 1
---

# Option and Result

```sidex title="option.sidex"
/// An optional value of type `T`.
variant Option<T> {
    /// No value.
    None,
    /// Some value of type `T`.
    Some: T
}
```

```sidex title="result.sidex"
/// A result.
variant Result<T, E> {
    Ok: T,
    Err: E
}
```

To use <code className="type-name">Option</code> and <code className="type-name">Result</code> in your schemas, you have to explicitly import them:

```sidex
import ::sidex::{option::Option, result::Result}
```
