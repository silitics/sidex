# @sidex/types

Types and conversion helpers for [Sidex](https://oss.silitics.com/sidex)'s
built-in types in their JSON representation.

`I64` and `U64` use numbers up to a magnitude of `Number.MAX_SAFE_INTEGER`
and decimal strings beyond it. Use `toBigInt` for exact arithmetic;
`toNumber` rounds large values to the nearest JavaScript number.
Float helpers encode non-finite values as strings and also read the legacy
`"+Infinity"` spelling.

```ts
import { Bytes, U64, Uuid } from "@sidex/types"

const size = U64.fromBigInt(9007199254740993n)
const exactSize = U64.toBigInt(size)
const signature = Bytes.fromUint8Array(new Uint8Array([0, 1, 255]))
const id = Uuid.parse("67E55044-10B1-426F-9247-BB680E5FE0C8")
```

`Date.is` checks the `YYYY-MM-DD` shape.
`Timestamp.toDate` converts nanosecond timestamps to JavaScript's millisecond
precision.

## Tests

Tests run against the TypeScript sources with Node's built-in test runner and
type stripping. No build or dependency installation is required. From this
package directory, use Node 22.18 or later:

```sh
npm test
# Or with Nix:
nix shell nixpkgs#nodejs_22 -c node --experimental-strip-types --test tests/*.test.ts
```
