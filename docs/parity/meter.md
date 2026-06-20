<!-- parity consv=gap gpui=3 jetstream=6 specimen=gap -->
# Parity: Meter

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/meter.md`
- Svelte (authoritative): `packages/svelte/components/src/Meter.svelte`
- GPUI: `packages/gpui/components/src/primitives/meter.rs`
- Jetstream: `packages/jetstream/components/src/meter.rs`
- Spec: `packages/contracts/components/src/meter.rs` (`MeterSpec`)
- Specimens: svelte `packages/svelte/preview/src/specimens/MeterSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/meter.rs` · jetstream `packages/jetstream/preview/src/specimens/meter.rs`

## Contract ↔ Svelte

Contract and Svelte are nearly aligned on props/anatomy. Divergences are internal contradictions and one Rust-spec drift.

- **Track background mix percentage contradicts itself.** Svelte CSS (`Meter.svelte:69`) and contract §8 Track table both say `color-mix(... surface 96%, text-primary)`. But contract §8 Token Reference (line 179) and §11 Tier-2 (line 207) say "88% mix". Svelte is authoritative → **Fix: contract §8 Token Reference + §11 should say 96%, not 88%.**
- **Track-mix base token name.** Svelte mixes against `var(--poodle-surface)` (line 69); contract §8 Token Reference cites `--poodle-color-background-surface`. Same conceptual surface token, different name spelling. **Fix: reconcile token name in contract to match Svelte's `--poodle-surface`.**
- **`size` / `sizeRole` props absent from the Rust `MeterSpec`.** Contract §3 + Svelte (`size`, `sizeRole`) drive the xs–xl track-thickness ladder (§8 size variants). `MeterSpec` (`meter.rs:3-12`) has no `size`/`size_role` field and only a flat `track_height_rem()` returning `0.5` (`meter.rs:86-88`). This is a spec gap, not a Svelte gap. **Fix: add `size`/`size_role` to `MeterSpec` + a size-driven `track_height_rem()` ladder.**
- **`MeterSpec` default `max = 1.0`** (`meter.rs:24`) contradicts contract/Svelte default `max = 100`. **Fix: change spec default to `100.0`** (or document the 0–1 normalization convention the Jetstream specimen relies on).
- **`MeterSpec::fill_token()` returns `COLOR_ACCENT_BASE`** (`meter.rs:77-79`), i.e. accent blue. Contract §8 fill + §11 Tier-2 and Svelte (`Meter.svelte:78`) require `color.status.success` (green). Confirmed distinct tokens. **Fix: `fill_token()` must return `COLOR_STATUS_SUCCESS`.**

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] No `size` support — `track_height` is the flat `spec.track_height_rem()` (`meter.rs:91`); the xs–xl thickness ladder (contract §8) is not rendered. Wire size once `MeterSpec` gains the field.
- [ ] Fill is a flat `success_color` (`meter.rs:86`); contract §8 fill is a `linear-gradient(90deg, mix(success 82%, white), success)`. GPUI has no gradient — accept flat OR resolve the lighter endpoint via `color_mix(success, white, 0.82)`. At minimum note as a Tier-2 visual delta.
- [ ] Native `<meter>` semantics (value/min/max/low/high/optimum, aria-label) not emitted — only a header comment describes the intent (`meter.rs:93-96`). Tracked as the contract §11 Tier-1 accessibility item.
- note: GPUI correctly bypasses the buggy `spec.fill_token()` and hardcodes the token string `"color.status.success"` (`meter.rs:77`) — visually right, but it should consume a corrected `spec.fill_token()` once that is fixed.
- accepted: no ARIA (gpui has no accessibility API) — meter semantics cannot reach the a11y tree.

## Jetstream gap (vs Svelte + contract)

- [ ] **Wrong fill color.** `js_meter` resolves `spec.fill_token()` (`meter.rs:11`), which returns accent blue, not success green. Contract §8 + Svelte require `color.status.success`. Fix the spec token (see Contract↔Svelte) — Jetstream then inherits the fix.
- [ ] **Hardcoded radius literal** `rounded(999.0)` at `meter.rs:24, 30`. Resolve from `radius.pill` via the theme, not a raw `999.0`.
- [ ] **Hardcoded fixed track width** `track_w = rem_to_px(10.0)` at `meter.rs:20`. Contract §7 says width is parent-owned (`width: 100%`); the meter should fill its parent, not assume 10rem. Use `.grow()`/`w_full` and compute fill via fraction-of-parent, not an absolute width.
- [ ] **Fill via two sibling divs** (fill + remainder in a `flex_row`, `meter.rs:23-33`) is a layout hack; Svelte uses a single fill span at `width: {pct}%` inside an `overflow:hidden` track. Acceptable if width were parent-owned, but combined with the fixed 10rem it mis-sizes. Rework once width is parent-owned.
- [ ] No `size` support — flat `track_height_rem()` only; xs–xl ladder absent (contract §8). Depends on `MeterSpec` size field.
- [ ] No gradient fill (contract §8) — flat color. Same Tier-2 delta as GPUI; note or approximate the lighter endpoint via color-mix.
- accepted: no ARIA channel; native `<meter>` semantics live nowhere in Jetstream.

## Specimen parity

- Svelte covers: **Sizes ladder** (xs–xl via `showSizes`), Default (50%), With thresholds (82%, low/high/optimum + annotation), Low value (30% optimal range + annotation), Custom range (0–500 + annotation). (`MeterSpecimen.svelte`)
- GPUI covers: Default (50%), With thresholds, Low value, Custom range — all with annotations. — **missing: the Sizes ladder group** (no per-size specimen; GPUI can't vary thickness yet). (`gpui/.../meter.rs`)
- Jetstream covers: Low (25%), Half (50%), High (90%) only, all on a 0–1 range. — **missing: Sizes ladder, threshold/low/high/optimum specimens, Custom range, all annotations.** Specimen also diverges from contract §13 labels/values. (`jetstream/.../meter.rs`)

## Notes

- The single biggest defect is `MeterSpec::fill_token()` returning accent blue instead of success green — it makes the meter the wrong color in any target that trusts the spec (Jetstream does; GPUI sidesteps it by hardcoding the correct token string).
- Contract self-conflict (96% vs 88% track mix) must be resolved in the contract file, not in code — Svelte's 96% wins.
- low/high/optimum are display-inert in all targets (contract §4 confirms they only feed native `<meter>` semantics); no zone-based color shift is expected yet (contract §14 future follow-up).
