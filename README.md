<p align="center">
    <img src="./www/static/img/logo.svg" width="15%" alt="Sidex Logo">
</p>
<h1 align="center">
    Sidex
</h1>
<h4 align="center">
    A data modeling language for cross-stack data exchange.
</h4>
<p align="center">
  <a href="https://crates.io/crates/sidex-cli"><img alt="Sidex CLI Crate" src="https://img.shields.io/crates/v/sidex-cli?label=sidex-cli"></a>
  <a href="https://github.com/silitics/sidex/actions"><img alt="Tests" src="https://img.shields.io/github/actions/workflow/status/silitics/sidex/run-checks.yml?branch=main&label=tests"></a>
  <a href="https://oss.silitics.com/sidex/docs/getting-started"><img alt="Docs" src="https://img.shields.io/static/v1?label=docs&message=main&color=blue"></a>
  <a href="https://crates.io/crates/sidex-cli"><img alt="License: MIT/Apache" src="https://img.shields.io/crates/l/sidex-cli"></a>
</p>

> 🚧 **WORK IN PROGRESS** 🚧
>
> Sidex **is still under construction**. In particular, the functionality described here may not exist yet, may change considerably in the future, or may even be completely abandoned at a later point in time. We are actively working on the design, features, and vision of Sidex. ⚠️

_Sidex_ is a format- and language-agnostic data modeling language with a focus on type safety, extensibility, and developer ergonomics. Sidex aims to simplify data exchange between different programming languages and platforms via potentially multiple interchange formats and storage backends.

💡 **Idea:** Define your data model once with Sidex and then generate code for different languages and interchange formats from this single ground truth.

Sidex's type system and definition language are inspired by [Rust](https://www.rust-lang.org/). With _custom attributes_, similar to [Rust's attributes](https://doc.rust-lang.org/reference/attributes.html), the possibilities for code generation become almost limitless. You can easily develop your own code generator or extend an existing one to fit your needs.

To learn more, [checkout Sidex's website](https://oss.silitics.com/sidex/). 🚀

## ✨ Features

- **Schema-first** definition of data structures.
- Designed for **format- and language-agnostic** definitions.
- Modern **algebraic data types** and **non-null** by default.
- **Extensible** with user-defined opaque types.
- Support for **generics**, **recursive types**, and **custom attributes**.
- Language server and [VS Code extension](https://marketplace.visualstudio.com/items?itemName=silitics.sidex) for increased productivity.
- Out-of-the-box support for [Rust](https://rust-lang.org), [TypeScript](https://www.typescriptlang.org), [Python](https://www.python.org), and [JSON Schema](https://json-schema.org).

## ⚖️ Licensing

Sidex is licensed under either [MIT](https://github.com/silitics/sidex/blob/main/LICENSE-MIT) or [Apache 2.0](https://github.com/silitics/sidex/blob/main/LICENSE-APACHE) at your opinion.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache 2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

Made with ❤️ by [Silitics](https://www.silitics.com).
