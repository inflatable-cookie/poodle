# g16.023 — EditableList Simple Reorder Migration

Status: complete — merged in PR #104
Date: 2026-08-30
PR: https://github.com/inflatable-cookie/poodle/pull/104
Merge commit: `42e534942b9505a6fba83bbf88f806986ff2d0b5`
Card: `docs/roadmaps/g16/023-drag-drop-simple-reorder-migrations.md`
Handoff: `docs/handoffs/20260830-182242-g16-023-editable-list-reorder.md`
Governing refs: `docs/architecture/011-drag-and-drop-substrate.md`,
`docs/specs/069-dependable-drag-and-drop-substrate.md`,
`docs/contracts/components/editable-list.md`
Branch: `t3code/editable-list-reorder`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-c5d3d67c`

## Outcome

Svelte and React EditableList consume the g16.022 web drag substrate. Each
enabled reorderable row registers as one DOM source/target pair. The complete
item order is also registered as element-free logical keyboard targets, so
windowed keyboard reorder can address hidden pages without paging or unmounting
the source. Pointer, touch, and keyboard commit through one `applyReorder`
path and still report the complete next item order. HTML drag is gone.

The public component API is unchanged. Tabs, DockRegion, Tree, native
runtimes, and the parity-evidence ledger were not edited. Ledger remains
52 mounted / 122 missing.

## Review revision

PR #104 review required four component findings, then substrate
blockers on `4eed71875` and `0fb68f0b0`. This head:

- freezes logical-vs-DOM keyboard authority at pickup, so a mid-session
  registry add or `acceptedKinds` change cannot switch drop/announcement;
- selects logical keyboard mode by a matching subject kind and announces
  by the active sensor;
- clears accepted and rejected snapshot state when a logical resolver
  returns `null`;
- proves logical disable-before-drop and async revalidation;
- adds `registerKeyboardTarget` / `keyboardOrder` and paired Svelte/React
  bindings;
- proves ordinary ArrowUp (`before`) and both `windowSize` boundaries;
- uses genuine `pointerType: "touch"` hold-to-reorder and pre-hold cancel;
- treats `contenteditable` without `"false"` as interactive and proves
  `embeddedHandle` editing/action descendants do not start a drag;
- rebases onto current `main`.

## Behaviour that is now true

- A dedicated handle is the pointer sensor when present. Embedded-handle rows
  use the whole row; buttons, inputs, links, and contenteditable descendants
  do not start a drag.
- Disabled and non-reorderable rows stay inert.
- Keyboard Space/Enter picks up, arrows move intent, Space/Enter drops,
  Escape cancels. Logical `previous`/`next` map to `before`/`after`.
- `onReorder` / `onChange` still receive the complete next array.

## Evidence

- Framework-free: `test/headless-dom/drag-drop-controller.test.ts` (43).
- Svelte: `packages/svelte/components/test/EditableList.test.ts` (21).
- React: `packages/react/components/test/EditableList.test.tsx` (21).
- Custom-surface preservation: Svelte and React `DragDropProvider` tests.
- Chromium and WebKit: `effigy test:drag-drop-browser`.
- Active-source search: no EditableList `dragstart`, `dragover`, `drop`,
  `dragend`, or `DataTransfer`.
- `effigy ci:web`, `effigy docs:check`, `effigy check:parity-evidence-ledger`,
  `effigy qa`, and `git diff --check origin/main...HEAD` recorded at closeout.

## Continuation

`g16.024` is promoted for Tree nested intent and auto-scroll. Tabs stays with
DockRegion in `g16.026`.
