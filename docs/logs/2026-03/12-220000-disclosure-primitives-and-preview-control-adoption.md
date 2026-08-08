# Disclosure Primitives And Preview Control Adoption

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- added `Accordion` and `Collapsible` to `@pug/svelte-primitives` with
  contracts, package exports, and a new disclosure-boundary spec
- replaced the preview rail and most state or action controls with real Pug
  primitives instead of styled generic HTML controls
- widened the package export coverage manifest so the new disclosure and shell
  control primitives are counted as directly previewed

## Validation

- `bun run docs:lint`
- `bun run parity:report`
- `bun run docs:build`
- `git diff --check`

## Risks

- the preview still has a few intentional raw buttons for card-like navigation
  and media-strip selection where the interaction is more bespoke than a
  generic action button
- the disclosure primitives are Svelte-native implementations for now, not yet
  Bits-backed wrappers

## Next Task

Continue preview adoption by replacing the remaining bespoke card or strip
selection controls with a clearer primitive or contract-backed selection
pattern, or freeze the preview shell here and move into `g03.004` performance,
render-cost, and memory hardening.
