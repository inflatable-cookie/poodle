<!-- parity consv=ok gpui=8 jetstream=7 specimen=gap -->
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
- The `RatingSpec` (Rust) is a reduced model vs the Svelte/contract surface: it has `precision` (single field) instead of `step`, lacks `allow_clear`, `aria_label`, `default_value`, `hover`/`focus` state, and `value` is a plain `f64` (no null/empty). This is a spec gap that blocks both Rust targets — flagged under each target below rather than as a contract↔Svelte issue.

## GPUI gap (vs Svelte + contract)

- [ ] No partial/fractional fill — fill is binary via `spec.filled_count()` (`rating.rs:95,108`); contract §2/§4 require a clipped accent overlay sized by per-star ratio. Implement the Base + Fill + FillInner layers.
- [ ] No fractional (slider) mode — `step < 1` path absent; root is always a flex row, never `slider` role with stepped keyboard. Contract §6.
- [ ] `RatingSpec` lacks `step`/`allow_clear`/`default_value` (`rating.rs` spec) — GPUI cannot express half-star or clearable specimens; add fields then wire builders.
- [ ] No `allowClear` deselection on repeat click (`selectIndex` clear branch in Svelte:131 has no GPUI equivalent).
- [ ] No roving tabindex / focus-ring — `.focusable()` is set per star (`rating.rs:124`) but no `tabindex 0/-1` roving and no focus-visible outline (`accent.focusRing`). Contract §6 Tier 1.
- [ ] Inactive color hardcodes the 48% mix inline (`rating.rs:94`) instead of resolving `spec.inactive_color_token()` — and the spec token itself returns `COLOR_BORDER_SUBTLE` (`rating.rs:76` spec), not the contract's `text-secondary 48%`. Reconcile: make the token resolve the contract value, then consume it.
- [ ] Hover uses `active_color` text recolor (`rating.rs:132`); contract hover is an accent `drop-shadow` glow on the hovered item, not a recolor. Match the glow.
- [ ] Home/End keyboard missing — only Left/Right/Up/Down handled (`rating.rs:146-155`); contract §6 requires Home/End and Enter/Space select.
- accepted: no ARIA (gpui has no accessibility API) — radiogroup/radio/slider roles not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] No partial/fractional fill — `is_filled = (i as f64) < spec.value` (`rating.rs:22`) is binary; no clipped overlay. Contract §2/§4.
- [ ] No fractional (slider) mode — `step`/`precision<1` path absent.
- [ ] `inactive_color_token()` resolves `COLOR_BORDER_SUBTLE` (`spec rating.rs:76`); contract inactive is `text-secondary` at 48% alpha. Fix the spec token (or blend in the component) to match Svelte (`Rating.svelte:328`).
- [ ] No touch-target wrapper — stars are bare icons (`rating.rs:24`); contract §7 requires each item be a 2rem (size-scaled) square hit target. GPUI sizes it via `star_touch_size`; Jetstream omits.
- [ ] No size application to item box — only icon glyph is sized (`size_font_rem`); item width/height per size table (§8.Size adjustments) not applied.
- [ ] `RatingSpec` lacks `step`/`allow_clear` — cannot render half-star or clearable specimens.
- [ ] No `allowClear`, no keyboard, no roving focus (interaction lives in preview event loop, but no handlers wired at all here).
- accepted: no ARIA channel; interaction (click/keyboard) would live in preview `main.rs` event loop.

## Specimen parity

- Svelte covers: Default (5), 10-star scale, **Half-star steps** (fractional), **Clearable** (interactive clear), Disabled, plus size + density snippet grids. Interactive value display.
- GPUI covers: Default(5, interactive click), 10-star, Clearable-labeled (but renders plain value 2, no actual clear), Disabled, size + density grids. — missing: **Half-star/fractional**, real **Clearable** behavior.
- Jetstream covers: 1/3/5-of-5 static values, Disabled. — missing: **10-star scale**, **Half-star/fractional**, **Clearable**, **size grid**, **density grid**. Labels diverge from contract specimen set (§13).

## Notes

- Specimen `group()` helpers hardcode `text_size(11.0)` (jetstream `specimens/rating.rs:33`) and `px(12.0)` (gpui `specimens/rating.rs:52`) — specimen chrome, not component code, but worth normalizing to an eyebrow/token.
- The root blocker for both Rust targets is the impoverished `RatingSpec`: adding `step`/`allow_clear`/`default_value`/null-`value` and a fractional render path is prerequisite to Tier-1 parity. Until then leave fractional/clearable specimens unimplemented rather than faking them (CLAUDE.md "no mockups").
