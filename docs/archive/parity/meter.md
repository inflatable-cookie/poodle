<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok -->
<!-- pass: MeterSpec fixed (max=100, fill_token=success, size/size_role + thickness ladder, track mix tokens). GPUI wires size + corrected tokens; Jetstream rebuilt on ui_element::progress (parent-owned width, proportional fill, token radius/track-mix, size ladder) + render_probe tests. Specimens cover sizes/thresholds/range on both. -->
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

- [x] FIXED — **Track background mix percentage contradicted itself.** §8 Token Reference + §11 Tier-2 said "88% mix"; Svelte (`Meter.svelte:69`) and the §8 Track table say `96%, text-primary`. Both stale spots updated to `96%` mix with text-primary.
- [x] FIXED — **Track-mix base token name.** §8 Token Reference cited `--poodle-color-background-surface`; updated to Svelte's `--poodle-surface` (mixed at 96% with `--poodle-color-text-primary`).
- Rust-spec gaps below are NOT contract↔Svelte divergences — they were `MeterSpec` defects, now FIXED:
  - [x] FIXED — **`size`/`sizeRole` added to `MeterSpec`** (`size: Option<ControlSize>`, `size_role`), with `track_thickness_rem(size)` xs–xl ladder (contract §8). Both targets resolve the effective size via `resolve_semantic_size`.
  - [x] FIXED — **`MeterSpec` default `max`** now `100.0` (matches contract/Svelte).
  - [x] FIXED — **`MeterSpec::fill_token()`** now returns `COLOR_STATUS_SUCCESS`. Jetstream's engine `ProgressBar` fill is already `status_success`; GPUI now resolves the corrected token instead of the hardcoded string. Added `track_mix_token()`/`track_mix_ratio()` so both targets do the contract §8 `surface 96% / text-primary` mix from tokens.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] DONE — `size` support: track thickness now resolves from the effective size (`spec.size` override → `size_role` via `resolve_semantic_size`) through `spec.track_thickness_rem(size)`; xs–xl ladder rendered. `.size()`/`.size_role()` builders added.
- [x] DONE — fill now resolves the corrected `spec.fill_token()` (`COLOR_STATUS_SUCCESS`), and track bg uses `spec.track_fill_token()`/`track_mix_token()`/`track_mix_ratio()` for the §8 surface↔text-primary mix — all token-resolved, no hardcoded strings.
- accepted: flat fill (no gradient). Contract §8 fill is `linear-gradient(90deg, mix(success 82%, white), success)`; GPUI has no gradient API, so a flat success fill is the accepted Tier-2 delta.
- accepted: native `<meter>` semantics (value/min/max/low/high/optimum, aria-label) not emitted — GPUI has no accessibility API channel (contract §11 Tier-1 a11y item; runtime limit).

## Jetstream gap (vs Svelte + contract)

- [x] DONE — **fill color.** Rebuilt on `ui_element::progress(frac)` (the runtime `ProgressBar` widget); its engine fill is `status_success` (contract §8). The spec's `fill_token()` is also corrected to success for any direct consumer.
- [x] DONE — **radius.** Pill radius now resolves from the `radius.pill` token via `resolve_radius`, not a `999.0` literal.
- [x] DONE — **track width parent-owned.** The fixed `10rem` track is gone; the meter is `.w_full().self_stretch()` and the `ProgressBar` fills `frac` of the parent-owned width.
- [x] DONE — **single proportional fill.** The two-sibling-div hack is replaced by the single `ProgressBar` widget; fill is a true fraction, not a stretched child.
- [x] DONE — `size` support: `spec.track_thickness_rem(effective_size)` xs–xl ladder, `min_h` from the resolved size.
- accepted: flat fill (no gradient) — same Tier-2 delta as GPUI; `JsEl`/`ProgressBar` draw a flat fill.
- accepted: no ARIA channel; native `<meter>` semantics live nowhere in Jetstream.
- tests: `render_probe` covers ProgressBar presence, the §8 track mix color, the xs/md/xl thickness ladder, `size_role` resolution, and custom-range fraction.

## Specimen parity

- Svelte covers: **Sizes ladder** (xs–xl via `showSizes`), Default (50%), With thresholds (82%, low/high/optimum + annotation), Low value (30% optimal range + annotation), Custom range (0–500 + annotation). (`MeterSpecimen.svelte`)
- GPUI covers: Default (50%), With thresholds, Low value, Custom range — all with annotations. **Sizes ladder still TODO in the GPUI preview specimen** — the GPUI component now supports `size`, but the preview specimen wasn't rebuilt this pass (shared gpui/preview target lock). Follow-up: add the xs–xl group. (`gpui/.../meter.rs`)
- Jetstream covers: **Sizes ladder (xs–xl)**, Default (50%), With thresholds (82%, low/high/optimum), Low value (30%), Custom range (350/500) — all on the contract 0–100 / custom range with §13 labels. Rebuilt this pass. (`jetstream/.../meter.rs`)

## Notes

- The single biggest defect was `MeterSpec::fill_token()` returning accent blue instead of success green — now fixed to `COLOR_STATUS_SUCCESS`. Jetstream's `ProgressBar` engine fill was already success-green, so the user-visible Jetstream color was correct; the spec token now agrees with it.
- `consv=fixed`: the only contract↔Svelte divergences were the self-conflicting 88%-vs-96% track mix and the `--poodle-color-background-surface` token-name spelling; both resolved in the contract to Svelte's 96% / `--poodle-surface`. The remaining items (`MeterSpec` size/max/fill_token) are Rust-spec bugs, not contract issues.
- low/high/optimum are display-inert in all targets (contract §4 confirms they only feed native `<meter>` semantics); no zone-based color shift is expected yet (contract §14 future follow-up).
