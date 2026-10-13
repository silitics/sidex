---
title: Python
description: Parse and serialize shared models with Pydantic.
order: 2
---

The Python generator produces a typed package built on Pydantic 2. Records become models, wrappers become root models, and variants get case models and JSON adapters. It also emits a `py.typed` marker for type checkers.

## Generate a Package

Generate the [task bundle](/sidex/docs/getting-started/) into a Python package directory:

```sh
sidex generate py task_models
python -m pip install "pydantic>=2.5"
```

Use Python 3.12 or newer, as used by Sidex's Python integration tests. Run your application from a directory where `task_models` is importable.

## Parse and Serialize a Record

```python
from task_models.task import Task

task = Task.model_validate_json(
    '{"title":"Write a schema","completed":false}'
)

assert task.title == "Write a schema"
assert task.description is None

encoded = task.model_dump_json(by_alias=True, exclude_none=True)
print(encoded)
```

Pydantic validates incoming values according to its model configuration. `by_alias=True` writes JSON field names, and `exclude_none=True` omits absent optional fields.

Booleans and integers up to 32 bits require their JSON types. Integer values must fit the declared range. This keeps implicitly tagged variants from interpreting `1` as a boolean or `300` as a `u8`.

Python models use Python field identifiers. JSON aliases follow Sidex's naming rules, which default to camelCase. Do not assume that Python's field name is also the wire key.

## Variants and Wrappers

Use Pydantic's `TypeAdapter` for a generated variant type:

```sidex
#[json(tagged = adjacently, tag = "state", content = "value")]
variant Outcome {
    Pending,
    Complete: string,
}
```

After generating a package with this schema in `outcomes.sidex`:

```python
from pydantic import TypeAdapter
from task_models.outcomes import Outcome

adapter = TypeAdapter(Outcome)
value = adapter.validate_json('{"state":"Complete","value":"done"}')
encoded = adapter.dump_json(value, by_alias=True, exclude_none=True)
```

The adapter applies the wire representation specified by the schema. For wrappers, access the wrapped value through the Pydantic root model's `root` property.

## Access a Case Payload

Generated cases with a payload provide a `payload()` method. For an internally
tagged record case, the fields are inlined in the case model. `payload()`
constructs a fresh instance of the original record:

```sidex
record Point {
    x: i32,
    y: i32,
}

variant Position {
    Unknown,
    Known: Point,
}
```

With these definitions in `outcomes.sidex`:

```python
from pydantic import TypeAdapter
from task_models.outcomes import Position, Point

position = TypeAdapter(Position).validate_json('{"tag":"Known","x":3,"y":4}')
point = position.payload()
assert isinstance(point, Point)
assert (point.x, point.y) == (3, 4)
```

## Map Native Opaque Types

```sidex
#[json(type = "string")]
#[py(typ = "uuid.UUID")]
opaque UserId
```

The generator imports the native module and uses its type. Your Python environment must provide that module. Without a Python mapping, opaque types fall back to their JSON shape.

Set `opaque_lowering = "json"` to ignore native Python mappings, for example when testing raw JSON round trips:

```toml
[backend.py]
opaque_lowering = "json"
```

## Reuse Other Bundles

```toml
[backend.py.external]
identity = "identity_models"
```

Generate `identity` separately into an importable package. The generated bundle initializer resolves references between sibling schemas after loading them, including recursive references.

## Validation Rules

Pydantic performs model parsing, but Sidex's `#[validate(...)]` rules currently generate validators only for Rust and TypeScript. They are not enforced by the Python backend. Apply equivalent application checks when your Python code requires those constraints.
