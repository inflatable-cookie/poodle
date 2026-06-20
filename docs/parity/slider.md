<!-- parity consv=gap gpui=6 jetstream=9 specimen=gap -->
# Parity: Slider

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/slider.md`
- Svelte (authoritative): `packages/svelte/components/src/Slider.svelte`
- GPUI: `packages/gpui/components/src/primitives/slider.rs`
- Jetstream: `packages/jetstream/components/src/slider.rs`
- Spec: `packages/contracts/components/src/slider.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/SliderSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/slider.rs` · jetstream `packages/jetstream/preview/src/specimens/slider.rs`

## Contract ↔ Svelte

Single-thumb slider. Svelte and contract agree on props, anatomy, token targets, and CSS values; the divergences are narrow.

- `onValueChange` payload type: contract §5 says payload is `number`; Svelte clamps + snaps before emitting (`Slider.svelte:48-52` `handleInput`), so the emitted value is always the clamped/stepped value, not the raw input value. Behaviorally correct and stricter than the contract text. **Fix: contract §5 should note the payload is the clamped, step-snapped value, not the raw event value.**
- `max <= min` guard: Svelte derives `safeMax = max <= min ? min + 1 : max` (`Slider.svelte:43`) and uses `safeMax` for percentage, input `max`, and clamping. Contract §3/§9 do not document this clamp-to-`min+1` behavior. **Fix: document the `safeMax` guard in contract §3 notes (Svelte authoritative).**
- `aria-orientation`: contract §6 explicitly states it is NOT set on the input and orientation rides only on `data-orientation`. Svelte matches (no `aria-orientation` on the `<input>`, `Slider.svelte:65-77`). Aligned — noted so the GPUI/Jetstream sections don't flag its absence as a contract miss.
- No divergence found on: `value`/`min`/`max`/`step`/`orientation`/`disabled`/`ariaLabel`/`valueText`/`size`/`sizeRole`/`density` defaults, anatomy parts (root/track/fill/control), `--poodle-slider-percent` formula, all size-table thumb dimensions, focus-ring compound box-shadow, and disabled opacity token. These match exactly.

`consv=gap` is driven only by the two undocumented Svelte behaviors above (clamp-snap payload + `safeMax` guard); the visual/token contract is otherwise faithful.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] No keyboard adjustment — contract §6 + §11 Tier-1 require Arrow/Home/End/PageUp/PageDown. `Slider` is `.focusable()` (`slider.rs:228`) but registers no key handlers; arrows do nothing. Add key bindings that step value and fire `on_change`/`on_value_commit`.
- [ ] `on_value_commit` only fires on click, never on drag-release — documented delta (`slider.rs:10-11,288-301`), but contract §5 requires commit on release after a drag. The `on_mouse_move` drag path (`slider.rs:276-285`) emits `on_change` but no commit. Re-fire commit when drag ends.
- [ ] No vertical orientation — `spec.orientation` is a forwarded builder (`slider.rs:100-103`) but `into_element` ignores it; track is always horizontal (`w_full`, `slider.rs:163-175`). Contract §7/§10 require native vertical layout (1.5rem width, 10rem min-height). Branch layout on orientation.
- [ ] No size handling — `spec.size` builder exists (`slider.rs:116-119`) but thumb is hardcoded to `size.icon.md` (`slider.rs:150`); xs/sm/lg/xl thumb diameters from contract §8 size table are never applied. Resolve thumb size from `spec.size`.
- [ ] Hardcoded thumb shadow color/offsets — `hsla(0.0, 0.0, 0.0, 0.18)`, `offset point(px(0.0), px(2.0))`, `blur_radius px(8.0)`, `spread px(0.0)` at `slider.rs:191-196`. The `0.18` black and the `2.0`/`8.0` offsets are raw literals; resolve from shadow/elevation tokens, not inline `hsla`/floats.
- [ ] Hardcoded `track_radius = px(999.0)` at `slider.rs:148` and thumb-vs-track inset math `px(-(thumb_f - track_f) / 2.0)` at `slider.rs:180`. The `999.0` pill radius is a magic literal; resolve from a radius/full-pill token. (`track_f = rem_to_px(0.375)` at `slider.rs:146` is annotated as an accepted no-token-exists case — see Notes.)
- accepted: no ARIA — `aria-valuemin/max/now/text`, `aria-disabled` not expressible via the fluent Div builder (documented `slider.rs:6-8`).

## Jetstream gap (vs Svelte + contract)

- [ ] Track background color is wrong — code mixes surface toward **accent**: `surface.mix(accent, 0.88)` (`slider.rs:37`) with a comment claiming `color-mix(surface 88%, accent)` (`slider.rs:36`). Contract §8 + Svelte (`Slider.svelte:107`) both specify `color-mix(surface 88%, transparent)`. Mix surface toward transparent (alpha 0.88), not toward accent.
- [ ] No interaction at all — `js_slider` renders a static fill at `spec.value` (`slider.rs:39-45`); no drag, no keyboard, no `on_change`/`on_value_commit`, and preview `main.rs` has no slider event wiring. Contract §5/§6 + §11 Tier-1 require live value change + commit + keyboard. Add interaction (component callbacks or preview event loop).
- [ ] No vertical orientation — `spec.orientation` is ignored; layout is hardcoded horizontal flex-row (`slider.rs:80-88`). Contract §7 requires a vertical axis. Branch on orientation.
- [ ] Magic thumb-size formula `control_height_rem(effective_size) * 0.44` at `slider.rs:27` — invents a 0.44 ratio instead of the contract §8 size table (md = 1rem thumb). Hardcoded float; derive thumb diameter from the contract size table.
- [ ] Magic track-height `rem_to_px(0.25)` at `slider.rs:28` — contract §8 track thickness is `0.375rem`, not `0.25rem`. Wrong literal; use 0.375rem (or a track-height token).
- [ ] Magic container-height formula `control_height_rem(effective_size) * 0.56` at `slider.rs:29` — invents a 0.56 ratio; contract min-height is `1.5rem` (md). Hardcoded float; resolve from the size min-height table.
- [ ] Hardcoded fill/track corner radius `track_h * 0.5` at `slider.rs:54,61` and thumb radius `thumb_size * 0.5` at `slider.rs:47` — contract uses `999px` full-pill radius via token. Magic half-height math; resolve a pill-radius token.
- [ ] Hardcoded thumb border width `.border(1.0)` at `slider.rs:75` — contract §8 thumb border is `0.0625rem`. Raw `1.0` px; resolve from a border-width token / `rem_to_px(0.0625)`.
- [ ] No thumb box-shadow — contract §8 thumb requires `0 0.125rem 0.5rem` drop shadow; `js_slider` thumb (`slider.rs:67-77`) has bg + border only, no shadow. Add the shadow once a shadow primitive exists.
- accepted: no ARIA channel (no accessibility API on the Jetstream runtime).

## Specimen parity

- Svelte covers: Default (Volume 65, live readout), With step (Opacity 100, step 10, live readout), Disabled (40), and a `sizes` snippet (value 50 across xs–sm–md–lg–xl). Densities suppressed (`showDensities={false}`). (`SliderSpecimen.svelte`)
- GPUI covers: Default (Volume 65, interactive `on_change` + readout), With step (Opacity, step 10, interactive), Disabled (40), plus `specimen_layout` size grid (value 60) and density grid (value 60). — missing: **value-readout text uses a hand-built `div().text_sm()`** which is fine, but parity matches Svelte well; no vertical specimen (none in Svelte either). Closest to Svelte of the three.
- Jetstream covers: Default (50%), Low value (10%), High value (90%), Disabled (50). — missing: **With step** group (Svelte's step=10 case), **live value readout** under each slider (Jetstream sliders are static so no readout is possible), and the **size grid** (xs–xl) that Svelte + GPUI both show. Group labels also diverge (Default/Low/High vs Svelte's Default/With step/Disabled). `specimen=gap`.

## Notes

- Accepted GPUI literal: `track_f = rem_to_px(0.375)` (`slider.rs:146`) is annotated in-file — no slider-track-height token exists in the design system yet, so the fixed 0.375rem mirrors Svelte exactly. Same 0.375rem appears in the contract §8 track table. This is the one acceptable hardcode; the Jetstream `0.25rem` track height (`slider.rs:28`) is NOT — it's both undocumented and wrong vs contract.
- GPUI drag uses `on_children_prepainted` track bounds for hit math (`slider.rs:201-208`) — a sound approach; the gap is keyboard + drag-release commit, not pointer drag.
- The contract repeatedly distinguishes this single-thumb `Slider` from `RangeSlider` (§1 out-of-scope, §14 follow-up). All three implementations here are correctly single-thumb; no range-slider confusion present.
- Biggest cross-target theme: neither Rust target implements keyboard adjustment or vertical orientation, and Jetstream has no interaction whatsoever plus a wrong track-color mix and several invented size ratios that bypass the contract size table.
