# g16.015 — Compli Me Poodle 0.2.2 adoption

Status: **ready — independent Underlay-product lane**
Depends on: `g16.007`, `g16.009`, `g16.013`
Target repository: `/Users/tom/Dev/projects/compli-me`
Target base: `ee27d84964c61d406f90a5b9f8d4ed96e059d5b7`
Governing refs: `001-consumer-adoption-inventory.md`,
`013-underlay-reference-poodle-v022-adoption.md`, Compli Me root/admin/front/UI
`AGENTS.md`, and Compli Me working rules

## Outcome

Move Compli Me Admin and Front from Poodle 0.1.x plus sibling overrides to
public 0.2.2 without changing its Underlay-owned application shape.

## Scope

- Pin existing direct Admin and Front core/Svelte dependencies to exact `0.2.2`.
- Remove committed Poodle core/Svelte overrides; retain Underlay sources.
- Regenerate affected Bun locks narrowly; inspect every workspace lock for
  mechanically propagated metadata.
- Repair only Compli Me-owned compatibility failures caused by the bump.

## Out Of Scope

- Do not edit Underlay/Poodle, app APIs, deployment, containers, or templates.
- Do not add app exceptions, aliases, or new direct dependencies without a
  demonstrated package requirement.
- Do not launch the live stack.

## Acceptance

- Admin and Front resolve public Poodle 0.2.2 with registry integrity.
- No active old Poodle version, sibling Poodle source, or committed Poodle
  override remains.
- Local Underlay integration stays unchanged; lock churn is bounded.
- Admin, Front, UI, root validation, and broad QA are green or a reproduced
  pre-existing baseline is separated from the adoption result.

## Validation

- Use the Effigy-owned prepare/install path; inspect manifests and all Bun locks.
- Run `effigy admin/validate`, `effigy front/validate`, `effigy ui/validate`,
  `effigy validate`, and `effigy qa`.
- Run `git diff --check`.

## Stop Conditions

- Adoption needs an Underlay/template/public API decision.
- Install resolves duplicate or local Poodle.
- Lock churn is unrelated or validation requires an application exception.
- Evidence requires the visible stack.

## Evidence And Continuation

Record registry identities, retained Underlay links, lock review, compatibility
edits, and exact validation in the Compli Me PR. Do not merge. Independent of
`014` and `016`-`022`.
