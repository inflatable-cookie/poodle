# g18.012 — CodeEditor extensible language registry

Status: ready for review
Date: 2026-09-11
Branch: `ns-697c0380-bcc4-4433-9bce-a6c77fa0452a`
Card: `docs/roadmaps/g18/012-code-editor-extensible-language-registry.md`
Handoff: `docs/handoffs/20260911-g18-012-code-editor-extensible-language-registry.md`
Governing refs: `docs/contracts/components/code-editor.md`,
`docs/specs/070-compiled-web-distribution-contract.md`,
`packages/core/src/code-editor.ts`,
`packages/svelte/components/src/code-editor-engine.ts`,
`packages/react/components/src/code-editor-engine.ts`
Base: `origin/main` at `85755e12e94e56b3327712effea44a64a516e9d6`

## Outcome

CodeEditor's closed eleven-language catalogue is replaced by an open,
consumer-owned language registry in both web wrappers. Poodle fixes no
language vocabulary: `plain-text` stays built in, every other language id is a
serializable consumer-defined string admitted through an opaque
`CodeEditorLanguageRegistry` the host supplies via the new `languageRegistry`
prop. The one supported registry constructor is a narrow, substrate-explicit
CodeMirror adapter published only through the new
`./editor/codemirror` package subpaths, whose lazy loaders are typed to
resolve a CodeMirror `LanguageSupport`. All seven `@codemirror/lang-*`
grammars and `@codemirror/legacy-modes` are removed from both shell
manifests, the lockfile, and every emitted graph: consumers install exactly
the grammar packages their loaders name. Registry construction and resolution
fail closed (empty ids, non-function loaders, duplicate ids, a built-in
`plain-text` entry, unknown ids, rejected loads, and loads that do not resolve
to a language extension), each language loads lazily at most once per
registry instance, controlled switching reconfigures the live compartment
without remounting, and `performanceMode="plain"` never consults the
registry. This is a deliberate pre-1.0 contract correction: no alias, union,
or silent fallback preserves the closed catalogue.

## What changed

- Shared core (`packages/core/src/code-editor.ts`): `CodeEditorLanguage` /
  `CODE_EDITOR_LANGUAGES` / `isCodeEditorLanguage` are gone. In their place:
  `CodeEditorLanguageId` (serializable string), `CODE_EDITOR_PLAIN_TEXT`
  ("plain-text"), the opaque `CodeEditorLanguageRegistry` (`has`/`load`),
  `CodeEditorLanguageRegistryInput`, `CodeEditorLanguageLoader`, and
  `createCodeEditorLanguageRegistry` — the shared framework-neutral
  construction and refusal semantics (validation at construction; unknown-id
  and memoized-load behavior at resolution). The old `code-editor-languages`
  switch is deleted from both component packages.
- New adapter subpaths `packages/svelte/components/src/editor-codemirror.ts`
  and `packages/react/components/src/editor-codemirror.ts`, exported as
  `./editor/codemirror` from both shells (single isomorphic compiled file,
  like `./types`; not a dual Svelte entry). They wrap the shared constructor
  with loaders typed to `LanguageSupport` and refuse any resolved value that
  is not a language extension before it can reach an editor. The
  `@codemirror/language` import is type-only at the surface; importing the
  adapter loads no grammar.
- Both engines resolve the selected id through the host registry: mount and
  every controlled `update()` reconfigure the existing language compartment
  via `resolveLanguageExtension` (plain text and plain performance mode load
  no language; unknown ids and rejected or non-extension loads refuse before
  any false syntax state; registry identity changes re-resolve). Wrapper
  props, manifests, and editor entries carry `languageRegistry` through;
  `language` widens to `CodeEditorLanguageId`.
- Both shell manifests, `bun.lock`, the distribution build spec
  (`scripts/web-distribution/`), and `docs/specs/070` drop every grammar
  package: the shells pin only the base engine substrate (`commands`,
  `language`, `search`, `state`, `view`), gain the `./editor/codemirror`
  export targets and emitted files, and both build specs now forbid grammar
  modules in manifests and emitted graphs.
- Paired specimens (`CodeEditorSpecimen.svelte` / `.tsx`) construct a
  consumer registry naming exactly `@codemirror/lang-javascript` (TypeScript
  mode) and `@codemirror/lang-json` — two consumer-selected languages plus
  built-in plain text — with a live plain-text/typescript/json switch, and
  the preview packages (not Poodle) declare those grammar dependencies.
- Tests: shared core registry semantics (`packages/core/test/
  code-editor.test.ts`), paired component suites (unknown-id refusal,
  consumer-defined ids absent from Poodle source, lazy single loads across
  switching, plain-mode bypass, registry swaps, rejected/non-extension
  loads), packaging proof (`CodeEditorPackaging.test.ts`: base-only pinned
  set, grammar packages refused, adapter subpath exports), real-browser
  paired probe `test/code-editor-language-registry/` with effigy selectors
  `test:code-editor-language-registry{-chromium,-webkit}`, the packed-install
  consumer fixture `test/package-install/fixture/EditorLanguageRegistry.test.ts`
  plus the one-grammar consumer manifest in `test/package-install/
  web-preview.ts`, and preview registry suites exercising real
  `@codemirror/lang-json` parsing (`g18-012-code-editor-language-registry`).
- Component docs: contract (`docs/contracts/components/code-editor.md`),
  distribution spec (§ editor entries laws), and both generated
  `component-docs` artifacts/document sources now describe the open registry.

## Before/after baseline

Captured 2026-09-11; "before" measured by building `origin/main`
(`85755e12e94e56b3327712effea44a64a516e9d6`) in a detached worktree with the
same selectors, "after" measured at this branch's head.

- Installed dependencies: before, each shell manifest declared 8 grammar
  packages (`@codemirror/lang-{css,html,javascript,json,markdown,rust,yaml}`
  + `@codemirror/legacy-modes`) as hard dependencies, mirrored in
  `bun.lock`. After, zero grammar packages in either manifest or the
  lockfile; the grammar packages appear only as the preview consumers'
  own pinned dependencies (`lang-javascript`, `lang-json`).
- Emitted package graphs: before, `dist/editor.client.js`,
  `dist/editor.server.js`, and `dist/editor.js` each referenced all seven
  `@codemirror/lang-*` grammars by name (the closed switch named every
  grammar). After, no module in either shell's `dist/` matches
  `@codemirror/lang-*` or `@codemirror/legacy-modes` (the build spec now
  fails the build if one does), and the emitted `./editor/codemirror` files
  import the substrate only.
- Emitted preview chunks: before, the Svelte preview build emitted 8
  grammar-bearing dynamic chunks (`dist-*.js`, one per admitted language
  family). After, the Svelte preview build emits 3 `dist-*.js` chunks with
  no grammar module names; the consumer's two chosen grammars bundle into
  the consumer's own graph and an unselected grammar appears nowhere.
- Active-loader baseline: before, mounting any language statically imported
  its grammar into the editor graph; the switch was closed. After, loader
  counters prove exactly one load for the initially selected language, zero
  for unselected languages, memoized reuse on re-selection, and one
  additional load only when a newly admitted id is selected (see probe
  evidence below).

## Planted regression

All new suites bind the oracles against the old shape first: the core suite's
`CODE_EDITOR_LANGUAGES`/fixed-union assertions, the packaging suite's
fixed-grammar manifest pin, and the component suites' `languageFor`/closed
union usage all fail against `origin/main` (the removed exports no longer
exist, and the packaging test rejects manifests that re-declare grammars).
The packed-install fixture fails before this change because the shells'
tarballs carried grammar dependencies into the isolated consumer install.
With the implementation applied, all pass.

## Validation

- `bun test packages/core/test/code-editor.test.ts` — 17 pass (registry
  construction, refusal, and memoization semantics).
- `bunx vitest run packages/svelte/components/test/CodeEditor.test.ts
  packages/react/components/test/CodeEditor.test.tsx
  packages/svelte/components/test/CodeEditorPackaging.test.ts
  packages/svelte/preview/test/g18-012-code-editor-language-registry.test.ts
  packages/react/preview/test/g18-012-code-editor-language-registry.test.tsx`
  — 5 files, 113 pass.
- `bunx vitest run --project svelte-components --project react-components`
  — 363 files, 3041 pass.
- `bunx vitest run --project svelte-preview --project react-preview`
  — 27 files, 124 pass (including the g18.012 registry suites).
- `bunx vitest run --project a11y` — 183 pass.
- `effigy test:code-editor-language-registry` — all paired Chromium and
  WebKit checks pass for both frameworks (13 checks per framework per
  engine): lazy single initial load with zero unselected loads, one
  additional load per newly selected id, memoized re-selection, live
  plain-text/typescript/json switching on the same editor instance, typed
  text and undo history surviving two switches, the rejected load surfacing
  as an unhandled rejection while the previous language stays active with no
  remount, recovery through the memoized registry, and real mount-time
  unknown-id refusal (`unsupported language "cobol"`) caught through
  `svelte:boundary` / the React error boundary with no editor presented and
  the working editor untouched.
- `effigy test:web-pack-install` — clean: the isolated consumer installs
  exactly `@codemirror/lang-json`, both shipped `./editor/codemirror`
  adapters construct working registries from the packed archives, the chosen
  grammar loads and parses through them, and every unselected grammar is
  absent from the install graph (file absence plus failed dynamic import).
- `effigy svelte:package`, `effigy react:package` — clean (declaration emit,
  receipt, staged-dist, and dependency audits; forbidden grammar modules).
- `effigy svelte:build`, `effigy react:build` — clean (pre-existing chunk
  size warnings only).
- `effigy check:svelte-components` — 0 errors (4 pre-existing warnings).
  `effigy check:react-components` — 12 errors, all pre-existing in untouched
  files (the documented backlog; this branch's diff removes 6 errors and adds
  none). `effigy check:svelte-preview` — 0 errors. `effigy
  check:react-preview` — the documented pre-existing specimen
  `string`/`ControlSize` backlog only, none in touched files.
- `effigy docs:check` — pass. `git diff --check` — clean.

## Explicitly not done

- No general CodeMirror extension prop, theme, keymap, plugin, or DOM hook;
  the adapter exposes only language-loader construction and opaque registry
  use.
- No exhaustive bundled catalogue, no grammar dependency, no release, tag,
  publish, Desktop, native, or workflow mutation.
- g18.011 still runs after this task per the corrected ordering; g18.006
  stays paused and g18.009 stays dependency-queued.

## Continuation

g18.012 is ready for review. g18.011 runs the complete four-surface sweep
against the completed extensible registry (per the operator-corrected
ordering); g18.006 and g18.009 remain gated behind the sweep and operator
acceptance.
