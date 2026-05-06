# @sidex/validate

Runtime support for [Sidex](https://oss.silitics.com/sidex)'s validation
extension. The TypeScript codegen emits calls into this package for every
`#[validate(...)]` rule on a generated type.

## Usage

This package is consumed by code generated from `.sidex` schemas — you
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
`{ path, code, message }` errors rather than throwing — multiple errors
per call surface every problem at once, which is what UI forms want.
The shape of `ValidationError`, the helper signatures, and the
default-message vocabulary mirror the Rust runtime in `sidex-validate`
exactly: a `(code, path)` tuple from a TypeScript driver and a Rust
driver are byte-for-byte comparable.

## Adapting to UI form libraries

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

## License

Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE)
or [MIT license](./LICENSE-MIT) at your option.
