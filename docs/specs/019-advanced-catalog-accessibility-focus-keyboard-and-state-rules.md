# 019 Advanced Catalog Accessibility Focus Keyboard And State Rules

Status: active
Updated: 2026-03-11
Depends on: `003-accessibility-and-assistive-technology-baseline.md`, `009-form-shell-validation-and-action-row-rules.md`, `010-data-table-selection-bulk-action-and-virtualization-rules.md`, `016-command-palette-and-action-discovery-rules.md`, `018-dock-split-view-tabs-and-persistence-orchestration-rules.md`

## Purpose

Freeze the next hardening rules across the advanced Flint catalogue so richer
composites and workstation shells share one explicit posture for focus,
keyboard movement, and dynamic state semantics.

## Focus Containment Rule

Modal workstation surfaces such as command palettes must preserve:

- deterministic initial focus
- focus containment while open
- deterministic focus restoration when closed

This applies in both runtimes.
Svelte may implement containment with DOM focus management.
GPUI must provide equivalent modal focus scope through native focus handling.

## Boundary Navigation Rule

Where a component owns ordered results or candidates, keyboard boundaries must
be explicit.

At minimum that means using the appropriate subset of:

- Arrow navigation for adjacent results
- `Home` to move to the first item
- `End` to move to the last item
- `Enter` or `Space` to activate or toggle the current item where the pattern
  requires it

Host code should not have to invent those rules per surface.

## State Exposure Rule

Advanced components must expose dynamic state as text, not only visuals.

That includes:

- current sort direction
- selection count
- active command or candidate movement
- loading, error, empty, and no-results states
- toast or banner severity

Where the state changes materially while focus remains elsewhere, the Svelte
surface should expose an appropriate live-region summary.
GPUI must use native announcement mechanisms or equivalent accessible events.

## Semantic Honesty Rule

Components must not claim richer accessibility roles than their actual
interaction model supports.

Examples:

- a multi-select picker built from checkboxes should not pretend to be a
  listbox without listbox semantics
- a structured data table should expose native row and sort meaning instead of
  visually styled divs
- discovery panels should remain addressable navigation/group surfaces rather
  than anonymous button piles

Semantic clarity is more important than aspirational role usage.

## Table Hardening Rule

Data-table surfaces must preserve:

- native table semantics
- explicit `aria-sort` or equivalent sort-state exposure
- row-header meaning for the primary identifying column
- row-action naming tied to row context
- visible-scope selection naming for select-all behavior

## Picker Hardening Rule

Picker workflows must preserve:

- search before results in focus order
- textual status for result and selection counts
- keyboard movement across candidates
- explicit difference between single-choice and multi-choice behavior
- reachable confirm and cancel actions after candidate browsing

## Notification Rule

Transient notification surfaces must preserve severity and announcement posture.

- informational and success notifications may be polite
- danger-level failures may announce assertively when they materially affect the workflow
- toast actions and dismissal must remain independently reachable

## Framework Delta Rule

Framework differences are acceptable only when the semantic outcome remains
equivalent.

Examples of acceptable delta:

- DOM focus-trap utilities on Svelte versus native modal focus scope in GPUI
- web live regions versus platform-native announcements

Examples of unacceptable delta:

- dropping boundary keyboard movement on GPUI
- omitting state announcements in one runtime
- flattening table or picker meaning into anonymous panels

## Seed Evidence

- `docs/contracts/workstation/command-palette.md`
- `docs/contracts/workstation/action-discovery-panel.md`
- `docs/contracts/composites/data-table.md`
- `docs/contracts/composites/picker-shell.md`
- `docs/contracts/composites/relation-picker.md`
- `docs/contracts/composites/toast-stack.md`
- `packages/svelte/workstation/src/CommandPalette.svelte`
- `packages/svelte/workstation/src/ActionDiscoveryPanel.svelte`
- `packages/svelte/composites/src/DataTable.svelte`
- `packages/svelte/composites/src/PickerShell.svelte`
- `packages/svelte/composites/src/RelationPicker.svelte`
- `packages/svelte/composites/src/ToastStack.svelte`

## Next Task

Carry this hardening baseline into `g02.012` so examples and docs-site work do
not regress the now-explicit accessibility, focus, keyboard, and state rules.
