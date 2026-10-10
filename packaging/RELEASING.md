# Releasing Sidex

The release workflow publishes `sidex-cli` to npm and PyPI and uploads native
archives to GitHub Releases. Both registry packages provide the commands
`sidex` and `sidex-cli`.

The CLI version in `crates/sidex-cli/Cargo.toml` is the release version.
Tags must match it exactly, for example `v0.2.7` for `0.2.7`.
Supported prereleases use `X.Y.Z-alpha.N`, `X.Y.Z-beta.N`, or `X.Y.Z-rc.N`.
npm prereleases use the `next` dist-tag; stable releases use `latest`.
Maturin converts prerelease versions to PyPI's version format.

## Packages and Platforms

Each release contains one npm launcher, six npm binary packages, six Python
wheels, and six standalone archives:

| Platform | npm binary package | Python wheel compatibility |
| --- | --- | --- |
| Linux x64, glibc | `@sidex/cli-linux-x64-glibc` | manylinux2014 |
| Linux ARM64, glibc | `@sidex/cli-linux-arm64-glibc` | manylinux2014 |
| Linux x64, musl | `@sidex/cli-linux-x64-musl` | musllinux 1.2 |
| Linux ARM64, musl | `@sidex/cli-linux-arm64-musl` | musllinux 1.2 |
| macOS Intel | `@sidex/cli-darwin-x64` | macOS 11 or newer |
| macOS Apple Silicon | `@sidex/cli-darwin-arm64` | macOS 11 or newer |

The npm launcher uses exact optional dependency versions. npm selects the
matching OS, CPU, and libc package. There is no install script or binary download
from GitHub. Python wheels contain the native executable and a small
`sidex-cli` alias script. Users need Node.js 20 or newer for npm, or Python 3.10
or newer for PyPI. Neither installation requires Rust.

## One-Time Setup

### GitHub

Create environments named `npm` and `pypi` in the `silitics/sidex` repository.
The publishing jobs reference these environments. Configure any release
approval or tag restrictions appropriate for the repository.

The jobs run on GitHub-hosted runners and obtain short-lived registry
credentials through OIDC. No npm or PyPI publishing secrets are needed.

### PyPI

Before the first release, add a pending GitHub publisher at
[PyPI's publishing settings](https://pypi.org/manage/account/publishing/):

| Field | Value |
| --- | --- |
| Project name | `sidex-cli` |
| Owner | `silitics` |
| Repository | `sidex` |
| Workflow | `release.yml` |
| Environment | `pypi` |

PyPI will create the project on the first successful OIDC publish. A pending
publisher does not reserve the package name. See
[PyPI's pending publisher documentation](https://docs.pypi.org/trusted-publishers/creating-a-project-through-oidc/).

### npm

npm requires an existing package before configuring its trusted publisher.
The first publish of all seven packages therefore requires an authenticated
maintainer with access to the `@sidex` organization. Subsequent releases use OIDC.

1. Run the Release workflow manually on the branch containing these changes,
   leave `tag` empty, and leave `publish` disabled. Wait for all checks and six
   distribution jobs to pass.
2. Download its `npm-*` artifacts into `dist/npm` in a checkout of that same
   commit:

   ```sh
   gh run download RUN_ID --repo silitics/sidex --pattern 'npm-*' --dir dist/npm
   find dist/npm -mindepth 2 -name '*.tgz' -exec mv {} dist/npm/ \\;
   ```

   The second command moves the tarballs out of the per-artifact directories.
3. With Node.js 24 and npm 11.19.0, log in and publish the tested tarballs:

   ```sh
   npm login
   python3 packaging/npm.py publish
   ```

   This publishes the six binary packages before the launcher. If an identical
   artifact already exists, the command skips it. Different contents for the
   same version cause an error.
4. For `sidex-cli` and **each** binary package in the table above, add a GitHub
   trusted publisher in the package settings:

   | Field | Value |
   | --- | --- |
   | Organization | `silitics` |
   | Repository | `sidex` |
   | Workflow | `release.yml` |
   | Environment | `npm` |
   | Allowed actions | Enable direct `npm publish` |

Configure the trusted publishers when the next release is ready. npm requires
the first successful OIDC publish within two days of creating each configuration.
The manually published version cannot be published again to validate OIDC;
use a new version for the first automated npm release.

See [npm's trusted publishing documentation](https://docs.npmjs.com/trusted-publishers/)
and [trusted publisher prerequisites](https://docs.npmjs.com/cli/v11/commands/npm-trust#prerequisites).

## Subsequent Releases

1. Update the CLI version and run `cargo check -p sidex-cli` to update
   `Cargo.lock`. Commit both files.
2. Let Run Checks pass on the release commit.
3. Tag that commit and push the tag:

   ```sh
   git tag v0.2.8
   git push origin v0.2.8
   ```

The example assumes the npm bootstrap published `0.2.7` and the CLI version is
now `0.2.8`.

Release runs the Rust, TypeScript, and Python checks, builds every distribution,
and tests the installed commands on every target. Only after all targets pass
do the npm and PyPI publishing jobs run. The GitHub release is created after
both registries succeed.

For a dry run, dispatch Release with `publish` disabled. Set `tag` to test an
existing release tag, or leave it empty to test the selected ref. A manual
publishing run requires an existing tag.

If publishing fails after some packages have been uploaded, rerun the failed
jobs in the original workflow run. npm verifies existing artifact integrity;
PyPI skips existing files. Reuse the original artifacts. Published versions
are immutable; rebuilding a partly published version can produce different
contents and prevent a safe retry.

## Local Verification

With Rust 1.99.0, Node.js 24, Python 3.12, and uv installed:

```sh
python3 packaging/npm.py prepare
SIDEX_GIT_VERSION=0.2.7 uvx maturin==1.15.0 build --release --out dist/python
python3 packaging/npm.py binary --target x86_64-unknown-linux-gnu --binary target/release/sidex
python3 packaging/test.py --target x86_64-unknown-linux-gnu --version 0.2.7
```

Adjust the version and target to the checkout and host. Keep only the current
target's wheel in `dist/python`. A local Linux build uses the host's glibc
compatibility tag. The release workflow builds in manylinux and musllinux
containers to produce portable PyPI wheels.
