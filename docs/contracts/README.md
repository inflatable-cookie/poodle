# Contracts

Status: active
Updated: 2026-03-23

Contracts are the docs-first source of truth for Flint components.

Each component contract must describe one semantic component surface that both
Svelte and GPUI implementations can satisfy against the same review checklist.

## Structure

```text
docs/contracts/
  README.md
  template/
    component-contract-template.md
  foundation/
    ...
  composites/
    ...
```

## Grouping Rules

- `foundation/` for low-level reusable primitives and layout surfaces
- `composites/` for reusable higher-order application components
- workstation semantics are currently expressed through the shared contract
  crates plus the composite docs above, rather than a separate
  `docs/contracts/workstation/` subtree

App-specific DAW widgets do not belong in this contract surface. They build
above it in downstream repos such as Loophole.

## Current Seed Contracts

- `template/component-contract-template.md`
- `foundation/README.md`
- `foundation/accordion.md`
- `foundation/alert-dialog.md`
- `foundation/box.md`
- `foundation/breadcrumbs.md`
- `foundation/bulk-action-bar.md`
- `foundation/button.md`
- `foundation/calendar.md`
- `foundation/callout.md`
- `foundation/card.md`
- `foundation/checkbox.md`
- `foundation/code.md`
- `foundation/collapse-toggle.md`
- `foundation/collapsible.md`
- `foundation/color-picker.md`
- `foundation/combobox.md`
- `foundation/context-menu.md`
- `foundation/date-picker.md`
- `foundation/date-range-picker.md`
- `foundation/date-time-picker.md`
- `foundation/date-time-range-picker.md`
- `foundation/detail-row.md`
- `foundation/dialog.md`
- `foundation/drawer.md`
- `foundation/duration-input.md`
- `foundation/editable-label.md`
- `foundation/eyebrow.md`
- `foundation/field.md`
- `foundation/field-set.md`
- `foundation/file-upload.md`
- `foundation/form-actions.md`
- `foundation/grid.md`
- `foundation/hover-card.md`
- `foundation/icon-button.md`
- `foundation/icon-provider.md`
- `foundation/icon.md`
- `foundation/list-card-counter.md`
- `foundation/list-card.md`
- `foundation/menu.md`
- `foundation/menubar.md`
- `foundation/meter.md`
- `foundation/nav-card-grid.md`
- `foundation/nav-card.md`
- `foundation/navigation-menu.md`
- `foundation/number-entry.md`
- `foundation/order-by.md`
- `foundation/pagination-summary.md`
- `foundation/pagination.md`
- `foundation/pill.md`
- `foundation/pin-input.md`
- `foundation/popover.md`
- `foundation/progress.md`
- `foundation/radio-group.md`
- `foundation/range-calendar.md`
- `foundation/range-slider.md`
- `foundation/rating.md`
- `foundation/region.md`
- `foundation/resize-handle.md`
- `foundation/scroll-shell.md`
- `foundation/search-field.md`
- `foundation/segmented-control.md`
- `foundation/select.md`
- `foundation/separator.md`
- `foundation/skeleton.md`
- `foundation/slider.md`
- `foundation/spacer.md`
- `foundation/split-button.md`
- `foundation/stack.md`
- `foundation/status-bar.md`
- `foundation/status-indicator.md`
- `foundation/surface-elevation.md`
- `foundation/surface.md`
- `foundation/switch.md`
- `foundation/tab-strip.md`
- `foundation/table.md`
- `foundation/tabs.md`
- `foundation/text-area.md`
- `foundation/text-input.md`
- `foundation/time-ago.md`
- `foundation/time-field.md`
- `foundation/time-zone-select.md`
- `foundation/toggle-group.md`
- `foundation/toggle.md`
- `foundation/toolbar.md`
- `foundation/tooltip.md`
- `foundation/tri-state-switch.md`
- `foundation/zoned-date-time-picker.md`
- `composites/README.md`
- `composites/action-discovery-panel.md`
- `composites/app-header.md`
- `composites/audio-player.md`
- `composites/block-editor.md`
- `composites/card-radio-group.md`
- `composites/command-palette.md`
- `composites/confirm-action.md`
- `composites/data-table.md`
- `composites/detail-section.md`
- `composites/detail-shell.md`
- `composites/dock-region.md`
- `composites/editable-list.md`
- `composites/embed-input.md`
- `composites/embed-preview.md`
- `composites/empty-state.md`
- `composites/filter-toolbar.md`
- `composites/form-dialog.md`
- `composites/form-layout.md`
- `composites/log-list.md`
- `composites/markdown-editor.md`
- `composites/media-picker.md`
- `composites/media-preview.md`
- `composites/media-thumbnail.md`
- `composites/metric-tile.md`
- `composites/page-header.md`
- `composites/page-loading.md`
- `composites/picker-shell.md`
- `composites/relation-picker.md`
- `composites/reorderable-list.md`
- `composites/selection-summary.md`
- `composites/slug-field.md`
- `composites/split-view.md`
- `composites/toast-stack.md`
- `composites/video-player.md`

## Next Task

Keep this top-level index aligned with the foundation and composite contract
folders whenever contract files are added, removed, or regrouped.
