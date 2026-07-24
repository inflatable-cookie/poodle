# Component Contracts

Status: active
Updated: 2026-04-11

Component contracts define every reusable surface in Poodle, from low-level
primitives and layout surfaces through higher-order application components.

## Current Contracts

- `accordion.md`
- `action-discovery-panel.md`
- `alert-dialog.md`
- `app-header.md`
- `audio-player.md`
- `avatar.md`
- `block-editor.md`
- `box.md`
- `breadcrumbs.md`
- `bulk-action-bar.md`
- `button.md`
- `calendar.md`
- `callout.md`
- `card-radio-group.md`
- `card-toggle-group.md`
- `card.md`
- `checkbox.md`
- `code-input.md`
- `code.md`
- `collapse-toggle.md`
- `collapsible.md`
- `color-picker.md`
- `command-palette.md`
- `confirm-action.md`
- `context-menu.md`
- `data-table.md`
- `date-picker.md`
- `date-range-picker.md`
- `date-time-picker.md`
- `date-time-range-picker.md`
- `date-time-zone-picker.md`
- `debug-dialog.md`
- `detail-item.md`
- `detail-section.md`
- `detail-section-group.md`
- `detail-shell.md`
- `dialog.md`
- `dock-region.md`
- `drawer.md`
- `duration-input.md`
- `editable-label.md`
- `editable-list.md`
- `embed-input.md`
- `embed-preview.md`
- `empty-state.md`
- `error-boundary.md`
- `eyebrow.md`
- `field-set.md`
- `field.md`
- `file-upload.md`
- `filter-builder.md`
- `filter-toolbar.md`
- `form-actions.md`
- `form-dialog.md`
- `form-layout.md`
- `form-shell.md`
- `format-display-date.md`
- `format-file-size.md`
- `grid.md`
- `hover-card.md`
- `icon-button.md`
- `icon-provider.md`
- `icon.md`
- `inline-list-section.md`
- `inline-remediation.md`
- `list-card-counter.md`
- `list-card.md`
- `list-container.md`
- `list-grid.md`
- `log-list.md`
- `markdown-editor.md`
- `media-browse-panel.md`
- `media-picker.md`
- `media-preview.md`
- `media-thumbnail.md`
- `menu.md`
- `menubar.md`
- `meta-bar.md`
- `meta-item.md`
- `meter.md`
- `metric-tile.md`
- `nav-card.md`
- `navigation-menu.md`
- `number-input.md`
- `order-by.md`
- `page-header.md`
- `page-loading.md`
- `pagination-summary.md`
- `pagination.md`
- `password-requirements.md`
- `picker-shell.md`
- `pill.md`
- `popover.md`
- `progress.md`
- `radio.md`
- `radio-group.md`
- `range-slider.md`
- `rating.md`
- `region.md`
- `relation-picker.md`
- `remediation-banner.md`
- `resize-handle.md`
- `scroll-shell.md`
- `segmented-control.md`
- `select.md`
- `selection-summary.md`
- `separator.md`
- `sidebar-nav.md`
- `size-and-density.md`
- `skeleton.md`
- `slider.md`
- `spacer.md`
- `spinner.md`
- `split-button.md`
- `split-view.md`
- `stack.md`
- `state-tile.md`
- `status-bar.md`
- `status-indicator.md`
- `surface-elevation.md`
- `surface.md`
- `switch.md`
- `tab-strip.md`
- `table.md`
- `text.md`
- `text-link.md`
- `tabs.md`
- `text-input.md`
- `theme-select.md`
- `token-input.md`
- `time-ago.md`
- `time-input.md`
- `time-zone-select.md`
- `toast-host.md`
- `toast-stack.md`
- `toggle-group.md`
- `toolbar.md`
- `token-input.md`
- `tooltip.md`
- `treatment-tokens.md`
- `tree.md`
- `tri-state-switch.md`
- `ui-presentation-provider.md`
- `validation-summary.md`
- `video-player.md`

## Accessibility Rule

All components need explicit accessibility treatment. In particular:

- non-interactive layout primitives should stay accessibility-neutral by
  default
- semantic surfaces should opt into region or landmark semantics explicitly
- scroll containers must define keyboard reachability and focus behavior
- GPUI implementations must preserve equivalent semantics through native
  accessibility APIs where ARIA is not available

## Composition Rule

Higher-order components should:

- compose documented primitives rather than redefining them
- stay generic enough for Underlay-style product apps and Loophole-adjacent
  settings, library, and inspector surfaces
- keep data fetching, command wiring, persistence, and domain-specific row or
  card content outside the component contract itself
- keep accessibility explicit for heading hierarchy, region labeling, empty
  states, and collection-browse shells in both Svelte and GPUI

## Next Task

Keep this index aligned with the actual contract files whenever components are
added, promoted, or renamed so docs lint stays authoritative.
