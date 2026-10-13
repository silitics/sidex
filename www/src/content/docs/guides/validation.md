---
title: Validation
description: Share application constraints between Rust and TypeScript.
order: 1
---

A type describes a value's structure. A validation rule adds a constraint, such as a size limit or numeric range. Sidex checks rules in Rust and TypeScript.

Validators receive values whose structure is already known. In TypeScript, check incoming JSON with the generated `is` or `check` methods first.

## Declare Rules

Create `schemas/users.sidex`:

```sidex title="schemas/users.sidex"
record User {
    #[validate({ 1 <= _.size <= 64 }, message = "Use 1 to 64 characters.")]
    username: string,

    #[validate({ 0 <= _ <= 150 })]
    age: u32,

    #[validate({ 1 <= _.size })]
    nickname?: string,
}

#[validate({ 1 <= _.size <= 32 })]
wrapper Slug: string
```

Each attribute is one rule; repeated attributes accumulate. `_` is the validated value. Optional field rules run when the field has a value.

## Enable the Generators

Generated Rust types carry their rules, and the `sidex` crate checks them with its `validation` feature, which is enabled by default. TypeScript validators need the `validate` plugin:

```toml
[backend.ts]
plugins = ["validate"]
```

```sh
sidex check
sidex generate rust generated/rust
sidex generate ts generated/ts
```

`sidex check` parses and checks validation expressions, including invalid or unsupported regex patterns. Generation and application compilation remain necessary to verify target-specific expressions.

TypeScript code needs `@sidex/validate` alongside `@sidex/types`.

## Validate in Rust

```rust
let user = User::new("".into(), 30);

if let Err(report) = sidex::validation::validate(&user) {
    for error in report.errors() {
        println!("{}: {} ({})", error.path(), error.message(), error.code());
    }
}
```

`validate` checks the rules of a value and of every value nested in it, such as the rules of a `Slug` in a field. A report contains every violated rule.

Constructors and decoding do not check rules.

## Validate in TypeScript

Types and validator objects share a name:

```ts
import { User, Slug } from "./generated/users";
import { U32 } from "@sidex/types";

const report = User.validate({
  username: "",
  age: 30 as U32,
});

for (const error of report.errors) {
  console.log(error.path.toString(), error.code, error.message);
}

const fieldReport = User.fields.username.validate("");
const result = Slug.tryNew("release-notes");
if (result.ok) {
  console.log(result.value);
} else {
  console.log(result.report.errors);
}
```

Field validators are useful for forms. Their entries are objects with a `validate` method. Field keys follow JSON renaming, so a `display_name` field appears as `fields.displayName` under the default naming convention.

For incoming data, check its representation before validating rules:

```ts
const input: unknown = JSON.parse(responseText);
if (User.is(input)) {
  const report = User.validate(input);
  console.log(report.errors);
} else {
  console.log(User.check(input).errors);
}
```

`check` reports structural errors. `validate` checks rules on a typed value and does not check structure.

## Expression Reference

| Expression               | Constraint              |
| ------------------------ | ----------------------- |
| `0 <= _ <= 100`          | Inclusive numeric range |
| `_ < 100`                | Exclusive upper bound   |
| `_ == 0`, `_ != 0`       | Equality or inequality  |
| `1 <= _.size <= 64`      | Size bounds             |
| `matches(_, "^[a-z]+$")` | Regex match             |

`_.size` counts Unicode code points for strings, bytes for `bytes`, elements for sequences, and entries for maps. JavaScript's native `String.length` counts UTF-16 code units and can differ from this count.

Use regex syntax supported by both JavaScript and Rust's `regex` crate. Backreferences and lookaround are rejected. Logical operators, arithmetic, and format functions such as `is_email` are not built-in validation syntax.

## Error Reports

Errors carry a path, a stable code, and a human-readable message. Paths locate the value by field names and indices. Common codes include `min`, `max`, `eq`, `ne`, `min_size`, `max_size`, and `regex`. Other comparisons, such as comparisons of two fields, use `invalid`.

Override a message or code when the constraint has a domain-specific meaning:

```sidex
record Order {
    #[validate({ matches(_, "^[A-Z]{2}-[0-9]{4}$") },
        code = "format:order",
        message = "Use a code such as AB-1234.")]
    code: string,
}
```

## Current Limits

Validation attributes on opaque types are retained as metadata without checks. Use a wrapper when you need enforced constraints on a distinct type.

TypeScript record validators check the rules of that record and its fields. Unlike Rust, they do not check values nested in fields against the rules of their types. Call nested validators explicitly where needed.

The Python backend does not generate these validators, and JSON Schema output does not include the rules.
