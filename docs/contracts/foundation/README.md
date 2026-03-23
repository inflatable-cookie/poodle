# Foundation Contracts

Status: active
Updated: 2026-03-12

Foundation contracts define the lowest reusable surface in Flint.

## Current Contracts

- `accordion.md`
- `box.md`
- `stack.md`
- `inline.md`
- `grid.md`
- `table.md`
- `spacer.md`
- `surface.md`
- `separator.md`
- `scroll-shell.md`
- `calendar.md`
- `range-calendar.md`
- `button.md`
- `toggle.md`
- `toggle-group.md`
- `toolbar.md`
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
- `navigation-menu.md`
- `menubar.md`
- `menu.md`
- `context-menu.md`
- `hover-card.md`
- `tooltip.md`
- `popover.md`
- `dialog.md`
- `drawer.md`
- `combobox.md`
- `collapsible.md`
- `date-picker.md`
- `date-range-picker.md`
- `time-field.md`
- `time-zone-select.md`
- `date-time-picker.md`
- `date-time-range-picker.md`
- `zoned-date-time-picker.md`
- `text-input.md`
- `field.md`
- `form-actions.md`
- `text-area.md`
- `search-field.md`
- `editable-label.md`
- `number-entry.md`
- `pin-input.md`
- `pagination.md`
- `progress.md`
- `meter.md`
- `rating.md`
- `skeleton.md`
- `badge.md`
- `pill.md`
- `callout.md`
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

Use this expanded foundation surface while deciding which wider substrate
families still deserve promotion into first-class contracts, with disclosure
now explicit and deeper integration ownership still open.
