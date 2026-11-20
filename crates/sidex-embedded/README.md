# Sidex: Runtime for Embedded Targets

This crate is part of [Sidex](https://oss.silitics.com/sidex/). It provides the runtime for Rust types that the `rust-embedded` generator emits for `no_std` targets without an allocator, such as microcontrollers. It reads and writes Sidex's JSON representation, never allocates, and has no dependencies.

See [Rust for Embedded Targets](https://oss.silitics.com/sidex/docs/generation/rust-embedded/) for how to generate and use such types.
