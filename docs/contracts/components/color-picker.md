# ColorPicker

Status: detailed contract
Updated: 2026-03-16

## 1. Purpose

- Component name: `ColorPicker`
- Layer: `foundation`
- Summary: a popover-based color selection control with a 2D
  saturation/brightness gradient pad, hue strip, optional alpha slider,
  switchable hex/RGB/HSL numeric inputs, and optional preset swatches
- In scope: hex/RGB/HSL input modes, gradient picker, hue slider, alpha
  channel (opt-in), swatch presets, controlled/uncontrolled open state,
  disabled state
- Out of scope: eyedropper tool, named color presets

## 2. Anatomy

```text
[Root .color-picker]  <div>
  ├── [Controls .color-picker__controls]
  │     ├── [Trigger .color-picker__trigger]  <button>
  │     │     └── [Preview .color-picker__preview]  <span>
  │     └── [Text Input .color-picker__input]  <input type="text"> (conditional: showInput)
  └── [Surface .color-picker__surface]  <div role="dialog"> (conditional: isOpen)
        ├── [Picker Area .color-picker__picker-area]  (flex row)
        │     ├── [Gradient Pad .color-picker__gradient]  <div role="slider">
        │     │     └── [Thumb .color-picker__gradient-thumb]  <div>
        │     └── [Controls Panel .color-picker__controls-panel]  (flex column)
        │           ├── [Hue Wrap .color-picker__hue-wrap]  → Slider(min=0 max=360)
        │           ├── [Alpha Wrap .color-picker__alpha-wrap]  → Slider(min=0 max=100) (conditional: showAlpha)
        │           └── [Mode Section .color-picker__mode-section]
        │                 ├── [Mode Toggle .color-picker__mode-toggle]  → SegmentedControl (Hex/RGB/HSL)
        │                 └── [Inputs .color-picker__inputs]
        │                       ├── hex mode: text input + optional alpha NumberInput
        │                       ├── rgb mode: 3x NumberInput (R/G/B) + optional alpha NumberInput
        │                       └── hsl mode: 3x NumberInput (H/S/L) + optional alpha NumberInput
        └── [Swatches .color-picker__swatches]  <div role="listbox"> (conditional: swatches.length > 0)
              └── [Swatch .color-picker__swatch]  <button role="option"> (repeated)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | outer inline-flex container | position, max-width |
| Controls | yes | trigger + input row | gap, alignment |
| Trigger | yes | button that opens surface popover | size, border, radius, focus ring |
| Preview | yes | color swatch inside trigger | inset, border-radius, background |
| Text Input | no | inline hex text entry | border, radius, background, typography, focus ring |
| Surface | yes (when open) | popover panel containing picker controls | position, padding, border, radius, shadow, background |
| Picker Area | yes | flex row: gradient left, controls right | gap, alignment |
| Gradient Pad | yes | 2D saturation/brightness area | width, aspect-ratio, border-radius, cursor, pseudo-element gradients |
| Controls Panel | yes | stacked controls beside gradient | flex, gap |
| Gradient Thumb | yes | draggable circle indicating current S/V | size, border, shadow, pointer-events |
| Hue Wrap | yes | container for hue Slider with gradient override | min-height, track gradient |
| Alpha Wrap | no | container for alpha Slider with checkerboard | min-height, track gradient, checkerboard |
| Mode Toggle | yes | SegmentedControl switching hex/RGB/HSL | min-height |
| Inputs | yes | row of channel inputs for current mode | gap, alignment |
| Swatches | no | preset color grid | gap, flex-wrap, border-top |
| Swatch | no | individual preset color button | size, border, radius, hover, active |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string` | `"#6366f1"` | no | current hex color value (#RRGGBB or #RRGGBBAA); when supplied, the host owns updates through `onChange` |
| `swatches` | `string[]` | `[]` | no | preset hex color values |
| `showInput` | `boolean` | `true` | no | whether to show the inline hex text input |
| `showAlpha` | `boolean` | `false` | no | whether to show alpha slider and alpha channel inputs |
| `disabled` | `boolean` | `false` | no | disables all interaction |
| `ariaLabel` | `string` | `"Color picker"` | no | accessible label for the root element |
| `open` | `boolean \| null \| undefined` | `undefined` | no | picker visibility; omit for uncontrolled mode, or supply a boolean and own updates through `onOpenChange` |
| `defaultOpen` | `boolean` | `false` | no | initial open state when uncontrolled |
| `defaultMode` | `ColorInputMode` | `"hex"` | no | initial input mode (hex/rgb/hsl) |
| `onChange` | `((value: string) => void) \| null` | `null` | no | called when the selected color changes |
| `onOpenChange` | `((open: boolean) => void) \| null` | `null` | no | called when the picker open state changes |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |

### Controlled And Uncontrolled

- `value` is host-owned when supplied; update it through `onChange`
- `open` follows the DatePicker pattern: omit it for uncontrolled internal
  state, or supply a boolean and own updates through `onOpenChange`
- Internal HSV state (h, s, v, alpha) is derived from `value` prop

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | initial render (closed) | trigger shows value color, optional hex input |
| open | click trigger or `defaultOpen` | surface popover appears below trigger |
| disabled | `disabled=true` | reduced opacity, no pointer events |
| gradient dragging | pointerdown on gradient pad | thumb tracks pointer, S/V update live |
| swatch active | swatch matches current value | swatch has primary-colored border |
| swatch hovered | pointer over swatch | swatch scales up |

### Component States

| State | Type | Initial |
|-------|------|---------|
| `h` | `number` (0-360) | derived from `value` |
| `s` | `number` (0-100, HSV) | derived from `value` |
| `v` | `number` (0-100, HSV) | derived from `value` |
| `alpha` | `number` (0-1) | derived from `value` or 1 |
| `inputMode` | `ColorInputMode` | `defaultMode` |
| `isOpen` | `boolean` | `open ?? uncontrolledOpen` |

## 5. Callbacks

| Callback | When It Runs | Payload | Notes |
|----------|--------------|---------|-------|
| `onChange` | color changes via gradient, slider, input, or swatch | `string` | normalized hex string |
| `onOpenChange` | popover opens or closes | `boolean` | runs for both controlled and uncontrolled use |

## 6. Accessibility

### Semantics

- Root: `<div>` with `aria-label` from prop
- Trigger: `<button>` with `aria-haspopup="dialog"`, `aria-expanded`,
  `aria-controls` pointing to surface
- Surface: `role="dialog"`, `aria-label="Color picker"`
- Gradient pad: `role="slider"`, `aria-label="Saturation and brightness"`,
  `aria-valuetext` describing current S% and V%
- Disabled: `data-disabled` attribute on root
- Swatches container: `role="listbox"`, `aria-label="Color swatches"`
- Each swatch: `<button>` with `role="option"`, `aria-selected`, `aria-label`
  set to the hex value

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves through: gradient pad, hue slider, alpha slider, mode toggle, inputs, swatches |
| `Arrow keys` | on gradient pad: adjust S (left/right) and V (up/down) by 1 |
| `Shift + Arrow` | on gradient pad: adjust S/V by 10 |
| `Escape` | closes popover, returns focus to trigger |
| `Space` / `Enter` | on swatch, selects that color |

### Focus And Announcement

- Trigger shows focus ring on `:focus-visible`
- Gradient pad shows focus ring on `:focus-visible`
- All inputs and sliders have standard focus behavior
- Swatch shows outline on `:focus-visible`

## 7. Layout

### Sizing

- Root: `display: inline-flex` (no max-width — popover is absolutely positioned)
- Trigger: `2.25rem` square
- Inline hex text input: `6.5rem` wide, `2.25rem` tall
- Surface width: `24rem`, positioned absolutely below trigger
- Picker area: flex row — gradient pad left, controls panel right
- Gradient pad: `10rem` wide, `aspect-ratio: 1` (square)
- Controls panel: `flex: 1`, stacked vertically beside gradient
- Swatch buttons: `1.25rem` square

### Composition

- parent expectations: form fields, settings panels, toolbar popovers
- child expectations: no child slots; uses Slider, SegmentedControl,
  NumberInput internally via the shared numeric-entry implementation

## 8. Token Usage — Exact Values

### Root `.color-picker`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `inline-flex` |
| `flex-direction` | `column` |

### Root Disabled `.color-picker[data-disabled]`

| Property | Value |
|----------|-------|
| `opacity` | `var(--poodle-state-opacity-disabled)` |
| `pointer-events` | `none` |

### Trigger `.color-picker__trigger`

| Property | Value |
|----------|-------|
| `width` | `2.25rem` |
| `height` | `2.25rem` |
| `min-height` | `0` |
| `padding` | `0` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 62%, transparent)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `transparent` |
| `cursor` | `pointer` |
| `overflow` | `hidden` |

### Inline Text Input `.color-picker__input` (conditional: showInput)

| Property | Value |
|----------|-------|
| `width` | `6.5rem` |
| `height` | `2.25rem` |
| `min-height` | `0` |
| `padding` | `0 var(--poodle-space-control-x)` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-primary)` |
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `0.8125rem` |

### Surface `.color-picker__surface`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `top` | `100%` |
| `left` | `0` |
| `z-index` | `50` |
| `width` | `24rem` |
| `margin-top` | `0.25rem` |
| `padding` | `0.75rem` |
| `border` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `var(--poodle-color-background-elevated)` |
| `box-shadow` | `var(--poodle-shadow-lg)` |

### Picker Area `.color-picker__picker-area`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `gap` | `0.625rem` |
| `align-items` | `stretch` |

### Controls Panel `.color-picker__controls-panel`

| Property | Value |
|----------|-------|
| `flex` | `1` |
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `0.5rem` |
| `min-width` | `0` |

### Gradient Pad `.color-picker__gradient`

| Property | Value |
|----------|-------|
| `width` | `10rem` |
| `flex-shrink` | `0` |
| `aspect-ratio` | `1` |
| `border-radius` | `0.25rem` |
| `cursor` | `crosshair` |
| `touch-action` | `none` |
| `background-color` | _(inline: `hsl(h, 100%, 50%)`)_ |
| `::before` | `linear-gradient(to right, #fff, transparent)` |
| `::after` | `linear-gradient(to bottom, transparent, #000)` |

### Gradient Thumb `.color-picker__gradient-thumb`

| Property | Value |
|----------|-------|
| `width` | `0.875rem` |
| `height` | `0.875rem` |
| `min-height` | `0` |
| `border` | `0.125rem solid #fff` |
| `border-radius` | `50%` |
| `box-shadow` | `0 0 0 0.0625rem rgba(0,0,0,0.3), inset 0 0 0 0.0625rem rgba(0,0,0,0.1)` |

### Hue Slider Override `.color-picker__hue-wrap :global(.slider__track)`

| Property | Value |
|----------|-------|
| `background` | `linear-gradient(to right, #f00 0%, #ff0 17%, #0f0 33%, #0ff 50%, #00f 67%, #f0f 83%, #f00 100%)` |
| fill | hidden |

### Alpha Slider Override `.color-picker__alpha-wrap :global(.slider__track)`

| Property | Value |
|----------|-------|
| `background` | `linear-gradient(to right, transparent, var(--poodle-cp-alpha-color)), repeating-conic-gradient(checkerboard)` |
| fill | hidden |

### Text Input (surface) `.color-picker__text-input`

| Property | Value |
|----------|-------|
| `height` | `2rem` |
| `min-height` | `0` |
| `padding` | `0 0.375rem` |
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `0.75rem` |

### Input Label `.color-picker__input-label`

| Property | Value |
|----------|-------|
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.625rem` |
| `font-weight` | `var(--poodle-typography-label-weight)` |
| `color` | `var(--poodle-color-text-secondary)` |
| `text-transform` | `uppercase` |

### Swatches `.color-picker__swatches`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `gap` | `0.25rem` |
| `padding-top` | `0.25rem` |
| `border-top` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 42%, transparent)` |

### Swatch `.color-picker__swatch`

| Property | Value |
|----------|-------|
| `width` | `1.25rem` |
| `height` | `1.25rem` |
| `min-height` | `0` |
| `padding` | `0` |
| `border` | `0.125rem solid transparent` |
| `border-radius` | `0.1875rem` |
| `cursor` | `pointer` |

### Swatch Hover `.color-picker__swatch:hover`

| Property | Value |
|----------|-------|
| `transform` | `scale(1.15)` |

### Swatch Active `.color-picker__swatch--active`

| Property | Value |
|----------|-------|
| `border-color` | `var(--poodle-color-text-primary)` |
| `box-shadow` | `0 0 0 0.0625rem var(--poodle-color-background-surface)` |

### Size adjustments

| Size | trigger width/height | input height | input font-size |
|------|----------------------|--------------|-----------------|
| `xs` | `1.75rem` | `1.75rem` | `0.6875rem` |
| `sm` | `2rem` | `2rem` | `0.75rem` |
| `md` | `2.25rem` | `2.25rem` | _(base)_ |
| `lg` | `2.5rem` | `2.5rem` | `0.875rem` |
| `xl` | `2.75rem` | `2.75rem` | `0.9375rem` |

## 9. Svelte Notes

- Internal color model uses HSV (not HSL) for the gradient pad, as it maps
  naturally to the 2D white-corner/black-corner square
- Popover follows the DatePicker pattern: own surface div, `onMount` document
  listeners for outside click and Escape, `setOpen()` helper, controlled/uncontrolled
  open state
- Gradient pad uses CSS pseudo-elements for the white-to-transparent and
  transparent-to-black overlays; pointer tracking via `setPointerCapture`
- Hue and alpha sliders reuse the Slider component with `:global()` CSS
  overrides for custom track backgrounds and hidden fill
- Mode toggle uses SegmentedControl; channel inputs use NumberInput
- Color conversion utilities are in `color-utils.ts`: hexToRgb, rgbToHex,
  rgbToHsl, hslToRgb, rgbToHsv, hsvToRgb, hexToHsv, hsvToHex, etc.
- `min-height: 0` is applied to gradient thumb, slider wrappers, trigger,
  text inputs, and swatches to override the global button/input min-height reset
- `data-size` attribute on root reflects the resolved size for CSS variant styling
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::color_picker`
- GPUI should implement its own gradient pad rendering (GPU shader or
  equivalent) rather than CSS pseudo-elements
- Hue strip: render a horizontal gradient through all hue stops
- Alpha strip: render checkerboard pattern with color-to-transparent overlay
- Surface popover: use GPUI's overlay/popover primitives for positioning
- Slider, SegmentedControl, NumberInput: use GPUI equivalents
- `color-mix` values: GPUI computes equivalent alpha blending
- Swatch scale transform: GPUI applies 1.15x scale on hover

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] `value` prop meaning and hex format match
- [ ] `swatches` prop as array of hex strings matches
- [ ] `showInput` conditional rendering matches
- [ ] `showAlpha` conditional rendering matches
- [ ] `disabled` opacity and pointer-events behavior matches
- [ ] `open` / `defaultOpen` controlled/uncontrolled pattern matches
- [ ] `defaultMode` initial input mode matches
- [ ] `onChange` callback payload `string` matches
- [ ] `onOpenChange` callback payload `boolean` matches
- [ ] gradient pad role="slider" and aria-valuetext match
- [ ] surface role="dialog" matches
- [ ] trigger aria-haspopup and aria-expanded match
- [ ] swatch `role="option"` and `aria-selected` semantics match

### Tier 2: Visual Parity

- [ ] all five sizes visually match (height, padding, font-size per size table)
- [ ] root inline-flex without max-width matches
- [ ] trigger 2.25rem square matches
- [ ] surface width 24rem matches
- [ ] picker area flex row layout matches
- [ ] gradient pad 10rem wide matches
- [ ] controls panel flex:1 beside gradient matches
- [ ] surface padding 0.75rem matches
- [ ] surface border-radius uses radius-surface token
- [ ] surface background uses background-elevated token
- [ ] gradient pad aspect-ratio 1 matches
- [ ] gradient pseudo-element gradients match
- [ ] gradient thumb 0.875rem with white border matches
- [ ] hue slider gradient stops match
- [ ] alpha slider checkerboard pattern matches
- [ ] swatch 1.25rem square matches
- [ ] swatch hover scale 1.15 matches
- [ ] disabled opacity uses state-opacity-disabled token

### Tier 3: Implementation Freedom

- [ ] internal color model (HSV vs HSL) stays implementation-owned
- [ ] pointer tracking mechanism stays implementation-owned
- [ ] color conversion algorithm details stay implementation-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Replaces native `<input type="color">` with custom popover | native picker is inconsistent across platforms | allowed | unified UX across Svelte and GPUI |
| GPUI may use GPU shader for gradient pad | CSS pseudo-elements not available in GPUI | allowed | must produce visually equivalent gradient |

## 13. Specimen Definitions

### Basic Picker

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Basic picker | `value="#6366f1"` | Color trigger swatch with hex text input; clicking trigger opens picker surface; displays selected hex value below |

### With Swatches

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With swatches | `value="#6366f1"`, `swatches=["#ef4444","#f97316","#eab308","#22c55e","#3b82f6","#6366f1","#8b5cf6","#ec4899"]` | Color picker with preset swatch grid in the surface panel; active swatch has primary border |

### With Alpha

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With alpha | `value="#3b82f6"`, `showAlpha` | Color picker with alpha slider and alpha channel input visible in surface; displays selected value including alpha below |

### Default Open, RGB Mode

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default open, RGB mode | `value="#22c55e"`, `defaultOpen`, `defaultMode="rgb"` | Color picker surface initially open with RGB input mode selected in mode toggle; shows R/G/B number entry fields |

### Preview Only (No Input)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Preview only (no input) | `value="#6366f1"`, `showInput={false}` | Color trigger swatch only, no inline hex text input beside it |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | `value="#22c55e"`, `disabled` | Color picker with reduced opacity, no pointer events |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: theme editors, tag color selectors, chart color config
- future follow-up: eyedropper integration, named color presets
