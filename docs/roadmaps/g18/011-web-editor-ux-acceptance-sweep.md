# 011 — Web editor UX acceptance sweep

Status: ready behind g18.010 — operator-required release gate
Owner: Poodle web quality
Created: 2026-09-11
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/code-editor.md`,
`../../contracts/components/rich-text-editor.md`,
`../../architecture/003-component-docs-ia-and-implementation-substrates.md`,
`../../../packages/svelte/preview/`, `../../../packages/react/preview/`
Depends on: g18.010 Queue task `d5ece513-5f28-4bfe-9132-12a70cf7a89f`
complete and merged

## Outcome

Perform a systematic pre-release UX and interaction sweep of `CodeEditor`,
`RichTextEditor`, and `RichTextRenderer` across their real Svelte and React
specimen pages. Produce reproducible evidence and a complete severity-ranked
finding set. Do not resume the 0.4.0 candidate until every release-blocking
finding is repaired and the operator accepts the sweep outcome.

This is an acceptance sweep, not permission to redesign APIs or silently fix
unbounded findings in place.

## Ready-State Rubric

- [x] All three public surfaces have matched Svelte and React specimen pages.
- [x] The operator explicitly made the completed three-component sweep a gate
  before g18.006 resumes.
- [x] g18.006 is blocked with a clean retained workspace and no candidate
  mutations; g18.009 is Queue-held.
- [x] The sweep dimensions, evidence, severity, and continuation checkpoint are
  explicit.
- [ ] g18.010 focus treatment is merged and both previews are built from that
  exact source.

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
- Extensible syntax-language support is an accepted release requirement.
  Measure the current closed switch and dependency footprint precisely so the
  bounded follow-up can replace it with consumer-selected language modules.
  Do not redesign the API inside the sweep.

## Dispatch manifest

- **State:** dependency-queue behind g18.010; serial before resuming retained
  g18.006; g18.009 remains held
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

1. Build both previews from the exact accepted g18.010 source. Bind direct
   routes for all three specimens and record viewport/theme/density fixtures.
2. Sweep CodeEditor: exact editing and host echo, syntax visibility and language
   switching, search, line numbers, diagnostics/navigation, read-only/disabled,
   clipboard/undo/redo/IME, Tab escape, corrected focus treatment, scrolling,
   wrapping, empty and large-document postures.
3. Sweep RichTextEditor: controlled updates, selection and toolbar state,
   headings/marks/lists/code/quotes/rules, tables and table actions, links,
   images disabled/enabled, paste sanitization, keyboard/focus, read-only or
   disabled states defined by contract, scrolling and constrained layout.
4. Sweep RichTextRenderer against the same representative documents and
   feature configuration: no editable state, faithful typography/tables,
   safe links/images, invalid-document refusal, themes and constrained layout.
5. Compare Svelte and React side by side. Capture reproducible browser evidence
   for every discrepancy or visible defect; do not normalize differences in
   prose.
6. Inspect package loading for CodeMirror languages: distinguish initial-load,
   emitted-chunk, and installed-dependency cost. Establish the reproducible
   baseline for individually imported, consumer-selected Poodle language
   modules; do not change the API in this task.
7. Publish the severity-ranked capsule with exact routes, actions, expected and
   observed results. Run the focused browser board, both preview builds,
   relevant accessibility checks, docs QA, and `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| All three surfaces are genuinely swept | report checks only CodeEditor or treats renderer as editor mode | named checklist and evidence for three distinct pages in both frameworks |
| UX evidence is real | tests assert component presence but never interact or inspect rendering | browser actions, screenshots and computed/semantic assertions at representative states |
| Framework parity is honest | one wrapper fails but report averages results | paired result for every blocking scenario with discrepancies called out |
| Editing stays controlled | local editor changes diverge from host value or echo loops | exact before/action/callback/host-after evidence for both editors |
| Configuration is truthful | rich-text image or table behavior ignores selected feature policy | images-off/on and standard-table cases in both wrappers and renderer |
| Read-only rendering is inert | RichTextRenderer mounts `contenteditable`, editor plugins, or mutation paths | DOM and interaction refusal proof |
| Accessibility survives ordinary use | keyboard cannot enter/leave, focus vanishes, toolbar state is unnamed | keyboard journey and accessibility assertions on all interactive surfaces |
| Layout is usable | editor expands the page, clips controls, or collapses at Desktop-like width | normal and constrained viewport evidence across themes/density |
| Language cost is measured honestly | “dynamic” is claimed from source while all grammars ship/install | bundle graph, emitted chunks and installed dependency inventory |
| Findings cannot disappear | a red behavior is called non-blocking without owner or rationale | severity, evidence, owner and next action for every observation |
| Release remains gated | g18.006 resumes because the sweep PR exists despite open blockers | operator acceptance plus merged repairs before Queue continuation |

## Stop conditions

- Stop and report when a release-blocking defect is reproduced. Do not repair
  product code under the sweep task.
- Stop if representative browser evidence cannot run from the exact merged
  g18.010 source.
- Stop before public API decisions, release/candidate mutations, Desktop edits,
  or resuming g18.006/g18.009.

## Evidence

Operator direction on 2026-09-11: hold g18.006 until the testing sweep across
all three new components is complete. The three surfaces are CodeEditor,
RichTextEditor, and RichTextRenderer; admission is paired Svelte and React.
The operator also rejected a permanently closed Poodle language catalogue:
language support must be extensible because Poodle cannot own every grammar.

## Next task

Return findings to Chatterbox. Promote and merge every blocking repair, then
seek the operator's explicit sweep acceptance before resuming retained g18.006
task `17ac3fee-de90-4b32-9672-1134770bb086`. Keep g18.009 held meanwhile.
