# g02.005 Picker, Relation, And Selection Workflow Suite

Status: completed
Date: 2026-03-11
Owner: Flint Core

## Summary

- completed `g02.005`
- added reusable picker workflow composites at `packages/svelte/composites/src/PickerShell.svelte`, `packages/svelte/composites/src/RelationPicker.svelte`, and `packages/svelte/composites/src/SelectionSummary.svelte`
- extended the preview with host-controlled inline, popover-style, and modal-style relation selection flows, including query, selection mode, state posture, and confirm/cancel behavior
- added workflow contracts at `docs/contracts/composites/picker-shell.md`, `docs/contracts/composites/relation-picker.md`, and `docs/contracts/composites/selection-summary.md`
- added the normative picker baseline at `docs/specs/013-picker-relation-and-selection-workflow-rules.md`

## Validation

- `bun install`
- `bun run preview:build`
- `git diff --check`

## Notes

- this tranche intentionally lands workflow shells rather than pretending the full primitive select/radio surface is already implemented
- selection state, query execution, and commit policy remain host-owned

## Next Task

Open `docs/roadmaps/g02/006-media-preview-embed-and-asset-surface-suite.md` and build the next meaningful media and asset batch above the completed picker baseline.
