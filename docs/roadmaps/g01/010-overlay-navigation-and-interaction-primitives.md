# g01.010 Overlay, Navigation, And Interaction Primitives

Status: completed
Owner: Flint Core
Updated: 2026-03-11
Depends on: g01.003, g01.004, g01.005, g01.006, g01.007
Primary repos: `flint`

## Context

Tabs, menus, dialogs, drawers, tooltips, and popovers are essential for both
product UIs and workstation shells.

## Goals

- [x] define tabs and reorderable tab-strip semantics
- [x] define menu and context-menu semantics
- [x] define tooltip, popover, dialog, and drawer semantics
- [x] define focus trapping, dismissal, layering, and overlay state rules

## Execution Checklist

- [x] define the tab family and reorder model
- [x] define menu and context-menu trigger, positioning, and dismissal rules
- [x] define tooltip and popover behavior
- [x] define dialog and drawer lifecycle rules
- [x] tie overlay layering back to tokenized z-order and motion rules

## Acceptance Criteria

- [x] overlay and navigation primitives are documented
- [x] focus and dismissal behavior are explicit
- [x] layering and z-order rules tie back to tokens

## Deliverables

- [x] overlay/navigation contract set
- [x] tab and menu contract set

## Next Task

Open `g01.011` and define the first product composites built above the now
complete primitive layer.
