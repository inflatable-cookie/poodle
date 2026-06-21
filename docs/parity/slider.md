<!-- parity consv=fixed gpui=1 jetstream=1 specimen=gap -->
<!-- pass 42: GPUI slider keyboard CLOSED — Arrow Left/Down/Right/Up + Home/End +
     PageUp/PageDown now fire `on_change` with step-snapped (anchored at min),
     clamped values (contract §6 + §11 Tier-1); the sibling RangeSlider already
     proved this representable. Commit-on-keyup stays the GPUI 0.2.2 delta.
     Jetstream slider thumb drop-shadow CLOSED — `0 0.125rem 0.5rem black@0.18` via a
     custom BoxShadow on `style.shadow` (JsEl DOES expose box-shadow — the earlier
     "no shadow primitive" claim was wrong; same pattern as tooltip.rs). Offsets are
     rem; only black@0.18 is a noted literal (no shadow token matches). Tree-test added.
     Remaining gpui=1: native vertical (runtime). Remaining jetstream=1: native vertical
     (preview-loop); drag/keyboard are preview-loop, grouped as accepted. -->
<!-- pass 41: GPUI slider closed — thumb diameter now resolves from the contract §8
     size table (was pinned to size.icon.md), track pill radius from radius.pill (was
     px(999.0)), thumb shadow offset/blur now rem_to_px(0.125/0.5) (only the black@0.18
     color stays literal — no matching shadow token). Jetstream slider already carried the
     pass-29 fixes (tint track, size table, pill, border); count corrected. -->
<!-- pass 29: Jetstream track-color bug fixed — was surface.mix(accent, 0.88) (toward
     accent); now tint(surface, 0.88) = color-mix(surface 88%, transparent) per contract/
     Svelte (filled portion stays opaque accent). Invented size ratios → contract §8 size
     table (thumb xs0.75…xl1.25rem, track 0.375rem, min-height table); radius.pill; thumb
     border 0.0625rem. Probe-tested. Remaining jetstream: thumb drop-shadow (no shadow token)
     + drag/keyboard (preview loop). -->
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

- [x] FIXED: contract §5 now states the `onValueChange`/`onValueCommit` payload is the clamped, step-snapped value (`clamp(snapToStep(raw, min, step), min, safeMax)`), not the raw event value, matching Svelte's `handleInput`/`handleChange` (`Slider.svelte:48-58`).
- [x] FIXED: contract §3 now documents the `safeMax = max <= min ? min + 1 : max` bounds guard (new "Bounds Guard" subsection) and the `--poodle-slider-percent` formula uses `safeMax`, matching Svelte (`Slider.svelte:43,45`).
- [x] FIXED (orthogonality): contract §8 now adds a "Density And Vertical Padding" note stating density must not alter vertical padding / min-height (orthogonality-correct rule), and flags the Svelte `padding: 0.25rem 0` / `0.75rem 0` density rules (`Slider.svelte:207-208`) as a Svelte bug that Rust targets must NOT replicate.
- `aria-orientation`: contract §6 explicitly states it is NOT set on the input and orientation rides only on `data-orientation`. Svelte matches (no `aria-orientation` on the `<input>`, `Slider.svelte:65-77`). Aligned — noted so the GPUI/Jetstream sections don't flag its absence as a contract miss.
- No divergence found on: `value`/`min`/`max`/`step`/`orientation`/`disabled`/`ariaLabel`/`valueText`/`size`/`sizeRole`/`density` defaults, anatomy parts (root/track/fill/control), `--poodle-slider-percent` formula, all size-table thumb dimensions, focus-ring compound box-shadow, and disabled opacity token. These match exactly.

`consv=fixed`: the two undocumented Svelte behaviors (clamp-snap payload + `safeMax` guard) are now in the contract, plus an explicit density-orthogonality note flagging the Svelte vertical-padding bug. The visual/token contract is otherwise faithful.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] Keyboard adjustment CLOSED (pass 42) — `on_key_down` now handles Arrow Left/Down (−step), Right/Up (+step), Home (min), End (max), PageUp/PageDown (±step·10), all step-snapped (anchored at min via `step_clamp`) + clamped, firing `on_change` for live updates (`slider.rs` interaction block). Contract §6 + §11 Tier-1 satisfied for value semantics.
- [x] Thumb diameter from the contract §8 size table (`thumb_diameter_rem`, `slider.rs`) — xs 0.75 … xl 1.25rem, no longer pinned to md (pass 41).
- [x] Thumb shadow offset/blur from rem (`offset point(px(0.0), px(rem_to_px(0.125)))`, `blur px(rem_to_px(0.5))`) + `track_radius` from `radius.pill` (pass 41). Only the `black@0.18` color stays a noted literal — no shadow token matches (`elevation.shadow.*` use `rgba(17,22,29,…)`/different offsets).
- [ ] No vertical orientation — `into_element` ignores `spec.orientation`; track is always horizontal. Contract §7/§10 require native vertical (1.5rem width, 10rem min-height). **Open** — a layout/preview axis branch beyond the build-only surface; §12 blesses native-vs-rotate, not skipping.
- accepted: `on_value_commit` fires on click-release, not drag-release or key-up — GPUI 0.2.2 exposes no `on_mouse_up`/key-up in the fluent builder. Keyboard still emits live `on_change`. Documented runtime delta.
- accepted: no ARIA — `aria-valuemin/max/now/text`, `aria-disabled` not expressible via the fluent Div builder.

## Jetstream gap (vs Svelte + contract)

- [x] Track background `tint(surface, 0.88)` = `color-mix(surface 88%, transparent)` (`slider.rs:68`) — mixes toward transparency, not accent (pass 29). Probe-tested.
- [x] Thumb diameter + min-height from the contract §8 size table (`thumb_diameter_rem` / `min_height_rem`), track thickness `rem_to_px(0.375)` — no invented ratios (pass 29). Probe-tested.
- [x] Pill radius from `radius.pill`, thumb border `rem_to_px(0.0625)` (pass 29).
- [x] Thumb drop shadow CLOSED (pass 42) — `0 0.125rem 0.5rem black@0.18` via a custom `BoxShadow` on `style.shadow` (JsEl exposes box-shadow — `style.shadow: Option<BoxShadow>` is public, same pattern as `tooltip.rs`; the "shadow once a primitive exists" note was based on a wrong assumption). Offsets are rem; only black@0.18 is a noted literal (no shadow token). Tree-test added (`thumb_has_contract_drop_shadow`).
- [ ] No vertical orientation — `spec.orientation` is ignored; layout is hardcoded horizontal flex-row. Contract §7 requires a vertical axis. **Open** — preview/layout axis branch (preview-loop bound).
- accepted: No drag/keyboard interaction in the component — value change + commit + keyboard live in the preview event loop (consistent with other Jetstream primitives; no focus/input primitive on the runtime). The component renders track + fill + thumb at the spec's current value.
- accepted: no ARIA channel (no accessibility API on the Jetstream runtime).

## Specimen parity

- Svelte covers: Default (Volume 65, live readout), With step (Opacity 100, step 10, live readout), Disabled (40), and a `sizes` snippet (value 50 across xs–sm–md–lg–xl). Densities suppressed (`showDensities={false}`). (`SliderSpecimen.svelte`)
- GPUI covers: Default (Volume 65, interactive `on_change` + readout), With step (Opacity, step 10, interactive), Disabled (40), plus `specimen_layout` size grid (value 60) and density grid (value 60). — missing: **value-readout text uses a hand-built `div().text_sm()`** which is fine, but parity matches Svelte well; no vertical specimen (none in Svelte either). Closest to Svelte of the three.
- Jetstream covers: Default (50%), Low value (10%), High value (90%), Disabled (50). — missing: **With step** group (Svelte's step=10 case), **live value readout** under each slider (Jetstream sliders are static so no readout is possible), and the **size grid** (xs–xl) that Svelte + GPUI both show. Group labels also diverge (Default/Low/High vs Svelte's Default/With step/Disabled). `specimen=gap`.

## Notes

- Accepted track-height literal on both targets: `rem_to_px(0.375)` — no slider-track-height token exists yet, so the contract-exact 0.375rem is converted directly (faithful, not a magic value). Both targets now use it; the old Jetstream `0.25rem` literal was fixed in pass 29.
- Accepted noted color literal on both targets: the thumb shadow `black@0.18`. The contract value is `color-mix(black 18%, transparent)`; no shadow token resolves to it (`elevation.shadow.*` use a different color + offsets). Offsets/blur resolve from contract-exact rem; only the color is the literal.
- GPUI drag uses `on_children_prepainted` track bounds for hit math — a sound approach; with pass 42 the only remaining GPUI gap is native vertical (commit-on-keyup/mouse-up is the accepted 0.2.2 runtime delta).
- The contract distinguishes this single-thumb `Slider` from `RangeSlider` (§1 out-of-scope, §14 follow-up). All three implementations are correctly single-thumb.
- Cross-target state after pass 42: value/size/visual/token parity is complete on both Rust targets; keyboard value-semantics land on GPUI; the one shared open gap is **native vertical orientation** (both), which is a preview/layout axis branch beyond the build-only verification surface here.
