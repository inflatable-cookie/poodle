# g18.002 — CodeMirror web CodeEditor

Status: ready for review
Date: 2026-09-10
Branch: `ns-04d0b9b3-a9d6-46a6-82e7-c2b5bbc8bfe7`
Card: `docs/roadmaps/g18/002-codemirror-web-code-editor.md`
Handoff: `docs/handoffs/20260910-g18-002-codemirror-web-code-editor.md`
Governing refs: `docs/contracts/components/code-editor.md`,
`docs/contracts/components/tabs.md`, `docs/contracts/001-working-rules.md`,
`docs/architecture/001-poodle-system-shape.md`,
`docs/specs/070-compiled-web-distribution-contract.md`

## Outcome

One controlled Poodle `CodeEditor` for Svelte and React over pinned
CodeMirror 6 packages, plus the contracted `TabItem.pinned` behavior across
the normal Tabs cohort (core, both web runtimes, shared Rust, render, mounted
GPUI). Engine stays private behind dedicated `./editor` entries; root
consumers load no CodeMirror. No GPUI CodeEditor, no Desktop changes, no
release.

## What changed

- Core (`packages/core/src/code-editor.ts`, `tabs.ts`, `index.ts`,
  `styles/code-editor.css`): engine-independent types, prior-value change
  translation with overlap/bounds refusal, host-diagnostic validation,
  2 MiB envelope, `TabsPin` partition validation and reorder guards.
- Engines (`code-editor-engine.ts`, `code-editor-languages.ts`, mirrored in
  both shells): exactly 13 pinned `@codemirror/*` externals, compartment
  reconfiguration, controlled no-echo translation, host-owned diagnostics,
  search/history/line-numbers, read-only/disabled, Tab focus/indent with
  Escape exit, F8 diagnostic traversal, explicit plain performance mode.
- Shells: `CodeEditor.svelte` / `CodeEditor.tsx` (SSR-safe mount/update/
  destroy), `src/editor.ts` entries, `./editor` manifests, `code-editor.css`,
  `TabItem.pinned` with eligibility + commit refusal and `data-pinned`.
- Rust: `TabDefinition.pinned` + builder, headless machine guards, render
  mapping with pinned source/target unwiring and refused-drop rejection.
- Contracts/specs: CodeEditor §9 engine line-break note, spec 070 editor
  entries + dependency ownership, `THIRD_PARTY_NOTICES.md` CodeMirror pins.
- Tests: core (19 new), Svelte/React CodeEditor (12 each), Tabs pinned
  (5 each), SSR, packaging proofs, Rust unit + mounted GPUI pinned test,
  smoke-fixture value, frozen-count updates (styles 168, react exports +1).

## Validation

Core 1284 pass; web boards 2802 pass; a11y 180 pass; parity 178 pass;
`poodle-headless` 221 pass; render tabs 26 pass; mounted GPUI tabs 5 pass
plus new pinned test; distribution suites 57 + shell suites pass;
`svelte:package` and `react:package` build clean; `git diff --check` clean.
Census `--check` fails on a pre-existing nucleus receipt commit mismatch,
untouched by this task.

## Remaining staged-admission limits

Web-admitted only; native CodeEditor is future work with no placeholder.
Preview catalogue admission is reserved generation-index closeout. Release
and Desktop adoption need separate authority (see the adoption-request
handoff).

## Follow-up: content-aware ordinary JS scope (same branch)

The `web` gate false-positived on the owned manifest work: the ordinary
installed-package classifier labeled any `package.json` path a version
surface. Chatterbox ruling keeps the accepted dependency/export design and
repairs the classifier instead: `test/package-install/scope.ts` now derives
ordinary JS labels from manifest content (dependency/export changes accepted
only while version, name, `private`, and `publishConfig`/`registry`
transport posture hold; version/name mutations stay version surfaces,
publication/transport mutations stay registry surfaces, unparsable/added/
deleted manifests fail closed), with focused scope tests and
package-install README documentation. Strict g16.059 and candidate g16.054
paths are unchanged. Ordinary `test:web-pack-install` exits 0.
