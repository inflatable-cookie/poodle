# g16.024 — Tree Nested Intent And Auto-Scroll

Status: complete — pending PR
Date: 2026-08-30
PR: pending
Card: `docs/roadmaps/g16/024-drag-drop-tree-nested-intent-and-auto-scroll.md`
Handoff: `docs/handoffs/20260830-213507-g16-024-tree-nested-autoscroll.md`
Governing refs: `docs/architecture/011-drag-and-drop-substrate.md`,
`docs/specs/069-dependable-drag-and-drop-substrate.md`,
`docs/contracts/components/tree.md`
Branch: `t3code/tree-nested-auto-scroll`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-662c3f10`

## Outcome

Svelte and React Tree consume the g16.022 web drag substrate. Each enabled row
registers as a DOM source/target on the row, not the treeitem, so Space/Enter
keep selection and activate. Pointer geometry maps to before/inside/after
through `resolveNestedDropPosition`. Eligibility rejects self and own-subtree
targets during hover and again at drop. HTML drag is gone.

The shared controller owns one auto-scroll frame loop. Nested overflow
containers pick the nearest owner that can still scroll in the requested
direction; leave, cancel, drop, and unmount stop it.

Public `onReorder(from, to, position)` is unchanged. Alt+↑/↓ sibling moves,
selection, expansion, rename, checkboxes, and Svelte virtualization remain.
Virtual windows pin the active source so it cannot unmount mid-session. Tabs,
DockRegion, native runtimes, and the parity-evidence ledger were not edited.
Ledger remains 52 mounted / 122 missing.

## Review oracle

- Nested inner/outer scroll: controller unit test plus Chromium/WebKit probe.
- Before/inside/after: paired Tree tests against branch row geometry.
- Disable/removal before release: subtree rejection and source-removed drop.
- Active cancellation while scrolling: controller cancel plus browser Escape.
- Virtualization: pinned source stays mounted after a window jump.
- Drop-time revalidation: controller already re-runs `canDrop` at drop; Tree
  `onDrop` also requires a live source id.
- Terminal cleanup: `cleanupSession` stops the auto-scroll frame.

## Evidence

- Framework-free geometry: `packages/core/test/drag-drop-geometry.test.ts`.
- Framework-free auto-scroll: `packages/core/test/drag-drop-auto-scroll.test.ts`.
- Controller nested auto-scroll: `test/headless-dom/drag-drop-controller.test.ts`.
- Svelte: `packages/svelte/components/test/Tree.test.ts`.
- React: `packages/react/components/test/Tree.test.tsx`.
- Chromium and WebKit: `effigy test:drag-drop-browser`.
- Active-source search: no Tree `dragstart`, `dragover`, `drop`, `dragend`, or
  `DataTransfer`.
- Unchanged ledger: `effigy check:parity-evidence-ledger`.

## Continuation

`g16.025` is next: shared Rust and GPUI drag substrate. Tabs stays with
DockRegion in `g16.026`.
