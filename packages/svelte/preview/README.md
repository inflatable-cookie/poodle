# Pug Svelte Preview

Browser preview and first docs-site baseline for inspecting emitted token
artifacts, package ownership, catalog coverage, theme overlays, density modes,
and accessibility-relevant control states.

## Run

From the repository root:

```sh
bun install
bun run tokens:build
bun run docs:dev
```

Then open `http://localhost:4173`.

## Scope

- inspect live token artifact output rather than hand-copied demo values
- exercise theme, density, and control-size overlays
- provide the first catalog-style docs and examples surface while the larger
  docs-site program remains early
- make package and contract ownership visible alongside the live examples

## Remaining Harness Debt

- preview sections are still implementation-heavy and could use more intentional
  grouping inside the larger composite and workstation pages
- the surface is still a single-app preview, not a published docs system with
  search, permalinks, or generated contract pages
- GPUI implementation parity is still documented more strongly than it is
  demonstrated in a runnable harness
- packaging, versioning, and release policy still need to be defined before
  downstream repos should depend on the current package surfaces
- downstream adoption remains intentionally deferred until the release baseline
  tranche lands

## Next Task

Use this docs/preview surface to support `g03.001`, validating any migration or
compatibility policy against the real internal review surface.
