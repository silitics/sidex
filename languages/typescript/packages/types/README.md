# @sidex/types

Types, conversion helpers, and structural checks for
[Sidex](https://oss.silitics.com/sidex)'s JSON representation.

`I64` and `U64` use numbers up to a magnitude of `Number.MAX_SAFE_INTEGER`
and decimal strings beyond it. Use `toBigInt` for exact arithmetic;
`toNumber` rounds large values to the nearest JavaScript number.
Float helpers encode non-finite values as the strings `"NaN"`, `"+Infinity"`,
and `"-Infinity"`, and also read `"Infinity"`.

```ts
import { Bytes, U64, Uuid } from "@sidex/types"

const size = U64.fromBigInt(9007199254740993n)
const exactSize = U64.toBigInt(size)
const signature = Bytes.fromUint8Array(new Uint8Array([0, 1, 255]))
const id = Uuid.parse("67E55044-10B1-426F-9247-BB680E5FE0C8")
```

Each built-in has an `is` guard that accepts what the Rust reader accepts,
including calendar validity for `Date` and `Timestamp`. `Timestamp.toDate` converts nanosecond timestamps to JavaScript's millisecond
precision.

## Generated Structural Checks

The TypeScript generator emits `is` and `check` for each type. They use the
runtime in this package:

```ts
import { Task } from "./generated/task"

const input: unknown = JSON.parse(responseText)
if (Task.is(input)) console.log(input.title)
for (const error of Task.check(input).errors) {
  console.log(error.path, error.code, error.message)
}
```

Checks follow the Rust reader: unknown record fields are ignored and optional
`null` values count as absent. Generic types take a checker per type argument,
such as `Page.is(input, Task)`. See the
[TypeScript guide](https://oss.silitics.com/sidex/docs/generation/typescript/)
for details.

## Tests

Install the development dependencies, then run from this package directory
with Node 22.18 or later:

```sh
npm test
```
