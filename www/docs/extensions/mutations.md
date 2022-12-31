---
sidebar_position: 4
---

# Mutations

A _mutation_ describes a _semantically meaningful change_ to data of a certain type `T`.
As such, it must preserve the invariants of type `T`.
A mutation itself, that is, the description of a change, is a datum which has a type.
For instance, creating a user may be described by a mutation of type `CreateUser` which can be defined in Sidex as:

```sidex
record CreateUser {
    name: string,
    email: string,
}
```

Explicit mutation types have multiple advantages over “anything goes” ad-hoc modifications:

- They limit the ways in which state can be manipulated.
- They can be exchanged between different nodes in a distributed system.
- They, in principle, allow for conflict-free replicated data types (CRDTs).

Mutations should be deterministic, i.e., the same mutation applied to the same data should always result in the same change to the data.
Hence, a mutation induces a pure partial function from `T` to `T`.
It is partial in the sense that mutations may fail.
Mutations must not have any side effects or return anything besides the updated data.

## Mutable Types

Sidex allows types to be annotated with a `#[mutable(with T)]` attribute where `T` is a path to a data type which is the mutation type for the annotated type.

Here is an example:

```sidex
#[mutable(with MutateUser)]
record User {
    name: string,
    email: string,
}

#[mutation(for User)]
variant MutateUser {
    SetName: string,
    SetEmail: string,
}
```

Creating mutation types an implementing them can be tedious and repetitive.
Sidex helps you with that.

## Settable and Mutable Fields

Whenever a field can be independently set without violating a type's invariants, i.e., is marked with `#[settable]`, it can also be independently mutated. Note that the reverse is not necessarily true.

Two attributes:

- `#[settable]` enables the independent setting of the field.
- `#[mutable]` enables the independent mutation of the field.

where `#[settable]` implies `#[mutable]`.

In case of variants, `#[settable]` and `#[mutable]` refer to the associated type.

Changing the variant means changing the entire value and is thus achieved by setting on the outer type.

```sidex
#[mutation(for User)]
record SetName {
    name: string
}

#[mutation(for User)]
record SetEmail {
    email: Email
}

#[mutable(with UserMutation)]
record User {
    name: string,
    email: Email
}

#[mutation(for User with [SetName, SetEmail])]
derived UserMutation
```

Instead of defining these mutations manually, one may also write:

```
record User {
    #[setter]
    name: string,
    #[setter]
    email: Email
}
```

This will generate a mutation for setting the fields.

Mutations are reified, i.e., they have themselves a Sidex type.

```
generated UserMutation

record Xyz {
    test: Mutation<User>
}
```

```
variant UserMutation {
    SetName: string,
    SetEmail: Email,
}
```

# Synchronization

## Differential Synchronization

Diff and patch algorithms must preserve invariants. Mutations are not used as part of the synchronization scheme but only applied locally on each node.

## Operational Transform

Transform mutations which are based on an old state.

## Conflict-Free Replicated Data Types (CRDTs)

Provide a means to totally order mutations, e.g., by tracking their dependencies using hashes and resolving ties by node IDs.
Then, obtain the current state by applying the mutations to a default state in that order.

```

```
