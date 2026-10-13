"""Compare inline-field parsing and serialization with Sidex's Rust runtime."""

import json
import subprocess
from pathlib import Path

import pydantic
import pytest

REPO_ROOT = Path(__file__).resolve().parents[2]
ENVELOPE = {"envelopeId": "outer", "value": "x"}


@pytest.fixture(scope="module")
def models(generate):
    """Generate inline fixtures, including forward and cross-schema references."""
    return generate(REPO_ROOT / "tests/json", "inline_models").inline


def _assert_roundtrip(model, typ, expected, rust_roundtrip, fixture):
    """Check all dump entry points and read Rust's encoded output back in Python."""
    adapter = pydantic.TypeAdapter(typ)
    assert model.model_dump(by_alias=True) == expected
    assert model.model_dump(mode="json", by_alias=True) == expected
    text = model.model_dump_json(by_alias=True)
    assert json.loads(text) == expected
    assert adapter.dump_python(model, mode="json", by_alias=True) == expected
    assert json.loads(adapter.dump_json(model, by_alias=True)) == expected
    encoded = rust_roundtrip(fixture, text)
    assert encoded == expected
    assert adapter.validate_json(json.dumps(encoded)) == model


@pytest.mark.parametrize(
    "extra", [{}, {"note": None}, {"note": "inner", "extra.note": "extra"}]
)
def test_inline_records(models, rust_roundtrip, extra):
    """Share the original map across inline siblings and omit absent payload fields."""
    source = {**ENVELOPE, **extra, "unknown": 1, "ignored.payload": {"value": "wrong"}}
    original = source.copy()
    model = models.Envelope.model_validate(source)
    assert source == original
    assert model.payload.value == "x"
    expected = {
        **ENVELOPE,
        **{key: value for key, value in extra.items() if value is not None},
    }
    _assert_roundtrip(
        model, models.Envelope, expected, rust_roundtrip, "InlineEnvelope"
    )
    assert models.Envelope.model_validate_json(json.dumps(source)) == model
    without_aliases = model.model_dump(by_alias=False)
    assert without_aliases["envelope_id"] == "outer"
    assert "payload" not in without_aliases
    assert "extras" not in without_aliases
    if "extra.note" in expected:
        assert without_aliases["extra_note"] == expected["extra.note"]


def test_nested_inline(models, rust_roundtrip):
    """Flatten nested inline fields while retaining optional fields at each level."""
    expected = {
        **ENVELOPE,
        "note": "inner",
        "outerNote": "outer",
        "extra.note": "extra",
    }
    model = models.Nested.model_validate_json(json.dumps({**expected, "unknown": None}))
    _assert_roundtrip(model, models.Nested, expected, rust_roundtrip, "InlineNested")


@pytest.mark.parametrize(
    "name,fixture",
    [
        ("WrappedEnvelope", "InlineWrapped"),
        ("Generic", "InlineGeneric"),
        ("GenericWrapped", "InlineGenericWrapped"),
    ],
)
def test_wrapped_and_generic_records(models, rust_roundtrip, name, fixture):
    """Inline aliases, nested wrappers, and specialized generic record fields."""
    if name == "Generic":
        typ = models.InlineRecord[models.Payload]
    elif name == "GenericWrapped":
        typ = models.InlineRecord[models.WrappedAlias[models.PayloadAlias]]
    else:
        typ = models.WrappedEnvelope
    expected = (
        {"value": "x"} if name == "WrappedEnvelope" else {"payload": {"value": "x"}}
    )
    model = typ.model_validate_json(json.dumps(expected))
    _assert_roundtrip(model, typ, expected, rust_roundtrip, fixture)


def test_cross_schema_inline(models, rust_roundtrip):
    """Resolve deferred inline types from another schema and preserve required null."""
    model = models.CrossSchema.model_validate({"payload": None, "note": None})
    _assert_roundtrip(
        model,
        models.CrossSchema,
        {"payload": None},
        rust_roundtrip,
        "InlineCrossSchema",
    )


def test_empty_inline(models, rust_roundtrip):
    """Read and write an empty object when every inlined record field is optional."""
    empty = models.EmptyEnvelope.model_validate_json("{}")
    _assert_roundtrip(
        empty, models.EmptyEnvelope, {}, rust_roundtrip, "InlineEmptyEnvelope"
    )


@pytest.mark.parametrize(
    "raw", ["{}", '{"payload":{"value":"nested"}}', '{"value":null}']
)
def test_inline_requires_payload_fields(models, rust_roundtrip, raw):
    """Inline fields reject missing or invalid required payload fields."""
    with pytest.raises(pydantic.ValidationError):
        models.WrappedEnvelope.model_validate_json(raw)
    with pytest.raises(subprocess.CalledProcessError):
        rust_roundtrip("InlineWrapped", raw)


def test_native_construction(models, rust_roundtrip):
    """Keep inline model instances supplied as Python constructor arguments."""
    model = models.Envelope(
        envelope_id="outer", payload=models.Payload(value="x"), extras=models.Extras()
    )
    _assert_roundtrip(
        model, models.Envelope, ENVELOPE, rust_roundtrip, "InlineEnvelope"
    )


def test_deferred_generic_inline(models):
    """Cache inline field metadata before building deferred generic schemas."""

    class Deferred(models.InlineRecord[models.PayloadAlias]):
        model_config = pydantic.ConfigDict(defer_build=True)

    assert not Deferred.__pydantic_complete__
    metadata = Deferred._sidex_inline
    expected = {"payload": {"value": "x"}}
    model = Deferred.model_validate(expected)
    assert model.model_dump() == expected
    assert json.loads(pydantic.TypeAdapter(Deferred).dump_json(model)) == expected
    assert Deferred._sidex_inline is metadata


@pytest.mark.parametrize("tagging", ["Internal", "Adjacent", "External", "Implicit"])
def test_variant_inline_payload(models, rust_roundtrip, tagging):
    """Flatten record payload fields through every variant union and payload() method."""
    typ = getattr(models, tagging)
    if tagging == "Internal":
        expected = {"tag": "Envelope", **ENVELOPE}
    elif tagging == "Adjacent":
        expected = {"tag": "Envelope", "content": ENVELOPE}
    elif tagging == "External":
        expected = {"Envelope": ENVELOPE}
    else:
        expected = ENVELOPE
    model = pydantic.TypeAdapter(typ).validate_json(json.dumps(expected))
    _assert_roundtrip(model, typ, expected, rust_roundtrip, "Inline" + tagging)
    if tagging != "Implicit":
        assert model.payload().payload.value == "x"


@pytest.mark.parametrize("generic", [False, True])
def test_variant_generic_inline(models, rust_roundtrip, generic):
    """Substitute record parameters in concrete and generic internally tagged cases."""
    typ = models.GenericCase[models.Payload] if generic else models.Internal
    expected = {"tag": "Value" if generic else "Generic", "payload": {"value": "x"}}
    model = pydantic.TypeAdapter(typ).validate_json(json.dumps(expected))
    fixture = "InlineGenericCase" if generic else "InlineInternal"
    _assert_roundtrip(model, typ, expected, rust_roundtrip, fixture)
    assert model.payload().payload.payload.value == "x"


def test_concrete_case_field_type(models):
    """Validate concrete type arguments on flattened generic case fields."""
    adapter = pydantic.TypeAdapter(models.TypedCase)
    model = adapter.validate_json('{"tag":"Value","value":"x"}')
    assert model.payload().value == "x"
    with pytest.raises(pydantic.ValidationError):
        adapter.validate_json('{"tag":"Value","value":42}')


def test_case_payload_field_name(models):
    """Keep the payload accessor callable when a case record has a payload field."""
    model = pydantic.TypeAdapter(models.NamedCase).validate_json(
        '{"tag":"Value","payload":"x"}'
    )
    assert model.payload_ == "x"
    assert model.model_dump() == {"tag": "Value", "payload": "x"}
    assert model.payload().payload == "x"


def test_renamed_inline_fields(models, rust_roundtrip):
    """Preserve both values when renaming separates outer and inline wire names."""
    model = models.RenamedEnvelope(
        payload=models.Payload(value="inline"), value="outer"
    )
    expected = {"value": "inline", "outerValue": "outer"}
    _assert_roundtrip(
        model, models.RenamedEnvelope, expected, rust_roundtrip, "InlineRenamed"
    )
