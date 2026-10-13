"""Package and publish the Sidex crates.

`package` checks that every publishable crate builds from its package alone.
`publish` publishes the crates whose version is not on crates.io yet, in
dependency order, so that retrying a partially failed release is safe. Cargo
reads the token from `CARGO_REGISTRY_TOKEN`.
"""

import argparse
import json
import subprocess
import urllib.error
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
INDEX = "https://index.crates.io"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    commands.add_parser("package")
    commands.add_parser("publish")
    args = parser.parse_args()

    metadata = json.loads(
        subprocess.run(
            ["cargo", "metadata", "--format-version", "1", "--no-deps", "--locked"],
            cwd=ROOT,
            capture_output=True,
            text=True,
            check=True,
        ).stdout
    )
    members = set(metadata["workspace_members"])
    packages = [package for package in metadata["packages"] if package["id"] in members]
    # Cargo packages crates with `publish = false` unless they are excluded.
    excluded = [package["name"] for package in packages if package["publish"] == []]

    if args.command == "package":
        cargo("package", excluded)
        return

    for package in packages:
        name, version = package["name"], package["version"]
        if name not in excluded and is_published(name, version):
            print(f"Already published: {name}@{version}", flush=True)
            excluded.append(name)
    if len(excluded) == len(packages):
        print("All crates are published.")
        return
    cargo("publish", excluded)


def cargo(command, excluded):
    """Run a Cargo command for all workspace crates except the excluded ones."""
    arguments = ["cargo", command, "--workspace", "--locked"]
    for name in excluded:
        arguments += ["--exclude", name]
    subprocess.run(arguments, cwd=ROOT, check=True)


def is_published(name, version):
    """Indicate whether crates.io has the version, according to its sparse index."""
    name = name.lower()
    if len(name) <= 2:
        path = f"{len(name)}/{name}"
    elif len(name) == 3:
        path = f"3/{name[0]}/{name}"
    else:
        path = f"{name[:2]}/{name[2:4]}/{name}"
    request = urllib.request.Request(
        f"{INDEX}/{path}",
        headers={"User-Agent": "sidex-release (https://github.com/silitics/sidex)"},
    )
    try:
        with urllib.request.urlopen(request) as response:
            lines = response.read().decode().splitlines()
    except urllib.error.HTTPError as error:
        if error.code == 404:
            return False
        raise
    return any(json.loads(line)["vers"] == version for line in lines if line)


if __name__ == "__main__":
    main()
