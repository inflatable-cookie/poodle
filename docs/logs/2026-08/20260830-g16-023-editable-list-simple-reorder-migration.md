# g16.023 — EditableList Simple Reorder Migration

Status: complete — PR pending
Date: 2026-08-30
PR: pending
Card: `docs/roadmaps/g16/023-drag-drop-simple-reorder-migrations.md`
Handoff: `docs/handoffs/20260830-182242-g16-023-editable-list-reorder.md`
Governing refs: `docs/architecture/011-drag-and-drop-substrate.md`,
`docs/specs/069-dependable-drag-and-drop-substrate.md`,
`docs/contracts/components/editable-list.md`
Branch: `t3code/editable-list-reorder`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-c5d3d67c`

## Outcome

Svelte and React EditableList now consume the g16.022 web drag substrate.
Each enabled reorderable row registers as one source/target pair. Pointer,
touch-like pointer, and keyboard commit through one `applyReorder` result
path and still report the complete next item order. HTML `dragstart` /
`dragover` / `drop` / `dragend` / `DataTransfer` and the local
`draggingIndex` / `dropTargetIndex` / `grabbedIndex` session are gone.

The public component API is unchanged. Tabs, DockRegion, Tree, native
runtimes, and the parity-evidence ledger were not edited. Ledger remains
52 mounted / 122 missing.

## Substrate repairs the migration proved

Two reusable controller defects showed up on per-row list targets:

- Keyboard arrows now start from the source's spatial neighbour rather than
  wrapping to the first/last eligible target.
- Pointer activation ignores interactive descendants (buttons, inputs,
  links) unless that descendant is the registered handle.

## Behaviour that is now true

- A dedicated handle is the pointer sensor when present. Embedded-handle
  rows use the whole row; remove buttons and other interactive descendants
  do not start a drag.
- Disabled and non-reorderable rows register as disabled and stay inert.
- Keyboard Space/Enter picks up, arrows move drop intent, Space/Enter
  drops, Escape cancels. Idle arrows move focus. Focus returns to the
  moved row.
- `onReorder` / `onChange` still receive the complete next array.
- Curated specimens are unchanged.

## Evidence

- Framework-free: `test/headless-dom/drag-drop-controller.test.ts` (30),
  including relative keyboard intent and interactive-descendant skip.
- Svelte: `packages/svelte/components/test/EditableList.test.ts` (16).
- React: `packages/react/components/test/EditableList.test.tsx` (16).
- Custom-surface preservation: Svelte and React `DragDropProvider` tests.
- Chromium and WebKit: `effigy test:drag-drop-browser`.
- Active-source search: no EditableList `dragstart`, `dragover`, `drop`,
  `dragend`, or `DataTransfer`.
- `effigy docs:callback-drift`, `docs:capability-drift`, `docs:spec-drift`.
- `effigy ci:web`.
- `effigy docs:check`, `effigy check:parity-evidence-ledger`, `effigy qa`,
  and `git diff --check origin/main...HEAD` recorded at closeout.

## Continuation

After operator-authorized merge, promote `g16.024` for Tree nested intent
and auto-scroll. Tabs stays with DockRegion in `g16.026`.
