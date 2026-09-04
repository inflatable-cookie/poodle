# Consumer Sweep Intake — 2026-09-04

Status: open — first run of the recurring consumer defect intake lane; four
unresolved items await Chatterbox triage, ten are closed by `0.3.0`
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
   overrides (bovine-accelerator-desktop 3, soundcheck-library 2, figmatic,
   loophole, nucleus 1 each), mostly sidebar-style tabs: `__tooltip`,
   `__label`, `__tab > .poodle-icon`, `__panel`. Known: consumers want icon
   and label treatment and panel padding control. Unknown: whether one
   `appearance`/`density` axis or a small set of CSS custom properties covers
   it. Route: bounded discovery (read the 8 files, list the overridden
   properties) before any contract change. Not a card yet.
2. **Tree treeitem accessible name and hierarchy** — figmatic
   (`PAPERCUTS.md`, 2026-08-28): Longhorn's a11y snapshot shows treeitems
   with no name and no children. Defect in Poodle Svelte `Tree`. It also
   blocks Nucleus A1 for Tree. Route: one bounded card (accessible name from
   the visible label; `aria-owns`/nesting so children are discoverable);
   candidate for the next frontier.
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

After `0.3.0` publishes: re-run the sweep, expect the ten "fixed on main"
entries to close once pins move. Promote item 2 as a card and item 1 as a
discovery brief when the operator confirms. Remove this note when items 1–2
are promoted or rejected.
