<!-- parity consv=fixed gpui=0 jetstream=8 specimen=gap -->
# Parity: ColorPicker

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/color-picker.md`
- Svelte (authoritative): `packages/svelte/components/src/ColorPicker.svelte` (+ `color-utils.ts`)
- GPUI: `packages/gpui/components/src/primitives/color_picker.rs`
- Jetstream: `packages/jetstream/components/src/color_picker.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/ColorPickerSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/color_picker.rs` · jetstream `packages/jetstream/preview/src/specimens/color_picker.rs`

## Contract ↔ Svelte

Svelte implements the full contract surface (all props, anatomy parts, ARIA). Divergences:

- [x] **Inline trigger hex input typography.** Added an "Inline Text Input `.color-picker__input`" table to §8 documenting width `6.5rem`, height `2.25rem`, padding `0 space-control-x`, border `border-default`, radius `radius-control`, background `background-surface`, code-family `0.8125rem`. FIXED.
- [x] **Inline input width.** `6.5rem` now documented in §7 and the new §8 table. FIXED.
- **Density override touches trigger horizontal padding only** (lines 1021-1022) — compliant with the size/density rule (horizontal padding, not height). OK; note for completeness.
- `defaultMode` / mode toggle / NumberInput channel inputs / HSV model — all present in Svelte (lines 240-626), all in contract §2. OK.
- Net: contract is essentially complete; `consv=gap` is driven only by the two undocumented inline-input dimensions. Low-severity.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

GPUI moved to `packages/gpui/components/src/primitives/color_picker.rs` (full
surface build-out). Trigger swatch already reflected the real value; the surface
now renders the defining controls.

- [x] **Gradient pad built** — 2D saturation/value pad: base = pure-hue
  `hsl(h,100%,50%)` computed from the value, with two layered single-stop
  gradient children (white→transparent, transparent→black, mirroring the CSS
  `::before`/`::after`) and a thumb ring at current S/V. FIXED.
- [x] **Hue + alpha sliders built** — hue strip is six stacked two-stop
  gradient segments (GPUI 0.2.2 caps gradients at two stops; faithful layered
  approximation of the 7-stop CSS rainbow) with a thumb at current hue. Alpha
  strip is a transparent→color overlay on a neutral base (checkerboard
  approximated — no native repeating-conic-gradient) with a thumb at current
  alpha. Opt-in via `show_alpha`. FIXED.
- [x] **Mode toggle + channel inputs built** — real `SegmentedControl`
  (Hex/RGB/HSL) seeded to `default_mode`; channel inputs are labelled
  `NumberInput`s (R/G/B, H/S/L, +A when alpha) / hex code field, all showing the
  current value computed from the spec. FIXED.
- [x] Shadow literals removed — surface uses token-resolved
  `elevation_overlay_shadow()`. FIXED.
- [x] Hex-input height now `2rem` (matches Svelte surface text input). FIXED.
- [x] Swatch `--active` treatment added — active swatch (matches current value)
  gets text-primary border + surface ring; inactive border transparent. FIXED.
- note: surface-width `24rem`, gradient-pad `10rem`, swatch `1.25rem`, thumb
  `0.875rem`, gradient-thumb ring/alpha-thumb sizes — these are fixed
  rem-from-contract values rendered via `rem_to_px` (no dedicated tokens exist;
  same posture as Svelte's literal rem values). Hue/alpha thumb diameter uses
  the `size.icon.md` token (matching the GPUI Slider thumb).
- note: hue/alpha sliders are custom strips, not the shared `Slider` component —
  the GPUI `Slider` track has no custom-gradient-background hook, so composing it
  could not render the rainbow/checkerboard track. Structure + thumb match.
- accepted: interaction (gradient drag, slider drag, mode switch, hex/channel
  edit) is preview-event-loop bound — controls render at the current value; the
  embedded SegmentedControl/NumberInput carry their own preview wiring.
- accepted: no ARIA (gpui has no accessibility API) — role="dialog"/"slider"/"listbox"/"option", aria-valuetext not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] **Preview swatch shows the wrong color** — `js_color_picker` fills the trigger preview with `color.accent.base` (`color_picker.rs:40-46`) regardless of `spec.value`. The real hex value is never parsed to a color. This is a fake/placeholder visual (CLAUDE.md forbids). **Fix: parse `spec.current_value()` hex → Color and fill the preview with it.**
- [ ] **No surface at all** — only the trigger + inline hex input render. Gradient pad, hue/alpha sliders, mode toggle, channel inputs, swatch grid (contract §2) entirely absent. `js_color_picker` returns just the trigger row.
- [ ] Hardcoded preview-radius literal `rem_to_px(0.125)` at `color_picker.rs:44` — no token.
- [ ] Hardcoded controls-row gap literal `rem_to_px(0.5)` at `color_picker.rs:62` — resolve from a space token (Svelte `.poodle-color-picker__controls` gap is `0.5rem`).
- [ ] Inline input height uses `trigger_size` (`color_picker.rs:72`) — fine for md, but the contract inline input tracks the size table; verify per-size. Note.
- [ ] `swatches` prop accepted in specimen but never rendered (no swatch grid in component). Dead config.
- accepted: popover surface + all interaction (gradient drag, slider, mode switch, hex edit, swatch click) live in the preview `main.rs` overlay/event loop, per the file header — but none of those surface parts are emitted by the component for the loop to drive. **This is the core gap: the component must at minimum emit the surface anatomy.**
- accepted: no ARIA channel.

## Specimen parity

- Svelte covers: Basic picker, With swatches, With alpha, Default open + RGB mode, Preview only (no input), Disabled, sizes, densities (`ColorPickerSpecimen.svelte`).
- GPUI covers: Basic picker, With swatches, With alpha, Default open + RGB mode, Preview only (no input), Disabled, sizes, densities — **specimen set matches Svelte**, but the rendered surface is incomplete (no gradient/sliders/mode toggle), so the "alpha" / "RGB mode" specimens cannot show their distinguishing UI. Functional under-coverage → gap.
- Jetstream covers: With color, No color, With swatches, Disabled. — missing: **With alpha**, **Default open + RGB mode**, **Preview only**, **sizes**, **densities**; and "With swatches" renders no swatches (component emits none). Under-covers vs Svelte → `specimen=gap`.

## Notes

- The dominant gaps in both Rust targets are structural, not cosmetic: neither implements the gradient pad / hue+alpha sliders / mode toggle / channel inputs that constitute the component. GPUI at least renders an overlay shell with swatches; Jetstream renders only the trigger.
- Jetstream's accent-base preview fill is the clearest mockup violation across the three audited components — the trigger does not reflect the actual selected color.
- `consv=gap` is minor (two undocumented inline-input dimensions); the contract is otherwise a faithful description of the Svelte component.
