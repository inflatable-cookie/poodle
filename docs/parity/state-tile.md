<!-- parity consv=gap gpui=1 jetstream=8 specimen=gap -->
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

**No authoritative Svelte reference exists** — this is the top contract-level gap. The contract (`docs/contracts/components/state-tile.md:3`) even self-declares "Svelte component not yet built." The only Svelte markup matching the contract anatomy is embedded in `MetricTile.svelte`, which is a *different* component (the contract §8 explicitly distinguishes the lightweight `StateTile` from the heavier `MetricTile`). Treat the embedded markup as a visual hint only, not a contract reference.

Divergences between the contract and the MetricTile-embedded `.poodle-state-tile`:

- **No exported StateTile component.** Contract §1/§6 specify a standalone `StateTile` (`poodle_specs::StateTileSpec`). Svelte ships none. **Fix: extract a real `StateTile.svelte` from MetricTile's markup, or accept that StateTile is Rust-only and note it in the contract.**
- `trend` type. Contract §3 says `trend: string | null` (free-form: `"up"`/`"down"`/arbitrary, §8 cites `"flat"`). MetricTile's prop is a closed union `"up" | "down" | "flat" | null` (`MetricTile.svelte:19`). **Divergence: contract is broader; reconcile once a real Svelte impl lands.**
- `hasSparkline` flag vs `sparklineData`. Contract §3 has `hasSparkline: boolean` (host owns the chart; StateTile only signals). MetricTile instead takes `sparklineData: number[]` and renders the SVG itself (`MetricTile.svelte:21,32-68`). **Divergence: contract says signal-only, MetricTile renders. A faithful StateTile must NOT render sparkline data — only reserve the slot.**
- `density` prop. MetricTile exposes `density` with compact/comfortable variants (`MetricTile.svelte:22,146-158`). Contract §3 lists no `density` prop. **Fix: decide whether StateTile carries the density axis; if yes, add to contract §3 + §5.**
- `ariaLabel` + `aria-label` on root. MetricTile sets `aria-label={ariaLabel ?? \`${label}: ${value}\`}` (`MetricTile.svelte:50`). Contract §7 wants the root accessibility-neutral with label/value as plain text in source order — an auto `aria-label` conflicts with that. **Divergence: contract says neutral; MetricTile labels the root. Resolve per §7.**
- Trend indicator. Contract §2/§7 say trend meaning must live in *text* and any icon is decorative. MetricTile renders icon-only arrows (`trending-up`/`trending-down`/`arrow-right`, `MetricTile.svelte:74`) and only adds text when `trendLabel` is set. **Acceptable** (icon is `aria-hidden`), but contract requires the trend remain legible as text — fine when `trendLabel` present, thin when absent.
- Value typography. Contract §2 maps Value to `typography-heading`. MetricTile value is `font-size: 1rem` plain `<strong>` (`MetricTile.svelte:109-111`). Minor; reconcile token mapping when extracting.

## GPUI gap (vs Svelte + contract)

- [ ] **Implement StateTile in GPUI.** No component builder exists in `packages/gpui/components/src/` (neither composites nor primitives). The only artifact is an adapter stub returning an empty handle (`packages/gpui/adapter/src/render_editing_composites.rs:217-226`) — it draws nothing. Build a real `state_tile.rs` resolving root fill/border/radius/padding, label, value, trend row, and sparkline slot from `StateTileSpec` tokens; add a specimen.
- accepted: no ARIA (gpui has no accessibility API).

## Jetstream gap (vs Svelte + contract)

Component exists (`js_state_tile`) and resolves colors from tokens (`fill_token`/`border_token`/`trend_color_token`, `state_tile.rs:10-15`), but every dimension is a hardcoded rem literal fed to `rem_to_px`, not resolved from the spec/density. Per CLAUDE.md, a hardcoded rem→px is the same sin as a hardcoded px.

- [ ] Hardcoded padding `rem_to_px(1.0)` / `rem_to_px(0.75)` at `state_tile.rs:17-18` — resolve from a spec padding/space token (cf. `button.rs:75` using `control_space_x_rem(spec.density)`), not raw rem literals.
- [ ] Hardcoded gaps `rem_to_px(0.5)` (`state_tile.rs:19`) and trend-row `rem_to_px(0.25)` (`state_tile.rs:45`) — resolve from space tokens, not literals.
- [ ] Hardcoded type sizes: label `rem_to_px(0.75)`, value `rem_to_px(1.5)`, trend `rem_to_px(0.75)` at `state_tile.rs:20-22` — resolve from typography tokens (contract §2 maps Value→`typography-heading`, Label→`typography-label`). Note value is `1.5rem` here vs MetricTile's `1rem`.
- [ ] Hardcoded border width `.border(1.0)` at `state_tile.rs:26` — resolve from a border-width token.
- [ ] Hardcoded sparkline geometry `rem_to_px(2.0)` height + `rem_to_px(0.25)` radius at `state_tile.rs:71,77` — resolve from tokens (and per contract §1/§3 the host owns the sparkline; this draws a placeholder box rather than reserving a host slot).
- [ ] Radius hardcodes the literal token string `"radius.surface"` at `state_tile.rs:12` instead of a `StateTileSpec::radius_token()` method — add the accessor to the spec.
- [ ] No density support. Contract/MetricTile carry compact/comfortable density (different padding/gaps); `js_state_tile` ignores density entirely. Thread `spec.density` once the spec gains it.
- [ ] Spec lacks size/space/typography/radius token methods. `StateTileSpec` (`packages/contracts/components/src/state_tile.rs:38-52`) exposes only `fill_token`/`border_token`/`trend_color_token` — add dimension token accessors so the impl can stop hardcoding rem literals.
- accepted: interaction is N/A — StateTile is wholly static (contract §3), no event surface.

## Specimen parity

- Svelte covers: **nothing** — no `StateTileSpecimen.svelte` exists. (MetricTile is a separate component with its own coverage.)
- GPUI covers: **nothing** — no `specimens/state_tile.rs`. (Only `demo_app.rs:90-92` constructs `StateTileSpec` against the stub adapter, which renders an empty handle — not a real specimen.)
- Jetstream covers: With trend (up + down), With sparkline, Flat trend (`packages/jetstream/preview/src/specimens/state_tile.rs:12-49`). — missing: the **default / no-trend** state (contract §4 `default`), a **neutral arbitrary-string trend** distinct from `"flat"`, and **density** variants (compact/comfortable) once supported.

## Notes

- Two targets (Svelte, GPUI) have **zero real implementation**; Jetstream is the only working render. With no authoritative Svelte reference, the contract stands alone as the spec — `consv=gap` is driven by the missing Svelte component, not by a behavioral conflict.
- The contract deliberately separates StateTile (5 fields, signal-only sparkline) from MetricTile (renders its own sparkline from `sparklineData`). Do not "fix" StateTile by pointing it at MetricTile — they are different grain levels (contract §8).
- Jetstream's trend arrows are unicode glyphs (`↑`/`↓`/`→`, `state_tile.rs:48-52`) rather than registry icons (`trending-up`/`trending-down`/`arrow-right` as MetricTile uses) — registry-coverage choice, not a contract gap; both keep meaning in `trend_label` text per §7.
- The `consv=gap` here is structural (missing component), so there is no `[ ]`-style contract todo list; the actionable contract decision is "extract a real StateTile.svelte or declare StateTile Rust-only."
