"""Check that JSON inline restrictions apply before checks and every generator."""

import subprocess

import pytest


@pytest.mark.parametrize("command", ["check", "ir", "rust", "ts", "py", "json-schema"])
def test_inline_type_diagnostic(sidex_bin, tmp_path, command):
    """Reject a nonrecord inline field at its source location for every CLI entry point."""
    (tmp_path / "schemas").mkdir()
    (tmp_path / "sidex.toml").write_text(
        '[bundle]\nname = "inline_check"\nversion = "0.0.0"\n'
    )
    (tmp_path / "schemas/data.sidex").write_text(
        "record Envelope {\n    #[json(inline)]\n    payload: string,\n}\n"
    )
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
    assert (
        "`#[json(inline)]` requires a record type, or an alias or wrapper of a record."
        in result.stderr
    )
    assert "data.sidex:3:" in result.stderr
    assert "not a record" in result.stderr
    assert not (tmp_path / "out").exists()
