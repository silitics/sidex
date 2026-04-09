"""Pytest infrastructure for testing Sidex-generated Python code."""

import importlib
import subprocess
import sys
import tempfile
from pathlib import Path

import pytest

REPO_ROOT = Path(__file__).resolve().parent.parent.parent


def _build_sidex():
    """Build the sidex CLI binary and return its path."""
    subprocess.run(
        ["cargo", "build", "--bin", "sidex"],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
    )
    return REPO_ROOT / "target" / "debug" / "sidex"


def _generate(sidex_bin: Path, bundle_dir: Path, output_dir: Path):
    """Run `sidex generate py` for a bundle."""
    subprocess.run(
        [str(sidex_bin), "generate", "py", str(output_dir)],
        cwd=bundle_dir,
        check=True,
        capture_output=True,
    )


def _import_package(name: str, path: Path):
    """Import a generated package by wiring it into sys.modules."""
    spec = importlib.util.spec_from_file_location(
        name,
        str(path / "__init__.py"),
        submodule_search_locations=[str(path)],
    )
    pkg = importlib.util.module_from_spec(spec)
    sys.modules[name] = pkg
    spec.loader.exec_module(pkg)
    return pkg


@pytest.fixture(scope="session")
def sidex_bin():
    """Build the sidex CLI once per test session."""
    return _build_sidex()


@pytest.fixture(scope="session")
def generate(sidex_bin):
    """Factory fixture: generate Python code for a Sidex bundle.

    Returns a function ``(bundle_dir, package_name) -> module`` that generates
    Python code into a temporary directory, imports it, and returns the
    top-level package module.
    """
    # Keep temp dirs alive for the whole session.
    tmp_dirs: list[tempfile.TemporaryDirectory] = []

    def _run(bundle_dir: Path, package_name: str):
        tmp = tempfile.TemporaryDirectory(prefix=f"sidex_py_{package_name}_")
        tmp_dirs.append(tmp)
        out = Path(tmp.name) / package_name
        out.mkdir()
        _generate(sidex_bin, bundle_dir, out)
        return _import_package(package_name, out)

    yield _run

    for tmp in tmp_dirs:
        tmp.cleanup()
