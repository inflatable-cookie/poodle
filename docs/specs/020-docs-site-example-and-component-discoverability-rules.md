# 020 Docs Site Example And Component Discoverability Rules

Status: active
Updated: 2026-03-11
Depends on: `019-advanced-catalog-accessibility-focus-keyboard-and-state-rules.md`

## Purpose

Freeze the first serious docs-site and examples baseline so Flint can present one
adoption-facing surface that connects contracts, implementations, tokens, and
stateful examples.

## Docs Surface Rule

The first docs-site baseline may be implemented on top of the preview surface.

What matters is not whether the first version is a fully separate website.
What matters is that adopters have one explicit browser surface that presents:

- example navigation
- package ownership
- contract ownership
- token provenance
- stateful behavior

## Information Architecture Rule

The adoption-facing surface must provide:

- a top-level catalog or hub view
- navigation between major example families
- grouping by the same layers used in the repo and docs
- section-level metadata tying examples back to package and contract roots

The initial layer set is:

- tokens and artifacts
- foundation primitives
- product composites
- workstation shells

## Example Coverage Rule

Examples must not be default-only specimens.

For an example family to count as discoverable, it should expose at least one
meaningful subset of:

- ready/default posture
- error or invalid posture
- loading or pending posture
- empty or no-results posture
- keyboard or focus behavior where the contract owns it
- layout, docking, or persistence posture where workstation shells own it

## Discoverability Rule

Each example family should make it easy to answer three questions:

1. what contract layer owns this surface?
2. what package implements it today?
3. what example types are visible here?

If the browser surface cannot answer those questions quickly, discoverability is
still too weak.

## Minimum Adoption Bar

Before wider rollout, the docs/examples surface must make visible:

- token artifact provenance
- contract ownership
- implementation package ownership
- at least one live example per adopted family
- state and accessibility posture where the contract cares about them

This is the minimum bar, not the finished docs program.

## Raw Markdown Boundary Rule

Raw markdown remains the source of truth for contracts, specs, and roadmap
intent.

The docs-site baseline exists to improve inspection and adoption, not to
replace markdown authority.

## Seed Evidence

- `packages/svelte/preview/src/catalog.ts`
- `packages/svelte/preview/src/App.svelte`
- `packages/svelte/preview/src/app.css`
- `packages/svelte/preview/README.md`
- `docs/contracts/README.md`
- `docs/specs/README.md`

## Next Task

Carry this discoverability baseline through `g02.013` so the preview/docs
surface becomes a credible internal review tool before downstream adoption
starts in a later generation.
