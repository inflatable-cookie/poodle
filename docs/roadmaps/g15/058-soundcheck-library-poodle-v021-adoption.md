# g15.058 — Soundcheck Library Poodle 0.2.1 Adoption

Status: **complete — Soundcheck Library PR 5 merged at `a720f22`**
Depends on: `g15.055`, published npm `0.2.1`
Target repository: `/Users/tom/Dev/projects/soundcheck-library`
Governing refs: `README.md`, `../../../README.md`,
`../../logs/2026-08/20260823-g15-013-v021-release-certification.md`,
Soundcheck Library package READMEs and public peer contracts

## Outcome

Make Soundcheck Library develop and test against released Poodle `0.2.1`
while advertising the compatible `0.2.x` peer line to its consumers.

## Scope

- Pin the workspace/root Poodle Svelte dependency to exact `0.2.1`.
- Move both published library peer requirements from `^0.1.0` to
  `^0.2.1`.
- Remove committed Poodle core/Svelte `file:` overrides.
- Regenerate npm/Bun locks using the repository's authoritative package
  manager without unrelated upgrades.
- Repair only compatibility issues caused by Poodle `0.2.1`.
- Update package docs/tests if their stated peer or import surface is stale.

## Out Of Scope

- Do not publish or version Soundcheck Library.
- Do not expand its public component surface or add Poodle compatibility
  wrappers.
- Do not edit Poodle.

## Acceptance

- The root workspace installs exact Poodle Svelte/core `0.2.1` from npm.
- Both library peers are `^0.2.1`.
- No active manifest or lock resolution points to a sibling Poodle checkout.
- Clean install, type checks, package tests/builds, and the repository's broad
  headless QA pass.

## Stop Conditions

- The peer change needs a broader Soundcheck Library release-policy decision.
- Lock regeneration changes unrelated packages materially.
- Adoption exposes a Poodle release defect rather than a bounded consumer
  migration.

## Evidence

Record resolved versions/sources, changed manifests/locks, exact validation,
and any bounded migration in the Soundcheck Library PR. Do not merge.

## Closeout

Soundcheck Library PR
[#5](https://github.com/inflatable-cookie/soundcheck-library/pull/5)
merged on 2026-08-23 at `a720f22`. The workspace installs exact Poodle
Svelte `0.2.1`, both library peers advertise `^0.2.1`, committed sibling
overrides are gone, and both locks resolve Poodle core/Svelte from npm.

No package compatibility migration was required. Orchestrator review restored
ten unrelated platform `libc` constraints removed by npm lock regeneration;
`npm ci` accepted the corrected lock. The repository's 18 type errors and one
failing test were reproduced on `main` by the worker and are not adoption
regressions; 179 tests pass.
