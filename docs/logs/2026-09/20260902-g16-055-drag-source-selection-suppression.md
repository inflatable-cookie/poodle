# g16.055 — Drag Source Pointer-gesture Browser Suppression

Status: in-review
Date: 2026-09-02
Card: `docs/roadmaps/g16/055-drag-source-preactivation-selection-suppression.md`
Handoff: `docs/handoffs/20260902-002950-g16-055-drag-source-selection-suppression.md`
Governing refs: `docs/specs/069-dependable-drag-and-drop-substrate.md`,
`docs/contracts/components/tree.md`
Branch: `fix/g16-055-drag-source-selection-suppression`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-055-drag-source-selection`
Base: `origin/main` at `c1a527898e7425853359bd72b7113a8cf38b8d97`
Planning base `74f612d6b5dadb3b91bb62961d5396c64d7c1a95` is an ancestor.

## Outcome

The shared DOM controller now suppresses native text selection from the
accepted primary pointerdown, not from `activate()`. It writes `user-select`
and `-webkit-user-select: none` on the connected root and restores the exact
prior inline declarations on every exit. An activated pointerup or cancel arms
a one-shot capture listener that stops the browser compatibility click on that
source path; the guard expires on consume, the next press, timeout plus rAF,
disconnect, or destroy. Taps and pre-threshold releases still click. Keyboard
pickup is unchanged. Svelte and React inherit the controller. No Tree CSS,
public API, or Rust/GPUI change.

## Evidence

- Controller: suppression at pointerdown before threshold; empty/`text`/`all`
  restore; pointercancel, Escape, source loss, disconnect, destroy; interactive,
  no-drag, contenteditable, disabled, and secondary-button exclusions; touch
  candidate plus tolerance cancel; overlapping roots; activated compatibility
  click consumed for committed/rejected/failed/cancelled; async dropping click
  consumed; stale guard expires; tap and sub-threshold click still reach the
  source.
- Paired Svelte/React Tree: tap still selects once with no reorder; rename
  field is not suppressed and keeps `selectionStart`/`selectionEnd`; activated
  drag plus synthetic click does not select for convenience commit, authority
  reject/fail, no-intent cancel, or pending async commit; later click after
  expiry selects once.
- Chromium and WebKit: reorderable label drag has empty Selection before and
  after threshold and leaves `data-tree-select` empty; tap selects `beta`;
  non-reorderable drag produces a non-empty range.

## Oracle falsification

Recorded after the proof commit, then restored.

## Validation

- `bunx vitest run --project headless-dom test/headless-dom/drag-drop-controller.test.ts` — 70 pass
- `bunx vitest run packages/svelte/components/test/Tree.test.ts packages/react/components/test/Tree.test.tsx` — 94 pass
- `effigy test:drag-drop-browser-chromium` — pass
- `effigy test:drag-drop-browser-webkit` — pass (requires `-webkit-user-select` on the same root)
- `effigy drift:drag-inventory` — pass
- `effigy docs:check` — pass
- `effigy ci:web` — pass
- `git diff --check` — clean

## Release / adoption

Merge to source first. Ship in the gated `0.3.0` candidate. Do not open a
`0.2.x` lane. Figmatic may verify against the merged source head; package
adoption waits for the compiled `0.3.0` distribution.
