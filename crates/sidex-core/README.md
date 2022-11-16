# Sidex Core

This crate contains core data structures and functionality for dealing with _Sidex bundles_ and generating code for them. Sidex is a format- and language-agnostic data modeling and API definition framework with a focus on simplicity, extensibility, and developer ergonomics. To learn more, checkout [Sidex's website](https://oss.silitics.com/sidex/).

⚠️ **Warning:** This crate is primarily for internal purposes and may see frequent breaking changes. If you want to write a code generator for Sidex, please instead use [`sidex-gen`](https://crates.io/crates/sidex-gen) which re-exports more stable functionality. 

## Transformation Pipeline

Sidex code generators work on an intermediate representation of Sidex bundles   coined _Sidex Intermediate Representation_ (SIR). The SIR contains all type information necessary for effective code generation, for instance, the types of fields are resolved to their respective definitions. To this end, this crate implements the following transformation pipeline:

```plain
┌──────────────┐   Parsing    ┌──────┐   Elaboration    ┌─────┐
│ Text Sources │ ───────────► │ ASTs │ ───────────────► │ SIR │
└──────────────┘              └──────┘                  └─────┘
```

1️⃣ All source files are parsed to _Abstract Syntax Trees_ (ASTs).

2️⃣ The ASTs are then transformed by _elaboration_ to the SIR.
