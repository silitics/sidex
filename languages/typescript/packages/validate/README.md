# @sidex/validate

Runtime support for [Sidex](https://oss.silitics.com/sidex)'s validation
extension. The TypeScript codegen emits calls into this package for every
`#[validate(...)]` rule on a generated type.

## Usage

This package is consumed by code generated from `.sidex` schemas. You
typically don't import it directly. Schema authors declare validation
rules in `.sidex` files, and the codegen emits validators that import
from `@sidex/validate`:

```ts
import { CreateUserAction } from "./api/users"

const report = CreateUserAction.fields.email.validate(input)
if (report.errors.length > 0) {
  // surface report.errors[0].message etc.
}
```

A field-level validator returns a `ValidationReport` of zero or more
`{ path, code, message }` errors rather than throwing. Multiple errors
per call surface every problem at once, which is what UI forms want.
The shape of `ValidationError`, the helper signatures, and the
default-message vocabulary mirror the Rust runtime in `sidex-validate`
exactly: a `(code, path)` tuple from a TypeScript driver and a Rust
driver are byte-for-byte comparable.

Validators expect a typed value. Check incoming JSON first with the generated
`is` or `check` methods, which use `@sidex/types`:

```ts
const input: unknown = JSON.parse(responseText)
if (CreateUserAction.is(input)) {
  console.log(CreateUserAction.validate(input).errors)
} else {
  console.log(CreateUserAction.check(input).errors)
}
```

## Adapting to UI Form Libraries

Most form libraries expect a field validator that returns either
`undefined` (valid) or a `string` (the error message). Collapse the
report to its first error:

```ts
import type { ValidationReport } from "@sidex/validate"

function firstError(report: ValidationReport): string | undefined {
  if (report.errors.length === 0) return undefined
  return report.errors[0].message
}
```

## Tests

Tests run against the TypeScript sources with Node's built-in test runner and
type stripping. No build or dependency installation is required. From this
package directory, use Node 22.18 or later:

```sh
npm test
# Or with Nix:
nix shell nixpkgs#nodejs_22 -c node --experimental-strip-types --test tests/*.test.ts
```

The tests cover exact comparisons of 64-bit decimal strings, mixed
`number` and `bigint` operands after `numeric`, and non-finite floats.

## License

Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE)
or [MIT license](./LICENSE-MIT) at your option.
