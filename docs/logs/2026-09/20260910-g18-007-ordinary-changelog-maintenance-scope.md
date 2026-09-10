# g18.007 — Ordinary changelog maintenance scope

Status: implementation complete — awaiting independent exact-head review
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
