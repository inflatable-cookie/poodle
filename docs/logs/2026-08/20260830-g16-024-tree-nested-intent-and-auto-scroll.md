# g16.024 — Tree Nested Intent And Auto-Scroll

Status: complete — pending review
Date: 2026-08-30
PR: https://github.com/inflatable-cookie/poodle/pull/107
Card: `docs/roadmaps/g16/024-drag-drop-tree-nested-intent-and-auto-scroll.md`
Handoff: `docs/handoffs/20260830-213507-g16-024-tree-nested-autoscroll.md`
Governing refs: `docs/architecture/011-drag-and-drop-substrate.md`,
`docs/specs/069-dependable-drag-and-drop-substrate.md`,
`docs/contracts/components/tree.md`
Branch: `t3code/tree-nested-auto-scroll`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-662c3f10`

## Outcome

Svelte and React Tree consume the g16.022 web drag substrate. Each enabled row
is the drag source; the `treeitem` is the nested drop target so ancestor and
descendant can share a pointer. Geometry still comes from the row via
`resolveNestedDropPosition`. Eligibility rejects self and own-subtree targets
during hover and again at drop. HTML drag is gone.

The shared controller owns one auto-scroll frame loop. Nested overflow
containers pick the nearest owner that can still scroll in the requested
direction. The loop is demand-driven: leave and direction exhaustion drop the
queued frame; later pointer or layout movement can restart it. Cancel, drop,
and unmount still stop it.

Public `onReorder(from, to, position)` is unchanged. Selection, expansion,
rename, checkboxes, and Svelte virtualization remain. The twisty is marked
`data-poodle-no-drag` so expansion is not a row drag. Alt+↑/↓ resolves the
sibling with `treeSiblingReorderTarget` and calls `requestKeyboardDrop` over
the visible logical target catalogue. Space/Enter stay Tree
selection/activation. Virtual windows pin the active source so it cannot
unmount mid-session. Tabs, DockRegion, native runtimes, and the
parity-evidence ledger were not edited. Ledger remains 52 mounted / 122
missing.

## Review oracle

- Nested inner/outer scroll: controller unit test plus Chromium probe.
- Before/inside/after: paired Tree tests against branch row geometry.
- Overlapping ancestor/descendant: paired Tree tests; deepest treeitem wins.
- Disable/removal before release: paired Tree tests disable or unmount the
  live target before pointerup.
- Live Tree-state revalidation: paired tests rewrite `nodes` so `canDrop`
  fails at drop.
- Twisty: paired tests; pointerdown plus threshold movement does not reorder,
  click still expands.
- Auto-scroll leave/exhaustion/re-entry: controller tests; no queued frame
  after leave or exhaustion; edge re-entry restarts the loop.
- Active cancellation while scrolling: controller cancel plus browser Escape.
- Virtualization: pinned source stays mounted after a window jump.
- Terminal cleanup: `cleanupSession` stops the auto-scroll frame.
- Alt+↑/↓: paired Tree tests plus controller `requestKeyboardDrop` proofs for
  eligibility, logical-target authority, async disable, callbacks,
  announcements, and focus return.
- WebKit: probe labels synthetic touch as not native scroll proof; Chromium
  CDP proves native hold-versus-scroll.

## Evidence

- Framework-free geometry: `packages/core/test/drag-drop-geometry.test.ts`.
- Framework-free auto-scroll: `packages/core/test/drag-drop-auto-scroll.test.ts`.
- Controller nested auto-scroll, leave/exhaustion/re-entry, and
  `requestKeyboardDrop`: `test/headless-dom/drag-drop-controller.test.ts`.
- Svelte: `packages/svelte/components/test/Tree.test.ts`.
- React: `packages/react/components/test/Tree.test.tsx`.
- Chromium and WebKit: `effigy test:drag-drop-browser` (WebKit touch remains
  synthetic).
- Active-source search: no Tree `dragstart`, `dragover`, `drop`, `dragend`, or
  `DataTransfer`.
- Unchanged ledger: `effigy check:parity-evidence-ledger`.

## Continuation

`g16.025` waits for operator-authorized merge of PR #107. Tabs stays with
DockRegion in `g16.026`.
