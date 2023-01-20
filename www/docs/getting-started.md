---
sidebar_position: 1
---

# Getting Started 🚀

Sidex is currently distributed via [Cargo](https://doc.rust-lang.org/cargo/) and [crates.io](https://crates.io/).
To install the Sidex command line tool run:

```plain
cargo install sidex-cli
```

Instead, if you want the most recent version (possibly unstable) directly from the Git repository, run:

```plain
cargo install --git https://github.com/silitics/sidex.git sidex-cli
```

For the **best developer experience**, also install the [VS Code extension](tools/vs-code-extension.md).

:::tip

As Sidex is still under construction, it is best to install it directly from Git for now.

**Please [report any issues](https://github.com/silitics/sidex) you may have.**

:::

## Creating a Sidex Bundle

Sidex organizes data models and interface definitions into _bundles_.

To create a new Sidex _bundle_ named `my_bundle` run:

```plain
sidex new my_bundle
```

Every bundle consists of a flat collection of _schemas_ located in the `schemas` directory.

Here is a simple example of a schema you may place in the file `person.sidex`:

```sidex title="person.sidex"
// Import a type from another bundle and schema.
import ::std::result::Result

/// A *Universally Unique Identifier* (UUID).
#[json(type = "string")]  // In JSON this is a string.
#[rust(type = "::uuid::Uuid")]  // In Rust this is `::uuid::Uuid`.
opaque Uuid  // This is an opaque user-defined type.

/// A unique id identifying a person.
wrapper PersonId: Uuid  // This is a wrapper type.

/// A *role* of a person.
variant Role {  // This is a variant type.
    /// The person is an administrator.
    Admin,
    /// The person is an user.
    User,
}

/// A person.
record Person {  // This is a record type.
    /// The unique id of the person.
    id: PersonId,
    /// The full name of the person.
    name: string,
    /// The optional e-mail address of the person.
    email?: string,  // This field is optional.
    /// The role of the person.
    #[json(name = "level")]  // The JSON field name is `level`.
    role: Role,
    /// Unique ids of the person's friends.
    friends: [PersonId],  // A sequence of person ids.
}

/// Error type for API calls.
record Error {
    /// The error message.
    message: string
}

/// The main API interface.
interface Api {
    /// Lookup a person by their id.
    #[rest(get("/person/<id>"))]
    fun get_person_by_id(id: PersonId) -> Result<Person, Error>
}
```

To check the entire bundle for syntactic and semantic validity run:

```plain
sidex check
```

## Generating Code

To generate TypeScript type definitions in the directory `out-ts` run:

```plain
sidex generate typescript ./out-ts/
```

You can also generate Rust code by running:

```plain
sidex generate rust ./out-rs/
```

To learn more, checkout the [examples](https://github.com/silitics/sidex/tree/main/examples) or continue reading the [user guide](./guide).
