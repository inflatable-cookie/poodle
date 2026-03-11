# g01.010 Overlay, Navigation, And Interaction Primitives

Status: planned
Owner: Pug Core
Updated: 2026-03-11
Depends on: g01.003, g01.004, g01.005, g01.006, g01.007
Primary repos: `pug`

## Context

Tabs, menus, dialogs, drawers, tooltips, and popovers are essential for both
product UIs and workstation shells.

## Goals

- [ ] define tabs and reorderable tab-strip semantics
- [ ] define menu and context-menu semantics
- [ ] define tooltip, popover, dialog, and drawer semantics
- [ ] define focus trapping, dismissal, layering, and overlay state rules

## Execution Checklist

- [ ] define the tab family and reorder model
- [ ] define menu and context-menu trigger, positioning, and dismissal rules
- [ ] define tooltip and popover behavior
- [ ] define dialog and drawer lifecycle rules
- [ ] tie overlay layering back to tokenized z-order and motion rules

## Acceptance Criteria

- [ ] overlay and navigation primitives are documented
- [ ] focus and dismissal behavior are explicit
- [ ] layering and z-order rules tie back to tokens

## Deliverables

- [ ] overlay/navigation contract set
- [ ] tab and menu contract set

## Next Task

Open `g01.011` and define the first product composites built above the
primitive layer.
