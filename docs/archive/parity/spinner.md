<!-- parity consv=fixed gpui=2 jetstream=2 specimen=ok | pass: added SpinnerSpec token methods (ring/cell radius, per-size grid gap, track-opacity, opacity floor/peak); both targets now resolve ring stroke/track + grid cell/gap/radius/opacity from the spec; Jetstream ring renders the two-tone track+arc statically. Jetstream specimen expanded to grid across all 5 sizes + mixed-tone grid row + Context-tone host chips (current/accent/muted). Remaining open items are animation (preview-loop) + accepted no-ARIA. -->
# Parity: Spinner

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/spinner.md`
- Svelte (authoritative): `packages/svelte/components/src/Spinner.svelte`
- GPUI: `packages/gpui/components/src/primitives/spinner.rs`
- Jetstream: `packages/jetstream/components/src/spinner.rs`
- Spec: `packages/contracts/components/src/spinner.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/SpinnerSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/spinner.rs` · jetstream `packages/jetstream/preview/src/specimens/spinner.rs`

## Contract ↔ Svelte

Prop set matches (`variant`/`size`/`sizeRole`/`density`/`tone`/`ariaLabel`, all defaults agree). Divergences are in animation timing and anatomy detail. Svelte is authoritative — update the contract.

- [x] FIXED: contract §8 grid `cell animation` duration changed from `1.12s` to `1.24s`, matching Svelte (`Spinner.svelte:147`).
- [x] FIXED: contract §8 now documents the `0.2` idle floor (`cell idle opacity` row + `spinner-grid-idle` keyframe note), matching Svelte (`Spinner.svelte:146-148,180-185`).
- [x] FIXED: contract §8 records the `0.2 → 0.76` opacity range (`cell opacity range` row); Svelte phases peak at `0.76` (`Spinner.svelte:187-209`).
- [x] FIXED: contract §2 clarified — ring and grid each render a single wrapper span (`.spinner__ring` / `.spinner__grid`); the six `.spinner__cell` spans are children of the grid wrapper (`Spinner.svelte:57,59-66`).
- [x] (no contract change needed) `aria-live`: contract §6 prose matches Svelte, which emits `aria-live="polite"` only when `ariaLabel` is set; §9 Svelte Notes now states this explicitly for runtime reference.
- [x] FIXED: §9 Svelte Notes now documents the `class` / `style` / `...restProps` passthrough surface (`Spinner.svelte:13-25`).

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] FIXED Cell radius resolves `spec.cell_radius_rem()` (0.125rem) via `rem_to_px`; the raw literal is gone.
- [x] FIXED Grid cell/gap derive from spec: gap = `spec.grid_gap_rem()` (per-size table), cell side = `(spec.grid_width_rem() - gap) / 2`. The five-row raw `(cell, gap)` table in `spinner.rs` is removed.
- [x] FIXED Opacity band: ramp is now `spec.opacity_floor() + smooth * (spec.opacity_peak() - spec.opacity_floor())` (0.2 → 0.76); the inline `0.2`/`0.56` literals are gone.
- [ ] Ring track + top-color highlight: GPUI still rotates the single-color `spinner.svg` asset (`spinner.rs` ring branch). The contract two-tone (24% track + bright top arc) lives in the SVG asset, not component code — leaving the asset as the right mechanism. Open: confirm/author the asset encodes the 24% track + bright arc. (Spec now exposes `track_opacity()` if the GPUI ring is ever rebuilt from borders rather than an asset.)
- [ ] Ring size still uses `spec.size_px()` (pre-multiplied px constants) rather than `spec.ring_size_rem()` × `rem_to_px`. Accepted-style (centralized in spec) but flagged: it assumes a fixed 16px root. `ring_size_rem()` now exists for an exact-parity rebuild.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored on spec but `role="status"`/`aria-live` not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] No rotation animation — `build_ring` renders a static (now two-tone) ring; continuous rotation must be driven by the preview event loop / a runtime animation hook. Accepted runtime delta (noted inline): JsEl has no animation hook here.
- [ ] No grid opacity animation — `build_grid` renders a static single-frame snapshot of the snake (within the 0.2→0.76 band); continuous pulsing is preview-loop driven. Accepted runtime delta.
- [x] FIXED Ring border width resolves `spec.ring_border_width_rem()` (0.125rem); the raw literal is gone.
- [x] FIXED Track alpha resolves `spec.track_opacity()` (0.24); the inline `0.24` is gone. Ring now also draws `border_color_top(tone_color)` for the bright arc — a faithful static two-tone ring.
- [x] FIXED Grid gap uses `spec.grid_gap_rem()` (the real per-size table 0.0625–0.15625rem); the `width * 0.1` heuristic is removed, cell math derives from it.
- [x] FIXED Cell radius resolves `spec.cell_radius_rem()` (0.125rem); the static ramp now uses the contract band via `spec.opacity_floor()`/`opacity_peak()` (0.2 floor, 0.76 peak) instead of the old `[1.0..0.25]` curve.
- accepted: tone `Current` resolves to `color.text.primary` rather than literal inherited `currentColor` — Jetstream has no CSS inheritance; same approximation GPUI uses.
- accepted: no ARIA channel for `aria_label`.

## Specimen parity

- Svelte covers: Ring (5 sizes), CLI grid (5 sizes with mixed muted/accent/current tones), Context tones (current on inverse chip, accent chip, muted grid chip) (`SpinnerSpecimen.svelte`).
- GPUI covers: Ring (5 sizes), CLI grid (5 sizes, tones muted/current/accent/current/current), Context tones (inverse/accent/muted chips). — matches Svelte closely; the only gap is the unused `sizes`/`densities` snippets Svelte defines but does not render, so effectively at parity.
- [x] FIXED (Jetstream specimen): now covers Ring (5 sizes), CLI grid (5 sizes), CLI grid mixed tones (xs=muted, md=accent, rest=current), Tones (current/accent/muted, ring), and Context tones (current/accent/muted spinners hosted in bordered surface chips). All via the real `js_spinner`; chips are plain host containers. Delta vs Svelte: the inverse chip is dropped — Jetstream `tone="current"` always resolves to `color.text.primary` (no CSS inheritance, accepted delta), so a colour-inverting chip can't be rendered honestly. The Context group shows current/accent/muted on a real surface bg instead.

## Notes

- `consv=fixed`: contract §8 grid duration corrected to `1.24s` and the `0.2`–`0.76` opacity floor/peak now documented, unblocking runtimes from matching motion. Anatomy wrapper spans and passthrough props also reconciled to Svelte.
- Spec struct (`packages/contracts/components/src/spinner.rs`) exposes only `size_px()` and `tone_color_token()`. It has **no** token methods for ring border width, cell radius, grid cell/gap sizes, or the opacity range — so every Rust runtime is forced into hardcoded literals. Adding those spec methods is the root fix for most GPUI/Jetstream todos above.
- Jetstream ring + grid are structurally correct but motionless; GPUI animates both. The biggest single behavior gap is Jetstream having no spin/pulse at all.
- GPUI grid is a hand-rolled 3-row × 2-col flex stack (`spinner.rs:107-156`) rather than a true grid, but visually matches the 2×3 contract layout.
