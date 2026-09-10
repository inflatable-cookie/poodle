# Tabs card inactive surfaces

Status: open intake
Owner: Poodle Tabs
Created: 2026-09-10

## Issue

The `card` Tabs variant rounds every item but paints only the selected item.
Inactive tabs therefore read as labels floating beside one selected card rather
than as a row of cards.

## Operator-confirmed direction

- Every item in the `card` variant should visibly retain a card shape.
- Inactive cards use the semantic surface background.
- Selected cards use the existing selection strength: a dimmed accent for the
  tinted style and the full accent for the solid style.
- This is a Tabs visual-contract change, not a new app-specific component.

## Current evidence

- The contract describes inactive tabs as text-only and selected tabs as
  variant-filled.
- Shared web CSS applies the radius to every card item but applies background
  only to `[data-selected="true"]`.
- Shared Rust `render_card` likewise leaves inactive background unset; a
  focused native test asserts that absence.
- The existing `activeFill` values already carry `tint` and `solid`; a new
  selection-strength prop is unnecessary.

## Recommended contract shape

- `variant="card"` gives every item a rounded `color.background.surface`
  baseline.
- `activeFill="tint"` and `activeFill="solid"` replace that baseline on the
  selected card with the existing accent treatments.
- `activeFill="none"` suppresses only the accent selection fill. The selected
  card retains the same surface baseline as inactive cards, while selected text
  and any configured edge continue to carry selection.
- Hover should strengthen the inactive surface without impersonating selected
  tint. Disabled, drag, drop-target, outline, underline, close-button, and
  full-width behavior remain unchanged.
- Svelte, React, shared Rust composition, and GPUI should move together under
  the ordinary active cohort. Jetstream remains under its programme-level
  deferral.

## Unresolved confirmation

Confirm whether a surface fill plus the existing rounded radius is sufficient
for the inactive card shape, or whether every inactive card should also carry
a subtle border. The smaller recommendation is fill-only: it preserves the
existing `activeEdge="outline"` meaning and avoids making every card look
selected.

## Next check

After confirmation, promote the decision into the Tabs component contract and
one ready g18 task with paired web/native visual and interaction oracles.
