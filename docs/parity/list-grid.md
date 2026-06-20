<!-- parity consv=fixed gpui=1 jetstream=1 specimen=gap -->
# Parity: ListGrid

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/list-grid.md`
- Svelte (authoritative): `packages/svelte/components/src/ListGrid.svelte`
- GPUI: `packages/gpui/components/src/primitives/list_grid.rs`
- Jetstream: `packages/jetstream/components/src/list_grid.rs`
- Spec: `packages/contracts/components/src/list_grid.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/ListGridSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/list_grid.rs` · jetstream `packages/jetstream/preview/src/specimens/list_grid.rs`

## Contract ↔ Svelte

Svelte had a prop the contract omitted.

- [x] FIXED `maxColumns?: number | null` (default `3`) added to contract Props (floored, clamped to ≥1) + a new "Layout Behavior" section describing the auto-fill column-cap `calc()` derivation, matching Svelte (`ListGrid.svelte:9,17,32-39`).
- Contract default note for `gap` says default `1.25rem` (default variant) — Svelte confirms (`ListGrid.svelte:31`). `minItemWidth` numeric→`em`, default `360px` — Svelte confirms (`ListGrid.svelte:30`). These match.

## GPUI gap (vs Svelte + contract)

GPUI approximates CSS grid with `flex_wrap`; all dimensions resolve from tokens (`gap_token`, `min_item_width_token`, `header_actions_gap_token`, `header_margin_bottom_token`). Clean on tokens.

- [ ] No `maxColumns` support — `ListGridSpec` (`list_grid.rs:19-24`) has no `max_columns` field; the flex-wrap fallback can over-wrap past the Svelte 3-column cap. Add `max_columns` to spec + clamp cell `flex_basis`/`max_w` in `list_grid.rs:107-117`.
- accepted: no ARIA (layout primitive, contract §2 declares it ARIA-neutral).

## Jetstream gap (vs Svelte + contract)

Mirrors GPUI; all dimensions token-resolved. Clean on tokens.

- [ ] No `maxColumns` support — same spec gap; default cell is `min_w(min_w).flex_1()` (`list_grid.rs:60-63`) with no column cap.
- accepted: ARIA-neutral primitive (no delta).

## Specimen parity

- Svelte covers: Responsive default (`minItemWidth={14}`), With header actions (`minItemWidth={16}`, Export + IconButton), Compact (`ListGridSpecimen.svelte`).
- GPUI covers: Default (min 14em), With header actions, Compact (`list_grid.rs:46,87,95`). — matches Svelte 1:1.
- Jetstream covers: Default (wrap + min tile), Compact (`list_grid.rs:41-42`). — missing: **header-actions** group (the `with_header` path is exercised by the contract but not shown in a dedicated group; default group does pass a header label). Minor.

## Notes

- `consv=fixed`: the only Contract↔Svelte gap was the missing `maxColumns`, now added. The remaining `max_columns` spec/clamp work is Rust-side (tracked above).
- The `max_columns` cap is the only real cross-target gap; both Rust impls are token-clean and structurally faithful to the flex-wrap approximation documented in the contract.
