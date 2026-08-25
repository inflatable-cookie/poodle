# g16.018 — Songsprout Poodle 0.2.2 adoption

Status: **ready — independent Underlay-product lane**
Depends on: `g16.007`, `g16.009`, `g16.013`
Target repository: `/Users/tom/Dev/projects/songsprout`
Target base: `b031c1a3c0dc197b1b082b706a4a134d7e5174ad`
Governing refs: `001-consumer-adoption-inventory.md`,
`013-underlay-reference-poodle-v022-adoption.md`, Songsprout root/Bloom/Greenhouse
`AGENTS.md`, and Trellis authority

## Outcome

Move Bloom and Greenhouse from Poodle 0.1.x plus sibling overrides to public
0.2.2 while preserving Trellis-owned workspace and Underlay boundaries.

## Scope

- Pin Bloom and Greenhouse core/Svelte dependencies to exact `0.2.2`.
- Remove their committed Poodle overrides; retain Underlay sources.
- Regenerate Bloom and Greenhouse locks narrowly; inspect other JS locks for
  mechanically propagated metadata only.
- Repair only Songsprout-owned compatibility failures caused by the bump.

## Out Of Scope

- Do not edit Underlay/Poodle, Trellis contracts, Nursery, APIs, or product
  behavior.
- Do not add aliases, app exceptions, or unrelated dependency upgrades.
- Do not launch the live stack.

## Acceptance

- Bloom and Greenhouse resolve public core/Svelte 0.2.2 with one identity.
- No active old Poodle version, sibling Poodle source, or override remains.
- Underlay integration stays unchanged; lock churn is bounded.
- Both apps and the root workspace validate or a baseline is reproduced.

## Validation

- Use the Effigy-owned prepare/install path; inspect manifests and JS locks.
- Run `effigy bloom/validate`, `effigy greenhouse/validate`,
  `effigy validate`, and `effigy qa`.
- Run `git diff --check`.

## Stop Conditions

- Adoption needs an Underlay/Trellis/public API decision.
- Install resolves duplicate or local Poodle.
- Lock churn is unrelated or validation needs an app exception.
- Evidence requires a visible runtime.

## Evidence And Continuation

Record registry identities, retained Underlay links, lock review, compatibility
edits, and validation in the Songsprout PR. Do not merge. Independent of the
other product cards.
