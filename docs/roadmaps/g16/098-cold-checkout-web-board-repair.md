# g16.098 — Cold-Checkout Web Board Repair

Status: complete — merged in PR #203 at `c8636c699`
Type: validation board repair — no component or contract change
Opened: 2026-09-04
Depends on: none. Blocks: `g16.096` (PR #201 re-run) and `g16.097` (release
re-certification)
Governing refs: `vitest.config.ts`, `tasks/effigy.tasks.toml` (`ci:web`,
`docs:check`), `../../architecture/014-compiled-web-package-distribution.md`,
`056-web-distribution-contract.md`–`059-installed-web-distribution-certification.md`
Evidence: ci-web run `33874196422` on PR #201 and release dry-run
`33874116177` on tag `v0.3.0` both fail the same three `react-preview` suites
Dispatch manifest: `../dispatch.md`

## Goal

Make `effigy ci:web` (and therefore `qa` and `effigy release gates`) pass on
a cold checkout. Today the `react-preview` vitest project is the only project
in `vitest.config.ts` without `resolve: { alias: workspaceAliases }`, so
`@inflatable-cookie/poodle-react` resolves through the package `exports` map
to `dist/`, which `ci:web` builds (`react:package`) only after
`test:components` has already run. Every checkout that ever ran a package
build has `dist/` and passes; a fresh CI runner does not.

## Fixed Boundary

- Add `resolve: { alias: workspaceAliases }` to the `react-preview` project
  in `vitest.config.ts` so its tests resolve React source like every other
  project. This is the fix.
- Defence in depth: in `ci:web`, move `svelte:package` and `react:package`
  ahead of `test:components`, matching the order `docs:check` already uses,
  so anything that legitimately inspects `dist/` sees it. Keep
  `test:web-pack-install` after the builds.
- Prove the cold path: a test or script step that removes
  `packages/react/components/dist` and `packages/svelte/components/dist` in a
  temp copy (or runs from a fresh `git worktree add --detach`) and then runs
  the three previously failing suites green. Commit this proof before the
  fix so it bites.
- No workflow edits, no release surfaces, no component changes.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Cold checkout passes | delete both `dist/` trees, run `test:components` | the three `react-preview` suites pass |
| Alias is real | remove the alias again | the same three suites fail with "Failed to resolve import" |
| Order is safe | run `effigy ci:web` from a fresh detached worktree | green end to end |
| No behaviour change | `effigy ci:web` on a warm checkout | same pass counts as before |

## Validation

`effigy ci:web` from a fresh detached worktree with no `dist/`, `effigy
docs:check`, `git diff --check origin/main...HEAD`. Never run release,
workflow, windowed, or native-visual selectors.

## Owned Paths

`vitest.config.ts`, `tasks/effigy.tasks.toml` (`ci:web` sequence only), one
cold-path proof under `test/` or `scripts/web-distribution/`, this card's
execution log under `docs/logs/2026-09/`, root `PAPERCUTS.md` (append only).

Reserved for the coordinator at merge: `docs/roadmaps/g16/README.md`,
`docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`.

## Stop Conditions

Stop and report when the cold path fails for a reason other than the alias
or ordering, or when the fix would require touching package `exports`, the
distribution contract, or `.github/workflows/`. Escalation owner: Chatterbox.

## Continuation

On merge the coordinator rebases PR #201 (`g16.096`) and re-runs its board,
and starts `g16.097` re-certification from the new `main` tip.
