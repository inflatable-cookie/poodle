# 013 Picker, Relation, And Selection Workflow Rules

Status: active
Updated: 2026-03-11
Depends on: `009-form-shell-validation-and-action-row-rules.md`, `011-browse-shell-filter-search-and-loading-rules.md`, `012-detail-display-card-header-and-navigation-rules.md`

## Purpose

Freeze the first workflow rules for pickers and relation selection so inline, popover-style, and modal-style selection tasks share one documented meaning.

## Workflow Shell Rule

Picker workflows need explicit framing.

At minimum they must expose:

- local title and supporting context
- query entry or filter controls when search exists
- visible candidate set
- visible selected-summary state
- explicit confirm/cancel posture when the workflow is commit-based

## Selection Mode Rule

`single` and `multiple` selection are different workflow meanings.

They may share one shell.
They may not silently share the same keyboard, summary, and confirm semantics without documenting those differences.

## Variant Rule

Picker workflows may appear as:

- inline
- popover
- modal

Variant changes presentation and interruption level.
It does not change:

- candidate semantics
- selected-summary semantics
- or confirm/cancel meaning

## Search And Candidate Rule

Search is part of picker workflow framing when present.

The picker shell must keep search controls ahead of candidate results and make `empty` distinct from `no-results`.

Hosts still own remote search or filtering logic.

## Selection Summary Rule

Selected items must remain visible and removable through a dedicated summary surface.

Clear-selection is distinct from:

- canceling the workflow
- confirming the workflow
- removing one selected item

## Accessibility Rule

Both runtimes must preserve:

- picker title and local context
- searchable candidate browsing
- explicit single vs multiple selection meaning
- selected-summary visibility
- confirm/cancel reachability where the workflow uses them

Svelte should use native input, button, and list semantics first.
GPUI must recreate equivalent workflow structure and state in the native accessibility tree.

## Seed Evidence

- `docs/contracts/composites/picker-shell.md`
- `docs/contracts/composites/relation-picker.md`
- `docs/contracts/composites/selection-summary.md`
- `packages/svelte/composites/src/PickerShell.svelte`
- `packages/svelte/composites/src/RelationPicker.svelte`
- `packages/svelte/composites/src/SelectionSummary.svelte`
- `packages/svelte/preview/src/App.svelte`

## Next Task

Carry this picker workflow baseline into `g02.006` and later milestones so media, embeds, and richer asset relations reuse one selection-workflow posture.
