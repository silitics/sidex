"""Test timestamp precision and UTC wire formatting in generated Python."""

import datetime
from pathlib import Path

import pydantic
import pytest


@pytest.fixture(scope="module")
def timestamp(generate):
    """Generate a timestamp wrapper and expose its adapter."""
    package = generate(Path(__file__).parent / "timestamp", "timestamp_test_bundle")
    return pydantic.TypeAdapter(package.data.Instant)


@pytest.mark.parametrize("wire,expected", [
    ("2024-02-29T12:34:56.789Z", "2024-02-29T12:34:56.789Z"),
    ("9999-12-30T22:00:00.999999999Z", "9999-12-30T22:00:00.999999999Z"),
    ("2024-02-29T13:34:56.123456789+01:00", "2024-02-29T12:34:56.123456789Z"),
    ("1970-01-01T00:00:00.000Z", "1970-01-01T00:00:00Z"),
    ("2024-02-29t12:34:56.123456789z", "2024-02-29T12:34:56.123456789Z"),
])
def test_timestamp_roundtrip(timestamp, wire, expected):
    """Preserve nanoseconds and use the Rust backend's canonical UTC spelling."""
    typed = timestamp.validate_python(wire)
    assert isinstance(typed, datetime.datetime)
    assert timestamp.dump_json(typed).decode() == f'"{expected}"'


def test_timestamp_precision_limit(timestamp):
    """Reject fractions beyond the nanosecond precision supported by Rust."""
    with pytest.raises(pydantic.ValidationError):
        timestamp.validate_python("2024-02-29T12:34:56.1234567890Z")
