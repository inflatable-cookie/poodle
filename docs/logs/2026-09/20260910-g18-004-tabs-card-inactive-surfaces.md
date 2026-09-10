# g18.004 — Tabs card inactive surfaces

Status: merged
Merge: `ed6ed66050c5ba8bf62aaf27eee795ca5be052fa` (PR #239) on 2026-09-10
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

## Closeout

- Merge performed by the plugin as
  `ed6ed66050c5ba8bf62aaf27eee795ca5be052fa` on 2026-09-10 (PR #239),
  parents `2dbcb465` (main) and `19e9366e` (reviewed head); no
  post-review changes on the branch.
- Accepted review: independent exact-head `ready_to_merge` approval of
  head `19e9366ec4f0ce0070ceccd97f9d80f85b7180da` by betterthanclay
  ([comment #5625858043](https://github.com/inflatable-cookie/poodle/pull/239#issuecomment-5625858043)).
  Zero blocking findings; all nine acceptance invariants proved.
- Reviewed-head validation (reviewer-ran, tree left clean): core Tabs
  36 pass; Svelte/React card-surfaces 6 pass; `cargo test -p
  poodle-render --lib tabs` 27 pass; `headless_regressions tabs_` 6
  pass; `effigy drift:recipes` green; `git diff --check` clean.
- Non-blocking reviewer notes (deferred, no acceptance impact):
  `recipe-inventory.json` also catches up the pre-existing
  `--poodle-tabs-panel-padding` metric; Svelte/React CSS-half
  assertions use shipped-declaration matching with the real cascade
  computation in the core suite.
- Deferred: release/tag/publish; `g18.006` needs a Chatterbox recheck
  and explicit operator release authority. Return to Chatterbox.

## Stopped short of

Windowed capture and release/tag/publish, both out of scope.
