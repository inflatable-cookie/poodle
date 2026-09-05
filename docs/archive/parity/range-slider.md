<!-- parity consv=fixed gpui=1 jetstream=1 specimen=ok -->
<!-- pass 43: specimen=gap→ok. Both Rust targets backfilled to full contract-state
     coverage with REAL js_range_slider/RangeSlider (no hand-rolled fill — two thumbs +
     between-fill window come from the spec low/high). Jetstream gained With-step
     (25–45, 18–65, step 5), a positions group (narrow/full/low/high), custom
     min/max+step (0–500 step 50), the xs–xl size matrix, and densities. GPUI gained
     a positions group + custom-bounds group (size/density grids already supplied by
     specimen_layout). Both build clean (0 err). Live value readout / drag stay a
     preview-loop concern on the static Jetstream target (accepted). -->
<!-- pass 42: contract §6 reconciled to Svelte — `aria-orientation` is no longer
     required on the inputs (orientation rides on `data-orientation` only, matching
     Svelte + the Slider contract); §9 Known-Svelte-gap note replaced with an
     alignment note. consv=gap→fixed. Jetstream: both thumbs now carry the contract
     §8 drop shadow (0 0.125rem 0.5rem black@0.18) via a custom BoxShadow on
     `style.shadow` (JsEl DOES expose box-shadow — the earlier "no shadow primitive"
     was wrong; offsets are rem, only black@0.18 is a noted literal, same as GPUI).
     Probe/tree test added. GPUI thumb-diameter/step-anchor/shadow-offset items were
     already closed in code (pass 41) — doc reclassified. Remaining gpui=1: native
     vertical (runtime). Remaining jetstream=1: native vertical (preview-loop;
     drag/keyboard/focus are preview-loop, grouped as accepted). -->
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
- [x] FIXED (consv driver) — **`aria-orientation` reconciled to Svelte.** Svelte sets `data-orientation` on the root only (line 84) and does not emit `aria-orientation` on the inputs (lines 89-115). Svelte is the parity authority, and the sibling Slider contract already reports orientation via `data-orientation` only — so the prior §6 requirement (`aria-orientation` on both inputs) was the divergence, not Svelte. Pass 42 rewrote §6 to match Svelte ("`aria-orientation`: NOT set on the range inputs; orientation conveyed via `data-orientation`") and replaced the §9 Known-Svelte-gap note with an alignment note. This closes the last `consv=gap` driver → `consv=fixed`.
- **`aria-valuemin`/`aria-valuemax`/`aria-valuenow` rely on native input semantics** (min/max/value attrs present, lines 93-95 / 108-110) rather than explicit ARIA attrs. Acceptable — native range inputs expose these implicitly; contract §6 is satisfied. No action.
- **Step snapping anchored to `min`.** Svelte snaps via `snapToStep(raw, min, step)` (lines 57/71) so increments land on `min + n*step`. Contract §3 says only "increment size"; behavior is reasonable and authoritative. No action, but worth noting Rust impls snap to `0 + n*step` (see GPUI/Jetstream `step_clamp`) — a subtle off-grid divergence when `min` is not a step multiple.
- Lower/upper invariant: Svelte clamps lower to `≤ displayUpper` and upper to `≥ displayLower` (lines 57/71) and sorts the incoming pair via `Math.min/Math.max` (46-47). Matches contract §3. No action.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] Thumb-shadow offset/blur resolve from rem — `offset point(px(0.0), px(rem_to_px(0.125)))`, `blur_radius px(rem_to_px(0.5))` (`range_slider.rs:222-227`). Contract thumb box-shadow `0 0.125rem 0.5rem`. Only the `black@0.18` color stays a literal — no shadow token matches it (the `elevation.shadow.*` primitives use `rgba(17,22,29,…)` with different offsets/blur). Accepted noted literal.
- [x] Thumb diameter resolves per-size from the contract §8 table (`thumb_diameter_rem`, `range_slider.rs:208`) — xs 0.75 … xl 1.25rem, no longer pinned to md.
- [x] `step_clamp` anchored at `min` (`min + n*step`, `range_slider.rs:57-64`) — matches Svelte `snapToStep(raw, min, step)`.
- [ ] No vertical orientation — `into_element` ignores `spec.orientation` for layout and the mouse/key handlers early-return on non-horizontal (`range_slider.rs:355/389`). Contract §7 + GPUI Notes require native vertical. **Open** — needs preview/layout work beyond the build-only surface; the §12 vertical delta blesses *how* (native vs rotate), not skipping it.
- accepted: Track height fixed at `rem_to_px(0.375)` (`range_slider.rs:200`) — contract §7 ties track thickness to a size-invariant 0.375rem; a rem conversion of the contract-exact value, no per-size token exists. Faithful.
- accepted: Per-thumb focus / Tab cycling — single wrapper focus ring; keyboard maps Left/Down→low, Right/Up→high on one handler (`range_slider.rs:417-430`). GPUI 0.2.2 has no per-element focus within a stateless render tree; documented runtime delta. Per-thumb Home/End deferred with it.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored on spec, never emitted.
- accepted: `on_value_commit` fires on `on_click` release, not true mouse-up (GPUI 0.2.2 lacks `on_mouse_up` in the fluent builder; documented in header).
- accepted: pointer overlap/grab priority is platform-owned (contract §12 Known Delta).

## Jetstream gap (vs Svelte + contract)

- [x] Track thickness `rem_to_px(0.375)` (`range_slider.rs:71`) — contract §7/§8 0.375rem (pass 41).
- [x] Track background `tint(surface, 0.88)` = `color-mix(surface 88%, transparent)` (`range_slider.rs:85`) — mixes toward transparency, not accent (pass 41). Probe-tested.
- [x] Thumb diameter from the contract §8 size table (`thumb_diameter_rem`, `range_slider.rs`) — no magic ratio (pass 41). Probe-tested.
- [x] Container/track heights from the size table (`min_height_rem`, track `0.375rem`) — no heuristic (pass 41). Probe-tested.
- [x] Thumb drop shadow — both thumbs now carry the contract §8 `0 0.125rem 0.5rem black@0.18` via a custom `BoxShadow` on `style.shadow` (pass 42). JsEl DOES expose box-shadow (the earlier "no shadow primitive" was wrong — `style.shadow: Option<BoxShadow>` is public, same pattern as `tooltip.rs`). Offsets are rem; only black@0.18 is a noted literal (no token). Tree-test added (`both_thumbs_have_contract_drop_shadow`).
- [x] `step` snapping anchored at `min` (`snap_fraction`, `range_slider.rs:56-63`) — matches Svelte (pass 41).
- [ ] No vertical orientation — `js_range_slider` ignores `spec.orientation`; always horizontal flex-row. **Open** — needs the preview/layout axis branch (preview-loop bound).
- accepted: No focus ring + no keyboard/per-thumb focus — focus and key handling live in the preview event loop (no focus primitive on the Jetstream runtime), consistent with other Jetstream primitives. The component renders track + filled window + two thumbs at the spec's current values.
- accepted: no ARIA channel (`aria_label` unused).
- accepted: drag interaction lives in the preview event loop, not the component (consistent with other Jetstream primitives); the component is render-only.

## Specimen parity

- Svelte covers: Default (`[20,80]`, live `$lo – $hi`), With step (`[23,43]` min18 max65 step5, live `Ages`), Disabled (`[30,70]`), plus all five Sizes via the `sizes` snippet. Interactive `onValueChange` wired.
- GPUI covers: Default (interactive, live value text), With step (interactive), a Positions group (narrow/full/low/high, pass 43), custom min/max+step (0–500, step 50, pass 43), Disabled, plus Sizes + Densities groups via `specimen_layout`. Broadest coverage of the three. The value-text labels use `color_to_hsla(text_secondary)` + `text_sm()`, a preview chrome label.
- Jetstream now covers (pass 43): Default (20–80), With step (25–45, 18–65, step 5), a Positions group (narrow 45–55 / full 0–100 / low 0–25 / high 75–100), custom min/max+step (0–500, step 50), Disabled (30–70), the xs–xl Sizes matrix, and Densities — all REAL `js_range_slider` with the two thumbs + between-fill window resolved from the spec low/high (no hand-rolled fill). `step` and `size` are now exercised. The only Svelte item not mirrored is **interactivity / live value**, impossible on the static render-only Jetstream target (accepted). The group helper's `text_size(11.0)` is preview chrome (label, not the component). `specimen=ok`.

## Notes

- `consv=fixed` (pass 42): the former `aria-orientation` driver was the contract being *stricter* than Svelte, not a Svelte bug. Svelte (and the Slider contract) report orientation via `data-orientation` only; §6 now matches. The density vertical-padding mismatch (the other former driver) was already documented + justified in contract §8 (pass 41).
- Both Rust impls now snap steps anchored at `min` (matching Svelte `snapToStep(raw, min, step)`); the off-by-min divergence is closed on both targets.
- Only open gap on either Rust target: **native vertical orientation** (both). Contract §12 blesses *how* vertical is done (native vs rotate), not skipping it — but the work is a preview/layout axis branch beyond the build-only verification surface here, so it's tracked open, not closed.
- Visual/token parity is otherwise complete on both targets: track 0.375rem, track-bg `color-mix(surface 88%, transparent)`, per-size thumb diameter, pill radius, 0.0625rem border, and the `0 0.125rem 0.5rem black@0.18` thumb drop shadow all resolve from tokens/contract-exact rem (only black@0.18 is a noted literal — no shadow token matches it).
- Contract §12 Known Deltas already bless: pointer overlap handling, two-inputs-vs-single-control DOM pattern, CSS-rotation-vs-native vertical, and color-mix formula freedom.
