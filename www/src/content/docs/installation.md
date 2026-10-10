---
title: Installation
description: Run the CLI with npm or Python tooling, or build the current source.
order: 2
---

Sidex's command line tool checks schemas, formats them, and generates code. The executable is named `sidex`; npm and Python packages also provide `sidex-cli` as an alias.

## Run without Installing

With Node.js and npm:

```sh
npx sidex-cli --help
```

With uv:

```sh
uvx sidex-cli --help
```

These runners download a CLI package on first use. They do not require a Rust toolchain on supported platforms.

## Install for Repeated Use

Choose one package manager:

```sh
npm install --global sidex-cli
```

```sh
uv tool install sidex-cli
```

```sh
pipx install sidex-cli
```

Verify the installation:

```sh
sidex --version
sidex --help
```

Prebuilt packages support Linux and macOS on x64 and ARM64. Keep npm's optional dependencies enabled: the main npm package selects the matching platform package. It does not need installation scripts.

## Pin a CLI Version

Use one version across developer machines and CI so generated output stays reproducible:

```sh
npx sidex-cli@0.2.8 check
uvx --from sidex-cli==0.2.8 sidex check
```

The version above illustrates pinning. Choose a release that supports the features your schemas use. These docs follow the main branch and may include features added after a release.

## Build from Source

To use the code described by these docs, clone the repository and build the CLI. Install the Rust toolchain selected by `rust-toolchain.toml`, including Rustfmt.

```sh
git clone https://github.com/silitics/sidex.git
cd sidex
cargo build --bin sidex
./target/debug/sidex --help
```

For an installation into Cargo's binary directory:

```sh
cargo install --path crates/sidex-cli
```

## Editor Support

The repository contains a VS Code extension for syntax highlighting. To build a VSIX package from a source checkout:

```sh
mise run build:vscode-extension
```

Install the resulting `.vsix` with VS Code's **Extensions: Install from VSIX** command. The extension provides highlighting; it does not provide a language server or editor diagnostics. Use `sidex check` for diagnostics and `sidex fmt` for formatting.
