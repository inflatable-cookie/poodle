# 037 Disclosure Foundation Boundary

Status: active
Updated: 2026-03-12
Depends on: `028-primitive-baseline-and-bits-aligned-surface.md`, `029-advanced-primitive-promotion-and-substrate-mapping.md`, `004-overlay-focus-dismissal-and-layering-rules.md`

## Purpose

Define the disclosure family that belongs in Poodle foundation for web-oriented
products and docs surfaces, and make clear where single-block disclosure ends
and grouped disclosure begins.

## Foundation Disclosure Family

The current disclosure tranche promotes:

- `Collapsible`
- `Accordion`

These are foundation-safe because they own reveal or hide posture, not product
workflow semantics.

## Ownership Rule

`Collapsible` owns:

- one trigger
- one controlled or uncontrolled open state
- one revealable content region
- inline disclosure affordance and region linkage

`Accordion` owns:

- repeated disclosure items
- single or multiple expansion posture
- grouped trigger and panel semantics

Neither primitive owns:

- routing-aware navigation
- tree or outline semantics
- workstation panel orchestration
- arbitrary shell IA beyond grouped disclosure

## Promotion Rule

Disclosure primitives belong in foundation because they are generalized enough
for settings, documentation, FAQ, marketing, and admin surfaces, especially in
more web-oriented downstream products.

They should not be forced upward into composites just because the first strong
use cases happen inside larger docs or settings shells.

## Naming Rule

Poodle keeps both names explicit:

- `Collapsible` for one revealable block
- `Accordion` for grouped disclosure items

This is clearer than overloading one primitive to do both jobs through hidden
modes.

## Preview Rule

Preview adoption should exercise disclosure primitives directly in docs-shell
or utility control surfaces so they do not remain contract-only exports.

## Evidence

- `docs/contracts/components/collapsible.md`
- `docs/contracts/components/accordion.md`
- `packages/svelte/primitives/src/Collapsible.svelte`
- `packages/svelte/primitives/src/Accordion.svelte`
- `packages/svelte/preview/src/App.svelte`

## Next Task

Use the disclosure primitives in the preview/docs shell and then reassess which
remaining preview-only controls should be replaced by foundation primitives
before widening the catalogue further.
