# g16.057 — Core Build Substrate

Status: complete — awaiting orchestrator review
Date: 2026-09-02
PR: https://github.com/inflatable-cookie/poodle/pull/161
Card: `docs/roadmaps/g16/057-core-build-substrate.md`
Handoff: `docs/handoffs/20260902-102700-g16-057-core-build-substrate.md`
Governing refs: `docs/architecture/014-compiled-web-package-distribution.md`,
`docs/specs/070-compiled-web-distribution-contract.md`
Branch: `feature/g16-057-core-build-substrate`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-057-core-build-substrate`
Base: `origin/main` at `8377a936ef8bb55e30e44da55fe728e2e5ced429`

## Outcome

Core now builds through a repo-owned driver around Vite library mode and
separate `emitDeclarationOnly` declaration emit. Clean staging, explicit
sorted entries, unhashed public names, `chunks/[name].js`, exact CSS/token
copies, compiled icon modules, copied `poodle-icons` CLI, and a deterministic
`dist/.poodle-build.json` are in place. Core `package.json` exports, `files`,
`bin.poodle-icons`, and `sideEffects` point at `dist`. No Svelte or React
compilation, no versions, no release mutation.

Exact-head review of PR #161 closed five in-bounds findings. `health` and
`test:components` now run `core:build` first. Pack-install directory
membership treats `src`/`dist` as trees and `LICENSE` as a file. Stale token
`.js` siblings are gone; TypeScript is the authority. The staged audit covers
side-effect, dynamic, subpath, and bundled module edges plus Unix/Windows
workspace paths. Public `.d.ts`/`.d.mts` targets are required. `marked` left
the core manifest; root `devDependencies` still resolve the lexer fixture.
g16.059 receipt/consumer/two-pack work stayed out.

The second exact-head review closed three further fail-open edges. Receipt
inputs now include the package manifest and declaration config alongside every
source/asset/Vite input. Absolute paths are rejected by value across emitted
JavaScript, declarations, and parsed receipt JSON without mistaking URLs,
module schemes, protocol-relative URLs, or regex syntax for paths. npm alias
targets cannot hide any forbidden module in any dependency section.

The third exact-head review closed the remaining escaped-literal gap. The
source audit decodes JavaScript string escapes before classification, rejects
one-segment and deeper Unix filesystem values, and keeps only the bounded
core contract's protocol-relative network URLs and exact separator syntax.
Identifier and function names grant no path exception; regex literals remain
outside the quoted-string scan.

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
- Two clean builds: identical output lists, file hashes, and receipt bytes.
- Bundler and NodeNext: a no-paths consumer resolves `.`, `./icons`,
  `./icons/x`, `./icons/build`, `./tokens`, `./tokens/runtime`, `./tokens/css`,
  `./tokens/themes`, `./tokens/metadata`, and `./tokens/units`.
- Receipt inputs include `src/tokens/{index,runtime,units,themes,metadata}.ts`
  and omit the deleted `.js` siblings. Compiled `#region` comments name `.ts`.
- Receipt: schemaVersion 1, sorted keys, locked svelte/typescript/vite,
  `lanes: ["single"]`, `cssPolicy: core-owned`, `markdownPolicy: none`,
  `sourceMaps: false`, no timestamp or absolute path.

## Oracle

| Row | Plant | Result |
| --- | --- | --- |
| Stable public names | `dist/index-a1b2c3d4.js` | hashed filename forbidden |
| CSS inventory complete | remove `dist/styles/button.css` | missing staged public file |
| Output is source-free | `dist/planted.ts` | raw source forbidden |
| Receipt is reproducible | `timestamp` key in receipt JSON | receipt contains a timestamp |
| Card stays core-only | `from "svelte"` in `dist/index.js` | forbidden parser or shell module |
| Repository routes | delete `packages/core/dist`, then `health`; pack `files: ["dist"]` | `core:build` runs first; packed `package/dist/**` accepted; omitted dist tree rejected |
| TypeScript authority | sibling `src/tokens/units.js` next to `.ts` | parallel JavaScript source shadows TypeScript |
| Module-edge audit | `import "svelte"`; `import("react/jsx-runtime")`; bundled stub `marked`; sibling-workspace module; generic Unix roots; `"C:\\Users\\..."` | each fails closed; URLs/module schemes/regex remain valid |
| Public declarations | delete `dist/icons/icons/x.d.ts` | missing staged public file |
| No marked edge | `marked` in core `devDependencies` | package.json devDependencies lists forbidden module marked |
| Complete receipt inputs | omit `package.json` or `tsconfig.build.json` | focused receipt inventory assertion fails |
| Alias-safe dependency audit | `hidden: "npm:react-dom@1.0.0"` in any dependency section | alias target is rejected |
| Escape-aware path audit | `/workspace`, `/tmp`, `/a`, `/assets/app.js`, escaped slashes, or `\u002f` in any string context | decoded single-leading-slash value is rejected; protocol-relative URLs and exact separator strings stay valid |

All plants restored. Focused tests repeat them against disposable fixtures.

## Validation

- `bun test scripts/web-distribution/driver.test.ts scripts/web-distribution/core-build.test.ts test/package-install/archive-membership.test.ts` — 29/0
- Core unit tests — 1,221/0.
- Clean-state `effigy health` — pass after moving the ignored core `dist/` out
  of the worktree; the route rebuilt it before docs lint.
- `effigy test:web-pack-install` — pass, 10 files / 20 tests; packed core
  contained `package/dist/**` and the receipt.
- `effigy ci:web` — pass, including 376 component files / 3,602 tests and the
  current pack-install gate.
- `effigy docs:check` — pass.
- `git diff --check origin/main...HEAD` — pass.

No windowed, native-visual, or release selector. No g16.059 redesign.

## Diff scope

Owned surfaces: `scripts/web-distribution/**`, core manifest/tsconfig emit,
token source authority, `health` / `test:components` / `test:core-build`
wiring, the current pack-install directory-membership check, this card, this
log, papercuts. No Svelte/React component builds, versions, workflows, or
g16.059 certification work.

## Continuation

Accepted merge unlocks `g16.058`. `g16.054` remains blocked.
