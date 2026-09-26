# 021 Public Package API Stability And Parity Debt Baseline

Status: active
Updated: 2026-03-12
Depends on: `002-component-contract-template-and-parity-rules.md`, `../008-parity-evidence-documented-delta-and-downstream-extension-rules.md`, `020-docs-site-example-and-component-discoverability-rules.md`

## Purpose

Freeze the package-surface baseline that must exist before downstream repos are
allowed to depend on Poodle directly. This spec defines what counts as a public
package entry point, what remains internal, and which current parity gaps still
block adoption.

## Public Package Rule

Only these package entry points count as intentionally public in the current
baseline:

- `@inflatable-cookie/poodle-core/tokens`
- `@inflatable-cookie/poodle-core/tokens/runtime`
- `@inflatable-cookie/poodle-core/tokens/css`
- `@inflatable-cookie/poodle-core/tokens/themes`
- `@inflatable-cookie/poodle-core/tokens/metadata`
- `@inflatable-cookie/poodle-svelte`
- `@inflatable-cookie/poodle-svelte/types`
- `@inflatable-cookie/poodle-svelte-workstation`
- `@inflatable-cookie/poodle-svelte-workstation/types`
- `poodle-gpui-tokens`
- `poodle-gpui-primitives`
- `poodle-gpui-composites`
- `poodle-gpui-workstation`

Everything else in `packages/` should be treated as internal until a later
milestone explicitly promotes it.

## Entry-Point Rule

Public consumers may rely on:

- documented root exports
- documented `./types` or equivalent type-only subpaths
- generated token or metadata entry points that are named in package exports
- package-surface inventories that remain contract-backed and linted

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
- Underlay bridge code does not become a public API surrogate for Poodle packages

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

- any GPUI package surface beyond generated tokens, the current widened primitive baseline, the widened GPUI composite baseline, and the first GPUI workstation baseline
- any claim of Svelte/GPUI parity beyond documented contract intent, the token layer, and the current GPUI primitive contract substrate
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
- package documentation and contract mapping were too sparse to gate adoption cleanly

## Package Surface Integrity Rule

Public Svelte package surfaces must stay machine-checkable against their
documented contracts and README inventories.

At minimum, the current baseline should prove:

- exported public components map to real contracts in the owning layer
- public README surface inventories match the actual export surface
- root and type-only entry points named to downstream adopters remain present

This keeps package ergonomics, contracts, and public docs from drifting apart
as the surface grows.

### Downstream Readiness Debt

- the preview/docs surface is now usable, but it is still not a substitute for release policy
- adoption sequencing remains blocked until package ergonomics and parity debt are both explicit
- downstream repos should not import Poodle beyond internal experimentation until `g02.015` packaging rules exist

## Adoption Gate Rule

Downstream adoption should remain blocked until all of the following are true:

- public package entry points are explicitly documented
- package ergonomics and naming debt are explicitly reviewed
- parity debt is recorded with no implied completeness
- release and versioning posture is documented

## Seed Evidence

- `packages/svelte/tokens/package.json`
- `packages/svelte/components/package.json`
- `packages/svelte/components/package.json`
- `packages/svelte/workstation/package.json`
- `packages/gpui/tokens/Cargo.toml`
- `packages/gpui/primitives/Cargo.toml`
- `packages/gpui/composites/Cargo.toml`
- `packages/gpui/workstation/Cargo.toml`
- `packages/svelte/tokens/README.md`
- `packages/svelte/components/README.md`
- `packages/svelte/components/README.md`
- `packages/svelte/workstation/README.md`
- `packages/gpui/tokens/README.md`
- `packages/gpui/primitives/README.md`
- `packages/gpui/composites/README.md`
- `packages/gpui/workstation/README.md`
