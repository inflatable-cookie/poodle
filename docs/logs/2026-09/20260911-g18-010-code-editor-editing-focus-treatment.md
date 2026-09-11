# g18.010 — CodeEditor editing focus treatment

Status: merged
Merge: `a71b48573c7253dfd45f35e482b9bbc7432ea0ca` (PR #242) on 2026-09-11
Date: 2026-09-11
Branch: `ns-d5ece513-5f28-4bfe-9132-12a70cf7a89f`
Card: `docs/roadmaps/g18/010-code-editor-editing-focus-treatment.md`
Handoff: `docs/handoffs/20260911-g18-010-code-editor-editing-focus-treatment.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/contracts/components/code-editor.md`,
`packages/core/src/dom/input-modality.ts`,
`packages/core/src/styles/code-editor.css`
Base: `origin/main` at `d24dc54f3e1985ac6a78cc3c88861e9ff73c822d`

## Outcome

CodeEditor's outer focus treatment is now a component-local keyboard-entry
affordance instead of a document-modality side effect. Tab/Shift+Tab entry
arms the shared outline; pointer entry never does; the first real editing
intent (insertion, deletion, composition, clipboard mutation, undo, redo)
dismisses it; navigation-only keys preserve it; focus leaving the component
resets it; later keyboard re-entry restores it. The document
`data-poodle-input-modality` attribute is never written by the editor: while
the local treatment is dismissed the document stays `keyboard`, so every other
control keeps its correct focus origin. No public prop, no global modality
redesign, no other component touched.

## What changed

- New bounded framework-free helper
  `packages/core/src/dom/code-editor-focus-entry.ts`, exported from the core
  barrel (`CODE_EDITOR_FOCUS_ENTRY_ATTR`, `installCodeEditorFocusEntry`). It
  arms `data-focus-entry="keyboard"` on the component root when focus enters
  the editing surface while the document modality reads `keyboard` (pointer
  entry reads `pointer` because `pointerdown` precedes focus). Dismissal is
  transaction-driven: the engine reports every committed user edit
  transaction (typing, deletion, line commands, indentation, clipboard,
  history, drop), so bindings that repurpose key names (indent-mode Tab,
  copy-line arrows) and readOnly context are honored by construction. IME
  composition dismisses on `compositionstart` at the editing surface only.
  A pointer press inside the editor yields the affordance. Focusout to a
  target outside the component resets the state (search-panel round trips
  keep it). The document modality is never written.
- Shared CSS `packages/core/src/styles/code-editor.css`: the ring rule moved
  from
  `:root[data-poodle-input-modality="keyboard"] .poodle-code-editor:focus-within`
  to
  `:root[data-poodle-input-modality="keyboard"]
  .poodle-code-editor[data-focus-entry="keyboard"]:focus-within`. The
  document attribute still gates the ring, so a pointer press can never
  leave it painted, but the local entry attribute decides arming and
  dismissal.
- Identical wiring in both engines (`packages/svelte/components/src/
  code-editor-engine.ts`, `packages/react/components/src/code-editor-engine.ts`):
  install on engine creation against the viewport host, dismiss from the
  committed-edit update listener, dispose in `destroy()`. Engines remain
  byte-identical except their header comment.
- Component tests in both wrappers
  (`packages/svelte/components/test/CodeEditor.test.ts`,
  `packages/react/components/test/CodeEditor.test.tsx`): entry arming,
  pointer non-arming, navigation preservation, first-intent dismissal with
  document modality still `keyboard`, planted review oracle cases
  (Shift+Alt+ArrowDown copyLineDown and indent-mode Tab dismiss; copy and
  select-all chords and find-panel traversal preserve; pointer press inside
  the armed editor yields), clipboard/history/composition routes, host-value
  non-dismissal, search-panel round trip, engine-destroy cleanup.
- Paired browser fixture `test/code-editor-focus-entry/` (Svelte + React
  public components, real focus and typing in Chromium and WebKit) with
  effigy selectors `test:code-editor-focus-entry{-chromium,-webkit}`. The
  probe proves, per engine: pointer entry paints no outline, typing after
  pointer entry never paints one while the document modality flips to
  `keyboard`, Tab entry arms the treatment, Arrow/End preserve it, copy and
  select-all chords preserve it, find-panel Mod+F/Enter traversal preserves
  it, the first edit dismisses it with modality still `keyboard` and the
  caret still visible, Shift+Alt+ArrowDown copyLineDown duplicates the line
  and dismisses, exit resets, keyboard re-entry restores, and a pointer
  press inside the armed editor clears it.

## Review round (2026-09-11, head c638b2c3a)

Independent review verified the primary behavior and required three changes,
all rooted in the original keydown-name inference of editing intent:

1. Real editing mutations kept the ring: Shift+Alt+ArrowDown (copyLineDown)
   and indent-mode Tab (indentMore) changed the document without dismissal
   because `NON_EDITING_KEYS` was binding- and modifier-blind.
2. Find-panel search traversal (Mod+F then Enter) dismissed the treatment
   although the document was untouched — the planted "search traversal"
   navigation case.
3. Non-edit chords Ctrl/Cmd+C (copy) and Ctrl/Cmd+A (select all) dismissed
   because `key.length === 1` ignored the modifier context.
4. Minor: a pointer press inside an armed editor left the ring painted under
   pointer modality.

Fix: dismissal moved out of key/beforeinput name inference entirely. The
helper now only arms (keyboard-modality focus entry), yields on pointerdown
inside the editor, resets on focusout, and handles IME `compositionstart` at
the editing surface; the engine dismisses on committed user edit
transactions in the update listener, which already carries binding and
readOnly context. The CSS rule re-adds the document-modality gate so a
pointer press can never leave the ring painted. Planted oracle cases cover
each review finding in both component suites and the paired probe.

## Explicitly not done

- No global input-modality semantics change; the tracker file is untouched.
- No CodeEditor public prop, no specimen-only shortcut, no other component's
  focus CSS.
- g18.006/g18.009 workspaces, release lanes, versions, Desktop: untouched.

## Sequencing update (2026-09-11, operator)

g18.006 must not resume after g18.010. Main records g18.011 Queue task
`aad6b776-1c3e-438c-bc9c-4e8ba8750462` followed by g18.012 task
`697c0380-bcc4-4433-9bce-a6c77fa0452a`. g18.006 stays blocked and g18.009
held until the three-surface sweep, the extensible-language repair, all other
blocking repairs, and operator acceptance complete.

## Validation

- `bun test packages/core/test/code-editor.test.ts` — 12 pass.
- `effigy test:core` — 1301 pass, 0 fail.
- `bunx vitest run --project svelte-components --project react-components` —
  2960 pass, 0 fail (35 focus-entry cases per wrapper after the review round).
- `bunx vitest run --project a11y` — 182 pass.
- `effigy test:code-editor-focus-entry` — all paired Chromium and WebKit
  checks passed (both frameworks, all invariants above, including the planted
  review oracle cases).
- `effigy core:build` (declaration emit for the new core export), `effigy
  svelte:build`, `effigy react:build` — clean.
- `effigy check:svelte-components` — 0 errors (4 pre-existing warnings).
  `effigy check:react-components` — 12 errors, all pre-existing unrelated
  BlockEditorBlock `ref` typing (verified identical count on a clean stash).
- `bun packages/svelte/preview/scripts/focus-ring-drift.ts` — OK (treatment
  remains an outline on the ring element).
- `effigy docs:check` — pass.
- `git diff --check` — clean.

## Closeout

- Merge performed by the plugin as
  `a71b48573c7253dfd45f35e482b9bbc7432ea0ca` on 2026-09-11 (PR #242),
  with parents `9736952d2` (main) and `5a0c7ce9f` (reviewed head).
- Accepted review: independent exact-head `ready_to_merge` approval of
  head `5a0c7ce9f3037376bb6f2e7d712b71ef6927ef89` by betterthanclay
  ([comment #5631470927](https://github.com/inflatable-cookie/poodle/pull/242#issuecomment-5631470927)).
  Round 1 (`changes_required` at `c638b2c3a`,
  [comment #5631113407](https://github.com/inflatable-cookie/poodle/pull/242#issuecomment-5631113407))
  named three blockers — copyLineDown/indent-Tab mutations keeping the
  ring, find-panel traversal dismissing it, copy/select-all chords
  dismissing it — plus a stale pointer-press ring; round 2 re-ran the
  same adversarial scenarios at the exact head and confirmed all fixed.
  No merge blockers remained.
- Reviewed-head validation (reviewer ran at the exact head, tree left
  clean): `effigy test:core` 1301 pass; `bunx vitest run --project
  svelte-components --project react-components` 361 files / 2960 pass
  (35 focus-entry cases per wrapper); paired
  `test:code-editor-focus-entry` probe pass in Chromium and WebKit for
  both frameworks including the planted oracle cases;
  `check:svelte-components` 0 errors (4 pre-existing warnings);
  `check:react-components` 12 errors, all pre-existing in untouched
  files; `git diff --check` clean; CI `web` and `rust` pass at the head.
- Non-blocking reviewer notes (deferred, no acceptance impact): the PR
  description still describes the superseded keydown/`beforeinput`
  mechanism and stale pass counts; one log wording nit on the
  pre-existing error span; a TDZ hazard note on the update-listener
  install order; keyboard focus moving into the find panel re-arms a
  dismissed treatment, consistent with the oracle.
- Deferred: no release, tag, publish, Desktop, or native editor work
  starts from this task. `g18.011` stays held until parallel g18.013
  and its serial g18.014 merge; `g18.006` stays blocked and `g18.009`
  held until the sweep, all other blocking repairs, and operator
  acceptance complete.
