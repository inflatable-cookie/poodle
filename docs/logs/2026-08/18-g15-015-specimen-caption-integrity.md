# g15.015 — specimen caption integrity

Date: 2026-08-18
Card: `docs/roadmaps/g15/015-specimen-caption-integrity.md`
Branch: `t3code/fix-specimen-caption-integrity`

## Baseline

- Worker worktree: `/Users/tom/.t3/worktrees/poodle/t3code-d1ed0dee`
- `bun install` required before preview `svelte-check` matched the dispatch baseline.
- Preview workspace before edits: **428 errors in 25 files** (348 catalogue, 52 caption props, 28 residual).

## Fix classes

| Class | Count | Approach |
| --- | ---: | --- |
| Caption props | 52 | `title=` → `label=` on nine agent-surface Svelte specimens |
| `SpecimenGroup` | — | Optional `description` via muted `Text` in Svelte + React helpers |
| React pairing | 9 pages | Copied authored Svelte descriptions into React counterparts |
| Generated catalogue | 348 | `catalogue-ts` emits typed empty `collections` literals; `catalogue:build` |
| Recipe inventory script | 13 | Unified entry type; `import-meta.d.ts` for `import.meta.dir` |
| Contract drift scripts | 5 | `contract-role-drift` 2, `contract-spec-drift` 2, `contract-value-domain-drift` 1 — three `import.meta.dir`, two Bun globals (`spawnSync`, `Glob`) via `import-meta.d.ts` |
| `ListContainerSpecimen` | 5 | Renamed local `state` → `containerState` (Svelte 5 `$state` clash) |
| `SceneSpecimen` | 2 | Axis casts inside snippets; `as never` boundary on layout snippets |
| `DialogSpecimen` | 1 | Required `Field` `id` |
| `component-registry` | 1 | Typed empty `collections`; import `CatalogueCollectionId` |
| `licence.ts` | 1 | Narrow with `result.ok === false` before reading `problem` |

## Gate

- Added `check:svelte-preview` → `bunx svelte-check --workspace packages/svelte/preview --tsconfig ./tsconfig.json --threshold error`
- Composed into `check:svelte` (operator-approved `tasks/effigy.tasks.toml` change).

## Mutation proof

1. Restored `title=` on `AgentMessageSpecimen` → `check:svelte-preview` failed.
2. Restored `label=` → selector passed with **0 errors**. Mutation not committed.

## Tests

- `packages/svelte/preview/test/specimen-group.test.ts` — label + description rendering.
- `packages/svelte/preview/test/agent-caption-integrity.test.ts` — AgentMessage captions + description copy.

## Validation

| Selector | Result |
| --- | --- |
| `effigy check:svelte-preview` | 0 errors |
| `effigy check:svelte` | 0 errors (preview step included) |
| `effigy catalogue:check` | pass |
| `effigy react:build` | pass |
| `effigy docs:check` | pass |
| `effigy ci:web` | pass |
| `vitest --project svelte-preview` (new tests) | 4 passed |
| `git diff --check origin/main...HEAD` | clean after commit |

## Live review

**Open — operator checkpoint not run in this session.**

Nine Svelte routes (`#components/<slug>`):

- `#components/agent-message`
- `#components/agent-plan`
- `#components/agent-plan-record`
- `#components/agent-question`
- `#components/agent-question-record`
- `#components/agent-subagent`
- `#components/changed-files`
- `#components/tool-call`
- `#components/tool-call-group`

React gallery mirrors the same paths under the React preview dev server.

## Deviations

- None beyond the documented `SceneSpecimen` snippet typing boundary (`as never`).
