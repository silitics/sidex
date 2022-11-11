# Sidex Core

This crate contains core data structures and functionality for working with _Sidex definitions_ and generating code based on them. Sidex is a format- and language-agnostic data modeling and API definition framework with a focus on simplicity, extensibility, and developer ergonomics. To learn more, checkout [Sidex's GitHub repository](https://github.com/silitics/sidex).

⚠️ **Warning:** This crate is for internal purposes only and may see frequent breaking changes. If you want to write a code generator for Sidex definitions, please use [`sidex-gen`](https://crates.io/crates/sidex-gen), which re-exports more stable functionality, instead. 

## Transformation Pipeline

Sidex code generators work on an intermediate representation of Sidex schemas  coined _Sidex Intermediate Representation_ (SIR). The SIR contains all type information necessary for effective code generation, for instance, the types of struct fields are resolved to their respective definitions. To this end, this crate implements the following transformation pipeline:

```plain
┌──────────────┐   Parsing    ┌──────┐   Type Resolution    ┌─────┐
│ Text Sources │ ───────────► │ ASTs │ ───────────────────► │ SIR │
└──────────────┘              └──────┘                      └─────┘
```

1️⃣ All source files are parsed to _Abstract Syntax Trees_ (ASTs).

2️⃣ The ASTs are then transformed by _Type Resolution_ to the SIR.
