# 019 — MarkdownRenderer shared safe/trusted rendering

Status: active — Queue task `eac944cd-2ee0-4810-bd60-0976e3270e56`
Owner: Poodle web editors
Created: 2026-09-11
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/markdown-editor.md`,
`../../../packages/svelte/components/src/MarkdownEditor.svelte`,
`../../../packages/react/components/src/MarkdownEditor.tsx`,
`../../../packages/svelte/components/src/markdown.ts`,
`../../../packages/react/components/src/markdown.ts`,
`../../specs/070-compiled-web-distribution-contract.md`
Depends on: none

## Outcome

Add paired public Svelte and React `MarkdownRenderer` components through the
existing `./markdown` entry. Extract one private rendering path per framework
and make both `MarkdownEditor` preview and `MarkdownRenderer` use it.

Default all rendered Markdown to the shared safe HTML policy. Provide the
operator-approved explicit `htmlPolicy="trusted"` bypass for fully trusted
consumer content. Add dedicated specimen pages in both web previews before the
dependency-queued editor acceptance sweep and release candidate.

## Ready-State Rubric

- [x] `MarkdownEditor` already owns built-in `marked` parsing, a `renderHtml`
  customization callback, and shared preview prose CSS in both frameworks.
- [x] No standalone renderer or shared private preview component exists.
- [x] Current built-in and custom HTML reaches `{@html}` /
  `dangerouslySetInnerHTML` without sanitization.
- [x] The operator requested a renderer matching `RichTextRenderer` and
  confirmed safe-by-default with an explicit trusted opt-in.
- [x] The public props, security posture, empty posture, SSR semantics, package
  entry, and paired specimen requirement are fixed in the component contract.
- [x] The task is independent of active rich-text, CodeEditor, and Slider source
  and can execute in parallel when approved.

## Decisions

- Public component name is `MarkdownRenderer`; public source prop is required
  `value: string`.
- `renderHtml` keeps its existing parser customization meaning and is shared by
  editor and renderer. It does not imply trust.
- `htmlPolicy: "safe" | "trusted"` defaults to `safe` on both components.
  Safe sanitizes the complete output of either parser path. Trusted bypasses
  sanitization only through that explicit prop.
- The safer default deliberately hardens the pre-v1 editor contract. Record the
  migration for consumers that previously relied on unsanitized Markdown HTML;
  do not add a compatibility fallback that silently restores trust.
- Apply the policy after parsing so built-in and custom output cannot drift.
  Use one deterministic sanitizer contract in Svelte, React, and SSR builds.
- Safe mode removes executable elements/attributes and unsafe URL schemes while
  retaining the admitted Markdown prose surface. Add adversarial malformed,
  encoded, and namespace cases; a shallow tag regex is not acceptable.
- Trusted mode is intentionally dangerous and visually identical. Document
  that callers own source provenance and CSP; do not add automatic trust,
  environment detection, or a compatibility fallback.
- Extract rendering from each editor shell into a private component/helper.
  `MarkdownEditor` and `MarkdownRenderer` must call that path rather than copy
  parser, sanitizer, or markup branches.
- The standalone renderer has no toolbar, textarea, form behavior,
  contenteditable node, or editor-specific empty copy. `ariaLabel` optionally
  makes it one labelled region; otherwise normal document semantics remain.
- Reuse the same preview typography and density tokens without inheriting the
  editor border, toolbar, or split-pane layout.
- Export only from existing `./markdown` and existing direct-entry conventions.
  Keep `marked` isolated to that package graph and certify installed output.
- Add separate MarkdownRenderer pages to both public previews. Cover safe,
  trusted, custom-parser, empty, constrained-width, theme, and density postures.
- This is web-only admission. Do not claim GPUI/Jetstream parity or add a second
  Poodle Markdown AST.

## Dispatch manifest

- **State:** active in Queue task `eac944cd-2ee0-4810-bd60-0976e3270e56`;
  independent and running in parallel with g18.017 and the g18.013 chain;
  explicit prerequisite of g18.011 and therefore serial before g18.012,
  retained g18.006, and g18.009
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** MarkdownEditor shells/tests; new paired
  MarkdownRenderer shells/tests; private Markdown rendering/sanitization
  helpers; shared Markdown prose CSS; `./markdown` exports and package manifests;
  Markdown component docs/spec 070/package certification; paired preview
  catalogue/specimen/test surfaces; sanitizer dependency and lockfile only if
  justified by the contract; one g18.019 execution log
- **Reserved closeout surfaces:** RichTextEditor/Renderer, CodeEditor, Slider,
  public Markdown AST/plugin system, native/GPUI/Jetstream, g18 README/index/
  dispatch, g18.006/g18.009/g18.011 task state, workflows, versions,
  release/tag/publication, Desktop
- **Worker:** high-reasoning web component/security worker comfortable with
  deterministic HTML sanitization, SSR, package boundaries, and paired Svelte/
  React rendering
- **Excluded:** Markdown editing redesign; new toolbar commands; file/media
  management; syntax highlighting; arbitrary parser plugins; native work;
  release work; workflow changes; Desktop
- **Escalation:** Chatterbox for sanitizer policy ambiguity, inability to make
  SSR/browser output deterministic, package-boundary expansion outside
  `./markdown`, or any release/workflow mutation

## Work

1. Plant paired adversarial failures showing current preview execution-capable
   HTML, unsafe attributes/URLs, and custom-renderer output bypassing policy.
2. Define one deterministic sanitizer boundary and prove safe output for the
   admitted Markdown surface across Svelte/React SSR and browser mounts.
3. Extract the private content renderer in both frameworks. Route existing
   MarkdownEditor preview through it without changing editing, selection,
   toolbar, mode, or callback behavior.
4. Add public `MarkdownRenderer` with the fixed contract. Prove zero editor
   mechanics and exact output parity with MarkdownEditor preview.
5. Bind `htmlPolicy="trusted"` separately with an explicit trusted fixture.
   Prove safe remains the default and `renderHtml` alone cannot bypass it.
6. Rework shared CSS selectors so standalone prose receives identical document
   styling without editor chrome. Cover empty and constrained overflow.
7. Add exports, declarations, package metadata, side-effect/style ownership, and
   installed-tarball smoke for both frameworks through `./markdown`.
8. Add dedicated Svelte and React specimen pages with safe, trusted, custom,
   empty, theme, and density examples. Include visible policy labelling so the
   trusted specimen cannot be mistaken for the default.
9. Run focused paired component/SSR/browser/security tests, both package and
   preview builds, installed-package certification, accessibility checks,
   Effigy docs QA, and `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Default is safe | raw `<script>` or handler becomes live DOM | paired browser/SSR sanitizer corpus with zero executable output |
| Custom parser stays safe | `renderHtml` silently bypasses sanitizer | malicious custom output sanitized under default policy |
| Trust is explicit | parser choice or environment enables raw HTML | only exact `htmlPolicy="trusted"` bypasses sanitization |
| Trusted path is truthful | sanitizer still rewrites trusted output | exact retained controlled fixture plus warning docs/specimen |
| One rendering path | editor and renderer copy parsing or markup | source boundary plus equal-output tests for both components |
| Renderer is editor-free | hidden textarea/editor mounts | DOM/SSR absence of toolbar, textarea, and contenteditable |
| Semantics are stable | wrapper adds an unnamed landmark or breaks headings | default document tree and optional labelled-region assertions |
| Empty is neutral | standalone output says "Nothing to preview" | empty-root assertion distinct from editor preview copy |
| Styling matches | renderer prose drifts from preview | computed token/layout comparison across theme/density |
| SSR matches browser | sanitizer depends on browser-only DOM | deterministic SSR/hydration equivalence proof |
| Packages are real | source works but tarball lacks export/dependency/style | installed `./markdown` import/render smoke in both frameworks |
| Specimens are baseline | component ships with no catalogue page | dedicated routed Svelte/React pages and direct-route tests |
| Release remains gated | candidate resumes before four-surface sweep | merged g18.019 before g18.011 release and operator acceptance |

## Stop conditions

- Stop if safe mode cannot be deterministic across SSR and browser rendering.
- Stop if the implementation would retain an implicit trusted path or require a
  public Markdown AST/plugin contract.
- Stop before native work, workflow changes, release/version publication, or
  Desktop mutation.

## Evidence

Source inspection on 2026-09-11 found the paired editor previews directly call
`marked.parse()` or `renderHtml()` and inject the returned string. They share
CSS but no private content component. Tom confirmed `MarkdownRenderer`, paired
specimen pages, safe default output, and an explicit trusted opt-in.

## Next task

Merge g18.019 and the other editor/Slider blockers. Queue dependencies then
dispatch g18.011 as a four-surface web editor acceptance sweep and g18.012 after
it. Keep g18.006 paused until operator acceptance; g18.009 waits on g18.006.
