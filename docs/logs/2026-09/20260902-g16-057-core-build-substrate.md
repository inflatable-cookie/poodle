# g16.057 — Core Build Substrate

Status: complete — awaiting orchestrator review
Date: 2026-09-02
Card: `docs/roadmaps/g16/057-core-build-substrate.md`
Handoff: `docs/handoffs/20260902-102700-g16-057-core-build-substrate.md`
Governing refs: `docs/architecture/014-compiled-web-package-distribution.md`,
`docs/specs/070-compiled-web-distribution-contract.md`
Branch: `feature/g16-057-core-build-substrate`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-057-core-build-substrate`
Base: `origin/main` at `595bec72825a9b830edb2b46f82b4ece049f8e1b`
Planning base `b515f005a40b40005528a44933e49f4fd1c446c7` is an ancestor.

## Outcome

Core now builds through a repo-owned driver around Vite library mode and
separate `emitDeclarationOnly` declaration emit. Clean staging, explicit
sorted entries, unhashed public names, `chunks/[name].js`, exact CSS/token
copies, compiled icon modules, copied `poodle-icons` CLI, and a deterministic
`dist/.poodle-build.json` are in place. Core `package.json` exports, `files`,
`bin.poodle-icons`, and `sideEffects` point at `dist`. No Svelte or React
compilation, no `test:web-pack-install` edits, no version or release mutation.

## Driver

`scripts/web-distribution/` is the reusable substrate. Core fills a
`PackageBuildSpec`; shells can reuse staging, Vite library emit, declaration
emit, asset copy, receipt, and audit without inheriting core semantics.
`core:build` runs the core spec. Declaration emit uses
`packages/core/tsconfig.build.json` (`declarationMap: false`). The source
`noUncheckedIndexedAccess` flag stays on the check tsconfig; the emit config
is `strict` without that extra flag because the existing core tree is not
emit-clean under it.

## Evidence

- Inventories: 167 CSS files, 108 icon modules, 22 token CSS files, and the
  frozen JS export map agree with spec 070 and disk.
- Two clean builds: identical output lists, file hashes, and receipt bytes
  (542ms for the pair in `test:core-build`).
- Bundler and NodeNext: a no-paths consumer resolves `.`, `./tokens/runtime`,
  and `./icons/x`.
- Receipt: schemaVersion 1, sorted keys, locked svelte/typescript/vite,
  `lanes: ["single"]`, `cssPolicy: core-owned`, `markdownPolicy: none`,
  `sourceMaps: false`, no timestamp or absolute path.
- `test:core` 1221/0; `effigy docs:check` pass; `git diff --check` pass.

## Oracle

| Row | Plant | Result |
| --- | --- | --- |
| Stable public names | `dist/index-a1b2c3d4.js` | `hashed filename is forbidden: dist/index-a1b2c3d4.js` |
| CSS inventory complete | remove `dist/styles/button.css` | `missing staged public file(s): dist/styles/button.css` |
| Output is source-free | `dist/planted.ts` | `raw source is forbidden in staging: dist/planted.ts` |
| Receipt is reproducible | `timestamp` key in receipt JSON | `receipt contains a timestamp` |
| Card stays core-only | `import { onMount } from "svelte"` in `dist/index.js` | `forbidden parser or shell module entered dist/index.js` |

All plants restored. `AUDIT_OK` after restore. Focused tests repeat the same
plants against disposable fixtures.

## Validation

- `bun test scripts/web-distribution/driver.test.ts scripts/web-distribution/core-build.test.ts` — 12/0
- `bun run --cwd packages/core test` — 1221/0
- `effigy docs:check` — pass (runs `core:build` first)
- `git diff --check` — pass

No windowed, native-visual, pack-install, or release selector.

## Known follow-up

`test:web-pack-install` still special-cases only `files: ["src"]` when checking
directory archive members. `files: ["dist"]` packs `package/dist/**` and the
receipt, but not a bare `package/dist` entry. g16.059 owns that harness.
`health`/`docs:lint` need a prior `core:build` on a fresh checkout.

## Diff scope

Owned surfaces only: `scripts/web-distribution/**`, core manifest/tsconfig
emit, `core:build` / `test:core-build` wiring, this card, this log, papercuts.
No Svelte/React component builds, no `test:web-pack-install`, no versions,
workflows, or global roadmap front doors.

## Continuation

Accepted merge unlocks `g16.058`. `g16.054` remains blocked.
