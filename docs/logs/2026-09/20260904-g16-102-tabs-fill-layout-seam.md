# g16.102 — Tabs Fill Layout Seam

Status: complete — ready for coordinator review
Date: 2026-09-04
Card: `docs/roadmaps/g16/102-tabs-fill-layout-seam.md`
Handoff: `docs/handoffs/20260904-170000-g16-102-tabs-fill-layout-seam.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/contracts/components/tabs.md`
Branch: `feature/g16-102-tabs-fill-layout-seam`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-102-tabs-fill-layout-seam`
Planning base: `fbf930285` (`origin/main`; dispatch manifest revision 6 ancestor)

## Outcome

Tabs has one public seam for full-height surfaces in every active runtime.
`layout: "auto" | "fill"` (default `"auto"`) is contracted in the props
table, emitted as `data-layout` by Svelte and React, declared as
`TabsSpec.layout: TabsLayout` (`Auto` default, `Fill`) in `poodle-specs`, and
mapped in `poodle-render` onto the node vocabulary that already exists — the
root grows in its container (`flex_grow`), the panel grows with
`min_height: 0` and vertical `Scroll` overflow, and the strip carries no grow.
No new node capability; the drag and overflow-measurement code is untouched.
The core stylesheet implements `fill` on `.poodle-tabs[data-layout="fill"]`
(`height: 100%`, `grid-template-rows: auto minmax(0, 1fr)`; vertical fill
stretches one `minmax(0, 1fr)` row) with `min-height: 0; overflow: auto` on
the panel, and adds the `--poodle-tabs-panel-padding` hook whose default is
the historical panel padding. A standalone-strip recipe in
`docs/guides/011` states that a consumer rendering the strip without the
`children` snippet owns `aria-controls`/`aria-labelledby` linking. The three
consumer `:global` override sites named in the card stay consumer-side; the
seam makes their pattern obsolete.

## What landed

- Contract: `layout` row in the props table, §7 sizing bullets, §8
  Root/Panel fill tables, `--poodle-tabs-panel-padding` documented beside the
  Panel token table, Svelte `data-layout` note, GPUI `TabsLayout` note, Tier 2
  checklist row.
- Core CSS (`packages/core/src/styles/tabs.css`): fill rules for root and
  panel (horizontal + vertical), custom property declared on the root and
  consumed by the default panel; the card variant's flush `padding: 0` panel
  does not read the hook.
- Svelte/React: `layout` prop with default `"auto"`, `data-layout` emitted on
  the root. No other behavior change; existing Tabs tests pass unmodified.
- `poodle-specs`: `TabsLayout` enum, `TabsSpec.layout` field, `with_layout`
  builder, re-export from the crate root.
- `poodle-render`: root grow under `Fill` in `tabs_with_handlers`; panel
  grow/min-size/scroll mapping in `tabs_with_panel`.
- Tests: paired Svelte/React `TabsFillLayout` suites (oracle scenario DOM +
  stylesheet declaration proofs — happy-dom cannot cascade stylesheets at
  computed-value time, the same limitation the DockRegionTabPassThroughs
  underline-hook suite records); `poodle-specs` layout default/builder test;
  node-inventory proofs `fill_layout_grows_the_panel_and_leaves_the_strip_fixed`
  and `fill_layout_grows_the_root_and_auto_does_not`.
- GPUI specimen: one fill row in `packages/gpui/preview/src/specimens/tabs.rs`
  composed through `tabs_with_panel` inside a fixed-height host, with a
  long-content panel node so the strip stays fixed while the panel scrolls.

## Falsification

Plants run against this head, then reverted; every row failed as predicted.

| Row | Plant | Result |
| --- | --- | --- |
| Panel grows | delete the `TabsLayout::Fill` panel branch in `tabs_with_panel` | `fill_layout_grows_the_panel_and_leaves_the_strip_fixed` panicked: "fill panel grows" — `flex_grow` was `None` |
| Root takes the container | comment out the root grow branch in `tabs_with_handlers` | `fill_layout_grows_the_root_and_auto_does_not` panicked: "fill root takes the container" |
| Fill rules ship | delete the `.poodle-tabs[data-layout="fill"]` root rule from `tabs.css` | 3 stylesheet proofs failed (root sizing rule, row-track gating count, exact vertical panel rule) |
| Hook is real | delete the `--poodle-tabs-panel-padding` declaration from the root | hook-default proof failed; consumer padding override proof failed with it |
| Emission parity | remove `data-layout={layout}` from the React root | React suite lost both `data-layout` emission proofs |

## Validation

Focused (this head):

- Svelte/React Tabs suites incl. the new fill files — 13 files, 113 pass
  (vitest `svelte-components` + `react-components`)
- `bun test packages/core/test/tabs.test.ts` — 24 pass
- `cargo test --manifest-path packages/contracts/components/Cargo.toml` — 330 pass
- `cargo test` in `packages/render` — 638 pass, incl. the two fill proofs
- `effigy regressions:native` at `a9030c688` — 203 pass; receipt cohort repinned

Boards (exit 0):

- `effigy test:components`, `effigy docs:check`, `effigy ci:web`,
  `effigy test:contracts` — pass
- `git diff --check origin/main...HEAD` — pass

## Limits

- No consumer repositories touched; the bovine/soundcheck/figmatic overrides
  remain theirs to retire.
- No Jetstream surface: deferred backend, per the working rules.
- The GPUI specimen row is construction-level evidence; the mounted-window
  capture is the coordinator's conformance surface and was not run
  (no `*-windowed` selector ran locally).
