# g18.002 CodeEditor distribution evidence

Status: measured on the worker branch before PR
Date: 2026-09-10
Task: `docs/roadmaps/g18/002-codemirror-web-code-editor.md`
Base: `main` at `3cc66779e162047a95d38adfdb464b5330f6a200`

All facts below were read off the built `dist/` trees
(`core:build`, `svelte:package`, `react:package`) and the test runners on this
branch. Nothing here is hand-claimed.

## Package isolation

- Root bundles carry zero engine references (`grep -c codemirror`):
  `svelte/dist/index.client.js` 0, `svelte/dist/index.server.js` 0,
  `react/dist/index.js` 0. No `code-editor-engine` string in any root bundle.
- Both `./editor` entries reach the engine only through external imports
  (14 specifiers in each lane, client and server):
  `@codemirror/{commands,lang-css,lang-html,lang-javascript,lang-json,lang-markdown,lang-rust,lang-yaml,language,search,state,view}`,
  `@codemirror/legacy-modes/mode/{shell,toml}`.
- Editor declarations carry zero engine types (`grep -c codemirror`):
  `svelte/dist/editor.d.ts` 0, `react/dist/editor.d.ts` 0.
- `CodeEditor` is absent from both root barrels, the 176-name roster, and
  every successor denominator (`CodeEditorPackaging.test.ts`).
- The 176/171 roster counts are unchanged (`svelte-build.test.ts`,
  `CORE_STYLE_FILES` grows 167 → 168 for `code-editor.css` only).

## Pinned engine versions (both shell manifests, exact, MIT)

state 6.7.4, view 6.43.11, commands 6.11.0, search 6.7.2, language 6.12.4,
lang-javascript 6.2.5, lang-json 6.0.2, lang-html 6.4.12, lang-css 6.3.1,
lang-markdown 6.5.2, lang-rust 6.0.2, lang-yaml 6.1.3, legacy-modes 6.5.4.
Recorded in `THIRD_PARTY_NOTICES.md`.

## Behavior proof (all green on this branch)

- Core: 1284 pass (`bun test` in `packages/core`), including exact
  multi-range replay, UTF-16 astral offsets, diagnostic refusal, closed
  language set, 2 MiB envelope, and Tabs pinned partition/machine guards.
- Svelte + React component boards: 2802 pass, including 12 CodeEditor and
  5 Tabs-pinned cases per shell, 2 MiB boundary mounts (100 000 lines mount
  36 `.cm-line` nodes), SSR root-without-engine, and engine-leak/packaging
  proofs.
- a11y sweep 180 pass, Svelte↔React parity sweep 178 pass (CodeEditor
  covered by both automatically).
- Rust: `poodle-headless` 221 lib pass, headless tabs conformance pass,
  `poodle-render` tabs 26 pass, mounted GPUI `tabs_pinned_partitions` pass
  plus the 5 pre-existing mounted GPUI tabs regressions.
- Distribution: core-build/pack-archive/scope/gate suites 57 pass,
  shell-build suites pass, both packages build clean with receipts.

## Known staged-admission limits (unchanged by this task)

- `CodeEditor` is `web-admitted`, not parity-complete: no `TabsSpec`-style
  Rust declaration, no `poodle-render` composition, no GPUI implementation,
  no placeholder or receipt anywhere in native targets.
- Preview catalogue admission (specimen route, generation index) is reserved
  closeout work; focused tests are the specimen evidence for now.
- Engine line-break ownership (contract §9): CR/CRLF load as LF; everything
  else is byte-exact and every `onChange` replays against the owned document.
