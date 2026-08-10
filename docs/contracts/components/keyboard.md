# Keyboard

Status: detailed contract
Updated: 2026-08-11

## 1. Purpose

- Component name: `Keyboard`
- Layer: `foundation`
- Summary: virtual musical keyboard for pointer and computer-key note input,
  including a vertical piano-roll gutter orientation

## 2. Anatomy

```text
[Root] labelled keyboard adapter
  [Visual] aria-hidden, KeyboardVisualState-only renderer
  [Key controls] adapter-owned note buttons
```

## 3. Props And Inputs

| Prop | Type | Default | Notes |
| --- | --- | --- | --- |
| `size` | `ControlSize \| null` | `null` | explicit `xs`–`xl`; otherwise inherited |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | semantic inherited-size offset |
| `density` | `ControlDensity \| null` | `null` | compact, default, or comfortable treatment |
| `firstNote`, `lastNote` | `number` | `48`, `72` | inclusive MIDI range, clamped to `0..127` |
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | vertical is high-to-low, top-to-bottom gutter |
| `octaveShift` | `number` | `0` | computer-map shift in octaves |
| `computerKeyMap` | `Record<string, number>` | chromatic A–K map | key to semitone offset |
| `externalHeldNotes` | `number[]` | `[]` | host highlights; never emit note effects |
| `disabled` | `boolean` | `false` | releases local notes and blocks input |
| `ariaLabel` | `string \| null` | `"Keyboard"` | group name |
| `onNoteOn` | `(note, velocity) => void` | `undefined` | paired gesture start |
| `onNoteOff` | `(note) => void` | `undefined` | paired gesture end |

## 4. States And Behavior Machine

Classification: machine-backed (`keyboardTransition`). Pointer hit testing
resolves note and key depth before transition. Depth becomes MIDI velocity
`1..127`. A captured pointer retargets its physical input as it crosses keys,
emitting the old note's release before the new note's press. Leaving all keys
releases; re-entry presses the newly resolved key. Computer keys use the
configured map plus octave shift, ignore repeat keydowns, and release the exact
note started by that physical key.
Multiple input sources may hold one note; `noteOff` occurs only on final local
release. Range changes, disablement, and cancellation close active gestures.

Horizontal keys run low to high left-to-right. Vertical gutter keys run high
to low top-to-bottom and use left-to-right depth for velocity. External held
notes are a separate highlight set and do not enter gesture accounting.

## 5. Effects

`noteOn(note, velocity)` begins one per-note gesture. `noteOff(note)` ends it.
Every emitted start has one end, including cancellation and prop-driven range
or enabled changes.

## 6. Accessibility

The root is a labelled toolbar. Each visible note is an adapter-owned button
named with the shared note-name formatter and exposes local or external held
state through `aria-pressed`. Arrow navigation follows visual pitch order;
Space and Enter press and release the focused note. Drawing is aria-hidden.

## 7. Layout

Core geometry describes key order, kind, normalized start, length, and depth.
Black-key precedence belongs to core hit testing. Renderers do not calculate
note geometry.

## 8. Token Usage

Size changes key length, breadth, and label scale. Density changes key gaps,
border weight, and pressed inset without changing hit geometry or velocity.

`--poodle-recipe-keyboard-fill`, `-border`, `-white-key`, `-white-key-held`,
`-black-key`, `-black-key-held`, `-external-ring`, `-focus-ring`,
and `-disabled-opacity`.

## 9. Svelte Notes

The adapter owns pointer capture, computer-key listeners, focus, ARIA, and
effect callbacks. `KeyboardVisual` receives VisualState only.

## 9a. React Notes

React runs the shared web machine and geometry helpers. Its visual child has no
machine or event access.

## 10. GPUI Notes

The GPUI adapter resolves local coordinates and platform keys before the Rust
transition and exposes note-button accessibility children.

## 10a. Jetstream Notes

Jetstream consumes the same Rust transition, spec, VisualState node builder,
and accessibility tree.

## 11. Parity Checklist

- same range, octave map, velocity quantization, pointer retargeting,
  held-note accounting, and paired note effects
- same horizontal and vertical geometry and external highlighting
- same note names, focus order, and press semantics
- renderer never reads machine context

## 12. Known Deltas

Physical-key code normalization is adapter-specific. Resolved keys, notes,
effects, VisualState, and accessibility behavior are strict.

## 13. Specimen Definitions

All four previews provide horizontal input, vertical gutter, velocity bands,
computer-key mapping, octave shift, local chords, external highlights,
focus/pressed states, cancellation, range bounds, and disabled state.

## 14. Approval And Adoption Notes

The specimen page includes the full five-size and three-density matrices.
