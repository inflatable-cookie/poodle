# 003 Accessibility And Assistive Technology Baseline

Status: active
Updated: 2026-03-11
Depends on: `002-component-contract-template-and-parity-rules.md`

## Purpose

Freeze the baseline accessibility rules that all Poodle components must satisfy
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

If GPUI lacks a built-in equivalent for a required behavior, Poodle must document
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

Poodle must preserve the ability to support:

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

- `docs/contracts/components/box.md`
- `docs/contracts/components/surface.md`
- `docs/contracts/components/separator.md`
- `docs/contracts/components/scroll-shell.md`
- `docs/contracts/components/text-input.md`
- `docs/contracts/components/text-area.md`
- `docs/contracts/components/search-field.md`
- `docs/contracts/components/editable-label.md`
- `docs/contracts/components/number-entry.md`
- `docs/contracts/components/checkbox.md`
- `docs/contracts/components/radio-group.md`
- `docs/contracts/components/switch.md`
- `docs/contracts/components/select.md`
- `docs/contracts/components/slider.md`
- `docs/contracts/components/range-slider.md`
- `docs/contracts/components/banner.md`
- `docs/contracts/components/tabs.md`
- `docs/contracts/components/tab-strip.md`
- `docs/contracts/components/menu.md`
- `docs/contracts/components/context-menu.md`
- `docs/contracts/components/tooltip.md`
- `docs/contracts/components/popover.md`
- `docs/contracts/components/dialog.md`
- `docs/contracts/components/drawer.md`
- `docs/contracts/components/page-header.md`
- `docs/contracts/components/breadcrumbs.md`
- `docs/contracts/components/detail-item.md`
- `docs/contracts/components/detail-shell.md`
- `docs/contracts/components/filter-toolbar.md`
- `docs/contracts/components/empty-state.md`
- `docs/contracts/workstation/app-header.md`
- `docs/contracts/workstation/project-header.md`
- `docs/contracts/workstation/panel-header.md`
- `docs/contracts/workstation/panel-tabs.md`
- `docs/contracts/workstation/surface-tabs.md`
- `docs/contracts/workstation/dock-region.md`
- `docs/contracts/workstation/split-view.md`
- `docs/contracts/workstation/workspace-shell.md`
- `docs/contracts/workstation/command-palette-shell.md`
