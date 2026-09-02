# g16.058 — Shell Distributions

Status: complete — awaiting orchestrator review
Date: 2026-09-02
Card: `docs/roadmaps/g16/058-shell-distributions.md`
Handoff: `docs/handoffs/20260902-125334-g16-058-shell-distributions.md`
Governing refs: `docs/architecture/014-compiled-web-package-distribution.md`,
`docs/specs/070-compiled-web-distribution-contract.md`
Branch: `feature/g16-058-shell-distributions`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-058-shell-distributions`
Base: `origin/main` at `8d2bd25ca82a9e1af7af7b670ed7990e1b1f1aa7`

## Outcome

Svelte ships dual compiled lanes (`*.client.js` / `*.server.js`) from one
sorted entry inventory. Exports are `types`, `browser`, `default` — no
`import` condition, no `svelte` condition, no top-level `svelte` field. Public
`*.svelte` subpaths target compiled JS plus `*.svelte.d.ts`, never source.
`./markdown` owns AgentMessage/MarkdownEditor. `./types` is JS plus
declarations. Internals (`MenuSurface`, `DragDropProvider`) stay in
`dist/chunks/` and are not wildcard-importable.

React compiles to the same source-free/declaration/CSS standard and stays
`private: true`.

`@sveltejs/package` `emitDts` rejects TypeScript 7.0.2. Declarations are `tsc`
`emitDeclarationOnly` plus generated `Component` shims. Path audit skips
quoted values containing `<` or `>` so SSR HTML `"/</span>"` is not a fake
workspace path. `jsxDev: false` keeps React output off the jsx-dev runtime.

Preview coverage and the parity ledger now read `src/markdown.ts` as well as
root `index.ts`, so AgentMessage/MarkdownEditor stay in the 176 public
roster.

## Evidence

- Dual Svelte builds match. Client contains `svelte/internal/client`; server
  contains `svelte/internal/server`. Button CSS is `styles/button.css`, not
  markdown-editor. Button/index have no `from "marked"`; AgentMessage does.
- Bundler and NodeNext resolve root/direct/markdown/`./types` for both shells.
- Node default resolves `Button.svelte` to `Button.server.js`;
  `--conditions=browser` resolves `Button.client.js`. Direct client through
  `svelte/server` throws. Happy-dom mounts client Button at Svelte 5.56.8.
- Archives have no `src/`, no maps, no raw `.svelte`. `./types` has JS + dts.
- Missing `marked` fails `./markdown`. Direct Button/Select SSR works without
  it. After adding `marked@18.0.9`, root Button and markdown SSR render.
- React `Button` is a function; `private` remains true.

## Oracle

| Row | Plant | Result |
| --- | --- | --- |
| Lanes are distinct | `render(client).body` | throws; server body has `poodle-button` |
| Conditions describe environment | fake `"import": "./client.js"` before `default` | Node `import.meta.resolve` returns client.js |
| Source-free Svelte | `dist/Button.svelte` | raw source forbidden |
| Public wildcard is exact | `dist/MenuSurface.client.js` | unexpected staged file; installed `./MenuSurface.svelte` is `ERR_MODULE_NOT_FOUND` |
| Markdown isolated | Button/index `from "marked"` | graph assertion fails |
| Types reachable | delete `dist/types.js` | missing staged public file |
| React private | `private` cleared / `publishConfig.access: public` | publication-state check fails |
| Harness serial | `git diff origin/main -- test/package-install` | empty |

All plants restored. Focused tests repeat them against disposable fixtures.

## Validation

- `bun test` driver + svelte-build + react-build + shell-smoke + core-build —
  34/0
- `vitest` Button svelte + react — 7/0
- `effigy docs:check` — pass
- `git diff --check origin/main...HEAD` — pass
- No windowed, native-visual, or release selector
- `test:web-pack-install` not run and not edited

## Limits

- `test:web-pack-install` still reads source `src/index.ts` and expects 176
  names on root. That harness stays on g16.059. `ci:web` will fail it until
  then.
- Unbundled Node evaluation of the root barrel still loads AgentTranscript →
  AgentMessage → `marked` because those root-roster composers statically import
  AgentMessage. Direct `./Button.svelte` and the Button module graph stay
  parser-free. Full-barrel Node SSR needs `marked` present; missing-peer is
  proven on `./markdown`.
- Svelte `.svelte.d.ts` files are generic `Component` shims, not
  prop-accurate emit from `@sveltejs/package`.

## Diff scope

Owned: shell manifests/exports, root/markdown barrels, preview/workspace
aliases onto source, `scripts/web-distribution` shell driver + smoke, focused
tasks, preview coverage/ledger follow-through for `./markdown`, this card, this
log, papercuts. Not `test:web-pack-install`, versions, workflows, tags,
registries, or React publication state.

## Continuation

Accepted merge unlocks `g16.059`. `g16.054` remains blocked.
