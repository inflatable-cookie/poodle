# g16.020 — Figmatic Poodle 0.2.2 adoption

Status: **complete — PR 17 merged**
Depends on: `g16.007`, `g16.008`, `g16.011`
Target repository: `/Users/tom/Dev/projects/figmatic`
Target base: `775312c9ec9579ce4cac9d627c45cc21bdeb77fd`
Governing refs: `001-consumer-adoption-inventory.md`,
`011-nucleus-poodle-v022-adoption.md`, Figmatic `AGENTS.md`, working rules, and
Studio package authority

## Outcome

Move Figmatic Studio from committed sibling Poodle sources to exact public
0.2.2 while retaining local Longhorn integration.

## Scope

- Replace Studio core/Svelte `file:` dependencies with exact registry `0.2.2`.
- Remove only Poodle overrides; keep Longhorn dependency/override unchanged.
- Regenerate `studio/bun.lock` narrowly and prove one Poodle identity.
- Repair only Figmatic-owned compatibility failures caused by the bump.

## Out Of Scope

- Do not edit Longhorn/Poodle, native componentization architecture, harnesses,
  or product behavior unless a bounded compile repair requires it.
- Do not add aliases or unrelated dependency upgrades.
- Do not launch visible Studio/native demos.

## Acceptance

- Studio resolves published core/Svelte 0.2.2 with registry integrity.
- Local Longhorn peer converges on the same Svelte identity.
- No active sibling Poodle source or old version remains.
- Studio lock is bounded; build/check/QA stay green or baseline is reproduced.

## Validation

- Use Figmatic's Effigy-owned install path; inspect the Studio graph and lock.
- Run `effigy check:studio`, `effigy build`, `effigy test:native-componentize`,
  and `effigy qa` where all remain headless.
- Run `git diff --check`.

## Stop Conditions

- Adoption needs a Figmatic/Longhorn/native/public API decision.
- Install resolves duplicate or local Poodle.
- Lock churn affects unrelated workspaces.
- Evidence needs a visible app or demo.

## Evidence And Continuation

Record exact identities, Longhorn peer convergence, lock review, compatibility
edits, and validation in the Figmatic PR. Do not merge. Independent of the
other product cards.

## Review Result

PR [#17](https://github.com/inflatable-cookie/figmatic/pull/17) merged at
`a6286e88`. Studio resolves one registry Poodle 0.2.2 identity, the local
Longhorn peer converges, lock churn is bounded, and the requested headless
validation board passes.
