---
title: TypeScript
description: Generate JSON-shaped types, structural checks, and optional validation rules.
order: 1
---

The TypeScript generator emits type aliases matching the JSON representation. Records use wire field names, and variants become unions selected by their configured tags. Wrappers and primitive opaque types use nominal brands to keep distinct types separate.

## Generate and Import Types

Generate the [task model](/sidex/docs/getting-started/) and install its runtime:

```sh
sidex generate ts src/generated
npm install @sidex/types
```

Generated code supports TypeScript declaration emit (`declaration: true`) and composite projects (`composite: true`).

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

## Check Incoming Data

A type annotation does not check JSON received at runtime. For this, every generated type has a companion object with the same name. Its `is` method narrows an unknown value, and its `check` method reports every structural error:

```ts
import { Task } from "./generated/task";

const input: unknown = JSON.parse(responseText);
if (Task.is(input)) {
  console.log(input.title); // input is Task
}

const report = Task.check(input);
for (const error of report.errors) {
  console.log(error.path, error.code, error.message);
}
```

Generated TypeScript types define the canonical data model. Checks enforce built-in ranges and formats, field names, and variant tags. Unknown record fields are ignored, so older clients accept data with added fields. Optional fields are generated as `field?: T`; a present `null` is rejected unless `T` itself admits `null`, such as `json` or a nullable opaque type. Rust and Python readers may accept `null` as absent when decoding. Checks do not convert or copy values.

Each error has a `path` with wire names, such as `items[2].title`, a `code`, and a `message`. The codes are `type`, `missing`, `case`, and `key`. The `segments` of an error distinguish a key containing a dot from nested keys.

Checks do not run `#[validate]` rules. After `Task.is(input)` succeeds, call `Task.validate(input)` when the [validate plugin](/sidex/docs/guides/validation/) is enabled.

### Generic Types

A generic type takes a checker for each type argument:

```sidex
record Page<T> {
    items: [T],
}

alias Tasks: Page<Task>
```

```ts
import { Page, Task, Tasks } from "./generated/task";

if (Page.is(input, Task)) {
  const page: Page<Task> = input;
}
const report = Page.check(input, Task);
const taskPage = Page.checker(Task);
taskPage.is(input);

Tasks.is(input);
```

Types from external bundles must be generated with checks as well.

### Limitations

Checks see values after `JSON.parse`. It turns `1`, `1.0`, and `1e0` into the same number, so checks accept all three for integer types, while the Rust reader accepts only `1`. `JSON.parse` also rounds integers beyond `Number.MAX_SAFE_INTEGER`. Sidex writers send such 64-bit integers as decimal strings, which checks verify exactly.

The `idx` range is that of a 64-bit Rust target. An opaque type is checked against its declared JSON type only; its Rust implementation may impose further constraints.

## Nominal Types

A nominal type carries a compile-time identity even when its value is a string or number:

```sidex
wrapper TaskId: string
wrapper UserId: string
```

This prevents assigning a `TaskId` where a `UserId` is expected. A brand supplies type safety; it does not change the JSON value or validate it at runtime. Nullable opaque types retain `null` even when their other values carry a brand.

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

A mapping changes a generated type expression. It does not add a serializer or make other applications adopt the same wire representation. Checks still verify the Sidex wire representation: mapping `timestamp` to `string` still rejects an invalid timestamp. Map only to types that match the wire representation, such as `string` for `timestamp`, or `is` narrows to a type the value does not have.

Supplying a `types.table` replaces the default mapping table. Include every built-in mapping your model requires. See the [manifest reference](/sidex/docs/reference/manifest/#typescript-backend) before applying a custom table.
