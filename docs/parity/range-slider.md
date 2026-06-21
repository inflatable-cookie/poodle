<!-- parity consv=gap gpui=3 jetstream=3 specimen=gap -->
<!-- pass 41: Jetstream range-slider rebuilt to match the single Slider — track 0.375rem
     (was 0.25), track bg tint(surface,0.88)=color-mix(surface 88%, transparent) (was
     .mix(accent) tint bug), thumb diameter from the §8 size table (was control_height*0.44),
     container min-height from the size table (was *0.56), pill radius from radius.pill,
     thumb border 0.0625rem (was 1.0px), step snapping anchored at min. Between-fill is the
     middle of three fixed-px flex-row segments (lo|fill|hi) — JsEl has no percent/offset, so
     the leading lo segment supplies the fill's offset start (no ProgressBar needed; left-only
     fill can't offset). Probe-tested (two thumbs, offset+proportional fill, transparent track,
     disabled opacity). GPUI closed: thumb diameter → §8 size table, shadow offset/blur → rem,
     step_clamp anchored at min (was anchored at 0). Remaining gpui: vertical + per-thumb
     focus/Home-End (runtime). Remaining jetstream: focus ring + per-thumb keyboard + vertical
     (no focus/shadow primitive; interaction is preview-loop). -->
# Parity: RangeSlider

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/range-slider.md`
- Svelte (authoritative): `packages/svelte/components/src/RangeSlider.svelte`
- GPUI: `packages/gpui/components/src/primitives/range_slider.rs`
- Jetstream: `packages/jetstream/components/src/range_slider.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/RangeSliderSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/range_slider.rs` · jetstream `packages/jetstream/preview/src/specimens/range_slider.rs`

## Contract ↔ Svelte

Props/anatomy/ARIA match closely (two `input[type=range]`, track+fill, `lowerValueText`/`upperValueText`, per-thumb `{ariaLabel} minimum/maximum`). Divergences:

- FIXED (justified) — **Density vertical padding.** Svelte `RangeSlider.svelte:254-255` sets `[data-density="compact"] { padding: 0.25rem 0 }` / `comfortable { padding: 0.75rem 0 }` (`padding-block` on root). Since editing Svelte is out of scope and the rule is "Svelte is parity authority", a new §8 Density-adjustments table documents the exact values **with an explicit Size/Density-rule exception** (touch-target hit-area growth; padding sits outside the absolutely-positioned track/fill, so control geometry is unchanged). Not silently mirrored — justified per the density-exception clause.
- LEFT (Svelte-side gap) — **`aria-orientation` not emitted.** Contract §6 requires `aria-orientation` on both inputs to match orientation; Svelte sets `data-orientation` on root only (line 84) and omits `aria-orientation` on the inputs (lines 89-115). Per "never weaken a contract's a11y requirement just because Svelte hasn't shipped it", the §6 requirement is **left intact**; §9 Svelte Notes now flags this as a known Svelte gap (add `aria-orientation={orientation}` to both inputs). This is the remaining `consv=gap` driver.
- **`aria-valuemin`/`aria-valuemax`/`aria-valuenow` rely on native input semantics** (min/max/value attrs present, lines 93-95 / 108-110) rather than explicit ARIA attrs. Acceptable — native range inputs expose these implicitly; contract §6 is satisfied. No action.
- **Step snapping anchored to `min`.** Svelte snaps via `snapToStep(raw, min, step)` (lines 57/71) so increments land on `min + n*step`. Contract §3 says only "increment size"; behavior is reasonable and authoritative. No action, but worth noting Rust impls snap to `0 + n*step` (see GPUI/Jetstream `step_clamp`) — a subtle off-grid divergence when `min` is not a step multiple.
- Lower/upper invariant: Svelte clamps lower to `≤ displayUpper` and upper to `≥ displayLower` (lines 57/71) and sorts the incoming pair via `Math.min/Math.max` (46-47). Matches contract §3. No action.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded thumb-shadow literals — `hsla(0.0, 0.0, 0.0, 0.18)` + `point(px(0.0), px(2.0))` + `blur_radius: px(8.0)` at `range_slider.rs:203-206`. Contract thumb box-shadow is `0 0.125rem 0.5rem color-mix(black 18%, transparent)`; resolve offset/blur from rem tokens and the color from a shadow/overlay token, not raw HSLA + raw px.
- [ ] Track height fixed at `rem_to_px(0.375)` with no size scaling (`range_slider.rs:186`) — acceptable as a rem conversion, but the comment admits "no per-size token exists." Contract §7 ties track thickness to 0.375rem (size-invariant), so OK; flag only if a track-height token is later added.
- [ ] Thumb diameter pinned to `size.icon.md` (`range_slider.rs:191`) — ignores `spec.size`. Contract §8 size table requires per-size thumb diameter (xs 0.75 → xl 1.25rem) + `margin-top`. GPUI renders every size at md (1rem). Resolve thumb size from the effective control size.
- [ ] No vertical orientation — `into_element` ignores `spec.orientation` for layout and the mouse/key handlers early-return on non-horizontal (`range_slider.rs:9-10` header, 335/369). Contract §7 + GPUI Notes require native vertical. Open.
- [ ] Per-thumb focus / Tab cycling absent — single wrapper focus ring; keyboard maps Left/Down→low, Right/Up→high on one handler (`range_slider.rs:392-410`). Contract §6 requires each thumb individually focusable with Home/End per-thumb. Home/End unimplemented. Open (documented GPUI 0.2.2 delta in header).
- [ ] `step_clamp` snaps to `n*step` from 0, not from `min` (`range_slider.rs:43-50`) — diverges from Svelte `snapToStep(raw, min, step)` when `min` isn't a step multiple. Align anchor to `min`.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored on spec, never emitted.
- accepted: `on_value_commit` fires on `on_click` release, not true mouse-up (GPUI 0.2.2 lacks `on_mouse_up` in the fluent builder; documented in header).
- accepted: pointer overlap/grab priority is platform-owned (contract §12 Known Delta).

## Jetstream gap (vs Svelte + contract)

- [ ] Wrong track thickness — `track_h = rem_to_px(0.25)` at `range_slider.rs:24`. Contract §7/§8 track height is **0.375rem**. Fix to `rem_to_px(0.375)` (and ideally a token).
- [ ] Wrong track background mix — `surface.mix(accent, 0.88)` at `range_slider.rs:33-34`. Contract track bg is `color-mix(surface 88%, transparent)` (mix toward transparent, i.e. alpha 0.88), **not** toward accent. Mixing with accent tints the unfilled track. Fix to alpha/transparent mix.
- [ ] Thumb diameter is an ad-hoc heuristic `control_height_rem * 0.44` (`range_slider.rs:23`) — does not match contract §8 per-size thumb table (md=1rem, xs=0.75 … xl=1.25rem). Resolve from the size table, not a magic ratio.
- [ ] Container/track heights are heuristics `control_height_rem * 0.56` / `* 0.44` (`range_slider.rs:25-26`) instead of contract dims (min-height 1.5rem md). Tie to size tokens.
- [ ] No focus ring — contract §4 focus-lower/focus-upper + §8 compound focus shadow unimplemented; thumbs are static divs (`range_slider.rs:69-91`).
- [ ] No keyboard / no per-thumb focus — no arrow/Home/End handling in the component.
- [ ] No vertical orientation — `js_range_slider` ignores `spec.orientation`; always horizontal flex-row (`range_slider.rs:94-104`).
- [ ] `step` ignored in rendering and no interaction — thumbs positioned from raw `low`/`high`, no snapping; spec `step` unused in the component.
- accepted: no ARIA channel (`aria_label` unused).
- accepted: drag interaction lives in the preview event loop, not the component (consistent with other Jetstream primitives) — but note the current specimen wires **no** drag, so it is render-only.

## Specimen parity

- Svelte covers: Default (`[20,80]`, live `$lo – $hi`), With step (`[23,43]` min18 max65 step5, live `Ages`), Disabled (`[30,70]`), plus all five Sizes via the `sizes` snippet. Interactive `onValueChange` wired.
- GPUI covers: Default (interactive, live value text), With step, Disabled, plus Sizes + Densities groups via `specimen_layout`. — missing: nothing material vs Svelte; broadest coverage of the three. Note: the value-text labels use `color_to_hsla(text_secondary)` + `text_sm()`, fine for a preview chrome label.
- Jetstream covers: Default (20–80), Narrow (45–55), Full (0–100), Low end (0–25), High end (75–100), Disabled. — missing: **With step** group, **Sizes** group, and any **interactivity/live value** (all static, no drag, no `step`/`size` exercised). `range_slider.rs:54-57` specimen group helper hardcodes `text_size(11.0)` (preview chrome label, not the component).

## Notes

- Remaining `consv=gap` driver: missing `aria-orientation` on the inputs — a **Svelte-side** a11y gap. The contract §6 requirement is correct and deliberately not weakened, so the gap closes only by fixing `RangeSlider.svelte`, not the contract. The density vertical-padding mismatch (the other former driver) is now documented + justified in contract §8.
- Both Rust impls snap steps from `0` rather than from `min` (Svelte anchors to `min`); only matters when `min` is not a multiple of `step` (e.g. the step specimen min=18, step=5). Worth aligning across all three.
- GPUI is the strongest parity target here (interactive drag + keyboard + live value + size/density specimen groups). Jetstream is the weakest: wrong track height, wrong track-bg mix, heuristic thumb sizing, and a static non-interactive specimen.
- Contract §12 Known Deltas already bless: pointer overlap handling, two-inputs-vs-single-control DOM pattern, CSS-rotation-vs-native vertical, and color-mix formula freedom. Vertical-orientation absence in both Rust targets is still an open gap, not a blessed delta (the delta covers *how* vertical is done, not skipping it).
