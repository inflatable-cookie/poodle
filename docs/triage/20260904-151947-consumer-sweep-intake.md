# Consumer Sweep Intake — 2026-09-04

Status: open — promoted: Tree name (`g16.101`), `showTabs` (`g16.100`),
Tabs fill seam (`g16.102`); remaining: single-consumer Tabs asks, Keyboard
geometry (held elsewhere), and the ten entries that close with `0.3.0`
Captured: 2026-09-04
Owner: Chatterbox (planning)
Source: read-only sweep of 15 sibling repositories' `PAPERCUTS.md` and
triage files (report kept outside the repo); claims below were spot-checked
against consumer sources

## Headline

Ten of sixteen Poodle-attributed consumer entries are already fixed on
`main` but still open in the consumer because every consumer pins `0.2.2`
(HistoryCenter v3 and rejection codes ×5, Select ghost variant, Popover
trigger, licence types, ContextMenu trigger). Publishing `0.3.0` (`g16.097`)
and moving pins closes them; no Poodle card is needed. That is the strongest
argument for finishing the release before new component work.

## Unresolved, ranked

1. **Tabs styling seam** — 5 repos, 8 files write `:global(.poodle-tabs…)`
   overrides. Discovery inventory (2026-09-04, read-only, 8 files) grouped
   them:
   - **Fill-height layout** — 3 consumers, 4 files (bovine-accelerator-desktop
     ×2, soundcheck-library, figmatic): root `height: 100%;
     grid-template-rows: auto minmax(0, 1fr)`, panel `min-height: 0;
     overflow: auto`. The one real shared seam. Candidate: a `layout="fill"`
     (or `fillHeight`) prop on Tabs, portable, because GPUI panels already
     size by flex. Risk: box assumptions across renderers; needs a mounted
     GPUI proof.
   - **Spacing hooks** — figmatic, soundcheck-library: gap and panel padding.
     Tabs exposes `--poodle-tabs-list-gap` / `--poodle-tabs-content-gap`
     already (`packages/core/src/styles/tabs.css:7-13`); a
     `--poodle-tabs-panel-padding` hook is the only missing one. Candidate:
     one custom property, documented.
   - **Single-consumer asks** — label truncation (bovine), responsive
     icon-only strip (nucleus), drop-target border (loophole). Not seams yet;
     nucleus's ask conflicts with the contract rule that horizontal labels
     are never shed.
   - **Composition pattern** — standalone strip with an external panel
     (soundcheck-library): documentation, not a component change.
   Promoted 2026-09-04 as `g16.102` (fill layout, panel padding hook,
   standalone-strip recipe). Single-consumer asks (bovine label truncation,
   nucleus responsive icon-only, loophole drop-target border) stay here
   until a second consumer names them.
2. **Tree treeitem accessible name and hierarchy** — figmatic
   (`PAPERCUTS.md`, 2026-08-28): Longhorn's a11y snapshot shows treeitems
   with no name and no children. Defect in Poodle Svelte `Tree`. It also
   blocks Nucleus A1 for Tree. Route: one bounded card (accessible name from
   the visible label; `aria-owns`/nesting so children are discoverable);
   Promoted 2026-09-04 as `g16.101`.
3. **Keyboard vertical equal-pitch mode** — loophole. Already
   design-deferred in `20260902-000956-history-portfolio-holds.md`. No change.
4. **Icon glyph gaps (undo/redo/pin) and static catalogue metadata** —
   reported by the sweep for figmatic and soundcheck but not found by a
   direct grep of their papercuts. Unverified; drop unless a consumer names
   it.

## Usage facts recorded

- `Tree.onEditingChange`: 0 consumer hits. `OrderBy.onActiveSortChange`: 0
  consumer hits. All 15 consumers are Svelte.
- `DockRegion showTabs`: used by Longhorn `longhorn-poodle-svelte`
  (`LayoutDockRegion.svelte:121`, `dock.test.ts:36`) and required by Loophole
  (`PAPERCUTS.md:631`).
- Most-consumed components: Button 15 repos, Select 14, TextInput 14, Tabs
  13, EmptyState 12, Callout 11, Dialog 10, IconButton 10. Tabs is the
  fourth most-used component and the top override target; Select is the
  second most-used and had the ghost-variant defect. These are the
  highest-leverage rows for any consumer-facing repair.

## Next check

After `0.3.0` publishes, re-run the sweep; expect the ten "fixed on main"
entries to close once pins move. Remove this note when the single-consumer
Tabs asks are promoted or rejected. After `0.3.0` publishes, re-run the sweep and expect the
ten "fixed on main" entries to close once pins move. Remove this note when
item 1 is promoted or rejected.
