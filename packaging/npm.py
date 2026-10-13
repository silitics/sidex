"""Build npm distributions and binary archives from Sidex release artifacts.

Besides the CLI, the npm distributions include the `@sidex` libraries. All of
them share the version of the Cargo workspace.
"""

import argparse
import base64
import hashlib
import json
import os
import re
import shutil
import subprocess
import tarfile
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DIST = ROOT / "dist"
LIBRARIES = ROOT / "languages/typescript/packages"
LIBRARY_NAMES = ["types", "validate"]
PLATFORMS = [
    ("x86_64-unknown-linux-gnu", "ubuntu-24.04", "linux", "x64", "glibc"),
    ("aarch64-unknown-linux-gnu", "ubuntu-24.04-arm", "linux", "arm64", "glibc"),
    ("x86_64-unknown-linux-musl", "ubuntu-24.04", "linux", "x64", "musl"),
    ("aarch64-unknown-linux-musl", "ubuntu-24.04-arm", "linux", "arm64", "musl"),
    ("x86_64-apple-darwin", "macos-15-intel", "darwin", "x64", None),
    ("aarch64-apple-darwin", "macos-15", "darwin", "arm64", None),
]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    prepare = commands.add_parser("prepare")
    prepare.add_argument("--tag", default="")
    commands.add_parser("publish")
    binary = commands.add_parser("binary")
    binary.add_argument(
        "--target", required=True, choices=[row[0] for row in PLATFORMS]
    )
    binary.add_argument("--binary", required=True, type=Path)
    args = parser.parse_args()

    with (ROOT / "Cargo.toml").open("rb") as manifest:
        version = tomllib.load(manifest)["workspace"]["package"]["version"]
    if not re.fullmatch(
        r"(?:0|[1-9]\d*)\.(?:0|[1-9]\d*)\.(?:0|[1-9]\d*)"
        r"(?:-(?:alpha|beta|rc)\.(?:0|[1-9]\d*))?",
        version,
    ):
        parser.error("Release versions must be X.Y.Z or X.Y.Z-{alpha,beta,rc}.N")
    if args.command == "prepare":
        if args.tag and args.tag != f"v{version}":
            parser.error(f"Tag {args.tag!r} does not match version v{version}")
        for name in LIBRARY_NAMES:
            manifest = json.loads((LIBRARIES / name / "package.json").read_text())
            if manifest["version"] != version:
                parser.error(
                    f"@sidex/{name} has version {manifest['version']}, not {version}"
                )
        prepare_libraries()
        prepare_launcher(version)
        outputs = {
            "version": version,
            "matrix": json.dumps({"include": [platform_row(row) for row in PLATFORMS]}),
        }
        if output_file := os.environ.get("GITHUB_OUTPUT"):
            with open(output_file, "a") as output:
                output.writelines(f"{key}={value}\n" for key, value in outputs.items())
        print(f"Prepared the npm distributions of Sidex {version}")
    elif args.command == "publish":
        publish(version)
    else:
        row = next(row for row in PLATFORMS if row[0] == args.target)
        package = platform_row(row)
        directory = package_directory(package["platform"])
        native_binary = directory / "bin/sidex"
        native_binary.parent.mkdir()
        shutil.copy2(args.binary, native_binary)
        native_binary.chmod(0o755)
        manifest = package_metadata(version)
        manifest.update(
            name=f"@sidex/cli-{package['platform']}",
            os=[package["os"]],
            cpu=[package["cpu"]],
            files=["bin/sidex", "LICENSE-MIT", "LICENSE-APACHE"],
        )
        if package["libc"]:
            manifest["libc"] = [package["libc"]]
        write_manifest(directory, manifest)
        pack(directory)
        archives = DIST / "archives"
        archives.mkdir(parents=True, exist_ok=True)
        with tarfile.open(archives / f"sidex-{args.target}.tar.gz", "w:gz") as archive:
            archive.add(native_binary, arcname="sidex")


def prepare_libraries():
    """Test, build, and pack the `@sidex` libraries."""
    destination = DIST / "npm"
    destination.mkdir(parents=True, exist_ok=True)
    for name in LIBRARY_NAMES:
        directory = LIBRARIES / name
        for script in ("test", "clean"):
            subprocess.run(["npm", "run", script], cwd=directory, check=True)
        # Packing builds the library with its `prepare` script.
        subprocess.run(
            ["npm", "pack", "--pack-destination", str(destination)],
            cwd=directory,
            check=True,
        )


def platform_row(row):
    """Describe one release target for npm metadata and the Actions matrix."""
    target, runner, system, cpu, libc = row
    platform = f"{system}-{cpu}" + (f"-{libc}" if libc else "")
    return {
        "target": target,
        "runner": runner,
        "platform": platform,
        "os": system,
        "cpu": cpu,
        "libc": libc,
        "manylinux": "2014" if libc == "glibc" else "musllinux_1_2" if libc else "auto",
        "container": (
            f"quay.io/pypa/{'manylinux2014' if libc == 'glibc' else 'musllinux_1_2'}_"
            f"{'x86_64' if cpu == 'x64' else 'aarch64'}:latest"
            if libc
            else ""
        ),
    }


def prepare_launcher(version):
    """Pack a launcher with exact dependencies on every supported binary."""
    directory = package_directory("sidex-cli")
    shutil.copy2(ROOT / "packaging/npm/cli.cjs", directory / "cli.cjs")
    manifest = package_metadata(version)
    manifest.update(
        name="sidex-cli",
        bin={"sidex": "cli.cjs", "sidex-cli": "cli.cjs"},
        files=["cli.cjs", "LICENSE-MIT", "LICENSE-APACHE"],
        engines={"node": ">=20"},
        optionalDependencies={
            f"@sidex/cli-{platform_row(row)['platform']}": version for row in PLATFORMS
        },
    )
    write_manifest(directory, manifest)
    pack(directory)


def package_directory(name):
    """Create a fresh staging directory with package documentation and licenses."""
    directory = DIST / "staging" / name
    if directory.exists():
        shutil.rmtree(directory)
    directory.mkdir(parents=True)
    for filename in ("LICENSE-MIT", "LICENSE-APACHE"):
        shutil.copy2(ROOT / filename, directory / filename)
    shutil.copy2(ROOT / "packaging/README.md", directory / "README.md")
    return directory


def package_metadata(version):
    """Return metadata shared by all npm distributions."""
    return {
        "version": version,
        "description": "The Sidex data modeling and code generation tool.",
        "license": "MIT OR Apache-2.0",
        "author": "Silitics GmbH <info@silitics.com>",
        "homepage": "https://oss.silitics.com/sidex",
        "repository": {
            "type": "git",
            "url": "git+https://github.com/silitics/sidex.git",
        },
        "publishConfig": {"access": "public"},
    }


def write_manifest(directory, manifest):
    (directory / "package.json").write_text(json.dumps(manifest, indent=2) + "\n")


def pack(directory):
    """Use npm's own packer to retain standard package and executable handling."""
    destination = DIST / "npm"
    destination.mkdir(parents=True, exist_ok=True)
    subprocess.run(
        ["npm", "pack", "--ignore-scripts", "--pack-destination", str(destination)],
        cwd=directory,
        check=True,
    )


def publish(version):
    """Publish exact artifacts in dependency order, allowing safe job retries."""
    names = [f"@sidex/{name}" for name in LIBRARY_NAMES]
    names += [f"@sidex/cli-{platform_row(row)['platform']}" for row in PLATFORMS]
    names.append("sidex-cli")
    packages = []
    for name in names:
        filename = name.removeprefix("@").replace("/", "-")
        package = DIST / "npm" / f"{filename}-{version}.tgz"
        with tarfile.open(package) as archive:
            manifest = json.load(archive.extractfile("package/package.json"))
        if (manifest["name"], manifest["version"]) != (name, version):
            raise ValueError(f"Unexpected package metadata in {package}")
        packages.append((name, package))

    for name, package in packages:
        existing = subprocess.run(
            ["npm", "view", f"{name}@{version}", "dist.integrity", "--json"],
            capture_output=True,
            text=True,
            check=False,
        )
        if existing.returncode == 0:
            integrity = (
                "sha512-"
                + base64.b64encode(
                    hashlib.sha512(package.read_bytes()).digest()
                ).decode()
            )
            if json.loads(existing.stdout) != integrity:
                raise ValueError(
                    f"{name}@{version} already exists with different contents"
                )
            print(f"Already published identical artifact: {name}@{version}", flush=True)
            continue
        error = json.loads(existing.stdout).get("error", {})
        if error.get("code") != "E404":
            raise RuntimeError(existing.stderr)
        subprocess.run(
            [
                "npm",
                "publish",
                str(package),
                "--access",
                "public",
                "--tag",
                "next" if "-" in version else "latest",
            ],
            check=True,
        )


if __name__ == "__main__":
    main()
