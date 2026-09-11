# 016 — CodeEditor live line-number reconfiguration

Status: ready — operator-confirmed paired CodeEditor repair
Owner: Poodle web editors
Created: 2026-09-11
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/code-editor.md`,
`../../../packages/svelte/components/src/code-editor-engine.ts`,
`../../../packages/react/components/src/code-editor-engine.ts`,
`../../../packages/svelte/preview/src/specimens/CodeEditorSpecimen.svelte`,
`../../../packages/react/preview/src/gallery/specimens/CodeEditorSpecimen.tsx`
Depends on: none

## Outcome

Make the public `CodeEditor.lineNumbers` prop live in both web wrappers. A host
change from true to false must remove the logical-line gutter from the mounted
CodeMirror editor; changing it back must restore the gutter without remounting
or losing editor state.

This repairs an existing public contract. Do not redesign the configuration API
or bundle it with the later extensible-language work.

## Ready-State Rubric

- [x] The contract defines `lineNumbers` as a boolean configuration prop with a
  true default.
- [x] Both specimen controls update their framework state and pass the live
  value to `CodeEditor`.
- [x] Both engines conditionally add `lineNumbers()` only during initial
  `EditorState` construction.
- [x] Neither engine owns a line-number `Compartment` or reconfigures the
  extension in `update()`.
- [x] A fresh mount with `lineNumbers={false}` is already covered; the missing
  proof is mounted true→false→true reconfiguration.
- [x] The operator reproduced the inert toggle and approved the repair.

## Decisions

- Add one dedicated CodeMirror `Compartment` for line-number presentation in
  each framework engine. Configure it at mount and reconfigure it only when the
  applied `lineNumbers` value changes.
- Preserve the existing public prop, default and gutter semantics. No alias,
  compatibility shim or second configuration surface.
- Toggle the extension on the existing `EditorView`. Remounting the editor is
  not acceptable because it can discard selection, focus, history, search and
  diagnostics state.
- Bind the contract through real mounted behavior in both public wrappers.
  Source/TOML assertions or specimen-label changes alone do not prove the fix.
- Exercise true→false→true and false→true→false, including rapid controlled
  updates. The final applied prop must win.
- Prove value, selection, focus, undo history and active diagnostic behavior
  remain stable across the presentation-only reconfiguration.
- Keep the Svelte and React engine implementations behaviorally identical
  apart from their existing framework header.

## Dispatch manifest

- **State:** ready for immediate Queue dispatch in parallel with g18.013 and
  g18.015; serial before held g18.011 and retained g18.006; g18.009 remains held
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** paired CodeEditor engines and focused component,
  specimen and browser tests; the CodeEditor contract only if clarification is
  required; one g18.016 execution log
- **Reserved closeout surfaces:** public prop/type expansion; language registry
  work; unrelated components; g18 README/index/dispatch; g18.006/g18.009 and
  g18.011–g18.015 task/workspace/PR state; package versions/lockfile; workflows;
  release/tag/publication; Desktop; native/GPUI/Jetstream
- **Worker:** web-editor worker comfortable with CodeMirror compartments,
  controlled framework props and browser interaction proof
- **Excluded:** new configuration APIs; editor remounting; syntax-language
  registry work; visual redesign; release work; workflow changes; Desktop
- **Escalation:** Chatterbox for a required public API change, inability to
  preserve mounted editor state, conflict with g18.012 language architecture,
  or any release/workflow mutation

## Work

1. Add a failing paired regression that mounts line numbers on, changes the
   live prop off, and proves the logical-line gutter disappears without a new
   editor instance. Toggle on again and prove exact restoration.
2. Add the inverse initial-false path and a rapid controlled-update case so a
   stale asynchronous update cannot overwrite the latest host choice.
3. Introduce a line-number compartment in both engines and reconfigure it from
   the existing `update()` change-detection path.
4. Prove document value, selection, focus and undo history survive both
   transitions. Preserve an active diagnostic and its navigation/message state
   across the toggle.
5. Drive the Configuration toggle in both real specimen pages and assert the
   button state and visible gutter agree after each click.
6. Run focused paired component/browser tests, both package and preview builds,
   relevant accessibility checks, Effigy docs QA, and `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Live off removes numbers | button says off while `.cm-lineNumbers` remains | paired mounted click and gutter-absence assertion |
| Live on restores numbers | extension removal is one-way | same view restores exact logical-line labels |
| No remount | toggle recreates `EditorView` and loses state | stable view/content identity plus value, selection and focus proof |
| History survives | presentation toggle clears undo | edit, toggle twice, undo restores prior document |
| Diagnostics survive | gutter change resets active diagnostic/message | active diagnostic and F8 navigation remain coherent |
| Latest host value wins | rapid off/on resolves to stale off | controlled rapid-update final-state assertion |
| Initial false remains valid | compartment accidentally defaults on | false mount has no line-number gutter, then toggles live |
| Frameworks match | only one engine handles updates | identical Svelte/React browser journeys |
| Scope stays bounded | fix changes public types or language loading | exact existing boolean contract and focused engine diff |
| Sweep remains gated | g18.011 starts with the known inert control | merged repair before Queue hold release |

## Stop conditions

- Stop if a live toggle cannot preserve the existing mounted editor state.
- Stop if the repair requires a public API, package dependency, workflow or
  release mutation.
- Stop before folding in g18.012 syntax-language registry work or unrelated
  specimen presentation changes.

## Evidence

Inspection on 2026-09-11 confirmed both specimens update and pass their
`lineNumbers` state correctly. Both engine copies call `lineNumbers()` only in
the initial extension array and omit the prop from every update reconfiguration
branch. This makes the toggle label truthful while the mounted gutter remains
unchanged.

## Next task

After g18.013, g18.014, g18.015 and this repair merge, Chatterbox releases held
g18.011 for the full three-surface acceptance sweep. Keep g18.006 blocked and
g18.009 held.
