---
sidebar_position: 3
---

# Validation

The validation extension lets you attach declarative rules to fields,
records, wrappers, and opaque types via `#[validate({ ... })]`
attributes. Rules compile down to runtime validators in both the Rust and
TypeScript backends; an empty report means "valid" — no exceptions.

```sidex
record User {
    #[validate({ 1 <= _.length <= 254 })]
    #[validate({ matches(_, "^[^@]+@[^@]+$") }, message = "Not a valid email.")]
    email: string,

    #[validate({ 0 <= _ <= 150 })]
    age: u32,
}

#[validate({ 1 <= _.length <= 64 })]
wrapper Slug: string

#[validate({ _.min <= _.max })]
record Range {
    min: i32,
    max: i32,
}
```

Each `#[validate(...)]` carries one rule. Multiple attributes accumulate.

## Anatomy of a rule

```sidex
#[validate(
    { 0 <= _ <= 100 },
    message = "Value must be between 0 and 100.",
    code = "out_of_range",
)]
```

- The first positional `{ ... }` is the rule expression, captured
  verbatim and parsed by the plugin's expression DSL. (You can also
  spell it `expr = { ... }` if you prefer named args.)
- `message` (optional) — the human-readable message reported when the
  rule fails. Defaults to a built-in keyed on the resolved error code.
- `code` (optional) — overrides the default error code. Useful for
  surfacing a generic check (`regex`) under a domain-specific identifier
  (`format:phone`).

## Expression grammar

```text
expr     = primary (cmp_op primary)+        // chained comparison
         | call                              // top-level predicate call
         | primary                           // bare value (rare)
primary  = "_" ("." ident)*                  // _, _.length, _.min
         | number
         | string
         | call
call     = ident "(" arg ("," arg)* ")"
arg      = primary | string
cmp_op   = "<" | "<=" | "==" | "!=" | ">=" | ">"
```

Comparisons chain (`0 <= _ <= 100` is two segments — a min and a max).
The `_` placeholder denotes the validated value. `_.length` is the
Unicode code-point count for strings and the byte count for `bytes`.
`_.<field>` accesses record fields (used for cross-field rules).

`matches(_, "<regex>")` is the only built-in function in v1. Future
versions will add format checks (`is_email(_)`, `is_url(_)`, …).

## Recognized shapes

The recognizer maps common shapes to specific runtime helpers with
stable error codes:

| Source | Error code | Helper |
|---|---|---|
| `n <= _` / `n < _` | `min` | `min_inclusive` / `min_exclusive` |
| `_ <= m` / `_ < m` | `max` | `max_inclusive` / `max_exclusive` |
| `n <= _.length` | `min_length` | `min_inclusive` over `char_count` |
| `_.length <= m` | `max_length` | `max_inclusive` over `char_count` |
| `_ == n` / `_ != n` | `eq` / `ne` | `eq` / `ne` |
| `matches(_, "…")` | `regex` | `regex_match` |
| anything else | `predicate` | predicate fallback |

The `code` override on a `#[validate(...)]` replaces the recognized code.

## Generated code

### Rust

For each type with rules the Rust backend emits:

- `impl ::sidex_validate::Validate for T` — runs every rule and merges
  the results into one [`ValidationReport`].
- `T::try_new(inner) -> Result<T, ValidationReport>` for wrappers with
  rules — the safe construction path.
- A per-callsite `static REGEX_<n>: ::std::sync::OnceLock<Regex>` for
  every `matches(...)` rule, lazily compiled on first use.

```rust
use sidex_validate::Validate;

let user = User { email: "".into(), age: 30, nickname: None };
let report = user.validate();
for err in report.errors() {
    println!("{}: {}", err.path, err.message);
}
```

### TypeScript

For each type with rules the TS backend emits an entry under the same
`.ts` file:

```ts
import * as validators from "./data";

const report = validators.User.validate({ email: "", age: 30 });
for (const err of report.errors) {
    console.log(`${err.path}: ${err.message}`);
}

// Per-field validators are useful for forms.
const onChange = (v: string) => validators.User.fields.email(v);

// Wrappers expose tryNew, returning a tagged result.
const r = validators.Slug.tryNew("hello-world");
if (r.ok) { /* ... */ } else { /* r.report */ }
```

## Error model

Errors from a single `validate()` call are collected in a
[`ValidationReport`]; an empty report means "valid". Each error carries
three things:

- `path` — JSON-pointer-style location (`/users/3/email`).
- `code` — stable identifier the application can map to localized
  strings.
- `message` — the human-readable form (either the default for `code`
  or the user-supplied override).

The Rust and TypeScript runtimes produce identical `(path, code)`
tuples for the same input, by design — frontends and backends can
exchange validation results without translation.

## Three semantic decisions

These are pinned across runtimes so cross-target results agree:

1. **`_.length` is Unicode code points** for strings (Rust:
   `s.chars().count()`; TS: spread iteration). The native `String.length`
   in JavaScript would count UTF-16 code units, which disagrees with
   Rust on non-BMP characters.
2. **Regex flavor is the RE2-compatible subset of ECMAScript.** Avoid
   backreferences and lookaround; those work in JS but not in Rust's
   `regex` crate.
3. **Validation never throws.** A non-`ValidationReport` exception is
   always a bug in the validator itself.

## What's deferred

Non-breaking additions for future versions:

- `_.size` accessor for sequences and maps.
- Format functions (`is_email`, `is_url`, `is_uuid`, …).
- Logical combinators (`&&`, `||`, `!`).
- Arithmetic in expressions.
- Templated messages (`"must be between {min} and {max}"`).
- JSON Schema enrichment from validate rules.
- Python backend support (waiting on a plugin trait in `sidex-gen-py`).

[`ValidationReport`]: https://docs.rs/sidex-validate
