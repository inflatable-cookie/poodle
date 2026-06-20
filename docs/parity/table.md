<!-- parity consv=gap gpui=4 jetstream=5 specimen=gap -->
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

Contract §8 hardcodes resolved values that Svelte renders differently. Svelte is authoritative — fix the contract. The Rust impls followed the contract numbers, so they inherit the wrong values.

- **Cell/header padding.** Contract §8 says `padding: 0.6875rem 0.875rem`; Svelte uses `0.5rem 0.75rem` (`Table.svelte:113`). **Fix: contract padding → `0.5rem 0.75rem`.**
- **Header background formula.** Contract §8 says `color-mix(in srgb, var(--poodle-surface) 60%, var(--poodle-color-background-elevated))`; Svelte uses `color-mix(in srgb, var(--poodle-surface) 91%, var(--poodle-color-text-primary))` (`Table.svelte:126`). **Fix: contract header bg → 91% surface / text-primary.**
- **Table font-size / line-height.** Contract §8 `font-size: var(--poodle-typography-body-size)`, `line-height: var(--poodle-typography-body-lineHeight)`; Svelte hardcodes `font-size: 0.8125rem`, `line-height: 1.5` (`Table.svelte:97`). **Fix: contract table type → `0.8125rem` / `1.5` (or document the token resolves to these).**
- **Caption padding.** Contract §8 `padding: var(--poodle-space-panel-y)`; Svelte `0.625rem 0.75rem` (`Table.svelte:102`). **Fix: contract caption padding → `0.625rem 0.75rem`.**
- **Shell `aria-label`.** Contract §6 says `aria-label` goes on `<table>` when no caption; Svelte puts it on the `.poodle-table-shell` `<div>` (`Table.svelte:34`), not the `<table>`. **Fix: either move aria-label to `<table>` in Svelte (better a11y) or update contract §6 to say shell carries it.** (Svelte placement is weaker — flag, lean toward fixing Svelte.)
- **Size/density variants.** Svelte ships `data-size` (xs/sm/lg/xl) font + padding-block scaling and `data-density` padding-inline scaling (`Table.svelte:148-174`); contract §3 lists the props but §8 has no size/density token table. **Fix: add size/density adjustment tables to contract §8.**

## GPUI gap (vs Svelte + contract)

- [ ] Hardcoded cell padding `px(rem_to_px(0.5))` / `px(rem_to_px(0.75))` at `table.rs:130-131` — should resolve from a cell-padding token, not raw rem literals.
- [ ] Hardcoded type sizes: header `px(rem_to_px(0.6875))` (`table.rs:173`), caption `px(rem_to_px(0.8125))` (`table.rs:151`), caption `py(px(rem_to_px(0.625)))` (`table.rs:149`), and `line_height(relative(1.4))` throughout — resolve from tokens (Svelte uses `1.5` for body, so `1.4` is also wrong).
- [ ] No `size` / `density` support — builder has no size/density methods and `into_element` ignores them; Svelte scales font + padding per size and padding-inline per density.
- [ ] No `text-transform: uppercase` letter-spacing parity — GPUI uppercases the label string (`table.rs:181`) but never applies `letter-spacing: 0.04em` from contract §8.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored, scope=col/row semantics not emitted (contract §10 GPUI note acknowledges this).
- accepted: `color-mix` approximated via alpha-blend helper (contract Known Delta).

## Jetstream gap (vs Svelte + contract)

- [ ] Wrong cell padding — uses contract's stale `rem_to_px(0.6875)` / `rem_to_px(0.875)` at `table.rs:37-38`; Svelte is `0.5rem` / `0.75rem`. Resolve from a token once contract is fixed.
- [ ] Header background not mixed — `header_fill = resolve_color(spec.header_fill_token())` raw at `table.rs:24`; Svelte mixes `surface 91% / text-primary`. GPUI does the mix (`color_mix(surface, text_primary, 0.91)`), Jetstream does not. **Add the mix.**
- [ ] Hardcoded type/size literals: header `rem_to_px(0.6875)` (`table.rs:39`), caption `rem_to_px(0.75)` (`table.rs:40`, Svelte caption is `0.8125rem`), cell/empty `rem_to_px(0.8125)` (`table.rs:98,121`) — resolve from tokens.
- [ ] No `letter-spacing`/`text-transform` parity — header label not uppercased and `0.04em` spacing absent (`table.rs:71`). Svelte uppercases + spaces headers.
- [ ] No `size` / `density` support — `js_table` ignores `spec.size`/`spec.density`; no per-size font/padding scaling.
- accepted: no ARIA channel for scope=col/row (Jetstream has no a11y tree).

## Specimen parity

- Svelte covers: Standard table, With caption, Minimal key-value, Empty state, Sizes (xs–xl snippet), Densities (compact/default/comfortable snippet).
- GPUI covers: Standard, With caption, right-aligned numeric (mislabeled "Minimal key-value"), key-value, Empty. — missing: **Sizes** and **Densities** variant groups (component lacks the props anyway).
- Jetstream covers: With rows (caption), Empty table. — missing: **Minimal key-value**, **Sizes**, **Densities**; only 2 of the contract's specimens present.

## Notes

- The `consv=gap` driver is contract §8 carrying pre-Svelte resolved numbers (padding `0.6875/0.875` vs `0.5/0.75`; header mix `60%/elevated` vs `91%/text-primary`). Both Rust targets faithfully implemented the *contract*, so fixing the contract to match Svelte is the unblock — then re-derive the Rust literals as tokens.
- GPUI specimen's third group is labeled "Minimal key-value" but actually shows a right-aligned numeric table; the fourth group is the real key-value. Cosmetic label mismatch, not a contract gap.
- Both Rust targets render the table as nested flex rows (no native `<table>`), an accepted layout-freedom delta (contract §12).
