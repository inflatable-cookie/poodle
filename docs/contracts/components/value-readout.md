# Value Readout

Status: detailed contract
Updated: 2026-08-10

## 1. Purpose

- Component name: `ValueReadout`
- Layer: `foundation`
- Summary: formatted read-only audio-domain value display

Formatted, non-editable display of an audio-domain numeric value. It consumes
the shared core formatter used by audio controls.

## 2. Anatomy

```text
[Output]
  [Visual] aria-hidden, VisualState plus core-formatted text
```

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Notes |
| --- | --- | --- | --- |
| `size` | `ControlSize \| null` | `null` | explicit `xs`–`xl`; otherwise inherited |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | semantic inherited-size offset |
| `density` | `ControlDensity \| null` | `null` | compact, default, or comfortable treatment |
| `value` | `number` | `0` | plain value |
| `min` | `number` | `0` | normalization floor |
| `max` | `number` | `1` | normalization ceiling |
| `law` | `AudioValueLaw` | `linear` | normalized mapping |
| `format` | `AudioValueFormat` | number | display vocabulary |
| `disabled` | `boolean` | `false` | visual state |
| `ariaLabel` | `string \| null` | `null` | accessible name |

Classification: machine-backed via `valueReadoutVisualState`; formatting and
serializable visual output are core-owned. The Svelte shell renders a
read-only output element and never reformats machine state itself.

Supported formats: raw number, dB, Hz with kHz promotion, explicit kHz,
percent, ratio, milliseconds with seconds promotion, MIDI note names, and
semitones. Each format defines display text and parse behavior in core.

## 4. States

Enabled and disabled are the only component states. Text follows the current
plain value.

## 5. Events

None. This component is display-only.

## 6. Accessibility

The output carries an optional accessible name. Its VisualState-only child is
hidden from assistive technology so the formatted value is announced once.

## 7. Layout

Inline intrinsic readout with tabular numerals. Parent layout owns labels and
control alignment.

## 8. Token Usage

Size changes type scale and minimum width. Density changes inline and block
padding without changing formatted content.

`--poodle-recipe-value-readout-fill`, `-text`, `-border`, and
`-disabled-opacity`.

## 9. Svelte Notes

The shell computes text through core formatting and passes it beside
VisualState to the display-only renderer.

## 9a. React Notes

React uses the same formatter and shared CSS. `ValueVisual` receives only
VisualState plus already-formatted text.

## 10. GPUI Notes

The Rust formatter is the native conformance source. The node builder receives
only readout VisualState and formatted text.

## 10a. Jetstream Notes

Jetstream consumes the same native spec and node builder as GPUI.

## 11. Parity Checklist

- identical formatting and enabled state
- one accessible announcement
- no renderer-owned formatting

## 12. Known Deltas

None. Font rasterization is runtime-owned; text and token meaning are strict.

## 13. Specimen Definitions

All four previews show number, dB, Hz/kHz, percent, ratio, milliseconds, note
name, and semitone formatting plus negative, boundary, and disabled states.

## 14. Approval And Adoption Notes

The specimen page includes the full five-size and three-density matrices.

Shared formatter changes require golden-value updates.
