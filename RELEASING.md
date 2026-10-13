# Releasing Sidex

The crates, the CLI packages, and the `@sidex` libraries share one version.

## Release a Version

1. Set the version in `Cargo.toml` (`[workspace.package]`) and in the
   `package.json` files of `languages/typescript/packages/types` and
   `languages/typescript/packages/validate`. Run `cargo metadata` to update
   `Cargo.lock`, and merge the change.
2. Tag the merged commit as `vX.Y.Z` and push the tag.

The release workflow fails if a version does not match the tag. It runs all
checks, builds the distributions, and publishes:

- `@sidex/types`, `@sidex/validate`, the CLI platform packages, and `sidex-cli`
  to npm,
- the `sidex-cli` wheels to PyPI,
- a GitHub release with the native archives.

npm and PyPI use trusted publishing through OIDC, so the workflow needs no
secrets. Publishing skips versions that are already on a registry, so a failed
release can be rerun. Versions like `X.Y.Z-rc.N` are published to npm's `next`
tag.
