# Composite Contracts

Status: active
Updated: 2026-03-24

Composite contracts define reusable application- and product-style components
built from foundation primitives without embedding app-specific workflow logic.

## Current Contracts

- `action-discovery-panel.md`
- `app-header.md`
- `audio-player.md`
- `block-editor.md`
- `card-radio-group.md`
- `command-palette.md`
- `confirm-action.md`
- `data-table.md`
- `detail-section.md`
- `detail-shell.md`
- `dock-region.md`
- `editable-list.md`
- `embed-input.md`
- `embed-preview.md`
- `empty-state.md`
- `filter-toolbar.md`
- `form-dialog.md`
- `form-layout.md`
- `inline-list-section.md`
- `log-list.md`
- `list-container.md`
- `markdown-editor.md`
- `media-browse-panel.md`
- `media-picker.md`
- `media-preview.md`
- `media-thumbnail.md`
- `media-upload-status-panel.md`
- `metric-tile.md`
- `page-header.md`
- `page-loading.md`
- `picker-shell.md`
- `relation-picker.md`
- `selection-summary.md`
- `sidebar-nav.md`
- `split-view.md`
- `toast-host.md`
- `toast-stack.md`
- `video-player.md`

## Composition Rule

Composite contracts should:

- compose documented foundation primitives rather than redefining them
- stay generic enough for Underlay-style product apps and Loophole-adjacent
  settings, library, and inspector surfaces
- keep data fetching, command wiring, persistence, and domain-specific row or
  card content outside the composite contract itself
- keep accessibility explicit for heading hierarchy, region labeling, empty
  states, and collection-browse shells in both Svelte and GPUI

## Next Task

Keep this index aligned with the actual composite set whenever contract files
move between foundation and composite ownership.
