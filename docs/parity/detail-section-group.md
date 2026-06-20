<!-- parity consv=gap gpui=4 jetstream=2 specimen=gap -->
# Parity: DetailSectionGroup

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/detail-section-group.md`
- Svelte (authoritative): `packages/svelte/components/src/DetailSectionGroup.svelte`
- GPUI: `packages/gpui/components/src/composites/detail_section_group.rs`
- Jetstream: **ABSENT** — no `packages/jetstream/components/src/detail_section_group.rs` (only `detail_section.rs` exists)
- Specimens: svelte `packages/svelte/preview/src/specimens/DetailSectionGroupSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/detail_section_group_specimen.rs` · jetstream **ABSENT**

## Contract ↔ Svelte

`consv=gap`. Props align, but the contract spec lacks builder methods for three fields, so they can't be set on the Rust targets.

- All props match in meaning/default: `density` (`null`→context), `layout` (`"grid"`/`"stack"`, default `"grid"`), `minColumnWidth` (`"14rem"`), `itemMinColumnWidth` (`"12rem"`), `maxColumns` (`2|3|4|5`, default `4`), `ariaLabel` (`null`).
- Contract spec (`packages/contracts/components/src/detail_section_group.rs`) has `min_column_width`, `item_min_column_width`, `aria_label` fields but **no builder methods** (`with_min_column_width`, `with_item_min_column_width`, `with_aria_label`) — so consumers can't override them. **Fix spec: add the three builders.** (This is why `consv=gap`: spec surface lags Svelte's prop surface.)

## GPUI gap (vs Svelte + contract)

Renders grid as flex-row / stack as flex-col, but ignores most sizing props.

- [ ] Hardcoded `min_w(px(224.0))` at `detail_section_group.rs:36` — ignores `spec.min_column_width`; always forces 14rem. Resolve from spec.
- [ ] `max_columns` not constraining layout — flex-wrap + flex-1 allows unlimited columns; cap per `spec.max_columns`.
- [ ] `item_min_column_width` not propagated to descendant DetailSections (Svelte injects a CSS custom property). Forward it.
- [ ] No `with_density`/`with_layout`/sizing builders on the component — only `from_spec()` + `child()`; props settable only via spec constructor.
- accepted: no ARIA (gpui has no accessibility API); no responsive collapse (Svelte has a 34rem container query — host-driven delta).

## Jetstream gap (vs Svelte + contract)

- [ ] **Entire component missing.** Implement `js_detail_section_group()` in `packages/jetstream/components/src/detail_section_group.rs` per contract: grid/stack layout, `minColumnWidth`/`itemMinColumnWidth`/`maxColumns`/`density`/`ariaLabel`, all token-resolved. Single biggest gap for this component.
- [ ] Add the Jetstream specimen `packages/jetstream/preview/src/specimens/detail_section_group.rs` covering grid, stack, column-cap, and density variants.

## Specimen parity

- Svelte covers: grid layout (default, 3 sections auto-fit), stack layout (`itemMinColumnWidth="10rem"`), column cap (`maxColumns={2}`, `minColumnWidth="10rem"`), density variants (compact/default/comfortable).
- GPUI covers: default grid, stack with compact density. — missing: **column-cap variant** (maxColumns not constraining), **full density coverage** (only compact shown, not default/comfortable).
- Jetstream covers: nothing — no specimen exists.

## Notes

- Two structural gaps: Jetstream component absent entirely, and GPUI's `min_w(px(224.0))` hardcode plus missing maxColumns cap mean GPUI's grid doesn't actually honor the spec's sizing props.
- The `consv=gap` is a spec-builder gap, not a behavioral Svelte divergence — Svelte itself is correct.
