# Supported Languages

Support for different programming languages is realized via Sidex _backends_. A backend for a language usually comprises (1) a code generator and (2) a _runtime_ library providing functionality used by the generated code.

You can write your own code generator in Rust with the [`sidex-gen`](https://crates.io/crates/sidex-gen) crate. See [Third-Party Backends](#third-party-backends) for more details.

## Officially Supported Backends

Officially, we provide backends for the following programming languages:

- [Rust](./rust)
- [TypeScript](./typescript)

In the future, we may add officially supported backends for Python and C++.

For now, we do not plan to accept pull requests adding officially supported backends.

## Third-Party Backends

Sidex is a young project and there are no third-party backends yet.

:::info

If you wrote a backend for a language (may also be an officially supported one) [please let us know](https://github.com/silitics/sidex/discussions/new?category=show-and-tell) so we can add it here.

:::

If you develop a backend in Rust, please refrain from giving the crate a name of the form `sidex-gen-*`. While not enforceable on [crates.io](https://crates.io), we would like to reserve this namespace for officially supported backends. Instead, feel free to use a name of the form `sidex-contrib-*` clearly indicating that the backend is a third-party contribution.

If you think about developing a backend, it's probably best to have a look at the [Advanced Topics](../advanced).
