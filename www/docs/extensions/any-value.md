---
sidebar_position: 2
---

# Dynamic Typing

This extension defines an opaque type <code className="type-name">Any</code> for representing values of unknown type:

```sidex title="any.sidex"
/// A value of unknown type.
opaque AnyData

opaque TypeId

record Any {
    type_id: TypeId,
    data: AnyData,
}
```

Imagine you like to define an [RPC protocol](./rpc-api) with Sidex where arguments and return values of remote procedure calls can, from the perspective of the protocol, be _any_ value. This is where <code className="type-name">Any</code> is useful.

To use <code className="type-name">AnyV</code> in your schemas, you have to explicitly import it:

```sidex
import ::std::any::Any
```

As with any extension, backends and format mappings may or may not support <code className="type-name">Any</code>.

For JSON and the Rust backend, the following attributes on <code className="type-name">Any</code> are implicit:

```sidex
#[json(type = "any")]
#[rust(type = "::sidex::any::Any")]
```

As a result, it also works with the TypeScript backend out-of-the-box.
