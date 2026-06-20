<!-- parity consv=fixed gpui=4 jetstream=4 specimen=gap -->
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
- **`ariaLabel` not modeled in the Rust spec (code track).** Contract §3 + Svelte both expose `ariaLabel` (`Grid.svelte:14,34`). Contract is correct; `GridSpec` lacks an `aria_label` field — a spec-side code change, not a contract divergence.
- **`gap`/`padding` token map: contract already matches Svelte.** Verified contract §8 / §3 `SpaceScale` map equals Svelte `scaleToSpace` (`internal.ts:14-25`): `none`→`0`, `sm`→`space-inline-sm`, `md`→`space-panel-y`, `lg`→`space-panel-x`. No contract change needed. The Rust `GridSpec` resolving via `inline_gap`/`layout_inset` (wrong scale) is a code-track fix.
- **Single gap → two-axis gap (code track).** Svelte/CSS `gap` is one value on both axes; the contract documents one `gap` map. The Rust split into `resolved_column_gap`/`resolved_row_gap` resolving to different tokens is a code-track fix, not a contract divergence.

## GPUI gap (vs Svelte + contract)

CSS grid is emulated with `flex().flex_wrap()` + `flex_1()` children (`grid.rs:105,127`). No raw pixel/color literals in the component (clean). Open items:

- [ ] Gap resolves from the wrong token scale — `gap_x`/`gap_y` use `resolved_column_gap`/`resolved_row_gap` (`grid.rs:108-114`) which map md→`space-inline-md`/`space-stack-md`, not the contract `space-panel-y`. Fix at the spec (see Contract↔Svelte), then GPUI inherits the correct value.
- [ ] Padding resolves from the wrong scale — `resolved_padding()` → `layout_inset` (`grid.rs:102,117-122`) gives inline/stack, not the contract `space-panel-*`. Same spec-level fix.
- [ ] `columns`/`rows` track definitions not honored — `column_count()` parses a track count (`grid.rs:73-93`) but the result is discarded (`let _col_count`, `grid.rs:103`); every child gets uniform `flex_1()` (`grid.rs:127`). Mixed widths (`1fr 2fr`) and `repeat(auto-fit, minmax(8rem,1fr))` collapse to equal columns. Honor track ratios or document as accepted Taffy limit.
- [ ] `rows` prop ignored entirely — builder stores it (`grid.rs:48-51`) but `into_element` never reads `spec.rows`; no row-track sizing applied.
- accepted: no ARIA (gpui has no accessibility API) — `role`/`aria_label` not emitted; contract §6 ARIA opt-in unreachable.
- accepted: CSS grid → flex-wrap approximation is a platform-owned internal (contract §11 Tier 3).

## Jetstream gap (vs Svelte + contract)

CSS grid emulated with `div().flex_row().flex_wrap()` (`grid.rs:16-18`). No raw pixel/color literals in the component (clean). Open items:

- [ ] Gap resolves from the wrong token scale — `gap` uses `resolved_column_gap` (`grid.rs:20-22`) → md=`space-inline-md`, not contract `space-panel-y`. Spec-level fix (see Contract↔Svelte).
- [ ] Padding resolves from the wrong scale — `resolved_padding()` → `layout_inset` (`grid.rs:14,24-31`) gives inline/stack, not `space-panel-*`. Same spec-level fix.
- [ ] `columns`/`rows` track definitions not honored — `js_grid` never reads `spec.columns`; children flow with their own width into a flex-wrap row (`grid.rs:33-35`). No equal-column or ratio behavior; `1fr 2fr` and `repeat(auto-fit,…)` are not reproduced. Add track-ratio sizing or document as accepted Taffy limit.
- [ ] `rows` prop ignored — `spec.rows` never read; no row-track sizing. (Also no row-gap: Jetstream applies a single `.gap()`, so row spacing differs from GPUI which applies `gap_y`.)
- accepted: no ARIA channel — `role`/`aria_label` not emitted.
- accepted: CSS grid → flex-wrap approximation is platform-owned (contract §11 Tier 3).

## Specimen parity

- Svelte covers: **Three columns** (`1fr 1fr 1fr`, gap md), **Mixed column widths** (`1fr 2fr`, gap md), **Auto-fit responsive** (`repeat(auto-fit, minmax(8rem,1fr))`, gap sm) — all three contract §13 groups, with real `Surface` children (`GridSpecimen.svelte:7-30`).
- GPUI covers: all three groups — Three columns, Mixed column widths, Auto-fit responsive — with real `Surface` children (`gpui/.../grid.rs:42-107`). Visual fidelity limited by the equal-column flex approximation (mixed/auto-fit render as equal columns), but coverage is complete.
- Jetstream covers: **one** group only — "Default grid" with a 6-cell default `GridSpec::new()` (`jetstream/.../grid.rs:19-25`). **Missing: Three columns, Mixed column widths, Auto-fit responsive** (all contract §13 groups). Cells are ad-hoc `label()` with hand-coded `px/py/bg/rounded` (`grid.rs:16-17`), not `Surface` children — does not mirror Svelte. **Fix: add the three contract groups using real `js_surface` children, mirroring Svelte.**

## Notes

- Component code in both Rust targets is free of hardcoded pixel/color literals — token violations here are *resolution-path* errors (wrong scale via `inline_gap`/`layout_inset`) at the `GridSpec` level, not raw float/HSLA literals in the builders. The grep for `\.(h|w|gap|p|px|py|…)(<float>)` and `hsla|rgba?\(` returned nothing on both component files.
- Jetstream specimen cell styling (`px(rem_to_px(0.75))`, `bg(tint(accent,0.10))`, `rounded(rem_to_px(0.25))`, `grid.rs:16-17`) uses literal rem/tint values inline. This is specimen scaffolding, not component code, but it sidesteps `Surface`/token resolution and should be replaced with real `Surface` children when the missing specimen groups are added.
- The big `consv=gap` drivers: (1) the gap/padding token map in `GridSpec` resolves the wrong scale vs the contract/Svelte authority, and (2) `ariaLabel` is absent from the shared spec. Both belong fixed at the spec layer so all three targets inherit the correction.
- Layout-mapping limit (informational, not a todo): CSS grid track syntax — `repeat(auto-fit, minmax(…))`, explicit row tracks, named areas — has no Taffy equivalent. Both Rust targets fall back to flex-wrap, which the contract sanctions as Tier-3 implementation freedom (§11). The honest delta is mixed-ratio columns (`1fr 2fr`) collapsing to equal widths; worth either a track-ratio emulation pass or an explicit §12 Known Delta entry (currently §12 says `none`).
