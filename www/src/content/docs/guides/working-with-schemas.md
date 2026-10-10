---
title: Working with Schemas
description: Check, format, generate samples, and keep generated output reproducible.
order: 4
---

Schema changes affect every application using the model. A useful workflow checks the definition, keeps source formatting consistent, regenerates code, and tests the resulting wire values.

## Check and Format

From a bundle directory:

```sh
sidex check
sidex fmt
```

To remove unused imports:

```sh
sidex check --fix
```

The fix mode removes unused imports where possible. Run `sidex check` again afterward to perform the validation and lifecycle lint checks.

For CI, use a formatting check that leaves files unchanged:

```sh
sidex check
sidex fmt --check
```

`fmt` also accepts individual files and directories. Directory arguments are searched recursively; a bundle's schema directory itself is flat.

## Generate Reproducible JSON Samples

Sidex can generate random values for a named type:

```sh
sidex json generate tasks::task::Task --count 5 --seed 42
```

Without `--pretty`, each line is one JSON value, suitable for piping into a test harness. A seed reproduces the sample sequence for the same implementation and configuration.

Bound the generated data:

```sh
sidex json generate tasks::task::Task \
  --count 10 --seed 42 \
  --max-depth 4 --max-collection-len 3 --max-string-len 24
```

The generator follows the type structure and JSON mapping. It does not solve `#[validate(...)]` constraints, so a generated value may fail application validation. A depth bound prefers terminating recursive choices; a model without a terminating path cannot produce a finite value.

See the [CLI reference](/sidex/docs/reference/cli/#json-generate) for numeric representation flags.

## Override a Backend Setting

Try a configuration without editing the manifest:

```sh
sidex generate py generated/raw-models --config-override opaque_lowering=json
sidex generate rust generated/rust --config-override 'derive=["Debug","Clone"]'
```

Overrides use dot-separated paths. Values are parsed as JSON when possible and otherwise become strings. Escape a literal dot in a key segment with `\.`.

An override affects one generation command. Record settings in `sidex.toml` when they are part of your application's normal build.

## Check Generated Output

Pin a Sidex version, run generation in a clean directory, and compile or import the output with the runtime packages used by the application. For an API change, test JSON round trips in each target language.

Generation writes files but does not remove stale files for schemas you deleted or renamed. Clean the output directory as part of your build if you regenerate into an existing directory.

## Inspect the Resolved Model

```sh
sidex generate ir generated/ir
sidex validate-ir generated/ir/ir.json
```

IR output contains resolved types, source metadata, and parsed attributes. `validate-ir` deserializes the file and requires its text to match compact reserialization exactly. Pretty-printing changes the text and fails this check. Unordered metadata maps can also change key order during reserialization, so the current command can reject generated IR.

The command is a developer check for generated IR files, not a validator for application JSON.
