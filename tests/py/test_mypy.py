"""Type-check generated packages together with their vendored runtime."""

import subprocess
import sys
from pathlib import Path

import pytest

REPO_ROOT = Path(__file__).resolve().parents[2]


@pytest.mark.parametrize(
    "bundle", ["tests/json", "tests/py/timestamp", "tests/py/opaque"]
)
def test_generated_package_passes_strict_mypy(sidex_bin, tmp_path, bundle):
    """Check records, variants, generics, inline fields, and built-ins without ignores."""
    output = tmp_path / "models"
    subprocess.run(
        [str(sidex_bin), "generate", "py", str(output)],
        cwd=REPO_ROOT / bundle,
        check=True,
        capture_output=True,
    )
    checked = subprocess.run(
        [sys.executable, "-m", "mypy", "--strict", str(output)],
        cwd=tmp_path,
        text=True,
        capture_output=True,
        check=False,
    )
    assert checked.returncode == 0, checked.stdout + checked.stderr
