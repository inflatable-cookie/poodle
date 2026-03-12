# 2026-03-11 g01.004 Through g01.006 Contract Template And Substrate Baseline

## Changed

- added a real component-contract surface under `docs/contracts/` instead of
  leaving component semantics implied by roadmap text alone
- added the canonical template at
  `docs/contracts/template/component-contract-template.md`
- added the first two seed contracts as evidence that the template works across
  both a foundation primitive and a workstation-shell composite:
  - `docs/contracts/foundation/button.md`
  - `docs/contracts/workstation/panel-surface.md`
- added the first normative contract/parity spec at
  `docs/specs/002-component-contract-template-and-parity-rules.md`
- added `docs/architecture/003-component-docs-ia-and-implementation-substrates.md`
  to freeze:
  - contract information architecture
  - Bits-as-internal-substrate policy on the Svelte side
  - GPUI-native but token-driven substrate policy on the Rust side
  - shared parity and documented-delta posture
- updated the docs index surfaces so `contracts/` is now part of the visible
  documentation IA rather than hidden behind roadmap text
- closed `g01.004`, `g01.005`, and `g01.006` in the active roadmap surface

## Downstream Alignment

- used Aura and Spark shell needs as the immediate downstream pressure when
  choosing the first workstation example, which is why the seed composite is
  `PanelSurface` rather than a product-only composite
- kept the shell contract generic so it supports Aura/Spark first without
  collapsing Pug into Loophole-specific DAW widgets
- kept the Svelte substrate policy explicit that Bits may accelerate behavior
  but may not become the public contract
- kept the GPUI substrate policy explicit that generated Rust token artifacts,
  not hand-maintained constants, remain the source of truth

## Validation

- `bun packages/tokens/scripts/build-tokens.ts`
- `git diff --check`

## Remaining

- execute `g01.007` through `g01.010` to build the first primitive contract set
- turn the seed contract examples into the broader primitive catalogue
- keep implementation work aligned to the now-frozen contract and substrate
  rules

## Next Task

Open `docs/roadmaps/g01/007-layout-surface-and-scrolling-primitives.md` and
author the first foundation primitive contract tranche under `docs/contracts/`
using the new template and parity rules.
