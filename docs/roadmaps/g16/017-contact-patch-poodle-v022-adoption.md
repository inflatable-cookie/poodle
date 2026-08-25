# g16.017 — Contact Patch Poodle 0.2.2 adoption

Status: **ready — independent Underlay-product lane**
Depends on: `g16.007`, `g16.009`, `g16.013`
Target repository: `/Users/tom/Dev/projects/contact-patch`
Target base: `a6d2316f5c5248c1d27f7f595bfbf2b0de91127e`
Governing refs: `001-consumer-adoption-inventory.md`,
`013-underlay-reference-poodle-v022-adoption.md`, Contact Patch root/admin/front/UI
`AGENTS.md`, and Contact Patch workspace authority

## Outcome

Move Contact Patch Admin and Front from Poodle 0.1.x plus sibling overrides to
public 0.2.2 without changing its Underlay-owned UI architecture.

## Scope

- Pin every existing direct Admin/Front Poodle dependency to exact `0.2.2`.
- Remove committed Poodle core/Svelte overrides; retain Underlay integration.
- Regenerate affected web locks narrowly and prove one Poodle identity.
- Repair only Contact Patch-owned compatibility failures caused by the bump.

## Out Of Scope

- Do not edit Underlay/Poodle, book APIs, UI templates, or product behavior.
- Do not add unused direct dependencies, compatibility aliases, or app-specific
  validation exceptions.
- Do not launch the live stack.

## Acceptance

- Active Admin/Front Poodle dependencies resolve registry 0.2.2.
- No old Poodle version, sibling Poodle source, or committed override remains.
- Underlay sources stay unchanged; lock changes are bounded.
- Admin, Front, UI, root validation, and QA pass or a baseline is reproduced.

## Validation

- Use the Effigy-owned prepare/install flow; inspect all relevant Bun locks.
- Run `effigy cp-admin/validate`, `effigy cp-front/validate`,
  `effigy cp-ui/validate`, `effigy validate`, and `effigy qa`.
- Run `git diff --check`.

## Stop Conditions

- Adoption needs an Underlay/template/public API decision.
- Install resolves duplicate or local Poodle.
- Lock churn is unrelated or validation needs an app exception.
- Evidence requires a visible runtime.

## Evidence And Continuation

Record exact identities, lock review, compatibility edits, retained Underlay
sources, and validation in the Contact Patch PR. Do not merge. Independent of
the other product cards.
