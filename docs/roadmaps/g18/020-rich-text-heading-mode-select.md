# 020 — RichTextEditor heading mode select

Status: complete — merged as `d8ad00b848acf62cc51357fc0caa6dbd559ff7c9` (PR #251) on 2026-09-11
Owner: Poodle web components
Created: 2026-09-11
Governing refs: `../../contracts/components/rich-text-editor.md`,
`../../../packages/core/src/rich-text.ts`,
`../../../packages/svelte/components/src/RichTextEditor.svelte`,
`../../../packages/react/components/src/RichTextEditor.tsx`,
`../../../packages/svelte/components/src/rich-text-engine.ts`,
`../../../packages/react/components/src/rich-text-engine.ts`
Depends on: g18.014 Queue task `5b42e240-5e63-4bdd-8035-7f2285776c6e` and
g18.018 Queue task `6bb46544-966a-4af0-af7c-e53846ad0407` complete and merged

## Outcome

Replace RichTextEditor's separate H1–H3 toolbar buttons with one compact Poodle
text-mode select. The public command configuration remains granular and grows
to H1–H6, so each consumer chooses exactly which heading levels its toolbar
offers. The selector adds Normal text as its intrinsic off state and shows
Mixed when one selection spans different block modes.

Extend the shared ProseMirror schema, editor engines, validation, serialization,
and RichTextRenderer projection so H4–H6 are real document behavior in both
Svelte and React, not toolbar-only labels.

## Ready-State Rubric

- [x] The operator rejected three fixed heading buttons because they omit
  H4–H6 and approved one text-mode dropdown with Normal text plus H1–H6.
- [x] The operator chose consumer-configurable heading levels rather than an
  all-levels composite command.
- [x] The current public command registry and both engines admit only H1–H3.
- [x] Poodle Select already supports custom trigger and option rendering.
- [x] g18.014 and g18.018 own overlapping specimen and engine paths; this task
  is dependency-queued behind both instead of creating conflicting workspaces.
- [x] g18.011 is already queued and can accept this new dependency in place.

## Decisions

- Extend `RichTextCommand` with `heading-4`, `heading-5`, and `heading-6`.
  Do not add a composite `text-mode` command or public `paragraph` command.
- Resolve all admitted heading commands into one selector at the first heading
  command's toolbar position. Its menu contains Normal text plus exactly those
  configured levels, including sparse subsets; non-heading command order stays
  unchanged.
- Keep the trigger height and typography stable. It displays Normal text for a
  paragraph/current non-heading text block, Heading N for one active level, and
  Mixed for a heterogeneous multi-block selection.
- Custom-render menu options with the corresponding document type scale, but
  retain plain accessible labels and bounded row geometry.
- Selecting Heading N sets that exact level even when it is already active.
  Selecting Normal text converts eligible selected blocks to paragraphs. One
  selection action creates at most one document-changing transaction.
- Preserve focus, selection, history, controlled echoes, read-only/disabled
  behavior, toolbar roving, and link/table affordances. The selector is one
  toolbar stop; an open listbox owns its keyboard events.
- Extend schema validation and semantic rendering through H6 in editor and
  RichTextRenderer. Unknown and out-of-range levels still fail closed.
- Keep TipTap/ProseMirror and Select internals private. No arbitrary toolbar
  slots or engine extension API enters this task.

## Dispatch manifest

- **State:** complete; merged g18.020 unblocks the heading-mode leg of g18.011;
  `g18.011` stays serial before retained g18.006; g18.009 waits on g18.006
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** RichTextEditor contract; shared rich-text command,
  schema and presentation registries; paired Svelte/React rich-text engines,
  editor toolbar composition and renderer path; rich-text styles; paired
  RichTextEditor/RichTextRenderer specimens and focused component/browser/
  accessibility tests; public docs generated from these surfaces; one g18.020
  execution log
- **Reserved closeout surfaces:** g18 README/index/dispatch and task state;
  g18.006/g18.009/g18.011/g18.012 task/workspace/PR state; CodeEditor,
  MarkdownEditor/Renderer and unrelated controls; versions, workflows,
  release/tag/publication, Desktop, native/GPUI/Jetstream
- **Worker:** web component UX/accessibility worker comfortable with Svelte,
  React, ProseMirror schemas, TipTap command semantics, Select/listbox keyboard
  interaction and controlled editors
- **Excluded:** arbitrary extensions or toolbar slots; H7+; Markdown conversion;
  image/embed work; toolbar redesign outside the heading cluster; native parity;
  release or consumer mutations
- **Escalation:** Chatterbox if configurable sparse levels cannot project as one
  Select, Normal/Mixed semantics conflict with ProseMirror selection behavior,
  or the change requires a breaking toolbar API

## Work

1. Plant shared registry/schema tests proving H4–H6 currently fail admission
   and an explicit sparse toolbar subset resolves to one heading selector with
   exactly those levels.
2. Extend the shared schema and command metadata through H6. Prove editor and
   renderer validate, serialize and render all six levels identically, while
   H7 and malformed levels fail closed.
3. Compose one custom Poodle Select in both toolbars. Preserve all non-heading
   ordering/grouping and render Normal, active Heading N, and Mixed states with
   a stable trigger and bounded type-scale option previews.
4. Bind exact setting semantics for a caret, same-level selection,
   heterogeneous multi-block selection, and Normal conversion. Assert one
   transaction/change and retained selection/focus/history.
5. Bind toolbar and listbox keyboard ownership: the trigger is one roving stop,
   opening it transfers arrows/Home/End/selection/Escape to Select, and closing
   restores the toolbar journey without duplicate execution.
6. Update paired RichTextEditor and RichTextRenderer specimens with H4–H6,
   configurable full and sparse selectors, Mixed state, themes, density and a
   constrained-width posture.
7. Run focused core/component/browser/accessibility checks, both package and
   preview builds, docs QA, and `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Levels are consumer-configurable | enabling headings always exposes all six despite an explicit subset | paired full and sparse toolbar projections with exact option sets |
| Selector replaces buttons | H1–H6 appear as six controls or the old H1–H3 buttons remain | one trigger/listbox and zero separate heading buttons in both wrappers |
| Normal and Mixed are truthful | paragraph shows a heading, or heterogeneous blocks claim one level | caret and multi-block selection state assertions |
| Selection sets exact mode | choosing the already-active level toggles it off | same-level selection remains that level after one action |
| H4–H6 are real schema behavior | toolbar labels exist but value validation or renderer rejects them | round-trip editor/renderer semantic proof for every level |
| Invalid levels stay closed | H7 or malformed heading attrs render as paragraphs | explicit validation refusal before partial mount |
| Keyboard ownership is coherent | toolbar arrows move focus behind an open menu or command fires twice | paired roving/listbox keyboard journey and event-count assertions |
| Controlled editing survives | selecting a mode moves the caret, resets history or echoes callbacks | selection/focus/history and exact one-change evidence after host echo |
| Layout remains compact | H1 option changes trigger height or menu forces page overflow | computed geometry across modes, density and constrained widths |
| Frameworks match | React and Svelte differ in options, state, order or semantics | shared metadata plus paired mounted/browser assertions |
| Release stays gated | g18.011 or candidate work begins before this repair merges | live Queue dependency plus merged exact-head proof before sweep |

## Stop conditions

- Stop if the repair requires replacing the granular public toolbar command
  configuration with a composite or exposing engine objects.
- Stop if Select cannot provide the required listbox keyboard ownership; return
  the smallest reusable Select/Toolbar primitive gap rather than bypassing it.
- Stop before image-policy, controlled-echo, release, Desktop, native or
  retained-task mutations outside the explicit integration tests.

## Evidence

Operator UX review on 2026-09-11 identified that the merged toolbar exposes
only H1–H3 as separate buttons. Source inspection confirmed the public command
domain and admitted heading levels stop at 3, while the existing custom Select
supports rendered trigger/options. The operator approved H1–H6 plus Normal text
and explicitly selected per-consumer configurable levels.

Merged outcome: PR #251 merged as `d8ad00b848acf62cc51357fc0caa6dbd559ff7c9`
after exact-head independent review (PR comment `5636831897`, `ready_to_merge`)
at `7e5162b22dc3e5a7811faa8ef2e98c7301ef6ca0` with green rust/web checks.
Review-reported validation: `test:core` 1376 pass; `test:components` 415 files,
4124 pass; focused paired suites 132 pass; `test:rich-text-heading-mode` 96
checks across headless Chromium and WebKit for both wrappers; paired package
and preview builds, `docs:lint`, and `git diff --check` clean. Deferred
failures: `check:react-*` errors are the documented pre-existing backlog in
untouched files; the browser probe stays out of `ci:web` by the g18-008
precedent.

## Next task

g18.020 is merged with g18.014, g18.018 and g18.012 already merged. g18.011
performs the full four-surface acceptance sweep next, followed by operator
sweep acceptance and retained g18.006; g18.009 stays dependency-queued behind
it.
