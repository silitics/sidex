# Sidex Crates

These crates make up the core of Sidex.

- [`sidex-cli`](sidex-cli): Contains the Sidex command line utility.
- [`sidex-syntax`](sidex-syntax): Contains data structures for the AST, parser, and lexer. It is not re-exported to code generators.
- [`sidex-ir`](sidex-ir): Contains the IR and builders for it. 
- [`sidex-core`](sidex-core): Contains code for reading Sidex bundles and transforming them to Sidex IR.
- [`sidex-gen`](sidex-gen): Stable interfaces to be used by code generators.