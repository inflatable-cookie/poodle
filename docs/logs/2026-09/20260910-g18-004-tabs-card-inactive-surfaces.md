# g18.004 — Tabs card inactive surfaces

Status: ready for review
Date: 2026-09-10
Card: `docs/roadmaps/g18/004-tabs-card-inactive-surfaces.md`
Handoff: `docs/handoffs/20260910-g18-004-tabs-card-inactive-surfaces.md`
Governing refs: `docs/contracts/components/tabs.md`,
`docs/contracts/001-working-rules.md`,
`docs/architecture/001-poodle-system-shape.md`,
`docs/architecture/007-appearance-recipe-contract.md`,
`docs/specs/070-compiled-web-distribution-contract.md`
Branch: `ns-d2ae7066-0fda-4796-a65a-3fa5671a54d5`
Base: `origin/main` at `70c1b8c8a3235863d97dd549d413bffda4fe9dff`

## Outcome

Every `card` Tabs item keeps a card-shaped `color.background.surface` fill
on the item wrapper across Svelte, React, shared Rust composition, and
GPUI. Inactive cards have no border. Selected `tint` and `solid` replace
that base; `activeFill="none"` retains it. Pill and block stay unfilled
when idle and under `none`.

No new public prop, token, or variant. No interaction change. No release.

## What changed

- Shared CSS (`packages/core/src/styles/tabs.css`): card item wrappers
  resolve `--poodle-recipe-tabs-card-item-fill` to
  `var(--poodle-color-background-surface)`. `activeFill="none"` restores
  that base on selected card items instead of painting transparent.
- Shared Rust (`packages/render/src/tabs.rs`): every card item node, the
  wrapper that also owns close, gets the surface fill. Tint/solid still
  replace it on the selected item.
- Tests: core cascade computed-style suite; Svelte/React wrapper-ownership
  suites; Rust node assertions for inactive, closable, disabled, tint,
  solid, and cross-variant `none`; mounted GPUI projection through
  `map_style`.
- Generated recipe inventory: `--poodle-recipe-tabs-card-item-fill` plus
  the already-authored `--poodle-tabs-panel-padding` metric that the
  scanner had missed.

## Validation

- `bun test` core Tabs: 36 pass (`tabs.test.ts`, `tabs-pinned.test.ts`,
  `tabs-card-item-fill.test.ts`)
- Svelte/React: 36 pass on Tabs + card-surfaces; 44 pass on fill-layout,
  pinned, and interactions
- `cargo test -p poodle-render --lib tabs`: 27 pass
- `cargo test --test headless_regressions tabs_`: 6 pass, including
  `tabs_card_item_surfaces_project_through_mounted_gpui`
- `effigy drift:recipes` green; `git diff --check` clean

Nucleus M1/A1 `source_commit` is repinned in a follow-up commit against
this runtime head. No new visual capture.

## Concurrent paths

No collision with g18.003. This lane touched Tabs CSS, Tabs render mapping,
Tabs tests, recipe inventory, and the GPUI Tabs projection test. Rich-text
and package-manifest paths were left alone.

## Stopped short of

Windowed capture, release/tag/publish, g18 README / generation-index
closeout, and merge.
