# g16.102 — Tabs Fill Layout Seam

Status: ready
Type: additive capability — contract, core CSS, Svelte, React, Rust spec and
render, one recipe
Opened: 2026-09-04
Depends on: none
Governing refs: `../../contracts/components/tabs.md`,
`packages/core/src/styles/tabs.css`, `packages/render/src/tabs.rs`,
`../../contracts/001-working-rules.md`
Consumer evidence (read-only inventory 2026-09-04): three consumers in four
files override `.poodle-tabs`/`.poodle-tabs__panel` to make Tabs fill its
container (`height: 100%; grid-template-rows: auto minmax(0, 1fr)`; panel
`min-height: 0; overflow: auto`): bovine-accelerator-desktop
`ContentDetailTabs.svelte`, `SpineDetailTabs.svelte`; soundcheck-library
`BrowseContextController.svelte`; figmatic `OrganisationPanel.svelte`.
figmatic and soundcheck-library also override panel padding.
soundcheck-library `PluginInspector.svelte` renders the strip standalone
with an external panel.
Operator decision 2026-09-04: one small card for the fill seam, the panel
padding hook, and the standalone-strip recipe; single-consumer asks stay in
triage
Dispatch manifest: `../dispatch.md`

## Goal

Give consumers a public seam for full-height Tabs so they stop overriding
internals, in every active runtime.

## Fixed Boundary

- Contract: add `layout: "auto" | "fill"` (default `"auto"`) to the Tabs
  props table. `fill` makes the root take its container's block size and
  the active panel scroll within it; the strip keeps its natural height.
  Orientation-independent. Document that `fill` requires a sized container.
- Core CSS: implement `fill` on `.poodle-tabs[data-layout="fill"]` with the
  grid rows and `min-height: 0` / `overflow: auto` on the panel, using
  existing size tokens. Add one custom property
  `--poodle-tabs-panel-padding` (default: the current panel padding value)
  and document it beside the existing `--poodle-tabs-*` properties.
- Svelte and React: accept `layout`, emit `data-layout`, no other change.
- Rust: `TabsSpec` gains `layout: TabsLayout` (`Auto` default, `Fill`);
  `poodle-render` maps `Fill` to the flex-grow / min-size vocabulary the
  node model already has so the panel fills and scrolls in GPUI. One mounted
  or node-inventory proof that the panel grows and the strip does not.
- Recipe: add a short "standalone tab strip with an external panel" recipe
  to `docs/guides/011-page-shell-and-admin-recipes.md`, stating that the
  consumer then owns `aria-controls`/`aria-labelledby` linking.
- Out of scope: label truncation, responsive icon-only strips, drop-target
  border hooks. They stay in the consumer-sweep triage note.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Fill fills | `layout="fill"` in a 300 px-tall container with long panel content | panel scrolls; root height equals container; strip height unchanged (Svelte and React DOM tests) |
| Auto is unchanged | default layout | existing Tabs tests and specimen census pass byte-for-byte |
| Native parity | `TabsSpec { layout: Fill }` | node inventory shows the panel with grow/min-size and the strip fixed |
| Hook is real | set `--poodle-tabs-panel-padding: 0` | panel padding is 0 |
| Consumer override is obsolete | bovine `ContentDetailTabs.svelte` pattern reproduced in a test with `layout="fill"` and no `:global` | same computed layout |

## Validation

`effigy test:components` (Tabs suites), `effigy docs:check`, `effigy
ci:web`, `effigy test:contracts`, `cargo test -p poodle-render`,
`git diff --check origin/main...HEAD`. Never run windowed selectors.

## Owned Paths

`docs/contracts/components/tabs.md`, `packages/core/src/styles/tabs.css`,
`packages/core/src/tabs.ts` (type only, if the layout enum lives there),
`packages/svelte/components/src/Tabs.svelte`,
`packages/react/components/src/Tabs.tsx`, their tests,
`packages/contracts/components/src/tabs.rs`, `packages/render/src/tabs.rs`
and tests, one GPUI specimen row, `docs/guides/011-page-shell-and-admin-recipes.md`
(one recipe), execution log under `docs/logs/2026-09/`, root `PAPERCUTS.md`
(append only).

Reserved for the coordinator at merge: `docs/roadmaps/g16/README.md`,
`docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`.

## Stop Conditions

Stop if `fill` needs a change to the Tabs drag or overflow measurement code,
or if the native node vocabulary cannot express grow-with-min-size without a
new node capability. Escalation owner: Chatterbox.
