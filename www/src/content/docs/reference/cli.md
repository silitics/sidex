---
title: CLI Reference
description: Commands, backends, and flags supported by the Sidex CLI.
order: 0
---

`sidex` and `sidex-cli` invoke the same executable. Use `--help` on a command to inspect its arguments, and `--version` to identify the build.

Commands that load a bundle locate `sidex.toml` in the working directory or a parent directory. Paths to generation output are relative to the working directory.

## new

```sh
sidex new NAME
```

Creates `NAME/sidex.toml` with bundle version `0.1.0` and a `NAME/schemas` directory. Use a new directory name; this command writes the manifest.

## check

```sh
sidex check [BUNDLE_DIR] [--fix]
```

Loads schemas and reports definition errors and unused imports. Normal check mode also checks validation expressions and API lifecycle metadata. Error diagnostics cause a nonzero exit.

`--fix` removes unused imports where possible and formats affected files. It takes the fix path instead of running the validation and lifecycle lints; follow it with `sidex check`.

## fmt

```sh
sidex fmt [PATHS...] [--check]
```

Without paths, formats the current bundle's schema files. Explicit directories are searched recursively.

| Result in Check Mode            | Exit Code |
| ------------------------------- | --------- |
| Every file is already formatted | `0`       |
| A file would change             | `1`       |
| A file failed to format         | `2`       |

Without `--check`, the command writes formatted files.

## generate

```sh
sidex generate BACKEND OUTPUT [--config-override KEY=VALUE]...
```

| Backend       | Output                                              |
| ------------- | --------------------------------------------------- |
| `rust`        | `mod.rs`                                            |
| `ts`          | One `.ts` per schema and `index.ts`                 |
| `py`          | One `.py` per schema, `__init__.py`, and `py.typed` |
| `json-schema` | Type schema documents and `schema-defs.json`        |
| `ir`          | Compact `ir.json`                                   |

Configuration comes from `[backend.BACKEND]` in the manifest. Repeat `--config-override` to change individual settings for this invocation.

Override paths are dot-separated; `\.` escapes a literal dot in a key. The value is parsed as JSON when possible, otherwise stored as a string.

```sh
sidex generate ts out --config-override opaque_lowering=json
sidex generate rust out --config-override 'derive=["Debug"]'
```

Generation creates the output directory. It does not clean stale output files. Rust, TypeScript, and Python generate the selected bundle; JSON Schema walks loaded noninternal bundles.

## json generate

```sh
sidex json generate BUNDLE::SCHEMA::TYPE [OPTIONS]
```

Emits random JSON samples following a type's JSON shape. It does not enforce validation attributes.

| Flag                     | Default | Purpose                                           |
| ------------------------ | ------- | ------------------------------------------------- |
| `--count N`              | `1`     | Number of samples                                 |
| `--seed N`               | Random  | Reproduce a sample sequence                       |
| `--pretty`               | Off     | Pretty-print each sample                          |
| `--max-depth N`          | `6`     | Bound recursive generation                        |
| `--max-collection-len N` | `8`     | Bound generated collections and free-form objects |
| `--max-string-len N`     | `16`    | Bound strings and free-form keys                  |
| `--integers-as-strings`  | Off     | Emit `i64`, `u64`, and `idx` as decimal strings   |
| `--floats-as-strings`    | Off     | Permit `"NaN"`, `"Infinity"`, and `"-Infinity"`   |

Without `--pretty`, output is one compact JSON value per line. Limits bound random generation choices rather than validate application data.

The CLI currently has no `json validate` subcommand. Use a JSON Schema validator or a language runtime to parse application data.

## validate-ir

```sh
sidex validate-ir FILE
```

Deserializes Sidex IR and asserts exact equality with compact reserialization. Intended as a strict developer check. It is sensitive to object key order and can reject generated IR containing unordered metadata maps. This is not an application JSON validator.
