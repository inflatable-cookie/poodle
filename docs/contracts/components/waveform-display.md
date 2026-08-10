# Waveform Display

Status: detailed contract
Updated: 2026-08-10

## 1. Purpose

- Component name: `WaveformDisplay`
- Layer: `foundation`
- Summary: read-only inspector-scale peak-pyramid display with cursor and
  selection state

Timeline-scale waveform rendering is explicitly outside this contract.

## 2. Anatomy

```text
[Root] labelled cursor/selection adapter
  [Visual] aria-hidden, WaveformVisualState-only renderer
  [Accessible summary] adapter-owned cursor and selection text
```

## 3. Props And Inputs

| Prop | Type | Default | Notes |
| --- | --- | --- | --- |
| `size` | `ControlSize \| null` | `null` | explicit `xs`–`xl`; otherwise inherited |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | semantic inherited-size offset |
| `density` | `ControlDensity \| null` | `null` | compact, default, or comfortable treatment |
| `pyramid` | `WaveformPeakPyramid` | empty | reduced min/max levels; raw samples rejected |
| `visibleStart`, `visibleEnd` | `number` | full range | inclusive/exclusive sample viewport |
| `cursorSample` | `number \| null` | `null` | controlled cursor |
| `selection` | `{ start, end } \| null` | `null` | controlled ordered sample range |
| `columnCount` | `number` | measured width | requested columns, capped at 4,096 |
| `disabled` | `boolean` | `false` | blocks cursor/selection interaction |
| `ariaLabel` | `string \| null` | `"Waveform"` | accessible name |
| `onCursorChange` | `(sample) => void` | `undefined` | live cursor callback |
| `onSelectionChange` | `(selection) => void` | `undefined` | live selection callback |
| `onSelectionCommit` | `(selection) => void` | `undefined` | completed selection callback |

`WaveformPeakPyramid` has a sample count and ordered levels. Every level has a
positive integer `samplesPerPeak` and finite `{ min, max }` pairs in `-1..1`.

## 4. States And Behavior Machine

Classification: machine-backed (`waveformTransition`). The core validates the
pyramid, selects a fitting level, reduces it to at most
`WAVEFORM_MAX_COLUMNS = 4096`, and publishes the exact columns to draw.
Pointer positions are converted to sample indices before transition. Click
moves the cursor; drag owns an ordered selection. Arrows move the cursor, Shift
extends selection, Home/End target viewport bounds, and Escape clears it.

No transition edits samples or pyramid data. No renderer chooses a level.

## 5. Callbacks

Cursor movement emits `onCursorChange`. Drag and Shift navigation emit live
selection changes; release or atomic keyboard edits emit selection commit.

## 6. Accessibility

The root is a labelled slider for the cursor, with screen-reader value text for
viewport, cursor, and selection sample positions. Keyboard behavior is
adapter-owned. The dense visual bars are aria-hidden.

## 7. Layout And Scale Ceiling

The display fills an inspector or preview surface. Its VisualState contains no
more than 4,096 columns. Timeline tiles, streaming caches, background
reduction, raw PCM, and workstation GPU-scene integration are prohibited.
Changing that ceiling or scope requires an architecture decision.

## 8. Token Usage

Size changes display height and minimum width. Density changes column gap,
center-line weight, and cursor/selection weight without changing reduction.

`--poodle-recipe-waveform-display-fill`, `-border`, `-wave`, `-center-line`,
`-selection`, `-cursor`, `-focus-ring`, and `-disabled-opacity`.

## 9. Svelte Notes

Resize observation supplies a bounded column count. The adapter owns pointer
capture, keyboard events, and ARIA; `WaveformVisual` receives VisualState only.

## 9a. React Notes

React uses the same peak reduction and machine. Its SVG visual receives only
the selected columns and interaction flags.

## 10. GPUI Notes

The adapter supplies viewport width and local hit positions to the Rust core.
The node builder consumes reduced VisualState columns only.

## 10a. Jetstream Notes

Jetstream shares the Rust machine, spec, node builder, and bounded column
contract.

## 11. Parity Checklist

- same pyramid validation, level choice, aggregation, and 4,096-column cap
- same cursor/selection bounds, pointer mapping, and keyboard behavior
- no raw-sample or timeline path
- renderer never reads pyramid or machine context

## 12. Known Deltas

Column rasterization differs by backend. Selected min/max values, cursor,
selection, accessible text, and scale limit are strict.

## 13. Specimen Definitions

All four previews provide multilevel pyramid selection, zoomed viewport,
cursor, forward/reverse selection, keyboard movement, clipping bounds, empty,
disabled, and 4,096-column ceiling evidence.

## 14. Approval And Adoption Notes

The specimen page includes the full five-size and three-density matrices.
