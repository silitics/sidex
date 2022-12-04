# Sidex: Serde Runtime Helpers

This crate is part of [Sidex](https://oss.silitics.com/sidex/). It contains runtime helpers for [Serde](https://serde.rs/) interoperability.

**⚠️ !!!WARNING!!! ⚠️**

Without the `serde-private-content` feature, this crate comes with its own implementation for intermediate deserialized content. Unfortunately, it seems impossible to replicate Serde's own content machinery in an efficient way without access to Serde's internals. As a result, without the `serde-private-content` feature, deserialization may very often unnecessarily allocate and copy data around. Before stabilizing this crate, [Serde issue #741](https://github.com/serde-rs/serde/issues/741) should to be reconsidered and addressed.