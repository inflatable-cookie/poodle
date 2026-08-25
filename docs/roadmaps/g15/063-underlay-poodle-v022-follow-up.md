# g15.063 — Underlay Poodle 0.2.2 follow-up

Status: **complete — Underlay PR 5 merged at `d6fe7b9b`**
Depends on: `g15.057`, `g15.061`, published npm `0.2.2`
Target repository: `/Users/tom/Dev/projects/underlay`
Target base: `750005eb3c2b42df1d4152214abc8178e3cc0dda`
Governing refs: `057-underlay-poodle-v021-adoption.md`,
`061-v022-release-certification.md`, Underlay `AGENTS.md`, adapter and token
bridge contracts

## Outcome

Move Underlay's already-clean public Poodle dependency from exact 0.2.1 to
exact 0.2.2 and prove its adapter/template surface against the corrected
release without exposing Poodle through application-owned APIs.

## Scope

- Pin Underlay's Poodle Svelte dependency to exact `0.2.2`.
- Regenerate `bun.lock` so Poodle core and Svelte resolve exact 0.2.2 from npm.
- Repair only compatibility failures caused by 0.2.2; update Underlay-owned
  integration tests/docs only when installed-package evidence proves them
  stale.

## Out Of Scope

- Do not publish or version Underlay.
- Do not alter Underlay's public template API, bypass its adapters, reintroduce
  sibling Poodle links, or teach applications about Poodle.
- Do not edit Poodle or add a compatibility alias.

## Acceptance

- Underlay resolves Poodle Svelte/core 0.2.2 from the public registry.
- No active manifest or lock entry uses Poodle 0.2.1 or a sibling path.
- The dependency and lock diff contains no unrelated upgrade.
- Clean install, focused adapter/template checks, package checks, docs checks,
  and the repository's broad headless validation pass.

## Validation

- Run the repository's cheap health/orientation surface, then `effigy validate`.
- Run any narrower Poodle adapter/template selector identified by
  `effigy tasks` when `validate` does not already cover it.
- Run `git diff --check` and inspect the full `bun.lock` diff.

## Stop Conditions

- Adoption needs a public Underlay template/adapter contract decision.
- Lock regeneration materially changes unrelated packages.
- Validation exposes a Poodle release defect or requires browser focus.

## Evidence And Continuation

Record registry resolution, changed files, compatibility edits, and exact
validation in the Underlay PR. Do not merge. Once this PR lands, the
orchestrator may compile the Underlay-dependent product cards.

## Closeout

Underlay PR [#5](https://github.com/inflatable-cookie/underlay/pull/5)
merged on 2026-08-24 at
`d6fe7b9bac5e43002b458731aa4b6a0641ca89cd`. The root dependency and
`bun.lock` resolve Poodle core/Svelte 0.2.2 from npm with registry integrity
matching the published packages. No adapter, template, API, or source migration
was required and no unrelated lock entry changed.

Orchestrator validation passed a frozen Bun install, `effigy validate` (770
unit tests and 49 component tests), Svelte diagnostics, TypeScript, docs, and
Northstar checks. `git diff --check` was clean. The canonical review verdict is
[recorded on PR 5](https://github.com/inflatable-cookie/underlay/pull/5#issuecomment-5400712847).
