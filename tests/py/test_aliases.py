"""Old field keys remain readable without changing serialized JSON."""

import json
from pathlib import Path

import pydantic
import pytest


@pytest.fixture(scope="module")
def models(generate):
    """Generate record aliases and their inline and variant containers."""
    return generate(Path(__file__).resolve().parents[1] / "json", "alias_models").aliases


@pytest.mark.parametrize("key", ["threadId", "chatId", "conversationId"])
@pytest.mark.parametrize("title", ["title", "chatName", "oldTitle"])
def test_aliases(models, key, title):
    """Read aliases in records, inline fields, and internally tagged payloads."""
    source = {key: "t1", title: "Title"}
    expected = {"threadId": "t1", "title": "Title"}
    thread = models.Thread.model_validate_json(json.dumps(source))
    assert thread.model_dump() == expected
    assert json.loads(thread.model_dump_json()) == expected
    envelope = models.Envelope[int].model_validate({**source, "value": 7})
    assert envelope.model_dump() == {**expected, "value": 7}
    event = pydantic.TypeAdapter(models.Event).validate_python({"tag": "Updated", **source})
    assert event.model_dump() == {"tag": "Updated", **expected}
