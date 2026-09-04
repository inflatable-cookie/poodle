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

## Next check

Fold into the first consumer papercut sweep: grep the sibling apps for
`onEditingChange`, `onActiveSortChange`, and DockRegion tab usage before
deciding. Decide all three in one small contract batch, then either open a
card or re-kind the entries to `framework-idiom`. Remove this note when the
baseline has no `needs-decision` entries.
