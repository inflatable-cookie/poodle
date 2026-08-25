# g15.057 — Underlay Poodle 0.2.1 Adoption

Status: **complete — Underlay PR 4 merged at `750005eb`**
Depends on: `g15.055`, published npm `0.2.1`
Target repository: `/Users/tom/Dev/projects/underlay`
Governing refs: `README.md`, `../../../README.md`,
`../../logs/2026-08/20260823-g15-013-v021-release-certification.md`,
Underlay `AGENTS.md` and its Poodle adapter/token-bridge contracts

## Outcome

Make Underlay consume the released Poodle Svelte package from npm and prove its
adapter/template surface without leaking Poodle into application-owned APIs.

## Scope

- Pin Underlay's Poodle Svelte dependency to exact `0.2.1`.
- Remove committed Poodle core/Svelte `file:` overrides.
- Regenerate the Bun lock against the public registry without unrelated
  upgrades.
- Repair only compatibility issues caused by adopting Poodle `0.2.1`.
- Update Underlay-owned integration docs/tests if the installed-package path
  exposes stale guidance.

## Out Of Scope

- Do not publish or version Underlay.
- Do not widen Underlay's public template API, bypass its adapters, or teach
  application code about Poodle.
- Do not edit Poodle or add a compatibility alias.

## Acceptance

- Underlay resolves Poodle Svelte/core transitively from npm at `0.2.1`.
- No active manifest or lock entry resolves Poodle through a sibling
  `../poodle` path.
- A clean install, focused adapter/template checks, package build, docs checks,
  and Underlay's relevant broad headless QA pass.

## Stop Conditions

- Adoption requires a public Underlay template or adapter contract decision.
- Lock regeneration changes unrelated registry packages materially.
- Validation exposes a Poodle release defect or requires browser focus.

## Evidence

Record the public registry resolution, changed files, exact validation, and any
bounded migration in the Underlay PR. Do not merge.

## Closeout

Underlay PR [#4](https://github.com/inflatable-cookie/underlay/pull/4)
merged on 2026-08-23 at `750005eb`. The root dependency is exact `0.2.1`,
both committed sibling overrides are gone, and the lock resolves Poodle core
and Svelte `0.2.1` from npm. No adapter or template migration was required.

Orchestrator validation passed `effigy validate`: 125 unit files / 770 tests,
12 component files / 49 tests, Svelte and TypeScript checks, package exports,
guardrails, and Poodle prop-name checks. `git diff --check` was clean.
