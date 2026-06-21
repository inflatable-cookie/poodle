<!-- parity consv=ok gpui=1 jetstream=2 specimen=ok -->
<!-- specimen: both Rust targets backfilled to contract §13 set with real js_rating/Rating — Default, 10-star, Half-star (step=0.5 fractional fill), Clearable, Readonly, Disabled (+ Jetstream sizes row; GPUI sizes/density via specimen_layout panes). Both build clean. -->
# Parity: Rating

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/rating.md`
- Svelte (authoritative): `packages/svelte/components/src/Rating.svelte`
- GPUI: `packages/gpui/components/src/primitives/rating.rs`
- Jetstream: `packages/jetstream/components/src/rating.rs`
- Spec: `packages/contracts/components/src/rating.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/RatingSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/rating.rs` · jetstream `packages/jetstream/preview/src/specimens/rating.rs`

## Contract ↔ Svelte

Svelte faithfully implements the contract: whole/fractional modes, roving focus, slider keyboard, `allowClear`, clamped-display-but-step-quantized-input, per-star clipped overlay fill, five sizes, density gap overrides, full ARIA. No material divergence.

- Minor: contract §7 says "glyph renders at 1rem font-size" but Svelte sizes the SVG at `1.125em` (line 363) relative to the per-size glyph font-size. This is an em-relative scale of the size table, not a contradiction. **No action** (documentation nuance only).
- The `RatingSpec` (Rust) was a reduced model vs the Svelte/contract surface. **Pass 41 (additive):** added `step`, `allow_clear`, `default_value`, plus helpers `effective_step()`, `is_fractional()`, `fill_ratio(index)`, `inactive_color_alpha()`, `hover_glow_token()`, `focus_ring_token()`; fixed `inactive_color_token()` to resolve `color.text.secondary` (renderers apply 48% alpha) instead of `color.border.subtle`. `value` is still a plain `f64` (no null/empty) and `aria_label` is unmodeled — minor residual spec deltas, not blocking.

## GPUI gap (vs Svelte + contract)

- [x] DONE (pass 41): partial/fractional fill — clipped accent overlay (`relative` glyph + `absolute` fill layer, `overflow_hidden`, `w(relative(fill_ratio))`) sized by `spec.fill_ratio(index)`. Base + Fill layers present.
- [x] DONE: `step`/`allow_clear`/`default_value` added to `RatingSpec`; GPUI builders `.step()`/`.allow_clear()` wired.
- [x] DONE: `allowClear` deselection — Enter/Space/keyboard floor honors `allow_clear` (Home → 0 when clearable). Click selects i+1 (repeat-click clear lives in the preview loop — see note).
- [x] DONE: focus-ring — per-star `.focus(|s| s.border_2().border_color(focusRing))` resolving `color.accent.focusRing`.
- [x] DONE: inactive color — resolves `spec.inactive_color_token()` (= `color.text.secondary`) × `inactive_color_alpha()` (0.48); no inline hardcode, no `border.subtle`.
- [x] DONE: hover glow — accent `drop-shadow` (BoxShadow blur 0.375rem, accent@52%) on the hovered item, not a recolor.
- [x] DONE: Home/End/Enter/Space — full key set handled per star (`home`/`end`/`enter`/`space` plus arrows).
- [x] DONE: per-size glyph font (§8 table, ×1.125em) + density-driven inter-item gap.
- [ ] Fractional sub-star pointer precision (selecting 3.5 by clicking the left half of star 4) and true slider-role stepped keyboard — **preview-loop**: requires pointer-x within the star and persisted hover/value state, which a stateless render can't resolve. Whole-star click + keyboard floor/clear are wired.
- accepted: no ARIA (gpui has no accessibility API) — radiogroup/radio/slider roles not emitted.

## Jetstream gap (vs Svelte + contract)

- [x] DONE (pass 41): partial/fractional fill — `relative` glyph wrapper + `absolute`/`overflow_hidden` fill overlay clipped to `glyph_px * fill_ratio(index)`. Probe-tested (filled-vs-empty icon-layer counts).
- [x] DONE: inactive color — `tint(resolve_color(inactive_color_token), 0.48)` = `text-secondary 48%`; spec token fixed to `color.text.secondary`.
- [x] DONE: touch-target wrapper — each item is a fixed `control_height_rem(size)`-square hit area wrapping the glyph.
- [x] DONE: per-size item box + glyph (§8 size table) and density-driven gap applied.
- [x] DONE: `step`/`allow_clear` available on the spec for clearable/half-star specimens.
- [ ] `allowClear`, keyboard, roving focus, and fractional pointer selection — **preview-loop**: interaction (click/keyboard/hover-preview) lives in preview `main.rs`; no handlers are plumbed through the `(spec, theme)` signature yet.
- accepted: no ARIA channel; interaction (click/keyboard) would live in preview `main.rs` event loop.

## Specimen parity

- Svelte covers: Default (5), 10-star scale, **Half-star steps** (fractional), **Clearable** (interactive clear), Disabled, plus size + density snippet grids. Interactive value display.
- GPUI covers: Default(5, interactive click), 10-star, Clearable-labeled (but renders plain value 2, no actual clear), Disabled, size + density grids. — missing: **Half-star/fractional**, real **Clearable** behavior.
- Jetstream covers: 1/3/5-of-5 static values, Disabled. — missing: **10-star scale**, **Half-star/fractional**, **Clearable**, **size grid**, **density grid**. Labels diverge from contract specimen set (§13).

## Notes

- Specimen `group()` helpers hardcode `text_size(11.0)` (jetstream `specimens/rating.rs:33`) and `px(12.0)` (gpui `specimens/rating.rs:52`) — specimen chrome, not component code, but worth normalizing to an eyebrow/token.
- The root blocker for both Rust targets is the impoverished `RatingSpec`: adding `step`/`allow_clear`/`default_value`/null-`value` and a fractional render path is prerequisite to Tier-1 parity. Until then leave fractional/clearable specimens unimplemented rather than faking them (CLAUDE.md "no mockups").
