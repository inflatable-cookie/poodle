# Foundation Contracts

Status: active
Updated: 2026-03-23

Foundation contracts define the lowest reusable surface in Flint.

## Current Contracts

- `accordion.md`
- `alert-dialog.md`
- `box.md`
- `breadcrumbs.md`
- `bulk-action-bar.md`
- `button.md`
- `calendar.md`
- `callout.md`
- `card.md`
- `checkbox.md`
- `code.md`
- `collapse-toggle.md`
- `collapsible.md`
- `color-picker.md`
- `combobox.md`
- `context-menu.md`
- `date-picker.md`
- `date-range-picker.md`
- `date-time-picker.md`
- `date-time-range-picker.md`
- `detail-row.md`
- `dialog.md`
- `drawer.md`
- `duration-input.md`
- `editable-label.md`
- `eyebrow.md`
- `field.md`
- `field-set.md`
- `file-upload.md`
- `form-actions.md`
- `grid.md`
- `hover-card.md`
- `icon-button.md`
- `icon-provider.md`
- `icon.md`
- `list-card-counter.md`
- `list-card.md`
- `menu.md`
- `menubar.md`
- `meter.md`
- `nav-card-grid.md`
- `nav-card.md`
- `navigation-menu.md`
- `number-entry.md`
- `order-by.md`
- `pagination-summary.md`
- `pagination.md`
- `pill.md`
- `pin-input.md`
- `popover.md`
- `progress.md`
- `radio-group.md`
- `range-calendar.md`
- `range-slider.md`
- `rating.md`
- `region.md`
- `resize-handle.md`
- `scroll-shell.md`
- `search-field.md`
- `segmented-control.md`
- `select.md`
- `separator.md`
- `skeleton.md`
- `slider.md`
- `spacer.md`
- `split-button.md`
- `stack.md`
- `status-bar.md`
- `status-indicator.md`
- `surface-elevation.md`
- `surface.md`
- `switch.md`
- `tab-strip.md`
- `table.md`
- `tabs.md`
- `text-area.md`
- `text-input.md`
- `time-ago.md`
- `time-field.md`
- `time-zone-select.md`
- `toggle-group.md`
- `toggle.md`
- `toolbar.md`
- `tooltip.md`
- `tri-state-switch.md`
- `zoned-date-time-picker.md`

## Accessibility Rule

Foundation primitives still need explicit accessibility treatment even when
they are mostly structural. In particular:

- non-interactive layout primitives should stay accessibility-neutral by
  default
- semantic surfaces should opt into region or landmark semantics explicitly
- scroll containers must define keyboard reachability and focus behavior
- GPUI implementations must preserve equivalent semantics through native
  accessibility APIs where ARIA is not available

## Next Task

Keep this index aligned with the actual contract files whenever primitives are
added, promoted, or renamed so docs lint stays authoritative.
