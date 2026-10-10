# Sidex CLI

Sidex is a data modeling language for cross-stack data exchange. Its command line
tool checks schemas and generates Rust, TypeScript, Python, and JSON Schema.

Run the tool without a persistent installation:

```sh
npx sidex-cli check
uvx sidex-cli check
```

Or install it with Python tooling:

```sh
pipx install sidex-cli

sidex check
sidex generate ts ./out
```

You can also install it with `uv tool install sidex-cli`.

Both `sidex` and `sidex-cli` invoke the same native executable. Packages contain
prebuilt binaries for Linux and macOS on x64 and ARM64. A Rust toolchain is not
required on these platforms.

The npm package installs a matching platform binary as an optional dependency.
Keep optional dependencies enabled when installing it. Installation scripts are
not required.

See the [Sidex documentation](https://oss.silitics.com/sidex/docs/getting-started)
for schemas, configuration, and code generation.
