<!-- parity consv=gap gpui=6 jetstream=6 specimen=gap -->
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

Svelte is authoritative. A few contract values are stale vs Svelte:

- Actions column width: contract §11 `.data-table__actions-header`/`.data-table__actions` = `width: 7.5rem`; Svelte uses `width: 3.5rem` (line 813). **Fix: update contract to 3.5rem.**
- Toolbar/menu/cell paddings: contract §11 lists literal rem values (toolbar `0.5rem 0.75rem`, col-menu-item `0.375rem 0.5rem`, toolbar-btn `0.25rem 0.625rem`); Svelte resolves these from tokens (`space-control-y`/`space-panel-x`/`space-control-x`, lines 645,654,689). Same intent, but **contract should cite the tokens, not frozen rem values.**
- `th`/`td` padding: contract says `space-panel-y space-panel-x`; Svelte matches (line 719). OK.
- Svelte adds size matrix (`data-size` xs/sm/lg/xl) and density (`data-density` compact/comfortable) blocks (lines 942-1099) — contract §4 lists `size`/`sizeRole`/`density` props but §11 has no per-size token table. **Fix: document size/density spacing in contract.**
- `rowActions` rich model + `Menu`/`IconButton` ellipsis trigger, `onRowActionSelect`, `expandedRow`, `cell`, `empty` snippets, CSV export, column-visibility `Popover`, pagination footer with `Select` limit picker — all present in Svelte and contract. OK.

## GPUI gap (vs Svelte + contract)

- [ ] Hardcoded column/cell width px literals: select column `px(40.0)` (`data_table.rs:393,581`), expand column `px(32.0)` (`:442,619`), actions column `px(120.0)` (`:512,704`), mixed-checkbox bar `px(8.0)`/`px(2.0)` (`:424`), tone-pill `py(px(2.0))` (`:683`), filter chip `py(px(3.0))` (`:350`), pager button `py(px(4.0))` (`:814`). Replace each with a token-resolved value (e.g. selection width → `3.25rem` per contract `.data-table__selection`; actions → `3.5rem`).
- [ ] Selection checkbox is hand-drawn (`div` with border/fill, `:412-426`, `:587-595`) instead of composing the real `Checkbox` primitive — contract §10 "Composes: Checkbox". Mockup risk.
- [ ] Toolbar buttons (Columns/Export) are non-interactive `div`s with no click handler and no actual CSV export or column-visibility popover (`:284-334`) — contract §9 requires functional export + `Popover` menu.
- [ ] Filter row is rendered as read-only accent chips (`:340-358`), not the contract's per-column filter inputs (text/select/date) in a `<tr>` filter row.
- [ ] No row-actions menu (`rowActions`/`onRowActionSelect` rich model) — only a single text action button using `row_action_label`.
- [ ] No custom-cell / expanded-row snippet equivalent host hook; expanded row renders `row.summary` only (`:726-741`), not arbitrary host content.
- accepted: no ARIA / `<table>` semantics (gpui has no accessibility API; flat `div` grid).
- accepted: `position: sticky` approximated via `flex_shrink_0` (documented in file).

## Jetstream gap (vs Svelte + contract)

- [ ] No toolbar at all — `show_column_visibility` / `show_export` are never read; contract §3 anatomy Toolbar + §9 export/column-visibility absent.
- [ ] No filter row — `spec.filters` / filterable columns not rendered.
- [ ] No pagination footer — `spec.pagination` ignored; contract Footer (summary, limit selector, controls) absent.
- [ ] No row-actions column — `show_row_actions` / `row_action_label` / rich `rowActions` not rendered.
- [ ] Unsorted sortable columns use a literal glyph `"⇅"` as the sort affordance (`data_table.rs:90`) instead of an icon primitive; sort icon dims `rem_to_px(0.75)` hardcoded (`:83-84`) rather than an `icon` size token.
- [ ] Row-selection checkbox composes `js_checkbox` (good) but select-all/row checkboxes don't reflect the contract's mixed state via the real primitive consistently; verify `Mixed` path. (Header uses `with_mixed`; rows only `with_checked` — partial-selection mixed state on header OK, but rows fine.) — fold into: render the actions/expand columns and filter/pagination chrome.
- accepted: no ARIA; interaction (sort/select/expand/paginate) lives in preview event loop.
- note: `js_data_table_loading` exists for skeleton rows (contract loading posture) — good, keep.

## Specimen parity

- Svelte covers: "With sorting, column visibility, and export" (selection + sortable + toolbar), "With filters and pagination", "With custom cells and expanded rows", "Empty state" — the two contract-mandated specimens plus extras.
- GPUI covers: sorting+col-visibility+export, Sizes, filters+pagination (compact/striped/sticky), col-visibility+export+row-selection, custom cells+expanded rows, Empty state (6 groups). — missing: nothing vs contract specimen list, but several groups exercise chrome that is non-functional in the component (see GPUI gaps), so they demonstrate layout not behavior.
- Jetstream covers: Default, Sorted asc (Name), Sorted desc (Owner), Selectable rows, Empty state. — missing: **toolbar (column visibility + export)**, **filters + pagination**, **custom cells / expanded rows** — all blocked on the component not implementing that chrome.

## Notes

- Biggest Jetstream gap is breadth: it implements header + body + selection + sort indicator + empty + loading skeleton, but none of toolbar / filter row / pagination footer / row-actions. GPUI implements the layout of all of those but several are non-interactive placeholders (export, column-visibility, filters).
- The contract↔Svelte `7.5rem` vs `3.5rem` actions width and the frozen-rem-vs-token padding entries are the `consv=gap` drivers; update the contract to match Svelte.
