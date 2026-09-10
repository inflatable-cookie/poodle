# g18.002 — CodeMirror web CodeEditor

Status: merged
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

## Closeout

- Merge performed by the plugin as `308fa52c5cd68d9c776f320c368e4fb0896e4d4d` on 2026-09-10 (PR #236) after independent exact-head `ready_to_merge` review ([comment #5623576100](https://github.com/inflatable-cookie/poodle/pull/236#issuecomment-5623576100)) of head `6d875b30c7f36612ec7acec369038a97920c6ddc`.
- In-round repairs after the round-2 approval: content-aware ordinary JS package-manifest classifier (`test/package-install/scope.ts`; strict g16.059 and candidate g16.054 paths unchanged) and Nucleus M1/A1 receipt repin (62 receipt files plus manifest, `source_commit 2cf135d1` to `de7a97b7`, zero other changed lines; ledger regenerated with zero delta; V1/Lab bundle untouched). The pre-existing nucleus receipt mismatch noted above as untouched was repaired by this repin; `check:gpui-census` and ledger checks are green at the reviewed head.
- Reviewed-head validation (round 3): `test:core-build` 63 pass; `test:web-pack-install` exit 0; `check:gpui-census`, `test:gpui-census` 17 pass, `test:nucleus-parity-receipts` 17 pass; `regressions:native` 234 pass; component boards 3819 pass, parity 552, a11y 180 (round 2, unchanged surfaces); PR gates web + rust pass at `6d875b30`.
- Non-blocking reviewer notes (deferred, no acceptance impact): execution-log narration gap for the repin/relocation commits; absent-vs-empty `publishConfig` treated as equal in transport comparison.
- Deferred: release/tag/publish, Desktop adoption, GPUI CodeEditor, preview catalogue admission — all need separate authority. Return to Chatterbox.
