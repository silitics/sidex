"""Check that JSON field restrictions apply before checks and every generator."""

import subprocess

import pytest

CASES = [
    pytest.param(
        "record Envelope {\n    #[json(inline)]\n    payload: string,\n}\n",
        "`#[json(inline)]` requires a record type, or an alias or wrapper of a record.",
        3,
        "not a record",
        None,
        id="nonrecord",
    ),
    pytest.param(
        "record Payload { value: string }\n"
        "record Envelope {\n    #[json(inline)]\n    payload?: Payload,\n}\n",
        "`#[json(inline)]` requires a nonoptional field.",
        4,
        "optional inline field",
        None,
        id="optional",
    ),
    pytest.param(
        "record Envelope {\n    value: string,\n"
        '    #[json(name = "value")]\n    other?: string,\n}\n',
        "JSON field name `value` occurs more than once: "
        "`Envelope.other` conflicts with `Envelope.value`.",
        4,
        "duplicate JSON field `value` from `Envelope.other`",
        "first JSON field `value` from `Envelope.value`",
        id="renamed",
    ),
    pytest.param(
        "record Payload { value: string }\n"
        "record Envelope {\n    value: string,\n"
        "    #[json(inline)]\n    payload: Payload,\n}\n",
        "JSON field name `value` occurs more than once: "
        "`Envelope.payload.value` conflicts with `Envelope.value`.",
        5,
        "duplicate JSON field `value` from `Envelope.payload.value`",
        "first JSON field `value` from `Envelope.value`",
        id="outer-inline",
    ),
    pytest.param(
        "record Payload { value: string }\n"
        "record Nested { #[json(inline)] payload: Payload }\n"
        "record Envelope {\n    #[json(inline)] first: Nested,\n"
        "    #[json(inline)] second: Nested,\n}\n",
        "JSON field name `value` occurs more than once: "
        "`Envelope.second.payload.value` conflicts with `Envelope.first.payload.value`.",
        5,
        "duplicate JSON field `value` from `Envelope.second.payload.value`",
        "first JSON field `value` from `Envelope.first.payload.value`",
        id="nested-siblings",
    ),
    pytest.param(
        "alias Alias<T>: T\nwrapper Wrapped<T>: T\n"
        "record Payload<T> { value: T }\n"
        "record Nested<T> { #[json(inline)] payload: Wrapped<Alias<Payload<T>>> }\n"
        "record Envelope {\n    value: string,\n"
        "    #[json(inline)] nested: Alias<Nested<string>>,\n}\n",
        "JSON field name `value` occurs more than once: "
        "`Envelope.nested.payload.value` conflicts with `Envelope.value`.",
        7,
        "duplicate JSON field `value` from `Envelope.nested.payload.value`",
        "first JSON field `value` from `Envelope.value`",
        id="generic",
    ),
    pytest.param(
        "record Payload {\n    tag: string,\n}\nvariant Choice { Case: Payload }\n",
        "JSON field name `tag` occurs more than once: "
        "`Choice::Case.tag` conflicts with `Choice::Case tag`.",
        2,
        "duplicate JSON field `tag` from `Choice::Case.tag`",
        "first JSON field `tag` from `Choice::Case tag`",
        id="internal-tag",
    ),
]


@pytest.mark.parametrize("command", ["check", "ir", "rust", "ts", "py", "json-schema"])
@pytest.mark.parametrize("source,message,line,label,other_label", CASES)
def test_json_field_diagnostic(
    sidex_bin, tmp_path, command, source, message, line, label, other_label
):
    """Reject invalid JSON fields at their source location for every CLI entry point."""
    (tmp_path / "schemas").mkdir()
    (tmp_path / "sidex.toml").write_text(
        '[bundle]\nname = "inline_check"\nversion = "0.0.0"\n'
    )
    (tmp_path / "schemas/data.sidex").write_text(source)
    args = (
        ["check"]
        if command == "check"
        else ["generate", command, str(tmp_path / "out")]
    )
    result = subprocess.run(
        [str(sidex_bin), *args],
        cwd=tmp_path,
        text=True,
        capture_output=True,
        check=False,
    )
    assert result.returncode == 1
    assert message in result.stderr
    assert f"data.sidex:{line}:" in result.stderr
    assert label in result.stderr
    if other_label is not None:
        assert other_label in result.stderr
    assert not (tmp_path / "out").exists()
