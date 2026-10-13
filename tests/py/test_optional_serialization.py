"""Check optional-field omission and null preservation through Python and Rust."""

import json
import subprocess
from pathlib import Path

import pydantic
import pytest

REPO_ROOT = Path(__file__).resolve().parents[2]
PAYLOAD = {
    "requiredNull": None,
    "requiredJson": {"nested": None, "list": [None], "map": {"entry": None}},
    "values": [None, {"keep": None}],
    "entries": {"null": None, "object": {"keep": None}},
}


@pytest.fixture(scope="module")
def models(generate):
    """Generate the shared serialization fixture with deferred model building."""
    return generate(REPO_ROOT / "tests/json", "optional_serialization").serialization


@pytest.fixture(scope="module")
def rust_roundtrip():
    """Read Python's output with generated Rust types and encode it again."""
    subprocess.run(
        [
            "cargo",
            "build",
            "-p",
            "sidex-serde-tests",
            "--example",
            "python_roundtrip",
            "--locked",
        ],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
    )
    binary = REPO_ROOT / "target/debug/examples/python_roundtrip"

    def run(typ, text):
        result = subprocess.run(
            [str(binary), typ], input=text, text=True, check=True, capture_output=True
        )
        return json.loads(result.stdout)

    return run


def _assert_serialization(model, adapter, expected, rust_roundtrip, typ):
    """Compare all dump entry points and round-trip their JSON through Rust."""
    assert model.model_dump(by_alias=True) == expected
    assert model.model_dump(mode="json", by_alias=True) == expected
    text = model.model_dump_json(by_alias=True)
    assert json.loads(text) == expected
    assert adapter.dump_python(model, mode="json", by_alias=True) == expected
    assert json.loads(adapter.dump_json(model, by_alias=True)) == expected
    assert rust_roundtrip(typ, text) == expected


@pytest.mark.parametrize(
    "optional", [{}, {"optional.note": None, "optionalJson": None}]
)
def test_record_optional_none(models, rust_roundtrip, optional):
    """Omit missing and explicit optional None while preserving null in containers."""
    model = models.OptionalFields.model_validate({**PAYLOAD, **optional})
    assert model.optional_note is None
    assert model.optional_json is None
    assert model.model_dump() == PAYLOAD
    assert "optional_note" not in model.model_dump(by_alias=False)
    _assert_serialization(
        model,
        pydantic.TypeAdapter(models.OptionalFields),
        PAYLOAD,
        rust_roundtrip,
        "OptionalFields",
    )


def test_present_optional_fields(models, rust_roundtrip):
    """Keep populated optional fields, including nulls inside their JSON values."""
    expected = {**PAYLOAD, "optional.note": "present", "optionalJson": {"keep": None}}
    model = models.OptionalFields.model_validate(expected)
    _assert_serialization(
        model,
        pydantic.TypeAdapter(models.OptionalFields),
        expected,
        rust_roundtrip,
        "OptionalFields",
    )
    without_aliases = model.model_dump(by_alias=False)
    assert without_aliases["optional_note"] == "present"
    assert "optional.note" not in without_aliases


def test_required_json_null(models, rust_roundtrip):
    """A required json field keeps a top-level null."""
    expected = {**PAYLOAD, "requiredJson": None}
    model = models.OptionalFields.model_validate(expected)
    _assert_serialization(
        model,
        pydantic.TypeAdapter(models.OptionalFields),
        expected,
        rust_roundtrip,
        "OptionalFields",
    )


def test_generic_record(models, rust_roundtrip):
    """A concrete generic field keeps null, while its absent optional sibling is omitted."""
    typ = models.Box[pydantic.JsonValue]
    model = typ(payload=None, note=None)
    _assert_serialization(
        model,
        pydantic.TypeAdapter(typ),
        {"payload": None},
        rust_roundtrip,
        "Box",
    )


@pytest.mark.parametrize("tagging", ["Internal", "Adjacent", "External", "Implicit"])
def test_variant_record_payload(models, rust_roundtrip, tagging):
    """Omit absent record fields in every tagging mode, including flattened internal cases."""
    typ = getattr(models, tagging)
    adapter = pydantic.TypeAdapter(typ)
    record = models.OptionalFields.model_validate(PAYLOAD)
    if tagging == "Internal":
        model = models.Internal_Record(**PAYLOAD)
        expected = {"tag": "Record", **PAYLOAD}
    elif tagging == "Adjacent":
        model = models.Adjacent_Record(content=record)
        expected = {"tag": "Record", "content": PAYLOAD}
    elif tagging == "External":
        model = models.External_Record(value=record)
        expected = {"Record": PAYLOAD}
    else:
        model, expected = record, PAYLOAD
    _assert_serialization(model, adapter, expected, rust_roundtrip, tagging)


def test_internal_generic_payload(models, rust_roundtrip):
    """Keep required null and the defaulted tag in flattened generic record cases."""
    model = models.Internal_Generic(payload=None)
    expected = {"tag": "Generic", "payload": None}
    _assert_serialization(
        model,
        pydantic.TypeAdapter(models.Internal),
        expected,
        rust_roundtrip,
        "Internal",
    )


@pytest.mark.parametrize("tagging", ["Internal", "Adjacent", "External"])
def test_variant_null_payload(models, rust_roundtrip, tagging):
    """Preserve required null content even when a variant tag has a default."""
    typ = getattr(models, tagging)
    if tagging == "External":
        model = models.External_Null(value=None)
        expected = {"Null": None}
    else:
        model = getattr(models, tagging + "_Null")(content=None)
        expected = {"tag": "Null", "content": None}
    _assert_serialization(
        model,
        pydantic.TypeAdapter(typ),
        expected,
        rust_roundtrip,
        tagging,
    )


@pytest.mark.parametrize("tagging", ["Internal", "Adjacent"])
def test_unit_variant_tag(models, rust_roundtrip, tagging):
    """Keep defaulted tags on payload-free cases."""
    model = getattr(models, tagging + "_Empty")()
    _assert_serialization(
        model,
        pydantic.TypeAdapter(getattr(models, tagging)),
        {"tag": "Empty"},
        rust_roundtrip,
        tagging,
    )


def test_deferred_generic_model(models):
    """Cache optional fields before deferred or generic schemas are built."""

    class Deferred(models.Box[pydantic.JsonValue]):
        model_config = pydantic.ConfigDict(defer_build=True)

    assert not Deferred.__pydantic_complete__
    model = Deferred.model_validate({"payload": None, "note": None})
    assert model.model_dump() == {"payload": None}
    assert json.loads(pydantic.TypeAdapter(Deferred).dump_json(model)) == {
        "payload": None
    }


def test_native_scalar_encodings(models, rust_roundtrip):
    """Preserve native serializers for large integers, nonfinite floats, timestamps, and bytes."""
    expected = {
        "largeInt": "18446744073709551615",
        "specialFloat": "+Infinity",
        "at": "2026-10-13T10:00:00.123456789Z",
        "blob": "AA==",
    }
    model = models.Encodings.model_validate(expected)
    assert "note" not in model.model_dump()
    assert model.model_dump(mode="json", by_alias=True) == expected
    text = model.model_dump_json(by_alias=True)
    assert json.loads(text) == expected
    adapter = pydantic.TypeAdapter(models.Encodings)
    assert json.loads(adapter.dump_json(model, by_alias=True)) == expected
    assert adapter.dump_python(model, mode="json", by_alias=True) == expected
    assert rust_roundtrip("Encodings", text) == expected
