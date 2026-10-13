---
title: Getting Started
description: Create a shared task model and generate code from it.
order: 1
---

This guide uses a small task model shared by a service, a web app, and a Python tool. You need [Node.js/npm or uv](/sidex/docs/installation/) to run the CLI. The commands below use npm; replace `npx sidex-cli` with `uvx sidex-cli` if you prefer uv.

## Create a Bundle

A bundle groups related schemas and their generator configuration.

```sh
npx sidex-cli new tasks
cd tasks
```

Sidex creates `sidex.toml` and an empty `schemas` directory:

```text
tasks/
├── sidex.toml
└── schemas/
```

## Define a Task

Create `schemas/task.sidex`:

```sidex title="schemas/task.sidex"
//! Tasks shared by a service, a web app, and a Python tool.

/// A task to complete.
record Task {
    /// Short title shown in a task list.
    title: string,
    /// Whether the task is complete.
    completed: bool,
    /// Additional detail, when provided.
    description?: string,
}
```

A `record` has named fields. `title` and `completed` are required; `description?` may be absent. `///` comments become documentation in generated code, while `//!` documents the schema.

Check the model and format it:

```sh
npx sidex-cli check
npx sidex-cli fmt
```

## Generate Code

Run the generators you need from the bundle directory:

```sh
npx sidex-cli generate ts generated/ts
npx sidex-cli generate py generated/task_models
npx sidex-cli generate json-schema generated/json-schema
```

The TypeScript output contains `task.ts` and an `index.ts` that exports its module. The Python output is a package with `task.py`, `__init__.py`, and a `py.typed` marker. JSON Schema output includes `tasks.task.Task.schema.json`.

For Rust types with constructors and serialization, add the following to your manifest:

```toml title="sidex.toml"
[bundle]
name = "tasks"
version = "0.1.0"

[backend.rust]
derive = ["Debug", "Clone"]
plugins = ["builder", "serde"]
```

Then generate a Rust module:

```sh
npx sidex-cli generate rust generated/rust
```

The `types` plugin is enabled automatically. `builder` adds constructors, and `serde` adds serialization and deserialization implementations. Each language guide lists the runtime dependencies needed to compile its output.

## Exchange a Task as JSON

The shared JSON representation is:

```json
{
  "title": "Write the first schema",
  "completed": false,
  "description": "Generate code for the applications that share this model."
}
```

When no description is present, omit that key:

```json
{
  "title": "Write the first schema",
  "completed": false
}
```

An optional field permits absence. It does not make `null` a valid value for a `string` field. See [JSON mapping](/sidex/docs/guides/json/) for renaming and variant representations.

## Use the Generated Model

Choose a guide to connect the output to your application:

- [Rust](/sidex/docs/generation/rust/): generate during a Cargo build and use Serde.
- [TypeScript](/sidex/docs/generation/typescript/): import JSON-shaped types and check incoming data.
- [Python](/sidex/docs/generation/python/): parse and serialize with Pydantic.
- [JSON Schema](/sidex/docs/generation/json-schema/): describe JSON for validators and other tooling.

The complete [manifest](/sidex/examples/tasks/sidex.toml) and [schema](/sidex/examples/tasks/schemas/task.sidex) are available to download. To extend the model, continue with [types](/sidex/docs/language/types/) and [validation](/sidex/docs/guides/validation/).
