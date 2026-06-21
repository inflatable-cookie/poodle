<!-- parity consv=gap gpui=0 jetstream=0 specimen=ok -->
<!-- pass: GPUI StateTile built (composites/state_tile.rs, mirrors MetricTile); Jetstream rebuilt to contract — value now typography-heading (was 1.5rem), radius/border/typography/padding token-resolved via new spec methods, sparkline reserves a host slot (no synthetic data). Svelte still absent → consv=gap (sole authority is the contract). -->
# Parity: StateTile

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/state-tile.md`
- Svelte (authoritative): `packages/svelte/components/src/StateTile.svelte` — **MISSING.** No standalone `StateTile.svelte` exists and nothing is exported from `index.ts`. The `.poodle-state-tile` markup + CSS lives baked inside `packages/svelte/components/src/MetricTile.svelte:50-163` (not a reusable component).
- GPUI: `packages/gpui/components/src/{composites,primitives}/state_tile.rs` — **MISSING.** No component builder in `packages/gpui/components/src/`. Only an adapter stub exists at `packages/gpui/adapter/src/render_editing_composites.rs:217-226`, which returns a placeholder `GpuiElementHandle::new("state-tile", "StateTileSpec")` and renders nothing.
- Jetstream: `packages/jetstream/components/src/state_tile.rs` (exists, `js_state_tile`)
- Rust spec: `packages/contracts/components/src/state_tile.rs` (`StateTileSpec`)
- Specimens: svelte **none** (no `StateTileSpecimen.svelte`) · gpui **none** (no `specimens/state_tile.rs`) · jetstream `packages/jetstream/preview/src/specimens/state_tile.rs`

## Contract ↔ Svelte

**Svelte authority missing** — no `StateTile.svelte` exists and nothing is exported from `index.ts`. Per the parity rule ("If NO Svelte file: STOP, contract is sole authority, do NOT change it"), the contract is left unchanged and `consv` stays `gap`. This is the top contract-level gap. The contract (`docs/contracts/components/state-tile.md:3`) even self-declares "Svelte component not yet built." The only Svelte markup matching the contract anatomy is embedded in `MetricTile.svelte`, which is a *different* component (the contract §8 explicitly distinguishes the lightweight `StateTile` from the heavier `MetricTile`). Treat the embedded markup as a visual hint only, not a contract reference — it is NOT an authority for editing the StateTile contract.

Divergences between the contract and the MetricTile-embedded `.poodle-state-tile`:

- **No exported StateTile component.** Contract §1/§6 specify a standalone `StateTile` (`poodle_specs::StateTileSpec`). Svelte ships none. **Fix: extract a real `StateTile.svelte` from MetricTile's markup, or accept that StateTile is Rust-only and note it in the contract.**
- `trend` type. Contract §3 says `trend: string | null` (free-form: `"up"`/`"down"`/arbitrary, §8 cites `"flat"`). MetricTile's prop is a closed union `"up" | "down" | "flat" | null` (`MetricTile.svelte:19`). **Divergence: contract is broader; reconcile once a real Svelte impl lands.**
- `hasSparkline` flag vs `sparklineData`. Contract §3 has `hasSparkline: boolean` (host owns the chart; StateTile only signals). MetricTile instead takes `sparklineData: number[]` and renders the SVG itself (`MetricTile.svelte:21,32-68`). **Divergence: contract says signal-only, MetricTile renders. A faithful StateTile must NOT render sparkline data — only reserve the slot.**
- `density` prop. MetricTile exposes `density` with compact/comfortable variants (`MetricTile.svelte:22,146-158`). Contract §3 lists no `density` prop. **Fix: decide whether StateTile carries the density axis; if yes, add to contract §3 + §5.**
- `ariaLabel` + `aria-label` on root. MetricTile sets `aria-label={ariaLabel ?? \`${label}: ${value}\`}` (`MetricTile.svelte:50`). Contract §7 wants the root accessibility-neutral with label/value as plain text in source order — an auto `aria-label` conflicts with that. **Divergence: contract says neutral; MetricTile labels the root. Resolve per §7.**
- Trend indicator. Contract §2/§7 say trend meaning must live in *text* and any icon is decorative. MetricTile renders icon-only arrows (`trending-up`/`trending-down`/`arrow-right`, `MetricTile.svelte:74`) and only adds text when `trendLabel` is set. **Acceptable** (icon is `aria-hidden`), but contract requires the trend remain legible as text — fine when `trendLabel` present, thin when absent.
- Value typography. Contract §2 maps Value to `typography-heading`. MetricTile value is `font-size: 1rem` plain `<strong>` (`MetricTile.svelte:109-111`). Minor; reconcile token mapping when extracting.

## GPUI gap (vs Svelte + contract)

- [x] **StateTile built in GPUI.** `packages/gpui/components/src/composites/state_tile.rs` (`StateTile`, registered in `composites/mod.rs`). Mirrors `MetricTile`: resolves fill/border/radius/border-width, label, value (typography-heading), an optional decorative trend row (glyph + label text), and an optional reserved sparkline slot (host-owned — no synthetic chart data) — all from `StateTileSpec` token methods. Build-verified (`cargo build` in gpui/components). The adapter `RenderComponent<StateTileSpec>` stub at `render_editing_composites.rs:217-226` is left unchanged (matches the established pattern — real component lives in the components crate).
- accepted: no ARIA (gpui has no accessibility API).
- note: no GPUI specimen added — gpui/preview shares a target lock and is build-skipped this pass.

## Jetstream gap (vs Svelte + contract)

Component rebuilt to the contract. Colors and dimensions now resolve from spec token methods; the value typography is corrected to `typography-heading`.

- [x] Padding resolves from `space.panel.x` / `space.panel.y` (was `rem_to_px(1.0)` / `rem_to_px(0.75)`).
- [x] Gaps resolve from tokens — root `space.stack.sm`, trend-row `space.inline.xs` (were `rem_to_px(0.5)` / `rem_to_px(0.25)`).
- [x] Type sizes resolve from spec token methods — label `label_font_size_token()` (`typography.label.size`), value `value_font_size_token()` (`typography.heading.size` = 1rem, corrected from `1.5rem`), trend `trend_font_size_token()`.
- [x] Border width resolves from `border_width_token()` (`BORDER_WIDTH_DEFAULT`), was `.border(1.0)`.
- [x] Sparkline now reserves a **host slot** (contract §1/§2/§3 — host owns the chart). It renders an empty token-filled area (`sparkline_slot_token()` → `COLOR_BACKGROUND_SURFACE`, TOKEN GAP noted: no chart-surface token) with an `id` (`state-tile-sparkline`). note: slot height (2.0rem) is a contract-exact rem via `rem_to_px` (no token); radius reuses the root radius token.
- [x] Radius uses `StateTileSpec::radius_token()` (added), not the literal `"radius.surface"` string.
- [~] Density: `StateTileSpec` still has no `density` field (contract §3 lists no density prop — adding one is a contract decision, out of scope for this build-out pass). Not threaded.
- [x] Spec gained dimension/typography/radius token accessors (`radius_token`, `border_width_token`, `label_color_token`, `value_color_token`, `value_font_size_token`, `label_font_size_token`, `trend_font_size_token`, `sparkline_slot_token`, `trend_glyph`) — additive, contract-traceable.
- accepted: interaction is N/A — StateTile is wholly static (contract §3), no event surface.

## Specimen parity

- Svelte covers: **nothing** — no `StateTileSpecimen.svelte` exists. (MetricTile is a separate component with its own coverage.)
- GPUI covers: **nothing** — component built, but no `specimens/state_tile.rs` (gpui/preview build-skipped this pass).
- Jetstream covers: **Default (no trend)** ×2, With trend (up + down), **Neutral trend** (`"flat"` + arbitrary `"steady"`), With sparkline (reserved slot). Covers contract §4 default / up / down / neutral-string / with-sparkline. — missing only **density** variants (not in the contract; out of scope).

## Notes

- Two targets (Svelte, GPUI) have **zero real implementation**; Jetstream is the only working render. With no authoritative Svelte reference, the contract stands alone as the spec — `consv=gap` is driven by the missing Svelte component, not by a behavioral conflict.
- The contract deliberately separates StateTile (5 fields, signal-only sparkline) from MetricTile (renders its own sparkline from `sparklineData`). Do not "fix" StateTile by pointing it at MetricTile — they are different grain levels (contract §8).
- Jetstream's trend arrows are unicode glyphs (`↑`/`↓`/`→`, `state_tile.rs:48-52`) rather than registry icons (`trending-up`/`trending-down`/`arrow-right` as MetricTile uses) — registry-coverage choice, not a contract gap; both keep meaning in `trend_label` text per §7.
- The `consv=gap` here is structural (missing component), so there is no `[ ]`-style contract todo list; the actionable contract decision is "extract a real StateTile.svelte or declare StateTile Rust-only."
