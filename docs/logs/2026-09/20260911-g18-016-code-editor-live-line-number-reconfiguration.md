# g18.016 — CodeEditor live line-number reconfiguration

Status: merged
Merge: `5932bd0027cab2cf86c07a7878c3b48193252306` (PR #244) on 2026-09-11
Date: 2026-09-11
Branch: `ns-d5d7913c-e6f2-458c-a1e0-8eb12abf56d8`
Card: `docs/roadmaps/g18/016-code-editor-live-line-number-reconfiguration.md`
Handoff: `docs/handoffs/20260911-g18-016-code-editor-live-line-number-reconfiguration.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/contracts/components/code-editor.md`,
`packages/svelte/components/src/code-editor-engine.ts`,
`packages/react/components/src/code-editor-engine.ts`
Base: `origin/main` at `daf73e816cef4113a2caea4b89e59fb12a7120b8`

## Outcome

`CodeEditor.lineNumbers` is now a live configuration prop in both web
wrappers. A host change from `true` to `false` removes the logical-line
gutter from the mounted CodeMirror editor; changing it back restores the
gutter. The existing `EditorView` is reconfigured in place through one
dedicated `Compartment`, so value, selection, focus, undo history, and
active diagnostics survive every transition. No public prop, default, or
gutter semantics changed; no remounting; no other component touched.

## What changed

- Both engines (`packages/svelte/components/src/code-editor-engine.ts`,
  `packages/react/components/src/code-editor-engine.ts`) gained a
  `lineNumbersCompartment` beside the existing language, behavior, read-only,
  diagnostics, wrap, and tab-size compartments. It is configured at mount
  (`options.lineNumbers ? lineNumbers() : []`) and reconfigured from the
  existing `update()` change-detection path only when the applied
  `lineNumbers` value actually changes, so unchanged resends reconfigure
  nothing. The engines remain byte-identical except their framework header.
- Component tests in both wrappers
  (`packages/svelte/components/test/CodeEditor.test.ts`,
  `packages/react/components/test/CodeEditor.test.tsx`) plant the review
  oracle at the public wrapper: live true→false→true removes and restores the
  gutter with stable `.cm-editor` identity and preserved value; an initial
  false mounts without a gutter and toggles live in both directions; rapid
  controlled updates resolve to the latest host value in both orders; value,
  caret position (End then Enter at the held offset), focus
  (`document.activeElement`), and undo history survive both transitions; an
  active diagnostic, its message, and F8 navigation survive the toggle. An
  engine-level case proves view identity, document, and gutter state directly
  through `engine.update({ lineNumbers })`.
- Real-specimen tests (`packages/svelte/preview/test/g18-016-code-editor-
  line-numbers.test.ts`, `packages/react/preview/test/g18-016-code-editor-
  line-numbers.test.tsx`) render the actual `CodeEditorSpecimen` pages and
  drive the Configuration `line-numbers-toggle` button, asserting after each
  click that `aria-pressed` and the visible `.cm-lineNumbers` gutter agree
  and that the mounted editor and its document persist.
- Paired browser fixture `test/code-editor-line-numbers/` (Svelte + React
  public components with the specimen's toggle control, real clicks, typing,
  and focus in Chromium and WebKit) with effigy selectors
  `test:code-editor-line-numbers{-chromium,-webkit}`. The probe proves, per
  engine: the initial mount shows the gutter with the button on; a real
  toggle click removes the gutter and flips `aria-pressed` with no new
  editor instance (a marker attribute on `.cm-editor` survives); a second
  click restores the gutter on the same view; Tab entry, caret position
  (End then Enter lands a newline at the held offset), focus, and Mod+Z undo
  survive both transitions; an active diagnostic, its decoration mark, and
  F8 navigation survive live off and live on; rapid off/on and
  on/off/on/off click sequences resolve to the latest host value. The
  fixture toggles through the real control; state-preservation steps
  dispatch the control's click without moving focus so the proof isolates
  the reconfiguration from the button's own focus behavior.

## Planted regression

With the engine change stashed, all 12 new component cases (6 per wrapper)
fail — the gutter stays mounted while the button reports off — and the
browser probe fails its live-off checks in Chromium. With the change
applied, all pass.

## Explicitly not done

- No public API, package dependency, workflow, or release mutation.
- No language-registry (g18.012) work, no specimen presentation redesign,
  no other component, no Desktop, no native editor.
- g18.011 remains Queue-held; g18.006 stays blocked; g18.009 stays held.

## Validation

- `bunx vitest run packages/svelte/components/test/CodeEditor.test.ts
  packages/react/components/test/CodeEditor.test.tsx` — 82 pass (12 new
  line-number cases per wrapper, engine-level plus wrapper-level).
- `bunx vitest run packages/svelte/preview/test
  packages/react/preview/test` — 19 files, 90 pass (4 new specimen cases).
- `bunx vitest run --project svelte-components --project react-components`
  — 361 files, 2965 pass, 7 skipped.
- `bunx vitest run --project a11y` — 182 pass.
- `effigy test:code-editor-line-numbers` — all paired Chromium and WebKit
  checks passed for both frameworks (14 checks per framework per engine).
- `effigy svelte:package`, `effigy react:package` — clean.
- `effigy svelte:build`, `effigy react:build` — clean (pre-existing chunk
  size warnings only).
- `effigy check:svelte-components` — 0 errors (4 pre-existing warnings).
  `effigy check:react-components` — 12 errors, all pre-existing in untouched
  files (same count as the g18.010 baseline).
- `effigy docs:check` — pass.
- `git diff --check` — clean.

## Closeout

- Merge performed by the plugin as
  `5932bd0027cab2cf86c07a7878c3b48193252306` on 2026-09-11 (PR #244),
  with parents `37adebc2d7efbd99546bbca06693bb0b93291c53` (main) and
  `f63523ee488ef4d0183e1525769d4686ea3f5263` (reviewed head).
- Accepted review: independent exact-head `ready_to_merge` approval of
  head `f63523ee488ef4d0183e1525769d4686ea3f5263` by betterthanclay
  ([comment #5632111084](https://github.com/inflatable-cookie/poodle/pull/244#issuecomment-5632111084)).
  No blocking findings; no changes required.
- Reviewed-head validation (reviewer ran at the exact head, tree left
  clean): both `CodeEditor` component suites 41 svelte + 41 react pass;
  specimen tests `g18-016-code-editor-line-numbers` 2 svelte + 2 react pass;
  `effigy test:code-editor-line-numbers` all Chromium and WebKit checks pass
  for both frameworks; `bunx vitest run --project a11y` 182 pass;
  `effigy docs:check` pass; `git diff --check` clean; CI `rust` and `web`
  pass at the merge.
- Worker validation at the branch head: `CodeEditor` suites 82 pass (12 new
  line-number cases per wrapper); preview suites 19 files, 90 pass (4 new
  specimen cases); full component projects 2965 pass, 7 skipped; package and
  preview build selectors clean (pre-existing chunk-size warnings only);
  `check:svelte-components` 0 errors (4 pre-existing warnings);
  `check:react-components` 12 errors, all pre-existing in untouched files
  (same count as the g18.010 baseline).
- Deferred: no release, tag, publish, Desktop, native, language-registry,
  workflow, or package-API work starts from this task. `g18.011` stays held
  until g18.013, g18.014, and g18.017 merge; `g18.006` stays blocked and
  `g18.009` held until the sweep, all other blocking repairs, and operator
  acceptance complete.

## Continuation

After g18.013, g18.014, and g18.017 merge, Chatterbox releases held g18.011.
