# g16.016 — Composer Poodle 0.2.2 adoption

Status: **ready — independent Underlay-product lane**
Depends on: `g16.007`, `g16.009`, `g16.013`
Target repository: `/Users/tom/Dev/projects/composer`
Target base: `29a32c17d4a5e6f75da311c09776db08f118244a`
Governing refs: `001-consumer-adoption-inventory.md`,
`013-underlay-reference-poodle-v022-adoption.md`, Composer root/admin/front
`AGENTS.md`, and Composer workspace authority

## Outcome

Move Composer Admin and Front from Poodle 0.1.x plus sibling overrides to
public 0.2.2 while preserving current Underlay composition.

## Scope

- Pin every existing direct Admin/Front Poodle dependency to exact `0.2.2`.
- Remove committed core/Svelte Poodle overrides. Do not add an unused direct
  Svelte dependency merely because the override existed.
- Regenerate Admin and Front locks narrowly; prove transitive Svelte converges
  on 0.2.2 through current Underlay packages.
- Repair only Composer-owned compatibility failures caused by the bump.

## Out Of Scope

- Do not edit Underlay/Poodle, APIs, containers, or product behavior.
- Do not add compatibility aliases, app exceptions, or unrelated dependency
  upgrades.
- Do not launch the live stack.

## Acceptance

- All active Composer Poodle identities resolve from the registry at 0.2.2.
- No old Poodle version, sibling Poodle source, or committed override remains.
- Existing Underlay links remain; both web locks have bounded diffs.
- Admin/Front and root validation pass or a reproduced baseline is isolated.

## Validation

- Use Composer's Effigy-owned install path; inspect manifests and both web locks.
- Run `effigy composer-admin/validate`, `effigy composer-front/validate`,
  `effigy validate`, and `effigy qa`.
- Run `git diff --check`.

## Stop Conditions

- Adoption needs an Underlay/template/public API decision.
- Install resolves duplicate or local Poodle.
- Lock churn is unrelated or validation needs an app exception.
- Evidence requires a visible runtime.

## Evidence And Continuation

Record exact identities, transitive peer convergence, lock review, compatibility
edits, and validation in the Composer PR. Do not merge. Independent of the
other product cards.
