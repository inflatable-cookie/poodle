<!-- parity consv=ok gpui=8 jetstream=5 specimen=gap -->
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

- `MetricTileSpec` has **no `density` field** (`metric_tile.rs:38-54`); contract §3 + Svelte expose `density` (compact/default/comfortable) driving padding + gaps. Rust spec gap — neither Rust target can honor density. Document as a known spec gap (not a Svelte fix).
- `MetricTrend::Flat` color token is `COLOR_TEXT_SECONDARY` (`metric_tile.rs:23`); contract §8 + Svelte flat color is `--poodle-color-text-tertiary`. Spec is slightly off. **Fix: flat → `COLOR_TEXT_TERTIARY`.**
- `MetricTileSpec::gap_token()` returns `SPACE_STACK_SM` (`metric_tile.rs:139-141`); contract §8 root gap is `space.inline.sm`. Both resolve to 8px today so it's visually equal, but semantically wrong. **Fix: gap_token → `SPACE_INLINE_SM`.**

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] **Hardcoded color literal** for the trend background: `Hsla { a: trend_color.a * 0.12, ..trend_color }` at `metric_tile.rs:114-117` — derived inline, not from a token. Also the whole trend-chip background treatment is **not in the contract** (Svelte trend has no background/chip — it's an inline-flex text+icon row). **Remove the chip background or back it with a token.**
- [ ] **Wrong anatomy placement of trend.** GPUI puts the trend inline in the `value_row` beside the value (`metric_tile.rs:102-147`). Contract §2 + Svelte place `.state-tile__trend` as a **separate row below the body**, not next to the value. Move trend to its own child after the value row.
- [ ] **Value font size wrong.** Uses `typography.heading.size` (`metric_tile.rs:84, 108`); contract §8 value is `1rem` (body). Use a 1rem token, not heading size.
- [ ] **Hardcoded px literals throughout:** label `text_size(px(12.0))` + `line_height(relative(1.3))` (`:97-99`); value `line_height(relative(1.2))` (`:108`); value_row `gap(px(8.0))` (`:103`); trend chip `gap(px(4.0))`, `px(6.0))`, `py(px(2.0))`, `rounded(px(10.0))`, `text_size(px(12.0))` (`:122-128`); sparkline `chart_height px(24.0)`, `bar_width px(3.0)`, `bar_gap px(1.0)`, `mt(px(4.0))`, `rounded(px(1.0))` (`:159-185`). All must resolve from tokens (label 0.75rem, trend 0.75rem, body gap space.inline.md, etc.).
- [ ] **Sparkline is bars, not a line.** Contract §7/§8 + Svelte render an SVG polyline (`buildSparkline`, viewBox 0 0 64 24, stroke 1.5, 4rem×1.5rem); GPUI draws a bar strip (`metric_tile.rs:154-188`). Acceptable as a Tier-3 rendering-internal substitution **only if** dimensions/color match — but width is unbounded (one bar per point) vs fixed 4rem, and color uses trend/value color vs contract `text-tertiary`. Constrain to 4rem×1.5rem and use `color.text.tertiary`.
- [ ] **Sparkline color wrong.** Bars use trend color or value color (`metric_tile.rs:163-166`); contract §8 sparkline color is `color.text.tertiary`.
- [ ] No `density` support (spec lacks the field) — padding/gaps fixed.
- [ ] No light-theme surface-fill override (contract §8 light theme → `treatment-surface-fill`); GPUI uses the same `fill_token()` regardless of theme.
- accepted: no ARIA (gpui has no accessibility API) — `aria-label`/`aria-hidden` not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] **No sparkline.** `js_metric_tile` renders label + value + optional trend only (`metric_tile.rs:29-85`); there is a `// (+ sparkline placeholder)` comment (`:43`) but no SVG/line is drawn. Contract §2 Body includes the sparkline; the "With sparklines" specimen group requires it. Implement the line (or a documented Tier-3 substitute at 4rem×1.5rem, `text.tertiary`).
- [ ] **Extra border not in contract.** Adds `border(1.0).border_color(color.border.subtle)` (`metric_tile.rs:31-32`). Contract §8 + Svelte root border is `0.0625rem solid transparent` (invisible). **Fix: transparent border, and the `1.0` width should be `0.0625rem` resolved, not a raw `1.0`.**
- [ ] **Hardcoded px literals:** `border(1.0)` (`:31`), icon `rem_to_px(0.875)` (`:64`), trend-row `gap(rem_to_px(0.25))` (`:68`). The `rem_to_px(...)` calls at least flow through rem, but `1.0` border width is a raw literal; resolve all three from tokens (border-width token, trend-arrow 0.875rem, trend gap 0.25rem).
- [ ] **Trend uses `label_font` (0.75rem) for the trend label** (`metric_tile.rs:79-80`) — correct size, but the icon is sized `rem_to_px(0.875)` while the contract trend-arrow font is 0.875rem and trend text is 0.75rem; verify icon vs text sizing matches contract §8 (arrow 0.875rem, trend text 0.75rem). OK as-is but confirm.
- [ ] No `density` support (spec lacks the field) and no light-theme surface-fill override (contract §8). Padding/gaps fixed; same surface fill in both themes.
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
