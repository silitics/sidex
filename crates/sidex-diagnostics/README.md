# Sidex: Error Reporting and Diagnostics

This crate is part of [Sidex](https://oss.silitics.com/sidex/). It contains functionality for error reporting and diagnostics.

**ℹ️ Note:** In most cases, in particular, when writing code generators, you should use this crate via its re-export in [`sidex-gen`](https://docs.rs/sidex-gen/).

Sidex's focus on developer ergonomics necessitates that *diagnostics* like errors, warnings, and hints are collected and displayed to the developer using Sidex. To this end, this crate provides a unified diagnostics machinery.
