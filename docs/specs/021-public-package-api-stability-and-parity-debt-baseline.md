# 021 Public Package API Stability And Parity Debt Baseline

Status: active
Updated: 2026-03-11
Depends on: `002-component-contract-template-and-parity-rules.md`, `008-parity-evidence-documented-delta-and-downstream-extension-rules.md`, `020-docs-site-example-and-component-discoverability-rules.md`

## Purpose

Freeze the package-surface baseline that must exist before downstream repos are
allowed to depend on Pug directly. This spec defines what counts as a public
package entry point, what remains internal, and which current parity gaps still
block adoption.

## Public Package Rule

Only these package entry points count as intentionally public in the current
baseline:

- `@pug/svelte-tokens`
- `@pug/svelte-tokens/runtime`
- `@pug/svelte-tokens/css`
- `@pug/svelte-tokens/themes`
- `@pug/svelte-tokens/metadata`
- `@pug/svelte-primitives`
- `@pug/svelte-primitives/types`
- `@pug/svelte-composites`
- `@pug/svelte-composites/types`
- `@pug/svelte-workstation`
- `@pug/svelte-workstation/types`
- `pug-gpui-tokens`

Everything else in `packages/` should be treated as internal until a later
milestone explicitly promotes it.

## Entry-Point Rule

Public consumers may rely on:

- documented root exports
- documented `./types` or equivalent type-only subpaths
- generated token or metadata entry points that are named in package exports

Public consumers may not rely on:

- deep imports into `src/`
- preview-only files
- artifact paths outside the exported token package surface
- internal helper modules that were never named in package exports

## Ownership Rule

Package ergonomics must preserve ownership boundaries:

- token artifacts remain canonical and generated
- Svelte packages expose reusable component and type entry points
- preview code remains a consumer, not a hidden source of public API truth
- GPUI token bindings do not imply GPUI component parity
- Underlay bridge code does not become a public API surrogate for Pug packages

## Current Stability Posture

The current package surfaces fall into three buckets:

### Stable Enough To Package Soon

- token artifacts and token helper entry points
- Svelte root exports for implemented primitives, composites, and workstation shells
- explicit type-only subpaths for Svelte packages

### Usable Internally But Not Yet Adoption-Ready

- component prop naming and event naming consistency across the Svelte package family
- preview-driven examples that still shape perception of API quality
- workstation orchestration helpers that need clearer stability language

### Not Yet Adoption-Ready

- any GPUI package surface beyond generated tokens
- any claim of Svelte/GPUI parity beyond documented contract intent and the token layer
- any assumption that Underlay adoption can begin before package and parity cleanup is complete

## Parity Debt Register

The current adoption-blocking parity debt is explicit:

### GPUI Implementation Debt

- GPUI tokens exist, but GPUI primitives, composites, and workstation shells do not yet exist as comparable public packages
- no runnable GPUI parity harness exists for the surfaces already implemented in Svelte
- no component-level GPUI evidence exists for the advanced `g02` catalogue

### API Consistency Debt

- component package entry points were implicit rather than intentionally bounded before this tranche
- downstream consumers still need stronger guidance on what is stable, internal, or preview-only
- package documentation was too sparse to gate adoption cleanly

### Downstream Readiness Debt

- the preview/docs surface is now usable, but it is still not a substitute for release policy
- adoption sequencing remains blocked until package ergonomics and parity debt are both explicit
- downstream repos should not import Pug beyond internal experimentation until `g02.015` packaging rules exist

## Adoption Gate Rule

Downstream adoption should remain blocked until all of the following are true:

- public package entry points are explicitly documented
- package ergonomics and naming debt are explicitly reviewed
- parity debt is recorded with no implied completeness
- release and versioning posture is documented

## Seed Evidence

- `packages/svelte/tokens/package.json`
- `packages/svelte/primitives/package.json`
- `packages/svelte/composites/package.json`
- `packages/svelte/workstation/package.json`
- `packages/gpui/tokens/Cargo.toml`
- `packages/svelte/tokens/README.md`
- `packages/svelte/primitives/README.md`
- `packages/svelte/composites/README.md`
- `packages/svelte/workstation/README.md`
- `packages/gpui/tokens/README.md`

## Next Task

Use this baseline while executing `g02.015`, turning the newly explicit package
surface into a concrete packaging, versioning, and release posture.
