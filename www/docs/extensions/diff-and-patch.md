---
sidebar_position: 4
---

# Diff and Patch (Draft)

The _diff and patch_ extension can be used to generate code for diffing and patching data.

Among other things, this extension is useful for version control by storing diffs and for exchanging incremental updates.

As a typical use case, a client may send modifications for an entity in the form of patches to a server.

The structure of patches is itself defined by a Sidex schema:

```sidex title="patch.sidex"
import ::sidex::any::AnyValue

/// A patch for a value of type `T`.
variant Patch<T> {
    /// Replaces the value altogether.
    Replace: T,
    /// Incrementally modifies parts of the value.
    Incremental: [Mutation],
}

/// A mutation applies a specific operation to a part of a value.
record Mutation {
    /// The path identifying the part to apply the operation to.
    path: Path,
    /// The operation to apply to the part identified by the path.
    operation: Operation,
}

/// A path identifies a part of a value.
alias Path = [PathSegment]

/// A segment of a path.
variant PathSegment {
    /// Identifies a field of a record type.
    Field: string,
    /// Identifies an element of a sequence type.
    Index: idx,
    /// Identifies an entry of a map type.
    Key: AnyValue,
}

/// An operation.
variant Operation {
    /// Set the value of a field, element, or entry.
    ///
    /// The path must identify the field, element, or entry.
    Set: AnyValue,
    /// Remove an element of a sequence or an entry of a map.
    ///
    /// The path must identify the element or entry.
    Remove,
    /// Append an element to a sequence.
    ///
    /// The path must must identify the sequence.
    Append: AnyValue,
    /// Insert an element into a sequence.
    ///
    /// The path must identify the insertion position.
    Insert: AnyValue
}

/// An error applying an update.
variant Error {
    /// The path does not point to a value.
    InvalidPath,
    /// The operation is not valid on the path.
    InvalidOperation,
}
```

The `Patch` type can then be imported and used as follows:

```sidex
import ::sidex::{
    patch::{Error, Patch},
    rpc::Stream,
}

alias EntityId: ...

record Entity {
    ...
}

service Api {
    fun apply(id: EntityId, patch: Patch<Entity>) -> Result<(), Error>
    fun subscribe(id: EntityId) -> Stream<Patch<Entity>>
}
```

When enabling the `patch` extension for a specific generator backend, code for diffing and patching shall be generated for all eligible type definitions automatically unless opted-out with `#[patchable(no)]`.
