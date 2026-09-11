# 021 — CodeEditor token-bound syntax presentation

Status: ready for operator-approved dispatch — release-blocking g18.011 repair
Owner: Poodle web components
Created: 2026-09-11
Governing refs: `../../contracts/components/code-editor.md`,
`../../../packages/svelte/components/src/code-editor-engine.ts`,
`../../../packages/react/components/src/code-editor-engine.ts`,
`../../../test/code-editor-language-registry/`
Depends on: g18.012 complete and merged

## Outcome

Make `CodeEditor` syntax visibly distinct in `performanceMode="full"` by
installing one private CodeMirror highlight style mapped to Poodle semantic
colour tokens in both web distributions. Keep `performanceMode="plain"`
unhighlighted and keep language grammars consumer-selected through the merged
g18.012 registry.

This is the bounded repair for g18.011 finding F1. After it merges, resume the
same g18.011 Queue task and continue the interrupted four-surface sweep from
the accepted repair head.

## Ready-State Rubric

- [x] g18.011 reproduced zero syntax spans and one default text colour for
  TypeScript and JSON in Svelte and React under Chromium and WebKit.
- [x] Both engine twins load the selected CodeMirror language extension but
  install no `syntaxHighlighting` extension or `HighlightStyle`.
- [x] The CodeEditor contract already requires full mode to retain syntax and
  maps engine classes to Poodle semantic tokens inside the distribution.
- [x] g18.012 keeps grammar packages outside Poodle; this repair needs no
  closed language catalogue or grammar dependency.
- [x] The repair can remain private to the paired engine distributions.
- [ ] Operator approves Queue dispatch and the in-place g18.011 dependency.

## Decisions

- Add one private `HighlightStyle` and install it with CodeMirror's
  `syntaxHighlighting` extension in both engines.
- Map stable Lezer tag groups to existing Poodle semantic CSS variables:
  comments and metadata to secondary text; keywords and control/module names
  to accent; strings to success; numbers, booleans and constants to info;
  types and definitions to warning; invalid syntax to danger; ordinary names
  remain primary text. Weight or style may reinforce meaning but cannot be the
  only distinction.
- Use CSS variables in the generated CodeMirror rules so theme changes update
  mounted editors without engine recreation.
- Install the presentation only when a non-plain language is active in full
  mode. `plain-text` and `performanceMode="plain"` produce no syntax spans and
  perform no grammar load.
- Add `@lezer/highlight` as an exact direct dependency only if the paired
  distributions import its tags. This is base editor presentation machinery,
  not a bundled language grammar.
- Keep the style internal. Do not expose arbitrary CodeMirror extensions,
  highlight styles, raw tags, engine handles, or a consumer theme callback.
- Preserve g18.012 lazy loading, memoization, live switching, rejection, and
  package-cost contracts unchanged.
- Do not absorb g18.011 follow-up F12 about the React preview hash spelling;
  retain it in the sweep finding set for separate disposition.

## Dispatch manifest

- **State:** ready for operator-approved dispatch; once submitted, add this
  task as an in-place dependency of blocked g18.011; g18.006 and g18.009 remain
  closed
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** paired CodeEditor engine twins and focused tests;
  paired component package manifests and lockfile only for the direct Lezer
  highlight dependency; existing language-registry browser fixture and Effigy
  selectors; CodeEditor contract only for clarifying proven presentation
  semantics; one g18.021 execution log
- **Reserved closeout surfaces:** g18 README/index/dispatch and task state;
  g18.011 evidence/finding ownership and task thread; preview routing; other
  editor components; versions, changelog, workflows, release/tag/publication,
  Desktop, native/GPUI/Jetstream
- **Worker:** web component worker comfortable with CodeMirror 6 highlighting,
  Lezer tags, CSS custom properties, paired Svelte/React distributions and
  real-browser computed-style proof
- **Excluded:** grammar packages; public extension or theming APIs; syntax
  theme presets; parser changes; editor redesign; preview routing repair;
  release or consumer mutations
- **Escalation:** Chatterbox if CodeMirror cannot consume live CSS variables,
  full/plain switching requires remounting, a public API is needed, or the
  repair changes g18.012 loading/cost behavior

## Work

1. Plant paired failures proving a real TypeScript and JSON grammar produces
   no visible token distinction under the current engine, while plain mode
   remains unhighlighted.
2. Define the private token-bound `HighlightStyle` once per byte-identical
   engine twin and install it only with full non-plain language presentation.
3. Bind representative Lezer tags to the fixed semantic palette. Prove
   comments, keywords, strings, constants/types and invalid syntax receive
   meaningful non-default treatment without hard-coded theme colours.
4. Prove live language and full/plain switching updates the same mounted
   editor, keeps registry loads memoized, preserves value/selection/history,
   and removes/reinstates token presentation without remounting.
5. Extend the existing paired Chromium and WebKit registry probe. Assert real
   syntax spans, distinct computed colours for TypeScript and JSON, automatic
   Eclipse/Iceberg theme response, and zero syntax spans in plain mode.
6. Re-run package-cost proof: no language grammar becomes a Poodle dependency
   or eager package chunk. Certify installed Svelte and React `./editor`
   distributions.
7. Run focused paired component/browser/accessibility checks, both package and
   preview builds, docs QA, and `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Full syntax is visible | grammar parses but every token inherits default text colour | real TS/JSON spans with representative distinct computed colours in both wrappers and browsers |
| Plain stays plain | plain mode still tokenizes or loads the selected grammar | zero registry loads and zero syntax token spans |
| Palette follows Poodle | literal editor colours stay fixed across themes | CSS-variable rules plus live Eclipse/Iceberg computed-colour change without remount |
| Language boundary stays open | repair bundles a preferred grammar catalogue | installed dependency/chunk inventory with no grammar packages added |
| Switching is coherent | changing language or mode remounts and loses local state | same editor node, preserved value/selection/history and memoized load counts |
| Invalid syntax is legible | errors are styled only like ordinary text | planted malformed fixture receiving danger treatment without replacing diagnostics |
| Frameworks match | tag grouping or styles drift between engine twins | source-equivalence and paired browser assertions |
| Engine remains private | consumer must pass CodeMirror tags or extensions | unchanged public props/types and package API report |
| Sweep resumes honestly | a new sweep task abandons the accepted blocked callback | g18.021 added to existing g18.011 dependency graph, then that same task retried |
| Release stays gated | g18.006 resumes after repair but before completed sweep | blocked release tasks plus successful g18.011 continuation and operator acceptance |

## Stop conditions

- Stop if the fix requires a public CodeMirror extension/theme surface or a
  bundled language grammar.
- Stop if full/plain or language switching cannot retain the mounted editor's
  controlled state.
- Stop before preview-routing, other editor, release, Desktop, native or
  retained-task mutations.

## Evidence

The first g18.011 pass built both previews at `a639b1b78` and ran Chromium and
WebKit against both framework specimens. TypeScript initial load and JSON live
switch produced zero spans inside `.cm-content`; all lines retained the same
default colour in all four pairings. The accepted blocked callback identifies
the paired engine extension arrays as the source: they install the language
compartment but no `syntaxHighlighting`/`HighlightStyle` extension. The sweep
stopped before the three remaining surfaces, as required.

## Next task

After g18.021 merges, retry the existing g18.011 task from the accepted repair
head. Continue the full four-surface sweep, retain F12 as a follow-up, return
any further blocker to Chatterbox, and keep g18.006 closed until operator sweep
acceptance.
