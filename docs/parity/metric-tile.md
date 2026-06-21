<!-- parity consv=ok gpui=1 jetstream=1 specimen=gap pass=fixed-pass-41 -->
# Parity: MetricTile

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/metric-tile.md`
- Svelte (authoritative): `packages/svelte/components/src/MetricTile.svelte`
- GPUI: `packages/gpui/components/src/composites/metric_tile.rs`
- Jetstream: `packages/jetstream/components/src/metric_tile.rs`
- Spec: `packages/contracts/components/src/metric_tile.rs` (`MetricTileSpec`, `MetricTrend`)
- Specimens: svelte `packages/svelte/preview/src/specimens/MetricTileSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/metric_tile_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/metric_tile.rs`

## Contract ↔ Svelte

Contract and Svelte agree on props (label, value, ariaLabel, trend, trendLabel, sparklineData, density), anatomy (Root/Label/Body/Value/Sparkline/Trend/TrendArrow/TrendLabel), trend icon mapping, sparkline algorithm, and accessible name. No public-API divergence → `consv=ok`. Minor spec-drift notes (Rust spec, not Svelte):

- [x] FIXED `MetricTileSpec` now has a `density: ControlDensity` field (additive) plus density-resolved rem accessors (`root_gap_rem`/`padding_x_rem`/`padding_y_rem`/`body_gap_rem`) matching the contract §8 density table. Both Rust targets honor density.
- [x] FIXED `MetricTrend::Flat` color token → `COLOR_TEXT_TERTIARY` (was `COLOR_TEXT_SECONDARY`), matching contract §8 + Svelte.
- [x] FIXED `MetricTileSpec::gap_token()` → `SPACE_INLINE_SM` (was `SPACE_STACK_SM`); density variants override via `root_gap_rem`.
- Additive spec accessors also added: `sparkline_color_token`, `sparkline_width_rem`/`sparkline_height_rem`, `trend_font_size_rem`, `trend_arrow_font_size_rem`, `trend_gap_rem`, `border_width_rem`.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] FIXED Trend chip background removed — trend is now a plain inline icon+label row in the trend tone color (no chip bg, no hardcoded `Hsla` literal).
- [x] FIXED Trend moved to its own row below the body (was inline beside the value); matches contract §2 + Svelte.
- [x] FIXED Value font size now `value_font_size_rem()` (1rem), no longer `typography.heading.size`.
- [x] FIXED All px literals replaced with `px(rem_to_px(...))` of contract-exact rem (label 0.75rem, value 1rem, trend 0.75rem, trend-arrow 0.875rem, density-resolved root/body gaps + padding, transparent 0.0625rem border).
- [x] FIXED Sparkline constrained to 4rem×1.5rem and colored `color.text.tertiary`. Still a bar-strip approximation (GPUI has no raw SVG/polyline) — Tier-3 rendering substitution, dims/color now contract-exact. (note)
- [x] FIXED Sparkline color → `sparkline_color_token()` = `color.text.tertiary`.
- [x] FIXED `density` now supported via the new spec field + density-resolved rem accessors.
- [ ] No light-theme surface-fill override (contract §8 light theme → `treatment-surface-fill`). **Token gap:** no `treatment-surface-fill` semantic token exists in the Rust token set, so neither Rust target has a light-theme override path. GPUI uses the same `fill_token()` regardless of theme. (blocked on a new token)
- accepted: no ARIA (gpui has no accessibility API) — `aria-label`/`aria-hidden` not emitted.

## Jetstream gap (vs Svelte + contract)

- [x] FIXED Sparkline implemented — body now renders a fixed 4rem×1.5rem `color.text.tertiary` bar-strip approximation beside the value (JsEl has no SVG/polyline path; Tier-3 substitution, dims/color contract-exact). The "With sparklines" specimen group can now exist. (note)
- [x] FIXED Root border is now `0.0625rem solid transparent` (resolved width via `border_width_rem()`, fully transparent color), replacing the `border(1.0)` + visible subtle border color.
- [x] FIXED All px values resolve from rem accessors: border-width `border_width_rem()`, trend-arrow `trend_arrow_font_size_rem()` (0.875rem), trend gap `trend_gap_rem()` (0.25rem), density-resolved padding/gaps.
- [x] CONFIRMED Trend label uses `trend_font_size_rem()` (0.75rem); arrow icon sized `trend_arrow_font_size_rem()` (0.875rem) — matches contract §8.
- [x] FIXED `density` now supported via the spec field + density-resolved rem accessors.
- [ ] No light-theme surface-fill override (contract §8). **Token gap:** no `treatment-surface-fill` semantic token in the Rust set; same surface fill in both themes until that token is added. (blocked on a new token)
- accepted: interaction is n/a (display-only component); no event loop involvement needed.

## Specimen parity

- Svelte covers: **Basic tiles** (4: Components/Coverage/Open issues/Build time), **Trend indicators** (up/down/flat/up with labels), **Sparklines** (3, incl. one sparkline-without-trend "Memory"), **Density demo** (compact/default/comfortable via `densities` snippet), responsive grid. (`MetricTileSpecimen.svelte`)
- GPUI covers: Basic (4), Trend (up/down/flat), Sparkline (2). — **missing: sparkline-without-trend case, density demo.** Labels/values diverge from contract §12 (uses "Total Users"/"12,847" etc. instead of contract's "Components"/"85") — cosmetic, not a contract gap. (`gpui/.../metric_tile_specimen.rs`)
- Jetstream covers: Plain (3), Trend Up/Down/Flat (with labels), **Trend without label** (extra case). — **missing: any sparkline specimen (component can't render one), density demo.** (`jetstream/.../metric_tile.rs`)

## Notes

- `consv=ok`: the public Svelte API matches the contract. All open work is in the Rust specs + impls.
- Biggest GPUI defect: the contract-invented trend "chip" (background + padding + rounding via hardcoded literals) instead of Svelte's plain inline trend row, plus trend placed beside the value rather than on its own row below the body.
- Biggest Jetstream defect: sparkline is entirely unimplemented (placeholder comment only), so the "With sparklines" specimen group cannot exist.
- Both Rust targets share two structural blockers: `MetricTileSpec` has no `density` field and no light-theme override path, so neither density nor light-theme surface-fill (both in contract §3/§8) can be honored until the spec grows them.
