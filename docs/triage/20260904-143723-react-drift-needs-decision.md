# React Drift Baseline — Needs-Decision Entries

Status: open — three `needs-decision` entries in the React prop drift
baseline await a Chatterbox contract decision
Captured: 2026-09-04
Owner: Chatterbox (planning)
Source: `packages/svelte/preview/scripts/react-prop-drift.ts` `BASELINE`
(seeded by `g16.095`, merged in PR #202 at `f297774f4`)

## The three entries

1. **DockRegion `showTabs`** — Svelte-only. Baseline reason: spec-surface
   tranche awaiting `DockRegionSpec` tab fields (`g13.014`). Known: the Rust
   spec models `tabs_placement` only. Unknown: whether the field should
   exist portably or stay web-only. Route: contract decision in
   `docs/contracts/components/dock-region.md`; if portable, a small spec +
   render card; if web-only, `WEB_ONLY_PROPS` register entry and a React port.
2. **Tree `onEditingChange`** — React-only change callback for inline rename.
   Candidate for Svelte inclusion (working rules: "candidate, not automatic
   port"). Unknown: whether Svelte consumers need an editing-state callback
   or `$bindable` editing state is enough. Route: evaluate against Figmatic
   and Nucleus Tree usage; then add to Svelte and the contract, or record as
   React idiom.
3. **OrderBy `onActiveSortChange`** — React-only callback. Same evaluation as
   Tree; consumers to check: Soundcheck, Acowtancy admin lists.

## Evidence (sweep 2026-09-04)

- `Tree.onEditingChange` and `OrderBy.onActiveSortChange`: zero consumer
  hits across 15 Svelte consumers; they exist only in the React shells and
  specimens. Both are the change callback paired with a `$bindable` state
  (`editing`, `activeSort`), the pattern the working rules already call
  framework idiom.
- `DockRegion showTabs`: used by Longhorn's `longhorn-poodle-svelte`
  (`LayoutDockRegion.svelte:121`, `dock.test.ts:36`) and required by
  Loophole. It is a real cross-target capability, not a web-only attribute.

## Recommendation (awaiting operator confirmation)

1. Re-kind `tree` and `order-by` entries to `framework-idiom` with the
   pairing reason. No Svelte change. One-line baseline edit; fold into the
   next React-touching card or a papercut-class worker.
2. Promote `showTabs` to the portable spec: `DockRegionSpec` gains a
   `show_tabs` field, `poodle-render` honours it, the contract table moves
   the prop from web-only to portable, and the baseline entry is deleted.
   One small card; also closes the `g13.014` spec-surface tranche note.

## Next check

Remove this note when the baseline has no `needs-decision` entries.
