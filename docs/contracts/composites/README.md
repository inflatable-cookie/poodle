# Composite Contracts

Status: active
Updated: 2026-03-11

Composite contracts define reusable application- and product-style components
built from foundation primitives without embedding app-specific workflow logic.

## Current Contracts

- `card.md`
- `page-header.md`
- `breadcrumbs.md`
- `detail-row.md`
- `detail-section.md`
- `detail-shell.md`
- `filter-toolbar.md`
- `browse-search-shell.md`
- `data-table.md`
- `bulk-action-bar.md`
- `pagination-summary.md`
- `selection-summary.md`
- `picker-shell.md`
- `relation-picker.md`
- `media-thumbnail.md`
- `media-preview.md`
- `toast-stack.md`
- `empty-state.md`

## Composition Rule

Composite contracts should:

- compose documented foundation primitives rather than redefining them
- stay generic enough for Underlay-style product apps and Loophole-adjacent
  settings, library, and inspector surfaces
- keep data fetching, command wiring, persistence, and domain-specific row/card
  content outside the composite contract itself
- keep accessibility explicit for heading hierarchy, region labeling, empty
  states, and collection-browse shells in both Svelte and GPUI

## Next Task

Use this product-composite layer while executing `g02.010`, especially where
docking, split layouts, and shell orchestration need to compose onto the
now-stable browse, picker, media, hardening, command-discovery, and
workspace-shell surfaces.
