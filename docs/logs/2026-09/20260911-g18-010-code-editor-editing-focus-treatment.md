# g18.010 — CodeEditor editing focus treatment

Status: ready for review
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
  entry reads `pointer` because `pointerdown` precedes focus), disarms on
  keydown/beforeinput editing intent (typing, named edit keys CodeMirror
  keymaps handle, chords Ctrl/Cmd+X/V/Z/Y, IME `Dead`/`Process`/composing,
  clipboard/history `beforeinput` input types), resets on focusout to a
  target outside the component (search-panel round trips keep the state), and
  never writes the document modality.
- Shared CSS `packages/core/src/styles/code-editor.css`: the ring rule moved
  from
  `:root[data-poodle-input-modality="keyboard"] .poodle-code-editor:focus-within`
  to `.poodle-code-editor[data-focus-entry="keyboard"]:focus-within`. The
  document attribute no longer keys this component's ring.
- Identical wiring in both engines (`packages/svelte/components/src/
  code-editor-engine.ts`, `packages/react/components/src/code-editor-engine.ts`):
  install on engine creation against the viewport host, dispose in
  `destroy()`. Engines remain byte-identical except their header comment.
- Component tests in both wrappers
  (`packages/svelte/components/test/CodeEditor.test.ts`,
  `packages/react/components/test/CodeEditor.test.tsx`): entry arming,
  pointer non-arming, navigation preservation, first-intent dismissal with
  document modality still `keyboard`, clipboard/history/composition routes,
  search-panel round trip, engine-destroy cleanup.
- Paired browser fixture `test/code-editor-focus-entry/` (Svelte + React
  public components, real focus and typing in Chromium and WebKit) with
  effigy selectors `test:code-editor-focus-entry{-chromium,-webkit}`. The
  probe proves, per engine: pointer entry paints no outline, typing after
  pointer entry never paints one while the document modality flips to
  `keyboard`, Tab entry arms the treatment, Arrow/End preserve it, the first
  edit dismisses it with modality still `keyboard` and the caret still
  visible, exit resets, and keyboard re-entry restores it.

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
  361 files, 2947 pass, 7 skipped.
- `bunx vitest run --project a11y` — 182 pass.
- `effigy test:code-editor-focus-entry` — all paired Chromium and WebKit
  checks passed (both frameworks, all invariants above).
- `effigy core:build` (declaration emit for the new core export), `effigy
  svelte:build`, `effigy react:build` — clean.
- `effigy check:svelte-components` — 0 errors (4 pre-existing warnings).
  `effigy check:react-components` — 12 errors, all pre-existing unrelated
  BlockEditorBlock `ref` typing (verified identical count on a clean stash).
- `bun packages/svelte/preview/scripts/focus-ring-drift.ts` — OK (treatment
  remains an outline on the ring element).
- `effigy docs:check` — pass.
- `git diff --check` — clean.
