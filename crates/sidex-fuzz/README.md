# Sidex: Fuzz

This crate is part of [Sidex](https://oss.silitics.com/sidex/). It generates
random JSON instances conforming to a Sidex type's canonical JSON shape, as
defined by the type's `#[json(...)]` attributes.

The generator is deterministic given a seed: the same `(IR, type, seed)` tuple
always produces the same JSON value. Generation is recorded as a *choice
sequence* of `u64`s drawn from a seeded ChaCha8 PRNG, which a future shrinker
can reduce to minimize counterexamples.

## Use cases

- Conformance testing: feed generated samples through Rust/TS/Py round-trip
  harnesses and assert canonical-JSON equality.
- Example fixtures and documentation snippets.
- Default-value synthesis for types whose JSON shape is otherwise unbounded.
