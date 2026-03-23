# 027 Docs Completeness, Contract Linting, And Publish Pipeline

Status: active
Updated: 2026-03-12
Depends on: `020-docs-site-example-and-component-discoverability-rules.md`, `024-token-evolution-migration-and-compatibility-policy.md`, `025-parity-automation-and-harness-boundary.md`, `026-appearance-recipes-and-downstream-override-strategy.md`

## Purpose

Freeze what “docs complete enough to trust” means for Flint right now, what
contract files must satisfy structurally before they count as source of truth,
and what the repo may honestly call a publishable docs artifact while the
larger docs-site program is still preview-driven.

## Docs Completeness Rule

The current docs baseline is complete enough for repeat review only when all of
these remain true at once:

- every preview docs section has explicit title, package ownership, contract
  root, summary, and example-type coverage
- every docs family resolves only to known docs sections
- every parity target resolves to a known docs section with stable review routes
- every contract root referenced by the preview catalog or parity registry
  resolves to a real repo path
- the top-level contract index and layer indexes still enumerate the files they
  claim to expose
- public primitive package exports still map one-to-one to foundation contracts
  and the package README inventory

Completeness is therefore structural and cross-linked. It is not enough for a
markdown file or preview panel to exist in isolation.

## Contract Lint Rule

Component contracts under `docs/contracts/foundation/`,
`docs/contracts/composites/`, and `docs/contracts/workstation/` must satisfy a
shared lintable structure appropriate to the current seed-contract maturity.

The current required baseline is:

- title line
- `Status:` metadata
- `Updated:` metadata
- `## 1. Purpose`
- component identity and layer bullets inside the purpose section
- an accessibility section
- an explicit next-task section
- contiguous numbered headings starting at `1`

The full component-contract template remains the target end-state, but
`g03.003` does not require every older seed contract to backfill the entire
template immediately. The current linter rejects silent structural drift while
allowing compact seed contracts to coexist with richer template-aligned ones.

## Catalog And Parity Integrity Rule

Docs completeness depends on the preview catalog and parity registry staying in
sync.

The current lint baseline must prove:

- every docs section participates in parity registration
- every parity target carries automated checks
- every visually reviewed target carries manual review notes
- every parity target exposes at least one stable review route
- duplicate review URLs are rejected

This rule extends the `g03.002` parity baseline into a broader docs gate rather
than treating parity evidence as a disconnected side artifact.

## Index Completeness Rule

Contract indexes are part of the docs completeness surface.

The following files must stay aligned with the actual contract tree:

- `docs/contracts/README.md`
- `docs/contracts/foundation/README.md`
- `docs/contracts/composites/README.md`
- `docs/contracts/workstation/README.md`

If a contract file exists but is omitted from its layer index, or a layer index
claims a contract that does not exist, the docs baseline is incomplete.

## Public Surface Completeness Rule

The public Svelte package surfaces are also part of docs completeness.

The current lint baseline must prove:

- every default component export from the public Svelte packages maps to a real
  contract in its owning layer
- the package README public-surface inventory matches the actual exported root
  surface, including non-component helpers when they are intentionally public
- every exported public Svelte component-package entry point has an explicit
  preview-coverage posture recorded as either previewed or contract-only
- each README still names the root import and type-only import entry points

This keeps package surfaces, contracts, and public package docs from drifting
independently as the Svelte layer grows.

## Publish Pipeline Rule

Flint does not yet claim a public multi-version docs website.

The current publishable artifact is narrower and internal:

- generated parity evidence
- a validated static preview build under `packages/svelte/preview/dist`

The required publish-candidate pipeline is:

- `bun run docs:lint`
- `bun run parity:report`
- `bun run docs:build`

If token artifacts or theme files changed, `bun run tokens:build` must also run
before the docs build.

## Honesty Rule

The repo may say:

- docs/catalog coverage is linted
- contract structure is linted
- parity registration is validated
- a static preview build artifact exists

The repo may not say:

- a public docs publishing platform is complete
- all contract semantics are quality-approved just because structure lint passes
- GPUI parity is proven by docs completeness checks

## Current `g03.003` Baseline

The current baseline is implemented through:

- `packages/svelte/preview/scripts/lint-docs.ts`
- root-level `docs:lint` and `docs:check` scripts in `package.json`
- preview-level `docs:lint` script in `packages/svelte/preview/package.json`
- the existing parity evidence artifact and preview build commands

## Evidence

- `packages/svelte/preview/src/catalog.ts`
- `packages/svelte/preview/src/parity.ts`
- `packages/svelte/preview/scripts/build-parity-report.ts`
- `packages/svelte/preview/scripts/lint-docs.ts`
- `docs/contracts/README.md`
- `packages/svelte/primitives/src/index.ts`
- `packages/svelte/primitives/README.md`
- `packages/svelte/composites/src/index.ts`
- `packages/svelte/composites/README.md`
- `packages/svelte/workstation/src/index.ts`
- `packages/svelte/workstation/README.md`
- `packages/svelte/preview/README.md`

## Next Task

Use this baseline while executing `g03.004`, so performance hardening does not
reopen docs integrity, contract indexing, or publish-candidate expectations.
