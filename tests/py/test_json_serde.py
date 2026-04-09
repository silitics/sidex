"""Test JSON serialization round-trips for the Python backend.

Generates Python code from the ``tests/json`` Sidex bundle, then validates
that every fixture in ``tests/json/expected/`` can be deserialized into the
corresponding pydantic model and re-serialized to produce identical JSON.
"""

import json
from pathlib import Path

import pytest
from pydantic import TypeAdapter

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
FIXTURES = REPO_ROOT / "tests" / "json" / "expected"
JSON_BUNDLE = REPO_ROOT / "tests" / "json"


@pytest.fixture(scope="module")
def data(generate):
    """Generate and import the ``data`` schema module."""
    pkg = generate(JSON_BUNDLE, "json_test_bundle")
    return pkg.data


def _fixture_files(type_name: str):
    """Yield ``(stem, path)`` pairs for every fixture of a given type."""
    d = FIXTURES / type_name
    if not d.is_dir():
        return
    for p in sorted(d.glob("*.json")):
        yield p.stem, p


def _roundtrip(adapter: TypeAdapter, path: Path):
    """Deserialize *path*, re-serialize, and return both JSON values."""
    raw = path.read_text()
    expected = json.loads(raw)
    obj = adapter.validate_json(raw)
    reserialized = json.loads(adapter.dump_json(obj, by_alias=True, exclude_none=True))
    return expected, reserialized


class TestVariantInternallyTagged:
    """Internally tagged variant with custom tag/content field names."""

    def _adapter(self, data):
        return TypeAdapter(data.VariantInternallyTagged[int])

    @pytest.fixture(autouse=True)
    def _data(self, data):
        self.data = data

    @pytest.mark.parametrize(
        "name,path",
        list(_fixture_files("VariantInternallyTagged")),
        ids=[n for n, _ in _fixture_files("VariantInternallyTagged")],
    )
    def test_roundtrip(self, name, path):
        expected, actual = _roundtrip(self._adapter(self.data), path)
        assert actual == expected

    def test_variant_a_type(self, data):
        adapter = self._adapter(data)
        obj = adapter.validate_json((FIXTURES / "VariantInternallyTagged/data-variant-a.json").read_text())
        assert isinstance(obj, data.VariantInternallyTagged_A)

    def test_variant_e_inherits_record(self, data):
        adapter = self._adapter(data)
        obj = adapter.validate_json((FIXTURES / "VariantInternallyTagged/data-variant-e-with-optional.json").read_text())
        assert isinstance(obj, data.VariantInternallyTagged_E)
        assert isinstance(obj, data.VariantTestRecord)
        assert obj.a == 42
        assert obj.b == "Hello World!"

    def test_variant_f_inherits_generic_record(self, data):
        adapter = self._adapter(data)
        obj = adapter.validate_json((FIXTURES / "VariantInternallyTagged/data-variant-f-with-optional.json").read_text())
        assert isinstance(obj, data.VariantInternallyTagged_F)
        assert obj.x == 32
        assert obj.y == "Hello World!"

    def test_variant_g_generic_content(self, data):
        adapter = self._adapter(data)
        obj = adapter.validate_json((FIXTURES / "VariantInternallyTagged/data-variant-g.json").read_text())
        assert isinstance(obj, data.VariantInternallyTagged_G)
        assert obj.default_content_field == 42


class TestVariantExternallyTagged:
    """Externally tagged variant — unit variants are bare strings."""

    def _adapter(self, data):
        return TypeAdapter(data.VariantExternallyTagged[int])

    @pytest.fixture(autouse=True)
    def _data(self, data):
        self.data = data

    @pytest.mark.parametrize(
        "name,path",
        list(_fixture_files("VariantExternallyTagged")),
        ids=[n for n, _ in _fixture_files("VariantExternallyTagged")],
    )
    def test_roundtrip(self, name, path):
        expected, actual = _roundtrip(self._adapter(self.data), path)
        assert actual == expected

    def test_variant_a_is_model(self, data):
        adapter = self._adapter(data)
        obj = adapter.validate_json('"A"')
        assert isinstance(obj, data.VariantExternallyTagged_A)
        assert obj.root == "A"

    def test_variant_b_value_field(self, data):
        adapter = self._adapter(data)
        obj = adapter.validate_json('{"B": 42}')
        assert isinstance(obj, data.VariantExternallyTagged_B)
        assert obj.value == 42


class TestVariantAdjacentlyTagged:
    """Adjacently tagged variant — tag and content in separate fields."""

    def _adapter(self, data):
        return TypeAdapter(data.VariantAdjacentlyTagged[int])

    @pytest.fixture(autouse=True)
    def _data(self, data):
        self.data = data

    @pytest.mark.parametrize(
        "name,path",
        list(_fixture_files("VariantAdjacentlyTagged")),
        ids=[n for n, _ in _fixture_files("VariantAdjacentlyTagged")],
    )
    def test_roundtrip(self, name, path):
        expected, actual = _roundtrip(self._adapter(self.data), path)
        assert actual == expected


class TestVariantImplicitlyTagged:
    """Implicitly tagged variant — no tag field, matched by shape."""

    def _adapter(self, data):
        return TypeAdapter(data.VariantImplicitlyTagged[bool])

    @pytest.fixture(autouse=True)
    def _data(self, data):
        self.data = data

    @pytest.mark.parametrize(
        "name,path",
        list(_fixture_files("VariantImplicitlyTagged")),
        ids=[n for n, _ in _fixture_files("VariantImplicitlyTagged")],
    )
    def test_roundtrip(self, name, path):
        expected, actual = _roundtrip(self._adapter(self.data), path)
        assert actual == expected


class TestRecordTypes:
    """Record types with field name renaming."""

    @pytest.mark.parametrize(
        "name,path",
        list(_fixture_files("RecordFieldNamesDefault")),
        ids=[n for n, _ in _fixture_files("RecordFieldNamesDefault")],
    )
    def test_default_names(self, data, name, path):
        expected, actual = _roundtrip(TypeAdapter(data.RecordFieldNamesDefault), path)
        assert actual == expected

    @pytest.mark.parametrize(
        "name,path",
        list(_fixture_files("RecordFieldNamesScreamingSnake")),
        ids=[n for n, _ in _fixture_files("RecordFieldNamesScreamingSnake")],
    )
    def test_screaming_snake(self, data, name, path):
        expected, actual = _roundtrip(TypeAdapter(data.RecordFieldNamesScreamingSnake), path)
        assert actual == expected


class TestMaybeRef:
    """Implicitly tagged ``MaybeRef<T>`` — either a value or a ``$ref``."""

    @pytest.mark.parametrize(
        "name,path",
        list(_fixture_files("MaybeRefInteger")),
        ids=[n for n, _ in _fixture_files("MaybeRefInteger")],
    )
    def test_integer(self, data, name, path):
        expected, actual = _roundtrip(TypeAdapter(data.MaybeRef[int]), path)
        assert actual == expected

    @pytest.mark.parametrize(
        "name,path",
        list(_fixture_files("MaybeRefPoint")),
        ids=[n for n, _ in _fixture_files("MaybeRefPoint")],
    )
    def test_point(self, data, name, path):
        expected, actual = _roundtrip(TypeAdapter(data.MaybeRef[data.Point]), path)
        assert actual == expected

    def test_ref_alias(self, data):
        obj = data.Reference.model_validate({"$ref": "https://example.com"})
        assert obj.reference == "https://example.com"
        assert obj.model_dump(by_alias=True) == {"$ref": "https://example.com"}


class TestAny:
    """Recursive implicitly-tagged ``Any`` type."""

    @pytest.mark.parametrize(
        "name,path",
        list(_fixture_files("Any")),
        ids=[n for n, _ in _fixture_files("Any")],
    )
    def test_roundtrip(self, data, name, path):
        expected, actual = _roundtrip(TypeAdapter(data.Any), path)
        assert actual == expected
