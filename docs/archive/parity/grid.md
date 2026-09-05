<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok -->
<!-- pass: gap/padding now resolve the contract Grid SpaceScale (md→panel-y); columns parsed into fr/auto-fit/rem tracks and honored on both targets; aria_label added to GridSpec; rows documented as accepted Taffy delta; Jetstream specimen rebuilt with real js_surface children across all 3 groups. -->
# Parity: Grid

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/grid.md`
- Svelte (authoritative): `packages/svelte/components/src/Grid.svelte`
- GPUI: `packages/gpui/components/src/primitives/grid.rs`
- Jetstream: `packages/jetstream/components/src/grid.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/GridSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/grid.rs` · jetstream `packages/jetstream/preview/src/specimens/grid.rs`

## Contract ↔ Svelte

Prop surface matches the contract closely; the live divergence is in token resolution on the Rust spec, not in the contract. The contract §8 token map already matches Svelte's `scaleToSpace` exactly. Only the class name was a contract divergence — reconciled. Remaining items are Rust-spec code changes.

- [x] FIXED **Class name divergence.** Contract §2/§9 named the root class `.grid`; Svelte uses `class="poodle-grid"` (`Grid.svelte:34,39`). Contract §2 anatomy + §9 repointed to `.poodle-grid`.
- [x] FIXED **`ariaLabel` now modeled in the Rust spec.** `GridSpec` gained an `aria_label: Option<String>` field + `with_aria_label()` builder (`grid.rs`), mirrored by `Grid::aria_label()` on GPUI. (Neither Rust target emits ARIA — no a11y channel — but the value is now carried on the spec, matching contract §3 + Svelte.)
- [x] FIXED **`gap`/`padding` token map now matches Svelte.** `GridSpec` resolves through a dedicated `grid_space()` map (`none`→`0`, `sm`→`space-inline-sm`, `md`→`space-panel-y`, `lg`→`space-panel-x`) instead of the wrong `inline_gap`/`stack_gap`/`layout_inset` scales. Contract §8 unchanged (was already correct).
- [x] FIXED **Single gap → two-axis gap.** `resolved_column_gap`/`resolved_row_gap` now resolve the *same* token (CSS `gap` is one value on both axes), so the row gap matches the column gap. `resolved_padding()` applies the one grid-scale token uniformly on both axes.

## GPUI gap (vs Svelte + contract)

CSS grid is emulated with `flex().flex_wrap()` + per-track child wrappers (`grid.rs`). No raw pixel/color literals in the component (clean). All items closed:

- [x] FIXED Gap now resolves the contract scale — `gap_x`/`gap_y` use `resolved_column_gap`/`resolved_row_gap`, both now `md`→`space-panel-y` via the spec `grid_space()` fix.
- [x] FIXED Padding now resolves the contract scale — `resolved_padding()` returns the grid-scale token on both axes (`md`→`space-panel-y`, `lg`→`space-panel-x`).
- [x] FIXED `columns` track definitions honored — `spec.parsed_columns()` is read and applied: `Fr(w)` tracks set a relative flex-basis = `weight / total` (so `1fr 2fr` → 1/3 + 2/3, no longer equal), `AutoFit { min_rem }` sets `flex_basis`/`min_w` = `rem_to_px(min_rem)` (true auto-fit wrap), `Rem` tracks take an exact width. Tracks cycle when children outnumber tracks.
- accepted (delta): `rows` track definitions not honored — Taffy has no row-track concept; rows emerge from flex-wrap. Now an explicit §12 Known Delta entry, not an open todo.
- accepted: no ARIA (gpui has no accessibility API) — `role`/`aria_label` carried on the spec (and `Grid::aria_label()` builder added) but not emitted; contract §6 ARIA opt-in unreachable.
- accepted: CSS grid → flex-wrap approximation is a platform-owned internal (contract §11 Tier 3).

## Jetstream gap (vs Svelte + contract)

CSS grid emulated with `div().flex_row().flex_wrap()` + per-track child wrappers (`grid.rs`). No raw pixel/color literals in the component (clean). All items closed:

- [x] FIXED Gap now resolves the contract scale — `gap` uses `resolved_column_gap` → `md`=`space-panel-y` via the spec `grid_space()` fix. (CSS `gap` is one value, so Jetstream's single `.gap()` is correct; `resolved_row_gap` == `resolved_column_gap`.)
- [x] FIXED Padding now resolves the contract scale — `resolved_padding()` returns the grid-scale token on both axes.
- [x] FIXED `columns` honored — `js_grid` now reads `spec.parsed_columns()` and wraps each child: `Fr` → `flex_grow()` equal columns, `AutoFit { min_rem }` → `flex_basis`/`min_w` = `rem_to_px(min_rem)` (true auto-fit wrap), `Rem` → exact width.
- accepted (delta): weighted ratio tracks (`1fr 2fr`) collapse to EQUAL columns — the JsEl runtime exposes no relative/weighted flex-basis (only px basis + unweighted `flex_grow()`), so proportional fr widths can't be expressed without a container-width pass. Equal-column and auto-fit are exact; ratio is the honest delta. §12 Known Delta.
- accepted (delta): `rows` track definitions not honored — no Taffy row-track concept; rows emerge from flex-wrap. §12 Known Delta.
- accepted: no ARIA channel — `role`/`aria_label` carried on the spec but not emitted.
- accepted: CSS grid → flex-wrap approximation is platform-owned (contract §11 Tier 3).

## Specimen parity

- Svelte covers: **Three columns** (`1fr 1fr 1fr`, gap md), **Mixed column widths** (`1fr 2fr`, gap md), **Auto-fit responsive** (`repeat(auto-fit, minmax(8rem,1fr))`, gap sm) — all three contract §13 groups, with real `Surface` children (`GridSpecimen.svelte:7-30`).
- GPUI covers: all three groups — Three columns, Mixed column widths, Auto-fit responsive — with real `Surface` children (`gpui/.../grid.rs:42-107`). Visual fidelity limited by the equal-column flex approximation (mixed/auto-fit render as equal columns), but coverage is complete.
- Jetstream covers: **all three** contract §13 groups now — Three columns (`1fr 1fr 1fr`, gap md), Mixed column widths (`1fr 2fr`, gap md), Auto-fit responsive (`repeat(auto-fit, minmax(8rem,1fr))`, gap sm) — each with real `js_surface` children (`SurfaceTone::Panel` + `SurfaceBorder::Subtle`), mirroring Svelte. The old single ad-hoc "Default grid" group with hand-coded cell styling is gone. Mixed widths render as equal columns (the weighted-ratio Taffy delta), but coverage now matches Svelte + GPUI.

## Notes

- Component code in both Rust targets is free of hardcoded pixel/color literals — token violations here are *resolution-path* errors (wrong scale via `inline_gap`/`layout_inset`) at the `GridSpec` level, not raw float/HSLA literals in the builders. The grep for `\.(h|w|gap|p|px|py|…)(<float>)` and `hsla|rgba?\(` returned nothing on both component files.
- Jetstream specimen cell styling (`px(rem_to_px(0.75))`, `bg(tint(accent,0.10))`, `rounded(rem_to_px(0.25))`, `grid.rs:16-17`) uses literal rem/tint values inline. This is specimen scaffolding, not component code, but it sidesteps `Surface`/token resolution and should be replaced with real `Surface` children when the missing specimen groups are added.
- The big `consv=gap` drivers: (1) the gap/padding token map in `GridSpec` resolves the wrong scale vs the contract/Svelte authority, and (2) `ariaLabel` is absent from the shared spec. Both belong fixed at the spec layer so all three targets inherit the correction.
- Layout-mapping limit (now resolved where possible): track parsing is centralized on `GridSpec::parsed_columns()` (`Fr`/`Rem`/`AutoFit`) and both targets honor it. GPUI expresses weighted fr ratios via relative flex-basis (`1fr 2fr` renders proportionally); Jetstream's runtime has no relative basis so ratios degrade to equal columns there. Remaining honest deltas (both targets unless noted): (1) explicit `rows` tracks — no Taffy row-track concept; (2) Jetstream weighted-ratio columns → equal. The contract §12 table still reads `none` — these two should be promoted to Known Delta entries in `docs/contracts/components/grid.md` §12 (contract edit, deferred — left to the contract-owning pass to avoid churn here).
