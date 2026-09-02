# g16.058 — Shell Distributions

Status: complete — awaiting orchestrator review
Date: 2026-09-02
PR: https://github.com/inflatable-cookie/poodle/pull/162
Card: `docs/roadmaps/g16/058-shell-distributions.md`
Handoff: `docs/handoffs/20260902-125334-g16-058-shell-distributions.md`
Governing refs: `docs/architecture/014-compiled-web-package-distribution.md`,
`docs/specs/070-compiled-web-distribution-contract.md`
Branch: `feature/g16-058-shell-distributions`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-058-shell-distributions`
Base: `origin/main` at `3aca021622d17934d9bfb8f4518313d95d124f2b`

## Outcome

Svelte ships dual compiled lanes (`*.client.js` / `*.server.js`) from one
sorted entry inventory. Exports are `types`, `browser`, `default` — no
`import` condition, no `svelte` condition, no top-level `svelte` field. Public
`*.svelte` subpaths target compiled JS plus real `@sveltejs/package`
`*.svelte.d.ts`, never source. Root barrels hold 171 components.
`./markdown` owns AgentMessage, AgentPlan, AgentPlanRecord, AgentTranscript,
and MarkdownEditor. Direct subpaths stay. All five render synchronously in
browser and SSR. `./types` is JS plus declarations. Internals (`MenuSurface`,
`DragDropProvider`) stay in `dist/chunks/` and are not wildcard-importable.

React compiles to the same source-free/declaration/CSS standard and stays
`private: true`.

Distribution declarations stage through
`scripts/web-distribution/declaration-tools` pinned to TypeScript `6.0.3` and
`@sveltejs/package` 2.5.7. Root TypeScript stays `^7.0.2`. Bare generic
`Component` shims fail. Compiler/shim falsification uses a 30s subprocess
timeout. Path audit skips quoted values containing `<` or `>` so SSR HTML
`"/</span>"` is not a fake workspace path. `jsxDev: false` keeps React output
off the jsx-dev runtime. Vite library chunk names strip a trailing `.svelte`
so installed `vite-plugin-svelte` does not compile JavaScript as Svelte.

## Evidence

- Dual Svelte builds match. Client contains `svelte/internal/client`; server
  contains `svelte/internal/server`. Dist JS has no `.svelte` in filenames.
  Button CSS is `styles/button.css`, not markdown-editor. Button/index have
  no `from "marked"`; AgentMessage does. Root dts omits the five markdown
  names; `markdown.d.ts` has them.
- Bundler and NodeNext resolve root/direct/markdown/`./types` for both shells.
  Invalid Button/Select props/callbacks/snippets/bindables fail unsuppressed
  with ≥6 `TS2322` under both resolutions.
- Node default resolves `Button.svelte` to `Button.server.js`;
  `--conditions=browser` resolves `Button.client.js`. Direct client through
  `svelte/server` throws. Happy-dom mounts client Button at Svelte 5.56.8.
- Archives have no `src/`, no maps, no raw `.svelte`. `./types` has JS + dts.
- Missing `marked` fails `./markdown`. Root Button/React Button import without
  `marked`. The five markdown names are absent from both roots.
- After `marked@18.0.9`, AgentPlan and expanded AgentPlanRecord SSR/browser
  render `.poodle-agent-message` and the plan heading. React `renderToString`
  of both is not a Suspense abort.
- React `Button` is a function; `private` remains true.

## Oracle

| Row | Plant | Result |
| --- | --- | --- |
| Lanes are distinct | `render(client).body` | throws; server body has `poodle-button` |
| Conditions describe environment | fake `"import": "./client.js"` before `default` | Node `import.meta.resolve` returns client.js |
| Source-free Svelte | `dist/Button.svelte` | raw source forbidden |
| Public wildcard is exact | `dist/MenuSurface.client.js` | unexpected staged file; installed `./MenuSurface.svelte` is `ERR_MODULE_NOT_FOUND` |
| Markdown isolated | Button/index `from "marked"` | graph assertion fails |
| Root is parser-free | any of the five on root, or a lazy parser import | root leak or empty SSR plan body |
| Types reachable | delete `dist/types.js` | missing staged public file |
| Declarations preserve the API | bare `Component` shim on Button and Select | negatives compile (status 0); restore fails again |
| React private | `private` cleared / `publishConfig.access: public` | publication-state check fails |
| Harness serial | 059 receipt or below-floor negative | not present; existing pack proofs only |

All plants restored. Focused tests repeat them against disposable fixtures.

## Validation

- `effigy test:shell-build` — 31 pass / 0 fail (driver, svelte-build,
  react-build, shell-smoke; compiler/shim oracle 30s subprocess / 60s test)
- `effigy test:web-pack-install` / `ci:web` pack-install — 10 files / 19 tests
  plus HistoryEntry / Slider / Tree type proofs; roster root 171
- `effigy ci:web` — `test:components` 376 files / 3606 tests; public Svelte
  surface 171; svelte-check 0 errors
- `effigy docs:check` — pass (ledger 176, React specimens 176, docs:build)
- `git diff --check origin/main...HEAD` — clean
- No windowed, native-visual, or release selector

## Limits

- Existing pack-install could not stay green on compiled archives with only
  `roster.ts`. The frozen consumer ran Svelte `5.38.6` (cannot execute
  `5.56.8` client output) and Vite 7.3.1 left compiled React CSS imports for
  Node. Existing HistoryEntry/Slider/Tree React proofs also required
  `src/types.ts`. This card pinned the existing consumer to `5.56.8`, stubbed
  CSS the same way the disposable smoke already does, and retargeted those
  proofs to `dist/*.d.ts`. The packed React AgentPlan root-mount left with the
  171-root split. No 059 receipt, below-floor negative, or new certification
  probe.
- `ci:web` runs `svelte:package` and `react:package` before pack-install
  because archives are `dist`-only.
- Chunk JS must not be named `*.svelte.client.js` or installed
  `vite-plugin-svelte` compiles it as Svelte (`$` is reserved).

## Diff scope

Owned: shell manifests/exports, root/markdown barrels, synchronous plan
render, declaration-tools TS 6.0.3 path, focused build/smoke tests, existing
pack-install roster plus consumer retargets above, `ci:web` package steps,
this card, this log, papercuts. Not 059 receipt/new probes, versions,
workflows, tags, registries, native, Jetstream, or React publication state.

## Continuation

Accepted merge unlocks `g16.059`. `g16.054` remains blocked.
