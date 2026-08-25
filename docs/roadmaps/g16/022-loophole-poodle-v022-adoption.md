# g16.022 — Loophole Poodle 0.2.2 adoption

Status: **complete — PR 8 merged**
Depends on: `g16.007`, `g16.008`, `g16.012`
Target repository: `/Users/tom/Dev/projects/loophole`
Target base: `6f770f59c3a83a0da8ccd9ebcc03f15e7c5b88a2`
Governing refs: `001-consumer-adoption-inventory.md`,
`012-soundcheck-poodle-v022-adoption.md`, Loophole `AGENTS.md`, working rules,
and strict product guardrails

## Outcome

Move Loophole Desktop from committed sibling Poodle sources to exact public
0.2.2 while preserving local Longhorn integration and renderer boundaries.

## Scope

- Replace desktop core/Svelte `file:` dependencies with exact registry `0.2.2`.
- Remove only Poodle overrides; retain Longhorn dependency/override.
- Regenerate `apps/desktop/bun.lock` narrowly and prove one Poodle identity.
- Repair only Loophole-owned compatibility failures caused by the bump.

## Out Of Scope

- Do not edit Longhorn/Poodle, audio engine, renderer architecture, persistence,
  or product behavior beyond bounded compile repairs.
- Never run `cargo fmt --all`; use Loophole's scoped formatter task.
- Do not launch the visible desktop app.

## Acceptance

- Desktop resolves published core/Svelte 0.2.2 with registry integrity.
- Longhorn adapter peer converges on the same Svelte instance.
- No sibling Poodle path or old version remains; lock churn is bounded.
- Desktop typecheck, renderer build/tests, workspace tests, and QA are green or
  a reproduced baseline is separated.

## Validation

- Use Loophole's Effigy-owned install path; inspect the graph and full lock diff.
- Run `cd apps/desktop && bun run check`, `effigy build:renderer`,
  `effigy test:renderer`, `effigy test:workspace`, and `effigy qa`.
- Run `git diff --check`. Do not run dev/demo/visible selectors.

## Stop Conditions

- Adoption needs a Loophole/Longhorn/renderer/public API decision.
- Install resolves duplicate or local Poodle.
- Lock churn is unrelated or validation requires a compatibility shim.
- Evidence needs a visible app.

## Evidence And Continuation

Record exact identities, peer convergence, lock review, compatibility edits,
and validation in the Loophole PR. Do not merge. Independent of the other
product cards.

## Review Result

PR [#8](https://github.com/inflatable-cookie/loophole/pull/8) merged at
`92e01577`. The desktop graph resolves one registry Poodle 0.2.2 identity and
the Longhorn peer converges. Build, workspace tests, and QA pass. The single
renderer-test failure reproduces unchanged on `main`.
