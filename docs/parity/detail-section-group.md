<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok -->
<!-- specimen pass: Jetstream specimen created + registered (`detail_section_group.rs`, dispatch arm `detail-section-group`); GPUI specimen gained column-cap (maxColumns=2) + full density coverage. Both cover grid, stack, column-cap, and the three density variants. Both preview crates build clean. -->

<!-- pass: GPUI now resolves min_column_width + caps columns via flex_basis(relative(1/N)); forwarded builders added. Jetstream column min switched item_min→min_column_width; max_columns cap noted as JsEl gap (no % flex-basis). Spec gained with_min_column_width/with_item_min_column_width/with_aria_label + min/item_min_column_width_rem(). gpui build + spec test + jetstream probe (4) green. -->
# Parity: DetailSectionGroup

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/detail-section-group.md`
- Svelte (authoritative): `packages/svelte/components/src/DetailSectionGroup.svelte`
- GPUI: `packages/gpui/components/src/composites/detail_section_group.rs`
- Jetstream: `packages/jetstream/components/src/detail_section_group.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/DetailSectionGroupSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/detail_section_group_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/detail_section_group.rs`

## Contract ↔ Svelte

`consv=fixed`. Contract text matches Svelte's prop surface exactly; the remaining gap is a spec-builder (code) gap on the Rust targets, not a contract↔Svelte divergence.

- [x] All props match in meaning/default: `density` (`null`→context), `layout` (`"grid"`/`"stack"`, default `"grid"`), `minColumnWidth` (`"14rem"`), `itemMinColumnWidth` (`"12rem"`), `maxColumns` (`2|3|4|5`, default `4`), `ariaLabel` (`null`). Verified against `DetailSectionGroup.svelte:6-24`.
- [x] FIXED (contract enrichment) §7 now documents the density gap values, `--poodle-detail-section-group-min`/`-item-min` custom props, `data-max-columns` bound, and the `@container (max-width: 34rem)` single-column collapse.
- Spec-builder gap (`packages/contracts/components/src/detail_section_group.rs` lacks `with_min_column_width`/`with_item_min_column_width`/`with_aria_label`) is a **code** fix tracked under the Rust targets — out of scope for contract↔Svelte reconciliation. Svelte itself is correct.

## GPUI gap (vs Svelte + contract)

Grid columns now resolve `min_column_width` and cap at `max_columns`; stack forces one column.

- [x] DONE: hardcoded `min_w(px(224.0))` replaced with `min_w(px(rem_to_px(spec.min_column_width_rem())))` — column min now resolves from `spec.min_column_width`.
- [x] DONE: `max_columns` capped via `flex_basis(relative(1.0 / N - 0.01))` + `flex_grow().flex_shrink_0()` (mirrors `form_layout` precedent) so at most N columns fit before wrapping.
- [x] DONE: forwarded builders added (`density`/`layout`/`max_columns`/`min_column_width`/`item_min_column_width`) alongside `from_spec()` + `child()`.
- accepted: `item_min_column_width` descendant forwarding — GPUI has no CSS-custom-property channel to inject `--poodle-detail-section-item-min` into descendant DetailSections; the value lives on the spec but cannot be forwarded without a render-time prop bus.
- accepted: no ARIA (gpui has no accessibility API); no responsive collapse (Svelte has a 34rem container query — host-driven delta).

## Jetstream gap (vs Svelte + contract)

- [x] DONE: `js_detail_section_group(spec, theme, children)` — grid/stack layout, density gap, column min now resolves from `min_column_width` (was `item_min_column_width`, a contract drift) via the shared spec `min_column_width_rem()`. Stack branch added (single column, `w_full`). Registered in lib.rs, probe-tested (4 tests).
- accepted: `max_columns` cap is approximated — JsEl has no percentage flex-basis/max-width, so the column count can't be bounded to N by fraction the way GPUI (`flex_basis(relative(1/N))`) or the Svelte grid does. Wrapping + `min_w` keeps columns legible; hard cap is a JsEl-layout gap.
- [x] DONE: Jetstream specimen `packages/jetstream/preview/src/specimens/detail_section_group.rs` created + registered (`pub mod` + `"detail-section-group"` dispatch arm), covering grid, stack, column-cap, and density variants. Preview crate builds clean.

## Specimen parity

- Svelte covers: grid layout (default, 3 sections auto-fit), stack layout (`itemMinColumnWidth="10rem"`), column cap (`maxColumns={2}`, `minColumnWidth="10rem"`), density variants (compact/default/comfortable).
- GPUI covers: grid layout, stack layout, **column-cap variant** (`maxColumns=2`), **full density coverage** (compact/default/comfortable), each in a labeled group block.
- Jetstream covers: grid layout, stack layout, **column-cap variant** (`maxColumns=2`), **full density coverage** (compact/default/comfortable). Real `js_detail_section` children holding real `js_detail_item` rows.

## Notes

- Two structural gaps: Jetstream component absent entirely, and GPUI's `min_w(px(224.0))` hardcode plus missing maxColumns cap mean GPUI's grid doesn't actually honor the spec's sizing props.
- The `consv=gap` is a spec-builder gap, not a behavioral Svelte divergence — Svelte itself is correct.
