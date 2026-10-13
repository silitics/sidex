"""Validate documented wire representations with generated JSON Schema 2020-12."""

import json
import subprocess
from pathlib import Path

import pytest
from jsonschema import Draft202012Validator, FormatChecker

FIXTURE = (
    Path(__file__).resolve().parents[2] / "crates/sidex-gen-json-schema/tests/fixtures"
)


@pytest.fixture(scope="module")
def builtin_schemas(sidex_bin, tmp_path_factory):
    """Generate and check schemas, then return the built-in field schemas."""
    output = tmp_path_factory.mktemp("builtin_schemas")
    subprocess.run(
        [str(sidex_bin), "generate", "json-schema", str(output)],
        cwd=FIXTURE,
        check=True,
        capture_output=True,
    )
    for path in output.glob("*.schema.json"):
        schema = json.loads(path.read_text())
        Draft202012Validator.check_schema(schema)
        assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    definitions = json.loads((output / "schema-defs.json").read_text())
    return definitions["json_schema_fixture.types.Builtins"]["properties"]


@pytest.mark.parametrize(
    ("name", "minimum", "maximum"),
    [
        ("signed8", -(2**7), 2**7 - 1),
        ("signed16", -(2**15), 2**15 - 1),
        ("signed32", -(2**31), 2**31 - 1),
        ("unsigned8", 0, 2**8 - 1),
        ("unsigned16", 0, 2**16 - 1),
        ("unsigned32", 0, 2**32 - 1),
    ],
)
def test_small_integer_bounds(builtin_schemas, name, minimum, maximum):
    """Small integers enforce numeric bounds without accepting strings or booleans."""
    validator = Draft202012Validator(builtin_schemas[name])
    for value in [minimum, minimum + 1, 0, maximum - 1, maximum]:
        assert validator.is_valid(value), (name, value)
    for value in [minimum - 1, maximum + 1, 0.5, True, None, "1", [], {}]:
        assert not validator.is_valid(value), (name, value)


@pytest.mark.parametrize("name", ["signed64", "unsigned64"])
def test_integer64_representations(builtin_schemas, name):
    """64-bit schemas check decimal spelling and the unsigned numeric minimum."""
    schema = builtin_schemas[name]
    validator = Draft202012Validator(schema)
    assert schema["type"] == ["integer", "string"]
    assert "maximum" not in schema
    for value in [
        0,
        42,
        2**53 - 1,
        2**53,
        2**53 + 1,
        2**64,
        "0",
        "00042",
        "9007199254740993",
        "18446744073709551616",
    ]:
        assert validator.is_valid(value), (name, value)
    for value in [0.5, True, None, [], {}, "", "+42", "1.0", "1e3", " 1", "42x"]:
        assert not validator.is_valid(value), (name, value)
    if name == "signed64":
        assert "minimum" not in schema
        assert validator.is_valid(-42)
        assert validator.is_valid("-9223372036854775808")
    else:
        assert schema["minimum"] == 0
        assert not validator.is_valid(-1)
        assert not validator.is_valid("-1")


def test_idx_is_host_independent(builtin_schemas):
    """Indices have a nonnegative numeric shape without a host-sized maximum."""
    schema = builtin_schemas["index"]
    assert schema == {"type": "integer", "minimum": 0}
    validator = Draft202012Validator(schema)
    for value in [0, 42, 2**32, 2**64]:
        assert validator.is_valid(value)
    for value in [-1, 0.5, "1", True]:
        assert not validator.is_valid(value)


@pytest.mark.parametrize("name", ["float32", "float64"])
def test_floats(builtin_schemas, name):
    """Floats accept numbers and the canonical and legacy non-finite strings."""
    validator = Draft202012Validator(builtin_schemas[name])
    for value in [0, -0.5, 1.25, 1e100, "NaN", "Infinity", "+Infinity", "-Infinity"]:
        assert validator.is_valid(value)
    for value in [None, True, "0.5", "nan", "+NaN", "Inf", "NaN ", "Infinityx"]:
        assert not validator.is_valid(value)
    assert builtin_schemas[name]["type"] == ["number", "string"]


def test_string_annotations(builtin_schemas):
    """String schemas preserve format and encoding hints without extra patterns."""
    for name, annotation in [
        ("binary", {"contentEncoding": "base64"}),
        ("identifier", {"format": "uuid"}),
        ("instant", {"format": "date-time"}),
        ("calendar", {"format": "date"}),
    ]:
        assert builtin_schemas[name] == {"type": "string", **annotation}


@pytest.mark.parametrize(
    ("name", "accepted", "rejected"),
    [
        (
            "identifier",
            [
                "67e55044-10b1-426f-9247-bb680e5fe0c8",
                "67E55044-10B1-426F-9247-BB680E5FE0C8",
            ],
            ["67e5504410b1426f9247bb680e5fe0c8", "invalid"],
        ),
        (
            "calendar",
            ["2026-10-12", "2024-02-29"],
            ["2026-02-29", "2026-04-31", "2026-10-12T00:00:00Z"],
        ),
    ],
)
def test_standard_string_formats(builtin_schemas, name, accepted, rejected):
    """Standard format assertions validate UUIDs and calendar dates."""
    validator = Draft202012Validator(
        builtin_schemas[name], format_checker=FormatChecker()
    )
    for value in accepted:
        assert validator.is_valid(value), (name, value)
    for value in rejected:
        assert not validator.is_valid(value), (name, value)


def test_containers_and_json(builtin_schemas):
    """Containers validate values and integer-key spelling while json stays unrestricted."""
    sequence = Draft202012Validator(builtin_schemas["sequence"])
    assert sequence.is_valid([0, "9007199254740993", "18446744073709551615"])
    assert not sequence.is_valid(["1.5"])
    assert not sequence.is_valid([None])
    mapping = Draft202012Validator(builtin_schemas["map"])
    assert mapping.is_valid({"first": "-9223372036854775808", "second": 42})
    assert not mapping.is_valid({"first": "invalid"})
    for name in ["integerKeys", "aliasedKeys", "wrappedKeys"]:
        keys = Draft202012Validator(builtin_schemas[name])
        for key in ["0", "00042", "18446744073709551616"]:
            assert keys.is_valid({key: "value"})
        for key in ["-1", "+1", "1.5", "invalid"]:
            assert not keys.is_valid({key: "value"})
    signed_keys = Draft202012Validator(builtin_schemas["signedIntegerKeys"])
    assert signed_keys.is_valid({"-42": "value"})
    assert not signed_keys.is_valid({"+42": "value"})
    assert "propertyNames" not in builtin_schemas["map"]
    arbitrary = Draft202012Validator(builtin_schemas["arbitrary"])
    nested = Draft202012Validator(builtin_schemas["nested"])
    for value in [None, True, 2**64 - 1, 0.5, "text", [], {"anything": None}]:
        assert arbitrary.is_valid(value)
        assert nested.is_valid({"key": [value]})


def test_nested_and_recursive_references(sidex_bin, tmp_path):
    """Sibling definitions and recursive roots resolve under JSON Schema 2020-12."""
    (tmp_path / "sidex.toml").write_text(
        '[bundle]\nname = "references"\nversion = "0.1.0"\n'
    )
    schemas = tmp_path / "schemas"
    schemas.mkdir()
    (schemas / "data.sidex").write_text(
        "wrapper Id: string\n"
        "record Node { id: Id, children: [Node] }\n"
        "record Envelope { node: Node }\n"
        '#[json(tag = "result")]\n'
        'variant Response { #[json(content = "node")] Ok: Node, Failed }\n'
    )
    output = tmp_path / "output"
    subprocess.run(
        [str(sidex_bin), "generate", "json-schema", str(output)],
        cwd=tmp_path,
        check=True,
        capture_output=True,
    )
    node = {"id": "root", "children": [{"id": "child", "children": []}]}
    for name, sample in [
        ("Node", node),
        ("Envelope", {"node": node}),
        ("Response", {"result": "Ok", "node": node}),
    ]:
        schema = json.loads(
            (output / f"references.data.{name}.schema.json").read_text()
        )
        Draft202012Validator.check_schema(schema)
        Draft202012Validator(schema).validate(sample)
