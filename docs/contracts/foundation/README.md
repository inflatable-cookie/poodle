# Foundation Contracts

Status: active
Updated: 2026-03-11

Foundation contracts define the lowest reusable surface in Pug.

## Current Contracts

- `box.md`
- `stack.md`
- `inline.md`
- `grid.md`
- `spacer.md`
- `surface.md`
- `separator.md`
- `scroll-shell.md`
- `button.md`
- `icon-button.md`
- `checkbox.md`
- `radio-group.md`
- `switch.md`
- `tri-state-switch.md`
- `segmented-control.md`
- `select.md`
- `slider.md`
- `range-slider.md`
- `tabs.md`
- `tab-strip.md`
- `menu.md`
- `context-menu.md`
- `tooltip.md`
- `popover.md`
- `dialog.md`
- `drawer.md`
- `text-input.md`
- `field.md`
- `form-actions.md`
- `text-area.md`
- `search-field.md`
- `editable-label.md`
- `number-entry.md`
- `progress.md`
- `skeleton.md`
- `badge.md`
- `pill.md`
- `callout.md`
- `banner.md`
- `status-indicator.md`

## Accessibility Rule

Foundation primitives still need explicit accessibility treatment even when
they are mostly structural. In particular:

- non-interactive layout primitives should stay accessibility-neutral by
  default
- semantic surfaces should opt into region/landmark semantics explicitly
- scroll containers must define keyboard reachability and focus behavior
- GPUI implementations must preserve equivalent semantics through native
  accessibility APIs where ARIA is not available

## Next Task

Use this foundation surface while executing `g02.002`, especially as tables,
filters, and bulk actions start composing on top of the new form baseline.
