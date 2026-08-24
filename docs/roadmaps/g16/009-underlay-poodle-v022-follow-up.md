# g16.009 — Underlay Poodle 0.2.2 follow-up

Status: **ready — independent foundation lane**
Depends on: `g16.003`, `g16.007`, published npm `0.2.2`
Target repository: `/Users/tom/Dev/projects/underlay`
Target base: `750005eb3c2b42df1d4152214abc8178e3cc0dda`
Governing refs: `003-underlay-poodle-v021-adoption.md`,
`007-v022-release-certification.md`, Underlay `AGENTS.md`, adapter and token
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
