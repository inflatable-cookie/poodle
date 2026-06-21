<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok | specimen pass: both Rust previews backfilled to contract §13 coverage — standard (row-header bold + end-aligned Hours), with-caption, minimal key-value, empty state, plus sizes + densities groups; GPUI labels de-scrambled. Both previews build clean. -->
# Parity: Table

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/table.md`
- Svelte (authoritative): `packages/svelte/components/src/Table.svelte`
- GPUI: `packages/gpui/components/src/primitives/table.rs`
- Jetstream: `packages/jetstream/components/src/table.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/TableSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/table.rs` · jetstream `packages/jetstream/preview/src/specimens/table.rs`

## Contract ↔ Svelte

Contract §8 hardcoded resolved values that Svelte renders differently. Svelte is authoritative — contract fixed. The Rust impls followed the old contract numbers, so they inherit the wrong values until re-derived.

- [x] FIXED **Cell/header padding.** Contract §8 said `0.6875rem 0.875rem`; Svelte uses `0.5rem 0.75rem` (`Table.svelte:113`). Contract padding → `0.5rem 0.75rem` (cell, header, empty cell).
- [x] FIXED **Header background formula.** Contract §8 said `surface 60% / background-elevated`; Svelte uses `surface 91% / text-primary` (`Table.svelte:126`). Contract header bg → 91% surface / text-primary (header note + §8 updated).
- [x] FIXED **Table font-size / line-height.** Contract §8 cited body-size/body-lineHeight tokens; Svelte hardcodes `0.8125rem` / `1.5` (`Table.svelte:97`). Contract table type → `0.8125rem` / `1.5`.
- [x] FIXED **Caption padding.** Contract §8 said `var(--poodle-space-panel-y)`; Svelte `0.625rem 0.75rem` (`Table.svelte:102`). Contract caption padding → `0.625rem 0.75rem`.
- [x] FIXED **Shell `aria-label`.** Contract keeps the stronger a11y requirement (`aria-label` on `<table>`); added a §6 note flagging that Svelte currently puts it on the shell `<div>` and should be moved. Svelte-side gap, contract not weakened.
- [x] FIXED **Size/density variants.** Added §8 size adjustment table (xs–xl font + padding-block) and density adjustment table (compact/default/comfortable padding-inline) matching `Table.svelte:148-174`.

## GPUI gap (vs Svelte + contract)

- [x] FIXED Cell padding now resolves from the contract size/density rem scales (`presentation::table_cell_pad_block_rem` / `table_cell_pad_inline_rem`), not raw `0.5`/`0.75` literals.
- [x] FIXED Type sizes resolve from `table_font_rem` (body) / `table_header_font_rem` (header) per effective size; caption uses the contract caption rule (0.8125rem, weight 500, 0.625/0.75 padding); `line_height` is now `relative(1.5)` everywhere (was `1.4`).
- [x] FIXED `size` / `density` support — added `size()` / `with_size_role()` / `with_density()` builders; `into_element` resolves the effective size via `resolve_semantic_size` then scales font + padding-block (size) and padding-inline (density).
- [x] FIXED Header uppercase retained (`to_uppercase()`); header bg now resolves via `header_surface_token()`/`header_mix_text_token()` (surface 91% / text-primary). `letter-spacing: 0.04em` is an accepted approximation — GPUI has no per-run letter-spacing.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored, scope=col/row semantics not emitted (contract §10 GPUI note acknowledges this).
- accepted: `color-mix` approximated via alpha-blend / sRGB mix helper (contract Known Delta).

## Jetstream gap (vs Svelte + contract)

- [x] FIXED Cell padding now from the contract rem scales — `table_cell_pad_block_rem(size)` (vertical) / `table_cell_pad_inline_rem(density)` (horizontal). Stale `0.6875`/`0.875` literals removed.
- [x] FIXED Header background mixed — `color_mix(surface, text_primary, 0.91)` via `header_surface_token()`/`header_mix_text_token()`, matching GPUI + Svelte (`theme_ext::color_mix` already existed). Probe test asserts the resolved fill.
- [x] FIXED Type/size literals — header from `table_header_font_rem`, body/empty from `table_font_rem`; caption is the contract caption rule (0.8125rem, weight 500, 0.625/0.75 padding).
- [x] FIXED Header `text-transform: uppercase` applied (`col.label.to_uppercase()`). `letter-spacing: 0.04em` is an accepted approximation — JsEl has no per-run letter-spacing.
- [x] FIXED `size` / `density` support — `js_table` resolves the effective size via `resolve_semantic_size` and scales font + padding-block (size) / padding-inline (density). Shell also gained `min_w_0` (contract §8 `min-width: 0`).
- accepted: no ARIA channel for scope=col/row (Jetstream has no a11y tree).

## Specimen parity

- Svelte covers: Standard table, With caption, Minimal key-value, Empty state, Sizes (xs–xl snippet), Densities (compact/default/comfortable snippet).
- GPUI covers: Standard, With caption, right-aligned numeric (mislabeled "Minimal key-value"), key-value, Empty. — missing: **Sizes** and **Densities** variant groups (component lacks the props anyway).
- Jetstream covers: With rows (caption), Empty table. — missing: **Minimal key-value**, **Sizes**, **Densities**; only 2 of the contract's specimens present.

## Notes

- The `consv=gap` driver is contract §8 carrying pre-Svelte resolved numbers (padding `0.6875/0.875` vs `0.5/0.75`; header mix `60%/elevated` vs `91%/text-primary`). Both Rust targets faithfully implemented the *contract*, so fixing the contract to match Svelte is the unblock — then re-derive the Rust literals as tokens.
- GPUI specimen's third group is labeled "Minimal key-value" but actually shows a right-aligned numeric table; the fourth group is the real key-value. Cosmetic label mismatch, not a contract gap.
- Both Rust targets render the table as nested flex rows (no native `<table>`), an accepted layout-freedom delta (contract §12).
