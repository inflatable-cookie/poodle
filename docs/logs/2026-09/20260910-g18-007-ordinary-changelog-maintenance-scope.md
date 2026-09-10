# g18.007 — Ordinary changelog maintenance scope

Status: merged
Merge: `ef2e46bb949a766e844e48f071119c9576c6f723` (PR #240) on 2026-09-10
Date: 2026-09-10
Branch: `ns-e914c1a0-63cc-45f5-9b7f-5bbad4bb0f03`
Card: `docs/roadmaps/g18/007-ordinary-changelog-maintenance-scope.md`
Handoff: `docs/handoffs/20260910-g18-007-ordinary-changelog-maintenance-scope.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/specs/044-deprecation-change-control-and-release-channel-operations.md`,
`test/package-install/README.md`

## Outcome

Ordinary installed-package CI now admits one narrow changelog-maintenance
range: `CHANGELOG.md` plus one dated Northstar execution log, only when both
changelog revisions have identical release semantics and an empty Unreleased
section. Strict and historical candidate modes are unchanged.

## What changed

- `test/package-install/scope.ts` parses a fail-closed release inventory of
  versions, dates, reference links, and normalized entry payloads. It ignores
  structural section headings, wrapping, list syntax, and the bounded labels
  used when prose becomes a Keep a Changelog entry.
- The ordinary release-path rejection is removed only for a two-file
  maintenance range with equal inventories. Missing or extra logs, mixed
  source/docs ranges, malformed changelogs, non-empty Unreleased content, and
  any release-semantic mutation remain red.
- Focused tests bind the retained normalization as a generic local fixture and
  plant version, date, link, entry, Unreleased, removed-release, mixed-range,
  missing-log, and ambiguous-shape counterexamples.
- The web-preview falsification harness plants semantic-entry and mixed-range
  changelog attacks. Package-install documentation records the boundary.

## Validation

- Direct retained-range proof: `70c1b8c8..40f9e0a3` passes ordinary scope
  with exactly `CHANGELOG.md` and the g18.005 execution log.
- `effigy test:core-build`: 66 pass, 0 fail, including 30 focused scope
  tests and byte-exact retained before/after fixtures.
- `effigy test:web-pack-install`: exit 0 at committed source
  `4daa36f4d949807d297ea9892d136ca415e27c07`; 22 packed-consumer tests pass
  and both new changelog falsification receipts fail as required.
- `effigy ci:web`: 3819 component tests, 66 core/distribution tests, 22
  packed-consumer tests, and the remaining headless web board pass.
- `effigy docs:check`: exit 0. `effigy doctor`: 18 ok, 3 warning-only scan
  findings, 0 errors.
- `git diff --check`: clean.

## Boundaries

No changelog, workflow, package version, release candidate, registry, publish,
tag, product, Desktop, or retained g18.005 lane was edited. After this PR
merges, Queue should retry validation on retained g18.005 PR #238.

## Closeout

- Merge performed by the plugin as
  `ef2e46bb949a766e844e48f071119c9576c6f723` on 2026-09-10 (PR #240),
  with parents `d34bb322f` (main) and `1ee1f4c3c` (reviewed head).
- Accepted review: independent exact-head `ready_to_merge` approval of
  head `1ee1f4c3c8ad2c0b6acc0942fded0e74bfe7aaa7` by betterthanclay
  ([comment #5626126027](https://github.com/inflatable-cookie/poodle/pull/240#issuecomment-5626126027)).
  No merge blockers remained.
- Reviewed-head validation (reviewer re-ran at the exact head, tree left
  clean): `test:core-build` 66 pass including 30 focused scope tests;
  `test:web-pack-install` 22/22 packed-consumer tests with both new
  changelog falsification receipts rejecting as required; `ci:web` 3819
  component tests plus the full headless web board; `docs:check` clean;
  `doctor` 18 ok, 3 warning-only scans, 0 errors; `git diff --check` clean.
  PR CI at the reviewed head: `rust` and `web` pass.
- Non-blocking reviewer notes (deferred, no acceptance impact): entry
  section membership is pooled per release, so same-release entry reorder or
  section moves are admitted; swapping among the three closed canonical
  labels over an identical payload is treated as syntax.
- Deferred: retry validation on retained g18.005 PR #238. No release, tag,
  publish, or Desktop work starts from this task.
