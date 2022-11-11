**🚧 TODO:** These are only loose thoughts right now. Publish as a blog post, when I had time to sort them out and structure them better.

This post is mainly supposed to address the elephant in the room – why is there a need for yet another data modeling and API definition framework when we already have [Protocol Buffers](https://developers.google.com/protocol-buffers), [FlatBuffers](https://google.github.io/flatbuffers/), [gRPC](https://grpc.io/), [GraphQL](https://graphql.org/), [JSON Schema](https://json-schema.org/), and [OpenAPI](https://www.openapis.org/) just to name a few?

### Why Sidex?

All of the existing solutions listed above have in common that they, in some form or another, cater to a specific interchange format or transport mechanism. OpenAPI is great when you are building a REST API or aiming to formalize your existing one. JSON Schema is perfect if your primary interchange format is JSON and you wish to take full advantage of JSON's flexibility. Protocol Buffers, FlatBuffers, and gRPC are a really good fit if you care about compact and backwards-compatible binary serialization. And, finally, GraphQL has its own specific use case of providing flexibility to frontend engineers by enabling them to flexibly request only the data needed for a specific interaction with a user.

**If your needs are fully covered by any of these existing solutions, please go ahead and use them.**

Now, with Sidex we pursue a different approach aiming to decouple the definition of data models and APIs from any specific interchange format or transport mechanism.[^1] For our projects at [Silitics](https://silitics.com/), we have found that depending on the circumstances different formats and transport mechanisms for what is fundamentally the same data or API are often needed or at least beneficial.

#### An Example

Imagine you are building a web application for managing tasks. The tasks are stored in a database. As part of the application, you also build a REST API accessing the database for managing tasks. With Sidex you can specify the data storage and REST API within a unified schema:[^2]

```sidex title="tasks.sidex"
/// Uniquely identifies a specific [`Task`] in the system.
alias TaskId = idx

/// Uniquely identifies a specific [`Person`] in the system.
alias PersonId = idx

/// A task.
#[orm(table = "tasks")]
record Task {
    /// The unique [`TaskId`] of the task.
    ///
    /// Note that this field is optional such that we can
    /// represent tasks which have no [`TaskId`] yet.
    #[orm(primary_key)]
    id?: TaskId,
    /// A human readable description of the task.
    description: string,
    /// A flag indicating whether the task has been completed.
    completed: bool,
    /// An optional assignee identified by a [`PersonId`].
    #[orm(foreign = Person)]
    assignee?: PersonId,
}

/// A person.
#[orm(table = "persons")]
record Person {
    /// The unique [`PersonId`] of the person.
     #[orm(primary_key)]
    id?: PersonId,
    /// The full name of the person.
    name: string,
}

/// An error returned by the API.
#[http(error)]
variant Error {
    /// Access to the requested resource has been denied.
    #[http(code = 403, message = "Not authorized.")]
    AccessDenied,
    /// The requested resource has not been found.
    #[http(code = 404, message = "Resource not found.")]
    NotFound,
}

/// Definition of the task manager API.
service TaskManager {
    /// List tasks optionally filtered by an assignee.
    ///
    /// For the REST API the assignee is part of the query. In addition,
    /// there is a limit for the number of tasks and an offset.
    #[get("/tasks?<assignee>&<offset>&<limit>")]
    fun tasks(assignee?: PersonId, offset?: idx, limit?: idx) -> [Task]

    /// Add a task.
    #[post("/tasks")]
    fun add_task(task: Task) -> TaskId

    /// Get a task by its id.
    #[get("/tasks/<id>")]
    fun get_task(id: TaskId) -> Result<Task, Error>

    /// Update a task.
    #[put("/tasks/<id>")]
    fun update_task(id: TaskId, task: Task) -> Result<(), Error>
}
```

Using Sidex _generator backends_ you can then generate all the boring boilerplate code for accessing the database from Rust and Python, a Python class or Rust trait for the implementing the `TaskManager` backend, and a TypeScript REST API client for building a beautiful frontend.[^3]

#### Differentiating Factors

Sidex is centered around a powerful type system supporting modern algebraic data types as well as generics and adopting non-null by default. It does not prescribe a specific serialization format or transport mechanism. Instead, it supports attributes of the form `#[ … ]` which can be used to fine tune storage, serialization, and different kinds of APIs. While we plan to standardized some attributes, you are free to augment your definitions with custom attributes and roll your own generator backends. In fact, Sidex provides (Rust) helper libraries empowering you to customize it to your needs.

Speaking of extensibility, Sidex also allows you to define your own primitives as _opaque types_:

```sidex
#[json(type = "string")]
#[rust(type = "::uuid::Uuid")]
opaque Uuid

#[json(type = "number | string | null | bool | array | object")]
#[rust(type = "::serde_json::Value")]
opaque JsonValue
```

In particular, this enables interoperability with other specification mechanisms such as JSON Schema:

```sidex
#[json(schema = "../json-schema/VerySpecificValue.schema.json")]
#[rust(type = "::serde_json::Value")]  // For Rust, we do not care.
opaque VerySpecificValue
```

To summarize, Sidex differentiates itself by the following factors from existing solutions:

1. It is format- and language-agnostic, i.e., it does not prescribe any particular serialization format nor programming language so you can choose the right tools for the job.

2. Sidex is based on a powerful type system enabling you to build rich data models and APIs at scale.

3. Sidex

As a start, we plan to provide out-of-the-box support for Rust, TypeScript, JSON and some binary format. Overtime, we hope, we will broaden the set of supported languages, formats, and applications.

[^1]: Whether or not this will be a success remains to be seen.
[^2]: Of course, you can also specify a separate data model schema and import it into the API schema. It is also possible to write independent schemas, if it better fits your use case.
[^3]: At least that is the vision. At the time of writing, we are not there yet.

# Motivation

If you want to learn more about why we came up with Sidex, this page is for you. Otherwise, feel free to skip this section.

The inspiration for Sidex was born out of [Rust's](https://rustlang.org) macro system – arguably one of Rust's greatest features. Type definitions and other language items can be annotated with macros that generate additional code. An example may look something like this:

```rust
#[derive(Clone, Debug)]
pub Person {
    pub name: String,
    pub age: u8,
}
```

Here, the `Clone` and `Debug` [_derive macros_](https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros) generate code for copying an instance of a `Person` and for printing a debug representation of a `Person`, respectively. While `Clone` and `Debug` are builtin derive macros, libraries can also define their own macros.

In Rust, derive macros are very powerful and heavily relied upon by the ecosystem to reduce boilerplate. For instance, the highly popular [Serde](https://serde.rs) crate uses derive macros to generate code for serializing and deserializing data structures. Another famous example is [Diesel](https://diesel.rs/), an ORM which also relies on derive macros to generate code for interacting with a database. For instance, with Diesel, one can generate code for inserting a new user into a database as follows:

```rust
#[derive(Insertable)]
#[table_name="users"]
struct NewUser<'a> {
    name: &'a str,
    hair_color: Option<&'a str>,
}
```

After working with Rust for a while, we got used to the power and convenience of derive macros. A `derive(This)` here and `derive(That)` there can really go a long way to speed up development by generating huge amounts of code you would otherwise write manually.

Now, the idea of Sidex is to generalize this system and make it independent of Rust. You write type definitions in the Sidex definition language and Sidex will provide the necessary framework to generate code for multiple programming languages and purposes.

In contrast to Rust macros, Sidex code generators have access to full type information. Rust's macro system is purely syntactical, i.e., when a derive macro runs, it sees types basically as strings. A derive macro does not have any access to more elaborated type information because, when the macro runs, this information is not available yet. A macro generates code and as such must run before any stages of the compiler that assume that the code is fully there. In that sense, Sidex provides a more powerful way to generate code by exposing full type information.

## Bridging Language Boundaries

Our initial use case for Sidex has been a web application where the frontend has been written in TypeScript and the backend in Rust. Working on this application, we soon began investigating the possibility to also generate TypeScript code from our type definitions in Rust. While this is possible,[^other-langs] it has not been as convenient and flexible as we wished it to be.

[^other-langs]: For instance with [`ts-rs`](https://github.com/Aleph-Alpha/ts-rs) or via JSON Schema with [Schemars](https://graham.cool/schemars/).

## 🤔 Rationale

**Why schema-first?**

A schema-first approach has multiple advantages: (1) It constrains the definable types to a sensible language-independent set. (2) By not clouding definitions with language details, it enables focusing on the important aspects of the data being exchanged. (3) It allows developing tooling independent of any programming language. (4) It enables the independent evolution and adaption of the data modeling and API definition language. (5) It can be used independently of any particular programming language.

**Why yet another language?**

Existing approaches are often specific to certain serialization formats, do not explicitly support algebraic data types, do not support arbitrary user-defined opaque types, have nullable fields by default, or/and are overly complex by supporting much more structures/types than Sidex.

**Why are there no tuple types?**

Essentially tuple types are like struct types but without named fields. We believe that it is better to be explicit about field names in order to avoid confusion and misuse, in particular, when multiple elements of a tuple are of the same type. Hence, Sidex does not support tuple types.

**Why are there only flat modules?**

For our use cases, we found that a deeper nesting is unnecessary and it also complicates code generation. Hence, we decided to go with a flat _modules_ directory. If you think you have a good argument and use case for deeper nesting, feel free to [open a discussion on GitHub](https://github.com/silitics/sidex/discussions/new?category=ideas).
