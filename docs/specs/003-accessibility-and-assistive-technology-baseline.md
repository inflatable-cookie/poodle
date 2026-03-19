# 003 Accessibility And Assistive Technology Baseline

Status: active
Updated: 2026-03-11
Depends on: `002-component-contract-template-and-parity-rules.md`

## Purpose

Freeze the baseline accessibility rules that all Pug components must satisfy
across Svelte and GPUI, with explicit attention to the fact that GPUI cannot
rely on HTML semantics or ARIA alone.

## Core Rule

Accessibility support is part of the canonical component contract.

It is not:

- a web-only concern,
- a post-implementation audit,
- or a best-effort GPUI follow-up.

## Semantic Mapping Rule

### Svelte

The Svelte side should use semantic HTML first and ARIA only where native HTML
semantics are insufficient.

### GPUI

The GPUI side must provide equivalent semantic meaning through native
accessibility APIs and accessible node structures when HTML/ARIA are not
available.

Equivalent meaning includes:

- role or control type
- accessible name
- state
- value
- focusability
- relationship to controlled content

If GPUI lacks a built-in equivalent for a required behavior, Pug must document
and implement a native workaround rather than silently dropping the semantic.

## Focus Rule

All interactive components must provide:

- keyboard reachability
- visible focus
- logical focus order
- focus restoration when overlays or temporary surfaces close

Visual focus appearance may differ between Svelte and GPUI.
Focus visibility and movement semantics may not.

## Keyboard Rule

Keyboard behavior is Tier 1 strict parity.

That includes:

- activation keys
- directional navigation in composites
- escape and dismissal behavior
- home/end or page navigation where the pattern requires it
- keyboard scrolling for explicitly focusable scroll regions

## Announcement Rule

Dynamic changes that matter to assistive technology must be announced in both
runtimes.

Examples:

- dialogs opening or closing
- validation errors
- command completion/failure status
- loading state changes when they materially affect progress

Svelte may use live regions and ARIA patterns.
GPUI must use platform-native announcement mechanisms or equivalent accessible
event signaling.

## Decorative Element Rule

Purely decorative nodes must stay out of the accessibility tree in both
runtimes.

Examples:

- decorative separators
- spacers
- non-semantic background shells
- purely visual icon flourishes when a textual label already exists

## Region And Group Rule

Addressable regions and groups must be explicit.

Structural containers should remain neutral by default.

When a container becomes:

- a named region,
- a group,
- a landmark,
- or a focusable scroll destination,

its contract must define the exact labeling and focus behavior for both
runtimes.

## Overlay And Composite Navigation Rule

Overlays and tab-navigation primitives must explicitly define:

- invocation relationships
- roving-focus or trapped-focus behavior
- dismissal rules
- focus restoration
- and announcement behavior when content or modality changes

This is especially important for GPUI, where HTML and ARIA patterns do not
exist as a fallback implementation path.

## Reduced Motion And Contrast Rule

Pug must preserve the ability to support:

- reduced motion preferences
- sufficient contrast
- visible focus under varied themes

This baseline does not require every component to solve those concerns fully
yet, but contracts and tokens must not block them.

## Information Architecture Rule

Composite shells must preserve accessible information hierarchy rather than only
visual arrangement.

That includes:

- heading structure for page and section surfaces
- breadcrumb current-location semantics
- label/value relationships in detail views
- meaningful empty-state text independent of illustrations
- focus continuity when list, grid, and detail shells swap between ready,
  empty, loading, and error states

## GPUI-Specific Implementation Expectations

GPUI implementations must explicitly account for:

- accessible tree node creation
- focus order and focus restoration
- state/value exposure for controls
- announcement of dynamic state changes
- keyboard scrolling and keyboard navigation where the platform does not hand
  it to the implementation automatically
- shell-region hierarchy, collapse state, and resizable-divider semantics in
  workstation layouts

Missing GPUI accessibility behavior is a contract bug, not a visual delta.

## Contract Author Checklist

Every contract author must define:

- semantic role expectations
- accessible naming rules
- keyboard behavior
- focus behavior
- dismissal and focus-restoration behavior where overlays or temporary surfaces
  exist
- announcement behavior when dynamic changes exist
- decorative-node behavior where relevant
- GPUI-native accessibility expectations in the GPUI notes section

## Seed Evidence

The first primitive contracts that explicitly exercise this baseline are:

- `docs/contracts/foundation/box.md`
- `docs/contracts/foundation/surface.md`
- `docs/contracts/foundation/separator.md`
- `docs/contracts/foundation/scroll-shell.md`
- `docs/contracts/foundation/text-input.md`
- `docs/contracts/foundation/text-area.md`
- `docs/contracts/foundation/search-field.md`
- `docs/contracts/foundation/editable-label.md`
- `docs/contracts/foundation/number-entry.md`
- `docs/contracts/foundation/checkbox.md`
- `docs/contracts/foundation/radio-group.md`
- `docs/contracts/foundation/switch.md`
- `docs/contracts/foundation/select.md`
- `docs/contracts/foundation/slider.md`
- `docs/contracts/foundation/range-slider.md`
- `docs/contracts/foundation/banner.md`
- `docs/contracts/foundation/tabs.md`
- `docs/contracts/foundation/tab-strip.md`
- `docs/contracts/foundation/menu.md`
- `docs/contracts/foundation/context-menu.md`
- `docs/contracts/foundation/tooltip.md`
- `docs/contracts/foundation/popover.md`
- `docs/contracts/foundation/dialog.md`
- `docs/contracts/foundation/drawer.md`
- `docs/contracts/composites/page-header.md`
- `docs/contracts/composites/breadcrumbs.md`
- `docs/contracts/composites/detail-row.md`
- `docs/contracts/composites/detail-shell.md`
- `docs/contracts/composites/filter-toolbar.md`
- `docs/contracts/composites/empty-state.md`
- `docs/contracts/workstation/app-header.md`
- `docs/contracts/workstation/project-header.md`
- `docs/contracts/workstation/panel-header.md`
- `docs/contracts/workstation/panel-tabs.md`
- `docs/contracts/workstation/surface-tabs.md`
- `docs/contracts/workstation/dock-region.md`
- `docs/contracts/workstation/split-view.md`
- `docs/contracts/workstation/workspace-shell.md`
- `docs/contracts/workstation/command-palette-shell.md`

## Next Task

Use this accessibility baseline with the harder `g02.011` and later work,
especially now that advanced focus, keyboard, and state semantics have been
made explicit for the broader catalogue.
