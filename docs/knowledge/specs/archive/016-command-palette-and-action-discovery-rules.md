# 016 Command Palette And Action-Discovery Rules

Status: active
Updated: 2026-03-11
Depends on: `006-workstation-shell-and-panel-system-rules.md`, `../015-loading-empty-error-notification-and-remediation-rules.md`

## Purpose

Freeze the first deeper workstation rules for command palettes and inline action discovery so modal launchers and persistent discovery surfaces share one explicit meaning.

## Palette Depth Rule

The command palette is more than a shell once command discovery is in scope.

At minimum the richer surface must expose:

- query entry
- ordered command results
- grouped or scoped result structure when more than one command family is present
- explicit active-result posture for keyboard movement
- explicit result states for loading, no-results, empty, and error cases

## Ranking Rule

Ranking remains host-owned.

The shared surface may render ordered results and document expected stability.
It may not hide ranking behavior behind an opaque implementation-only contract that downstream consumers cannot reason about.

## Grouping Rule

Grouped command sections should remain legible.

They may represent:

- navigation
- workspace actions
- asset actions
- recent actions
- or other host-owned command families

Group headings and action identity must survive filtering and keyboard movement.

## Discovery Surface Rule

Inline discovery and modal command launch are complementary, not interchangeable.

- the modal palette is optimized for direct query and broad recall
- inline discovery is optimized for visible suggestion, recency, and scoped rediscovery

Hosts may use either surface.
They should not assume the palette is the only accessible discovery path for important workstation actions.

## Keyboard Rule

Both runtimes must preserve:

- shell invocation shortcut support where the host chooses to expose it
- arrow movement across active results
- command commit from keyboard selection
- `Escape` dismissal with deterministic focus restoration

Pointer-only discovery is not sufficient.

## Result-State Rule

Command discovery must reuse the broader hardening posture.

Loading, no-results, empty, and error states must remain:

- textual
- visually distinct
- and actionable where recovery is possible

## Accessibility Rule

Both runtimes must preserve:

- named launcher surface semantics
- query-to-results relationship
- grouped result labeling
- active-result exposure
- and direct action reachability from inline discovery panels

Svelte should use native text, button, and dialog semantics first.
GPUI must recreate equivalent command-discovery meaning in the native accessibility tree and keyboard system.

## Seed Evidence

- `docs/contracts/workstation/command-palette-shell.md`
- `docs/contracts/workstation/command-palette.md`
- `docs/contracts/workstation/action-discovery-panel.md`
- `packages/svelte/workstation/src/CommandPalette.svelte`
- `packages/svelte/workstation/src/ActionDiscoveryPanel.svelte`
- `packages/svelte/preview/src/App.svelte`
