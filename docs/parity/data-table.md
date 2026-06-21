<!-- parity consv=fixed gpui=1 jetstream=0 specimen=gap | pass 41: GPUI px literals → contract-rem (selection 3.25/actions 3.5/expand icon+pad); Jetstream gained toolbar+filter chips+pagination footer+row-actions column+real sort icon (⇅ removed); probe tests added -->
# Parity: DataTable

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/data-table.md`
- Svelte (authoritative): `packages/svelte/components/src/DataTable.svelte`
- GPUI: `packages/gpui/components/src/composites/data_table.rs`
- Jetstream: `packages/jetstream/components/src/data_table.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/DataTableSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/data_table.rs` · jetstream `packages/jetstream/preview/src/specimens/data_table.rs`

## Contract ↔ Svelte

Svelte is authoritative. All stale contract values reconciled. FIXED.

- [x] Actions column width → `3.5rem` (also added `white-space: nowrap`). FIXED.
- [x] Toolbar/menu/cell paddings → now cite tokens: toolbar `space-control-y space-panel-x` + gap `space-inline-md`; toolbar-btn `space-control-y space-control-x` + gap `space-inline-sm`; col-menu-item `space-control-y space-control-x` + gap `space-inline-md`. FIXED.
- `th`/`td` padding: already `space-panel-y space-panel-x`. OK, no change.
- [x] Size matrix + density blocks → added "Size Variants" and "Density Variants" tables to §11 (per-size cell typography, cell `padding-inline`, selection width, sort/action min-height, toolbar-btn typography/padding/icon; density row `padding-block` + footer `padding-inline`). FIXED.
- `rowActions` rich model + `Menu`/`IconButton` ellipsis trigger, `onRowActionSelect`, `expandedRow`, `cell`, `empty` snippets, CSV export, column-visibility `Popover`, pagination footer with `Select` limit picker — all present in Svelte and contract. OK.

Note: DataTable density legitimately owns row `padding-block` (compact `0.25rem`, comfortable `panel-y × 1.25`) — Svelte ships this and it is the canonical row-density axis for a tabular surface, so it is documented as the CLAUDE.md case-by-case exception, not stripped.

## GPUI gap (vs Svelte + contract)

- [x] Hardcoded column/cell width px literals → all replaced with contract-rem
  values resolved through new `presentation::data_table_selection_width_rem`
  (size table; md `3.25rem`) and `data_table_actions_width_rem` (`3.5rem`).
  Select/actions cells, expand column (icon.sm + symmetric inline padding),
  mixed-checkbox bar (`0.5`/`0.125rem`), tone-pill (`0.125rem`), filter chip
  (`0.1875rem`), pager button (`0.25rem`) now zero raw px. Actions header also
  gained its "Actions" label. FIXED (pass 41).
- accepted (functional breadth, layout-present / interaction host-owned per
  contract host-ownership note): selection checkbox hand-drawn rather than
  composing the `Checkbox` primitive; toolbar Columns/Export render chrome but
  the CSV export + visibility `Popover` are host-owned; filter row renders chips
  not per-column text/select/date inputs; row-actions is a single text button
  (no rich `rowActions` menu); expanded row renders `row.summary` not an
  arbitrary host snippet. These are interaction/host-snippet gaps, not the
  "no hardcoded px" contract violation, and GPUI is build-verified only.
- accepted: no ARIA / `<table>` semantics (gpui has no accessibility API; flat `div` grid).
- accepted: `position: sticky` approximated via `flex_shrink_0` (documented in file).

## Jetstream gap (vs Svelte + contract)

- [x] Toolbar — `show_export` / `show_column_visibility` now render Export +
  Columns buttons (icon + label, token-resolved) above the table. Export/popover
  behaviour is host-owned (preview loop). FIXED (pass 41).
- [x] Filter row — `spec.filters` now render as accent chips in a bordered
  filter row (`column_id: value`, `radius.pill`); per-column inputs host-owned. FIXED.
- [x] Pagination footer — `spec.pagination` now renders a footer with the
  `first–last of total` summary, `Page n of m`, and Prev/Next controls
  (disabled-state tint). Clicks host-owned. FIXED.
- [x] Row-actions column — `show_row_actions` now renders the fixed `3.5rem`
  actions header + a per-row `row_action_label` button (accent, focusable). FIXED.
- [x] Sort affordance — removed the literal `"⇅"` glyph (Svelte shows nothing on
  unsorted columns); active-sort arrow icon now sizes from `size.icon.sm`, not a
  hardcoded `0.75rem`. Selection/actions widths resolve from the size table. FIXED.
- accepted: select-all uses the real `Mixed`/`Checked` checkbox state; rows use
  `with_checked` — matches the contract (rows are binary).
- accepted: no ARIA; interaction (sort/select/export/filter/paginate/row-action)
  lives in the preview event loop.
- note: `js_data_table_loading` exists for skeleton rows (contract loading posture) — good, keep.
- note: status-pill cells (Svelte custom-cell pattern) render via `cell_tones`
  with a `color-mix` pill bg, mirroring the GPUI build.

## Specimen parity

- Svelte covers: "With sorting, column visibility, and export" (selection + sortable + toolbar), "With filters and pagination", "With custom cells and expanded rows", "Empty state" — the two contract-mandated specimens plus extras.
- GPUI covers: sorting+col-visibility+export, Sizes, filters+pagination (compact/striped/sticky), col-visibility+export+row-selection, custom cells+expanded rows, Empty state (6 groups). — missing: nothing vs contract specimen list, but several groups exercise chrome that is non-functional in the component (see GPUI gaps), so they demonstrate layout not behavior.
- Jetstream covers: Default, Sorted asc (Name), Sorted desc (Owner), Selectable rows, Empty state. — missing: **toolbar (column visibility + export)**, **filters + pagination**, **custom cells / expanded rows** — all blocked on the component not implementing that chrome.

## Notes

- Biggest Jetstream gap is breadth: it implements header + body + selection + sort indicator + empty + loading skeleton, but none of toolbar / filter row / pagination footer / row-actions. GPUI implements the layout of all of those but several are non-interactive placeholders (export, column-visibility, filters).
- The contract↔Svelte `7.5rem` vs `3.5rem` actions width and the frozen-rem-vs-token padding entries are the `consv=gap` drivers; update the contract to match Svelte.
