# g16.014 — Acowtancy Poodle 0.2.2 adoption

Status: **ready — independent Underlay-product lane**
Depends on: `g16.007`, `g16.009`, `g16.013`
Target repository: `/Users/tom/Dev/projects/acowtancy`
Target base: `a535969bab2a1b919d382e3e129d8eb95043b2b8`
Governing refs: `001-consumer-adoption-inventory.md`,
`013-underlay-reference-poodle-v022-adoption.md`, Acowtancy root and scoped
`AGENTS.md`, Acowtancy working rules, and Froyo package contract

## Outcome

Move Cream, Dairy, and Froyo from Poodle 0.1.x to public 0.2.2 while preserving
the monorepo's Underlay bundle, shared-package boundaries, and single root lock.

## Scope

- Pin Cream and Dairy core/Svelte dependencies to exact `0.2.2`.
- Move Froyo's Svelte peer from `^0.1.0` to `^0.2.2`.
- Regenerate root `bun.lock` narrowly and prove one Poodle runtime identity.
- Repair only Acowtancy-owned compatibility failures caused by the bump.

## Out Of Scope

- Do not change Underlay, Poodle, API behavior, Farmyard, or product semantics.
- Do not add sibling Poodle overrides, compatibility aliases, or package-local
  lockfiles.
- Do not launch the live stack or another visible application.

## Acceptance

- Cream and Dairy resolve exact registry core/Svelte 0.2.2.
- Froyo advertises `^0.2.2`; all consumers converge on one Svelte instance.
- No active old Poodle version or sibling Poodle source remains.
- Root lock churn is limited to Poodle and mechanically required peer metadata.

## Validation

- Use `effigy workspace:js:prepare`; inspect the dependency graph and full lock
  diff.
- Run `effigy cream/validate`, `effigy dairy/validate`, `effigy froyo/check`,
  `effigy validate`, and `effigy qa`.
- Run `git diff --check`.

## Stop Conditions

- Adoption needs an Underlay, Froyo, public API, or product decision.
- Install resolves duplicate Poodle versions or a local Poodle checkout.
- Lock regeneration changes unrelated packages.
- Required evidence needs the live or visible app stack.

## Evidence And Continuation

Record exact versions, peer convergence, lock review, compatibility edits, and
validation in the Acowtancy PR. Do not merge. This lane is independent of
`015`-`022`.
