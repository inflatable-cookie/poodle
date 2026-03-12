# Contracts

Status: active
Updated: 2026-03-11

Contracts are the docs-first source of truth for Pug components.

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
  workstation/
    ...
```

## Grouping Rules

- `foundation/` for low-level reusable primitives and layout surfaces
- `composites/` for reusable higher-order application components
- `workstation/` for reusable shell and panel-system components that remain
  general across desktop/pro-tool apps

App-specific DAW widgets do not belong in this contract surface. They build
above it in downstream repos such as Loophole.

## Current Seed Contracts

- `template/component-contract-template.md`
- `foundation/box.md`
- `foundation/stack.md`
- `foundation/inline.md`
- `foundation/grid.md`
- `foundation/spacer.md`
- `foundation/surface.md`
- `foundation/separator.md`
- `foundation/scroll-shell.md`
- `foundation/button.md`
- `foundation/icon-button.md`
- `foundation/checkbox.md`
- `foundation/radio-group.md`
- `foundation/switch.md`
- `foundation/tri-state-switch.md`
- `foundation/segmented-control.md`
- `foundation/select.md`
- `foundation/slider.md`
- `foundation/range-slider.md`
- `foundation/tabs.md`
- `foundation/tab-strip.md`
- `foundation/menu.md`
- `foundation/context-menu.md`
- `foundation/tooltip.md`
- `foundation/popover.md`
- `foundation/dialog.md`
- `foundation/drawer.md`
- `foundation/text-input.md`
- `foundation/field.md`
- `foundation/form-actions.md`
- `foundation/text-area.md`
- `foundation/search-field.md`
- `foundation/editable-label.md`
- `foundation/number-entry.md`
- `foundation/progress.md`
- `foundation/skeleton.md`
- `foundation/badge.md`
- `foundation/pill.md`
- `foundation/callout.md`
- `foundation/banner.md`
- `foundation/status-indicator.md`
- `composites/card.md`
- `composites/page-header.md`
- `composites/breadcrumbs.md`
- `composites/detail-row.md`
- `composites/detail-section.md`
- `composites/detail-shell.md`
- `composites/filter-toolbar.md`
- `composites/browse-search-shell.md`
- `composites/data-table.md`
- `composites/bulk-action-bar.md`
- `composites/pagination-summary.md`
- `composites/list-shell.md`
- `composites/grid-shell.md`
- `composites/selection-summary.md`
- `composites/picker-shell.md`
- `composites/relation-picker.md`
- `composites/media-thumbnail.md`
- `composites/media-preview.md`
- `composites/embed-shell.md`
- `composites/toast-stack.md`
- `composites/empty-state.md`
- `workstation/README.md`
- `workstation/app-header.md`
- `workstation/project-header.md`
- `workstation/panel-surface.md`
- `workstation/panel-header.md`
- `workstation/panel-tabs.md`
- `workstation/surface-tabs.md`
- `workstation/dock-region.md`
- `workstation/split-view.md`
- `workstation/workspace-shell.md`
- `workstation/command-palette-shell.md`
- `workstation/command-palette.md`
- `workstation/action-discovery-panel.md`
- `workstation/shell-status-bar.md`

## Next Task

Carry the contract surface into `g03.001`, defining how token and contract
evolution can happen without destabilizing the newly explicit package surface.
