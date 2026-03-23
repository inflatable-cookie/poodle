# g04.007 List Interaction, Reordering, And Sort Controls

Status: planned
Owner: Flint Core
Updated: 2026-03-14
Depends on: g04.001
Primary repos: `flint`

## Goals

- [ ] implement ReorderableList as a composite with drag-handle reordering
- [ ] implement AutonomousList as a composite for self-managing add/remove/
  reorder lists
- [ ] implement OrderBy as a composite sort-control toolbar for data views

## Execution Checklist

- [ ] write contract for ReorderableList: items array, drag handles, drop
  indicators, onReorder callback, keyboard reorder support
- [ ] implement ReorderableList composite in `@flint/svelte-composites`
- [ ] write contract for AutonomousList: items with add/remove/reorder, inline
  editing, empty state, max items
- [ ] implement AutonomousList composite in `@flint/svelte-composites`
- [ ] write contract for OrderBy: sort fields, active sort, direction toggle,
  multi-sort, reset
- [ ] implement OrderBy composite in `@flint/svelte-composites`
- [ ] create specimens for all three components
- [ ] register in component-registry.ts and specimen registry

## Acceptance Criteria

- [ ] ReorderableList renders items with drag handles and supports mouse/touch
  drag reordering
- [ ] ReorderableList supports keyboard reordering with Alt+Arrow keys
- [ ] ReorderableList fires onReorder with the new item order
- [ ] AutonomousList renders a list with add button, inline remove, and optional
  reorder
- [ ] AutonomousList supports inline editing of item text
- [ ] OrderBy renders sort field buttons with direction indicators
- [ ] OrderBy supports single and multi-field sorting
- [ ] all components pass build and render in the preview catalogue

## Next Task

Open `g04.008` and implement code display and color selection components.
