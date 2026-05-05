# API stability example

Demonstrates `#[deprecated]`, `#[unstable]`, and `#[since]` across record
fields, top-level definitions, and variant cases.

```sh
cd examples/api-stability
sidex check                  # lints stability rules (cascading deprecation, etc.)
sidex generate rust   ./out  # emits #[deprecated(...)] + doc preludes
sidex generate ts     ./out  # emits @deprecated / @experimental / @since JSDoc
sidex generate py     ./out  # stability prelude in pydantic docstrings
sidex generate json-schema ./out
```

`sidex check` will warn when a non-deprecated definition refers to a
deprecated type, and error out when the bundle version catches up to
a `remove_in` deadline that's still present in the schema.
