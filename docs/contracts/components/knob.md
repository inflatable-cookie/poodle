# Knob

Status: detailed contract
Updated: 2026-08-10

## 1. Purpose

- Component name: `Knob`
- Layer: `foundation`
- Summary: audio-oriented rotary value control with renderer-neutral behavior

## 2. Anatomy

Audio-oriented single-value control with linear, logarithmic, exponential,
stepped, or bipolar-center mapping.

```text
[Root] role=slider
  [Visual] aria-hidden, VisualState-only renderer
    [Track] [Arc] [Indicator]
  [Entry] conditional numeric text input
```

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Notes |
| --- | --- | --- | --- |
| `size` | `ControlSize \| null` | `null` | explicit `xs`–`xl`; otherwise inherited |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | semantic inherited-size offset |
| `density` | `ControlDensity \| null` | `null` | compact, default, or comfortable treatment |
| `value` | `number` | `0` | bindable plain value |
| `min` | `number` | `0` | lower plain bound |
| `max` | `number` | `1` | upper plain bound |
| `law` | `AudioValueLaw` | `linear` | normalized mapping |
| `defaultValue` | `number` | `0` | reset target |
| `dragMode` | `"vertical" \| "circular"` | `"vertical"` | pointer mapping |
| `dragSensitivity` | `number` | `160` | pixels for a full vertical sweep |
| `keyboardStep` | `number` | `0.01` | plain-value nudge |
| `format` | `AudioValueFormat` | number | display and parse vocabulary |
| `automation` | `AudioAutomationState` | `"none"` | host display state |
| `disabled` | `boolean` | `false` | interaction guard |
| `ariaLabel` | `string \| null` | `null` | accessible name |
| `onValueChange` | `(value: number) => void` | `undefined` | live value effect |
| `onValueCommit` | `(value: number) => void` | `undefined` | commit effect |
| `onGestureBegin` | `() => void` | `undefined` | gesture start effect |
| `onGestureEnd` | `() => void` | `undefined` | gesture end effect |

## 4. States And Behavior Machine

Classification: machine-backed (`knobTransition`). The core owns normalized
mapping, hover/focus/drag state, pointer delta and circular angle mapping, fine
adjustment, wheel changes, reset, keyboard nudges, and type-in commits.
Adapters provide pointer coordinates and modifier facts, then execute effects.

- Vertical drag up increases value; down decreases it.
- Circular drag maps the pointer angle over the standard 270 degree sweep.
- Shift selects fine adjustment at one tenth sensitivity.
- Wheel up increases and wheel down decreases.
- Double-click restores `defaultValue`.
- Enter opens type-in. Escape cancels. Enter or blur commits valid text.
- Arrows nudge; Page Up/Down use ten steps; Home/End select bounds.

## 5. Callbacks

`onValueChange` reports live changes. `onValueCommit` reports atomic changes
and drag end. `onGestureBegin` and `onGestureEnd` pair exactly once around
pointer drags.

## 6. Accessibility

Root exposes slider role, name, min/max/now, formatted value text, disabled
state, and keyboard operation. The visual renderer is hidden from assistive
technology. The entry uses a labelled text input drawing the standard ring
(`border-width-focus` solid `accent-focusRing`, offset `0.125rem`) while
editing; root focus shows the machine ring (box-shadow on the visual).

## 7. Layout

The standard renderer is an intrinsic square. Parent layout owns its label and
optional readout.

Size changes the square diameter across the `xs`–`xl` ladder. Density changes
ring weight without changing hit-testing or value geometry.

## 8. Token Usage

`--poodle-recipe-knob-track-fill`, `-track-border`, `-arc-fill`,
`-indicator-fill`, `-indicator-shadow`, `-entry-fill`, `-entry-border`,
`-entry-text`, `-focus-ring`, and `-disabled-opacity`.

## 9. Svelte Notes

The root adapter owns DOM events and ARIA. `KnobVisual` receives only
VisualState. Styles live in `poodle-core/styles/knob.css`.

## 9a. React Notes

The React shell runs the same `knobTransition`, formatting, and hit-test
helpers and shares `knob.css`. `KnobVisual` accepts only VisualState.

## 10. GPUI Notes

The GPUI specimen drives the Rust knob machine and passes its serializable
VisualState to the shared node renderer. The adapter owns pointer geometry,
focus, keys, wheel intent, and accessibility exposure.

## 10a. Jetstream Notes

Jetstream consumes the same Rust machine, spec, and node builder as GPUI. Its
adapter maps the slider role, formatted value, and gesture events into the
runtime accessibility and input systems.

## 11. Parity Checklist

- same laws, reset, fine mode, keyboard, wheel, entry, and gesture effects
- same formatted accessible value text
- renderer never reads machine context

## 12. Known Deltas

Native renderers use a token-themed ring plus position indicator because the
shared node vocabulary has no conic-gradient primitive. Value, focus,
interaction, and accessibility semantics remain strict.

## 13. Specimen Definitions

All four previews provide standalone groups for linear/default reset,
logarithmic frequency, bipolar center, stepped values, fine drag, circular
mode, disabled state, automation state, type-in, and the keyboard bounds.
Each page also includes the full five-size and three-density matrices.

## 14. Approval And Adoption Notes

Phase 1 review in Loophole gates further audio-family work.
