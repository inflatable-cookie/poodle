# 011 — Web editor UX acceptance sweep

Status: blocked on F1 — g18.021 syntax-presentation repair awaiting operator-approved dispatch
Owner: Poodle web quality
Created: 2026-09-11
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/code-editor.md`,
`../../contracts/components/rich-text-editor.md`,
`../../architecture/003-component-docs-ia-and-implementation-substrates.md`,
`../../../packages/svelte/preview/`, `../../../packages/react/preview/`
Depends on: g18.012–g18.020 complete and merged; g18.021 repair pending

## Outcome

Perform a systematic pre-release UX and interaction sweep of `CodeEditor`,
`RichTextEditor`, `RichTextRenderer`, and `MarkdownRenderer` across their real
Svelte and React specimen pages. Produce reproducible evidence and a complete
severity-ranked finding set. Do not resume the 0.4.0 candidate until every
release-blocking finding is repaired and the operator accepts the sweep outcome.

This is an acceptance sweep, not permission to redesign APIs or silently fix
unbounded findings in place.

## Ready-State Rubric

- [x] All four public surfaces have matched or planned Svelte and React specimen
  pages; g18.019 owns the missing MarkdownRenderer pair.
- [x] The operator made the original three-component sweep a gate, then
  expanded it to four surfaces with MarkdownRenderer
  before g18.006 resumes.
- [x] g18.006 is blocked with a clean retained workspace and no candidate
  mutations; g18.009 is dependency-queued behind it without a manual hold.
- [x] The sweep dimensions, evidence, severity, and continuation checkpoint are
  explicit.
- [x] g18.010 focus treatment is merged and both previews are built from that
  exact source.
- [ ] g18.013 replaces the link-like RichTextEditor command row with accepted
  grouped Poodle controls in both previews.
- [ ] g18.014 makes the Image Policy specimen visibly prove seeded and inserted
  images without external networking.
- [ ] g18.015 makes both public preview selectors rebuild matching package
  distributions before Vite starts.
- [ ] g18.016 makes the mounted CodeEditor line-number gutter follow live host
  configuration in both previews.
- [ ] g18.017 fixes the block Slider shape and removes value-dependent external
  text movement across active runtimes.
- [ ] g18.018 makes accepted RichTextEditor controlled echoes preserve caret,
  selection, history and focus in both web wrappers.
- [ ] g18.019 adds paired standalone MarkdownRenderer specimens on the shared
  safe/trusted MarkdownEditor preview path.
- [ ] g18.020 replaces the fixed H1–H3 buttons with one configurable
  Normal/H1–H6 text-mode select and extends real schema/renderer support through
  H6.
- [ ] g18.012 replaces the closed language catalogue with consumer-selected
  lazy providers before this final sweep begins.

## Decisions

- Exercise the real public `./editor` and `./rich-text` entries through both
  preview applications. Unit-only, DOM-presence-only, or source inspection is
  not UX acceptance.
- Cover representative normal use, keyboard use, pointer use, controlled host
  updates, refusal states, themes, density, and constrained layout. Reuse
  focused tests for exhaustive data cases.
- Compare Svelte and React behavior directly. A defect in either wrapper blocks
  release when it affects the public contract or ordinary editor use.
- Record every observation as `blocking`, `follow-up`, or `accepted`. A green
  build cannot erase a visible or interaction finding.
- Blocking findings return to Chatterbox for bounded repair promotion. g18.006
  resumes only after those repairs merge and the operator accepts the final
  sweep capsule.
- Extensible syntax-language support is an accepted release requirement already
  owned by g18.012. Exercise its final public registry, lazy loading, refusal,
  language switching and package-cost claims; do not redesign the API inside
  the sweep.
- Heading levels remain granular toolbar configuration. Sweep both a full H1–H6
  selector and a sparse configured subset, including Normal and Mixed state;
  do not reinterpret g18.020 as an all-levels fixed control.

## Dispatch manifest

- **State:** dependency-queued as task
  `aad6b776-1c3e-438c-bc9c-4e8ba8750462` behind parallel g18.012 and active
  g18.014/g18.018→g18.020. The coordinator mutates those dependencies in place;
  this task remains serial before retained g18.006.
- **Completion:** one open non-draft evidence/test PR at a clean pushed head
  with exact-head independent review, or a blocked callback naming reproducible
  release-blocking findings; never merge product fixes from this sweep
- **Owned mutable paths:** focused browser fixtures/tests and test helpers for
  the three web editors; preview-only test instrumentation if required;
  reproducible screenshots/reports under the existing evidence convention;
  one g18.011 execution log
- **Reserved closeout surfaces:** component contracts and implementations;
  package manifests/API/distribution; g18 README, index and dispatch; g18.006,
  g18.009 and g18.010 task/worker/workspace/PR state; release files; Desktop
- **Worker:** high-reasoning web QA/interaction worker comfortable with Svelte,
  React, CodeMirror, TipTap/ProseMirror, accessibility, and visual inspection
- **Excluded:** product fixes; public API or language-module decisions;
  exhaustive combinatorial matrices; native/GPUI/Jetstream; versions, release,
  tag, publication, Desktop edits, workflow changes, windowed native selectors
- **Escalation:** Chatterbox for every blocking finding, public API decision,
  disagreement between wrappers/contracts, or ambiguous release disposition;
  operator decides sweep acceptance and g18.006 continuation

## Work

1. Build both previews from the exact accepted repair source. Bind direct
   routes for all four specimens and record viewport/theme/density fixtures.
2. Sweep CodeEditor: exact editing and host echo, syntax visibility and language
   switching, search, line numbers, diagnostics/navigation, read-only/disabled,
   clipboard/undo/redo/IME, Tab escape, corrected focus treatment, scrolling,
   wrapping, empty and large-document postures.
3. Sweep RichTextEditor: controlled updates, selection and toolbar state,
   configurable Normal/H1–H6 heading selection, Mixed state,
   marks/lists/code/quotes/rules, tables and table actions, links,
   images disabled/enabled, paste sanitization, keyboard/focus, read-only or
   disabled states defined by contract, scrolling and constrained layout.
4. Sweep RichTextRenderer against the same representative documents and
   feature configuration: no editable state, faithful typography/tables,
   safe links/images, invalid-document refusal, themes and constrained layout.
5. Sweep MarkdownRenderer against MarkdownEditor preview: safe and trusted
   policy, built-in and custom parsing, links/code/lists/quotes, empty output,
   SSR, themes, density and constrained layout. Prove no editor mechanics.
6. Compare Svelte and React side by side. Capture reproducible browser evidence
   for every discrepancy or visible defect; do not normalize differences in
   prose.
7. Verify g18.012 package loading for CodeMirror languages: distinguish
   initial-load, emitted-chunk, installed-dependency and active-loader cost.
   Exercise individually imported consumer-selected providers; do not change
   the API in this task.
8. Publish the severity-ranked capsule with exact routes, actions, expected and
   observed results. Run the focused browser board, both preview builds,
   relevant accessibility checks, docs QA, and `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| All four surfaces are genuinely swept | report omits either renderer or treats one as an editor mode | named checklist and evidence for four distinct pages in both frameworks |
| UX evidence is real | tests assert component presence but never interact or inspect rendering | browser actions, screenshots and computed/semantic assertions at representative states |
| Framework parity is honest | one wrapper fails but report averages results | paired result for every blocking scenario with discrepancies called out |
| Editing stays controlled | local editor changes diverge from host value or echo loops | exact before/action/callback/host-after evidence for both editors |
| Configuration is truthful | rich-text image or table behavior ignores selected feature policy | images-off/on and standard-table cases in both wrappers and renderer |
| Heading configuration is truthful | sparse configuration still exposes every level, or H4–H6 exist only as labels | full/sparse selector plus editor/renderer semantic proof in both wrappers |
| Read-only rendering is inert | RichTextRenderer mounts `contenteditable`, editor plugins, or mutation paths | DOM and interaction refusal proof |
| Accessibility survives ordinary use | keyboard cannot enter/leave, focus vanishes, toolbar state is unnamed | keyboard journey and accessibility assertions on all interactive surfaces |
| Layout is usable | editor expands the page, clips controls, or collapses at Desktop-like width | normal and constrained viewport evidence across themes/density |
| Language registry is accepted | sweep covers only the former closed catalogue or ignores refusal/switching | consumer-defined ids, lazy-loader counts, live switching and failure journeys |
| Language cost is measured honestly | “dynamic” is claimed from source while all grammars ship/install | before/after bundle graph, emitted chunks and installed dependency inventory |
| Findings cannot disappear | a red behavior is called non-blocking without owner or rationale | severity, evidence, owner and next action for every observation |
| Release remains gated | g18.006 resumes because the sweep PR exists despite open blockers | operator acceptance plus merged repairs before Queue continuation |

## Stop conditions

- Stop and report when a release-blocking defect is reproduced. Do not repair
  product code under the sweep task.
- Stop if representative browser evidence cannot run from the exact merged
  repair source.
- Stop before public API decisions, release/candidate mutations, Desktop edits,
  or resuming g18.006/g18.009.

## Evidence

Operator direction on 2026-09-11: hold g18.006 until the testing sweep across
all three new components is complete. The three surfaces are CodeEditor,
RichTextEditor, and RichTextRenderer; admission is paired Svelte and React.
The operator also rejected a permanently closed Poodle language catalogue:
language support must be extensible because Poodle cannot own every grammar.
Later on 2026-09-11, the operator added the paired MarkdownRenderer to the same
pre-release sweep and confirmed safe HTML by default with an explicit trusted
opt-in. The release gate therefore now covers four surfaces.

The operator then replaced the fixed H1–H3 toolbar posture with one text-mode
select and chose consumer-configurable heading levels. g18.020 owns that
schema/editor/renderer repair and is a new Queue prerequisite of this sweep.

The original graph put g18.012 after this sweep so the sweep could collect a
language-loading baseline. The operator corrected that order on 2026-09-11:
g18.012 owns its before/after baseline and this sweep accepts the completed
extensible registry.

The first sweep pass at `a639b1b78` stopped correctly on release-blocking F1.
Real TypeScript and JSON loaders produced zero syntax spans and one default
text colour in Svelte and React under Chromium and WebKit. Both CodeEditor
engines load the grammar extension but install no CodeMirror highlight style.
Chatterbox promoted bounded repair g18.021; this task and its accepted blocked
callback remain canonical and will resume after that repair merges. Follow-up
F12 records that React preview accepts `#components/code-editor` but not the
canonical `#/components/code-editor` spelling; it did not block the sweep.

## Next task

Dispatch and merge g18.021, add it to this existing Queue task in place, then
resume the interrupted sweep. Promote and merge every other blocking repair,
then seek the operator's explicit sweep acceptance before resuming retained
g18.006 task `17ac3fee-de90-4b32-9672-1134770bb086`. Keep g18.009
dependency-queued meanwhile.
