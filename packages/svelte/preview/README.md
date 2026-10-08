# Poodle Svelte Preview

Browser preview and first docs-site baseline for inspecting emitted token
artifacts, package ownership, catalog coverage, theme overlays, density modes,
and accessibility-relevant control states.

## Run

From the repository root:

```sh
bun install
effigy tokens:build
effigy svelte:preview
```

`svelte:preview` rebuilds core and the Svelte package, then starts Vite.
`svelte:run` is the low-level Vite command and can serve stale ignored dist
output. Then open `http://localhost:4173`.

## Specimen Capture

Open a component without the preview shell or specimen tab strip:

```text
http://localhost:4173/?capture=specimen#components/button
```

Capture mode renders the Examples content in the shared 1280 × 900 logical
frame with 24 pixels of padding. A Playwright context should use the reference
device scale of 1. Once the specimen is ready, its frame exposes
`data-capture-ready="<slug>"` and `data-capture-device-scale="<value>"`; the
marker is set after `document.fonts.ready` and two animation frames. The shared
frame values and scale notes are in the
[preview specimen capture guide](../../../docs/guides/preview-specimen-capture.md).

The headless browser proof for Button and Keyboard capture mode runs with:

```sh
effigy test:svelte-specimen-capture
```

To validate the docs baseline before a publish candidate:

```sh
effigy docs:lint
effigy docs:check
```

## Scope

- inspect live token artifact output rather than hand-copied demo values
- exercise theme, density, and control-size overlays
- exercise scoped appearance Recipe overrides without redefining token meaning
- provide the first catalog-style docs and examples surface while the larger
  docs-site program remains early
- make package and contract ownership visible alongside the live examples
- keep preview state URL-addressable by section, theme, density, and control
  size so review notes can point at stable surfaces
- act as the source for the generated parity route/report baseline

## Parity Report

Regenerate the current parity evidence artifact from the repository root:

```sh
bun run parity:report
```

This writes:

- `packages/svelte/preview/artifacts/parity-report.json`

The parity artifact now also records which public exports from
`@inflatable-cookie/poodle-svelte` exports are directly covered by preview sections versus still
being contract-only, and it now includes a cross-runtime summary sourced from:

- `packages/gpui/cross-runtime-parity-report.json`

That GPUI artifact carries the current side-by-side section set, intentional
delta register, and GPUI acceptance-harness alignment.

## Remaining Harness Debt

- preview sections are still implementation-heavy and could use more intentional
  grouping inside the larger composite and workstation pages
- the surface is still a single-app preview, not a published docs system with
  search, permalinks, or generated contract pages
- mounted GPUI implementation parity is still documented more strongly than it
  is demonstrated in a runnable harness
- the current parity report now joins Svelte and GPUI evidence more honestly,
  but it is still not a screenshot-regression or mounted GPUI interaction
  harness
- many public exports are now classified explicitly as contract-only because
  the preview still reviews them through broader suite sections rather than one
  direct specimen per export
- the current publish candidate is an internal static preview build, not yet a
  public docs-site deployment with versioned hosting or external release notes

Preview examples demonstrate component states and composition. Treat component
contracts as normative; preview-only glue is not a public application API.
