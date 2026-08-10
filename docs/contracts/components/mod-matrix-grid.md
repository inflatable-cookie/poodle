# Mod Matrix Grid

Status: detailed contract
Updated: 2026-08-11

## 1. Purpose

- Component name: `ModMatrixGrid`
- Layer: `foundation`
- Summary: generic source-by-destination grid of enabled parameter amounts

## 2. Anatomy

```text
[Root] labelled grid adapter
  [Column headers]
  [Rows]
    [Row header] [Cell controls]
  [Visual] aria-hidden, ModMatrixVisualState-only renderer
```

## 3. Props And Inputs

| Prop | Type | Default | Notes |
| --- | --- | --- | --- |
| `size` | `ControlSize \| null` | `null` | explicit `xs`–`xl`; otherwise inherited |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | semantic inherited-size offset |
| `density` | `ControlDensity \| null` | `null` | compact, default, or comfortable treatment |
| `sources` | `ModMatrixHeader[]` | `[]` | stable generic IDs and labels |
| `destinations` | `ModMatrixHeader[]` | `[]` | stable generic IDs and labels |
| `cells` | `ModMatrixCell[]` | `[]` | sparse or complete values with per-cell parameters |
| `step` | `number` | `0.01` | fallback keyboard nudge when a cell omits its own step |
| `disabled` | `boolean` | `false` | blocks edits and navigation changes |
| `ariaLabel` | `string \| null` | `"Modulation matrix"` | grid name |
| `onCellChange` | `(cell) => void` | `undefined` | live cell callback |
| `onCellCommit` | `(cell) => void` | `undefined` | atomic/gesture commit |

Headers are `{ id, label }`. Cells are `{ sourceId, destinationId, amount,
enabled, parameters? }`. Parameters are `{ min, max, step?, law? }` and belong
to the addressed cell, not its row, column, or grid. `law` uses the shared audio
law family. A range that crosses zero defaults to the bipolar-center law at
zero; a one-sided range defaults to linear. Omitted parameters preserve the
original `-1..1`, zero-centered bipolar behavior and use the grid `step`.

Duplicate header IDs and invalid ranges/laws are rejected. Missing cells
normalize to disabled zero with default parameters. Unknown cell IDs are
ignored. Amounts clamp through their own law; a stepped law also snaps. The
separate `step` parameter remains the normal keyboard nudge increment.

## 4. States And Behavior Machine

Classification: machine-backed (`modMatrixTransition`). Pointer activation
focuses a cell; Space toggles it. Arrows navigate by row/column, Home/End move
to row bounds, and Control+Home/End move to grid bounds. Page Up/Down nudges
amount using the focused cell's step; Shift uses one tenth step. Pointer drag
maps normalized vertical travel through that cell's range and law. Disabling a
cell preserves its amount and parameters.

## 5. Callbacks

Live drags and nudges emit cell change. Drag end and atomic toggle or keyboard
nudges emit commit. Callbacks include source/destination IDs and the resolved
parameters used for the change.

## 6. Accessibility

The adapter exposes a labelled grid with row and column headers. Each cell is a
focusable gridcell with enabled state, formatted amount, and its range. Roving
focus and all key translation are adapter-owned. Drawing is aria-hidden.

## 7. Layout

Rows follow caller source order; columns follow caller destination order.
VisualState is row-major and includes stable IDs, labels, focus, enabled state,
amount, resolved parameters, normalized amount, and normalized zero anchor.
The default grid is intrinsic-sized and
does not stretch when placed directly in a CSS grid; consumers may override
that alignment when a fluid matrix is intentional. Cell height follows the
size axis and never derives from column width. Bars expand from the published
zero anchor: centered for default bipolar cells, from the left for positive
unipolar cells, and from the right for negative unipolar cells. No synth
semantics are inferred.

## 8. Token Usage

Size changes header type and cell footprint. Density changes grid gaps,
padding, and indicator weight without changing navigation or amount laws.

`--poodle-recipe-mod-matrix-grid-fill`, `-border`, `-header`, `-cell`,
`-cell-enabled`, `-cell-disabled`, `-amount-negative`, `-amount-positive`,
`-zero`, `-focus-ring`, and `-disabled-opacity`.

## 9. Svelte Notes

The adapter owns roving focus, pointer capture, keyboard translation, and ARIA.
`ModMatrixVisual` receives VisualState only.

## 9a. React Notes

React uses the shared web transition and law. Its visual child receives only
the flattened VisualState.

## 10. GPUI Notes

The GPUI adapter maps pointer and platform keys into the Rust transition and
publishes row, column, and cell accessibility nodes.

## 10a. Jetstream Notes

Jetstream consumes the same Rust machine, spec, node builder, and grid
accessibility contract.

## 11. Parity Checklist

- same ID normalization, row-major ordering, per-cell parameters, and enable state
- same bounded navigation, roving focus, pointer drag, and keyboard nudge
- same header and formatted cell accessibility
- renderer never reads machine context

## 12. Known Deltas

Platform grid accessibility APIs differ. IDs, headers, values, focus order,
interaction effects, and drawing inputs are strict.

## 13. Specimen Definitions

All four previews provide sparse initialization, mixed bipolar/unipolar ranges,
positive/negative/zero amounts, disabled and enabled cells, focused cell,
pointer edit, keyboard navigation, row/grid bounds, empty axes, and disabled
grid.

## 14. Approval And Adoption Notes

The specimen page includes the full five-size and three-density matrices.
