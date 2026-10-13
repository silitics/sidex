---
title: TypeScript
description: Use generated JSON-shaped types and optional runtime validation.
order: 1
---

The TypeScript generator emits type aliases matching the JSON representation. Records use wire field names, and variants become unions selected by their configured tags. Wrappers and primitive opaque types use nominal brands to keep distinct types separate.

## Generate and Import Types

Generate the [task model](/sidex/docs/getting-started/) and install its runtime:

```sh
sidex generate ts src/generated
npm install @sidex/types
```

Each schema produces a `.ts` file. `index.ts` exports schema modules. Import a type directly:

```ts
import type { Task } from "./generated/task";

const task: Task = {
  title: "Write a schema",
  completed: false,
};

const json = JSON.stringify(task);
```

No separate conversion is needed to serialize a value already matching the JSON representation.

## Type Checking and Incoming Data

TypeScript type checking runs before execution. A type annotation or assertion does not validate JSON received at runtime:

```ts
const input: unknown = JSON.parse(responseText);
```

Use a runtime JSON Schema validator to check the structure of `input` against the [generated JSON Schema](/sidex/docs/generation/json-schema/). Once a value has a known shape, generated [validation rules](/sidex/docs/guides/validation/) can check application constraints.

## Nominal Types

A nominal type carries a compile-time identity even when its value is a string or number:

```sidex
wrapper TaskId: string
wrapper UserId: string
```

This prevents assigning a `TaskId` where a `UserId` is expected. A brand supplies type safety; it does not change the JSON value or validate it at runtime.

Built-in numeric types also use brands in `@sidex/types`. Like their JSON representation, `I64` and `U64` are numbers or, beyond `Number.MAX_SAFE_INTEGER`, decimal strings, so all values stay exact. Convert them explicitly:

```ts
import { U64 } from "@sidex/types";

const size = U64.fromNumber(1024);
const kibibytes = U64.toNumber(size) / 1024;
const exact = U64.toBigInt(size);
```

See the [JSON representation of built-in types](/sidex/docs/guides/json/#built-in-types).

Maps are objects. `entries` from `@sidex/types` lists the entries of a map with the map's key type, such as a nominal identifier type.

## Enable Validation

```toml title="sidex.toml"
[backend.ts]
plugins = ["validate"]
```

```sh
npm install @sidex/validate
sidex generate ts src/generated
```

The type generator is always enabled, so list only additional plugins. Types and their validator objects share an exported name:

```ts
import { User } from "./generated/users";

const report = User.fields.username.validate("");
for (const error of report.errors) {
  console.log(error.code, error.message);
}
```

See [validation](/sidex/docs/guides/validation/) for the corresponding schema and whole-record validation.

## Configure Imports and Mappings

External bundles can map to package imports:

```toml
[backend.ts.external]
identity = "@example/identity-models"
```

Type mappings use fully qualified Sidex paths:

```toml
[backend.ts.types.table]
"::core::builtins::timestamp" = "string"
```

A mapping changes a generated type expression. It does not add a serializer or make other applications adopt the same wire representation.

Supplying a `types.table` replaces the default mapping table. Include every built-in mapping your model requires. See the [manifest reference](/sidex/docs/reference/manifest/#typescript-backend) before applying a custom table.
