# Sidex Documentation

The documentation uses Astro and Silitics' shared `@silitics/astro-docs` and
`@silitics/astro-theme` packages, following Rugix's documentation site.

## Development

From this directory:

```sh
mise install
mise exec -- pnpm install
mise run dev
```

Open `http://localhost:4323/sidex/`. Development updates pages as you edit them.
Full-text search requires the static build.

## Build and Preview

```sh
mise run build
mise exec -- pnpm preview
```

The build writes static files and a Pagefind search index to `dist`. Serve its
contents at `/sidex/`, matching `base` in `astro.config.mjs`. To expose a pod
preview, run `podctl http publish --title "Sidex Docs" 4323`.

CI builds the site on PRs. A separate deployment job publishes the checked build
from pushes to `main` at `https://oss.silitics.com/sidex/`. In the GitHub
`docs` environment, configure the variable `SILITICS_SITES_ACCESS_KEY` and secret
`SILITICS_SITES_SECRET_KEY` with a publishing key for the `silitics-public-sites`
bucket. PR builds do not use this environment. The workflow archives the build
under `sidex/`; the domain is served by Core1's static-site configuration.

## Checks

```sh
mise exec -- pnpm check
mise run build
mise exec -- pnpm check:links
```

The link check uses Nix to run Lychee against the built site, including local
anchors. CI checks the Astro types and production build.

From the repository root, test the guide examples against the current compiler
and runtimes:

```sh
nix develop --command mise exec -- uv run --no-project --with pydantic --with jsonschema python www/scripts/check-examples.py
```

This check compiles the CLI and Rust and TypeScript examples, runs Python
round trips, validates generated JSON Schema and samples, and checks typed
attribute metadata. It requires the website dependencies to be installed.

## Content and Navigation

Write pages in `src/content/docs`. Set each page's title, description, and order
in its frontmatter. Navigation groups live in `src/site.ts`. The homepage and
local styles use the shared theme's components and tokens.

Sidex code blocks use the editor grammar. The downloadable task bundle lives in
`public/examples/tasks`; keep it identical to the getting started example.
The example check verifies this.

Search comes from the shared docs template. Its `docsSearch()` Astro integration
indexes every documentation page during the build, including headings and code
examples. Open search from the main header with Ctrl/Cmd+K or `/`, use the
arrow keys to select a result, and press Enter to open its matching section.
