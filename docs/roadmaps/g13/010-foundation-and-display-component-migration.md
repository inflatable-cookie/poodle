# g13.010 Foundation And Display Component Migration

Status: closed — superseded by the `g13.008` **revise** verdict
(`docs/roadmaps/g13/pilot-verdict-evidence.md` §7–8). This milestone describes
family-by-family migration to a generative model the verdict declines. It is
retained as evidence and is **not executable**. The replacement runway is
`g13.017`–`g13.020`.
Owner: Poodle core
Depends on: `g13.009`

## Objective

Migrate low-interaction primitives and display components first, hardening the
mechanical path before controls and overlays.

## Scope

- Layout, typography, icon, badge, status, progress, meter, card, list-shell,
  and other low-interaction contract families.
- Generate shared definition, registry, recipe, anatomy, and specimen inputs.
- Remove superseded Svelte, React, and native composition duplicates per wave.

## Acceptance

- Family contracts and public APIs stay stable.
- Zero undeclared runtime extensions.
- Four-runtime visual, recipe, accessibility, and axis evidence passes per
  migration wave.

## Next

`g13.011` migrates interactive controls, forms, and the audio family.
