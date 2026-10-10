"""Exercise installed npm and Python distributions through their public commands."""

import argparse
import os
import subprocess
import sys
import tempfile
import venv
from pathlib import Path

from npm import DIST, PLATFORMS, platform_row


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--target", required=True, choices=[row[0] for row in PLATFORMS]
    )
    parser.add_argument("--version", required=True)
    args = parser.parse_args()
    platform = platform_row(next(row for row in PLATFORMS if row[0] == args.target))
    launcher = DIST / "npm" / f"sidex-cli-{args.version}.tgz"
    native = DIST / "npm" / f"sidex-cli-{platform['platform']}-{args.version}.tgz"
    wheels = list((DIST / "python").glob("*.whl"))
    if len(wheels) != 1:
        parser.error(f"Expected one wheel for this target, found {len(wheels)}")

    with tempfile.TemporaryDirectory(prefix="sidex-packaging-") as temporary:
        directory = Path(temporary)
        python_env = directory / "python"
        venv.EnvBuilder(with_pip=True).create(python_env)
        subprocess.run(
            [
                str(python_env / "bin/python"),
                "-m",
                "pip",
                "install",
                "--no-index",
                str(wheels[0]),
            ],
            check=True,
        )
        smoke([str(python_env / "bin/sidex")], args.version)
        smoke([str(python_env / "bin/sidex-cli")], args.version)
        smoke(
            [
                "uvx",
                "--isolated",
                "--no-index",
                "--find-links",
                str(DIST / "python"),
                f"sidex-cli@{args.version}",
            ],
            args.version,
        )

        pipx_bin = directory / "pipx-bin"
        pipx_env = {
            **os.environ,
            "PIPX_HOME": str(directory / "pipx"),
            "PIPX_BIN_DIR": str(pipx_bin),
        }
        subprocess.run(
            [
                "uvx",
                "--from",
                "pipx==1.8.0",
                "pipx",
                "install",
                "--python",
                sys.executable,
                "--pip-args=--no-index",
                str(wheels[0]),
            ],
            env=pipx_env,
            check=True,
        )
        smoke([str(pipx_bin / "sidex-cli")], args.version)

        npm_env = directory / "npm"
        subprocess.run(
            [
                "npm",
                "install",
                "--prefix",
                str(npm_env),
                "--offline",
                "--ignore-scripts",
                "--no-audit",
                "--no-fund",
                str(launcher),
                str(native),
            ],
            check=True,
        )
        smoke([str(npm_env / "node_modules/.bin/sidex")], args.version)
        smoke([str(npm_env / "node_modules/.bin/sidex-cli")], args.version)
        smoke(
            ["npx", "--prefix", str(npm_env), "--offline", "--no", "--", "sidex-cli"],
            args.version,
        )

        missing_env = directory / "missing-optional"
        subprocess.run(
            [
                "npm",
                "install",
                "--prefix",
                str(missing_env),
                "--offline",
                "--ignore-scripts",
                "--omit=optional",
                "--no-audit",
                "--no-fund",
                str(launcher),
            ],
            check=True,
        )
        missing = subprocess.run(
            [str(missing_env / "node_modules/.bin/sidex-cli"), "--version"],
            capture_output=True,
            text=True,
            check=False,
        )
        assert missing.returncode == 1 and "optional dependencies" in missing.stderr, (
            missing
        )
    print(f"Installed distributions passed for {args.target}")


def smoke(command, version):
    """Check aliases, argument forwarding, exit codes, and embedded code generators."""
    with tempfile.TemporaryDirectory(prefix="sidex-smoke-") as temporary:
        directory = Path(temporary)
        result = subprocess.run(
            [*command, "--version"], check=True, capture_output=True, text=True
        )
        assert result.stdout.strip() == f"sidex {version}", result.stdout
        subprocess.run([*command, "new", "bundle"], cwd=directory, check=True)
        bundle = directory / "bundle"
        (bundle / "schemas/person.sidex").write_text(
            "record Person { name: string, age?: u32 }\n"
        )
        subprocess.run([*command, "check"], cwd=bundle, check=True)
        for backend in ("rust", "ts", "py", "json-schema"):
            output = directory / backend
            subprocess.run(
                [*command, "generate", backend, str(output)], cwd=bundle, check=True
            )
            assert any(path.is_file() for path in output.rglob("*")), (
                f"No {backend} output produced"
            )
        invalid = subprocess.run(
            [*command, "--sidex-invalid-argument"], capture_output=True, check=False
        )
        assert invalid.returncode == 2, invalid


if __name__ == "__main__":
    main()
