# g15.067 — Underlay Reference Poodle 0.2.2 adoption

Status: **complete — Underlay Reference PR 1 merged at `f5ea7d72`**
Depends on: `g15.061`, `g15.063`, published npm `0.2.2`
Target repository: `/Users/tom/Dev/projects/underlay-reference`
Target base: `8885661e10813bb9d8a3f6782c87a840b26bd0be`
Governing refs: `055-consumer-adoption-inventory.md`,
`063-underlay-poodle-v022-follow-up.md`, Underlay Reference `AGENTS.md`,
reference implementation notes, and Underlay adapter/template contracts

## Outcome

Move the canonical Underlay reference estate from Poodle 0.1.0 plus committed
sibling overrides to exact public 0.2.2. Prove the UI package, admin app, and
front app all use the registry release while preserving Underlay-owned public
patterns and local framework development.

## Scope

- Pin every direct Poodle core/Svelte application dependency in `acme-admin`,
  `acme-front`, and `acme-ui` to exact `0.2.2`.
- Remove the committed Poodle core/Svelte `file:` overrides from those three
  packages. Keep local Underlay and reference-package links unchanged.
- Regenerate the four package Bun locks narrowly; Poodle-free `acme-client`
  may change only when the workspace's supported install flow proves it is
  mechanically required.
- Repair only reference-app compatibility failures caused by the Poodle bump,
  preserving reusable Underlay patterns rather than adding app exceptions.

## Out Of Scope

- Do not publish packages, replace the local Underlay source, edit Underlay or
  Poodle, or change the template/public API.
- Do not refactor reference structure, bootstrap behavior, containers, or
  product examples beyond a bounded 0.2.2 compile repair.
- Do not add compatibility aliases or use a sibling Poodle checkout as proof.

## Acceptance

- All active reference packages resolve public Poodle core/Svelte 0.2.2 with
  registry integrity and no 0.1.0, 0.2.1, or sibling Poodle path.
- The three package manifests contain no committed Poodle override; local
  Underlay links remain intact.
- Lockfile changes are limited to Poodle resolution and mechanically necessary
  metadata.
- `effigy validate`, package checks, docs checks, and the broad headless
  reference board pass without application-specific template exceptions.

## Validation

- Use the repository's Effigy-owned install/validation flow; do not assume a
  host-side raw install changes the live container runtime.
- Inspect every manifest and all four lockfiles, then run `effigy validate`,
  `effigy qa`, and `effigy qa:docs`. Add package-local check selectors only
  where the aggregate board does not cover them.
- Run `git diff --check`.

## Stop Conditions

- Adoption requires an Underlay adapter/template or public API decision.
- The supported install flow cannot prove registry Poodle independently of the
  mounted sibling checkout.
- Lock regeneration materially changes unrelated dependencies or validation
  requires an app-specific exception.
- Validation exposes a Poodle 0.2.2 release defect.

## Evidence And Continuation

Record changed manifests/locks, registry versions/integrities, retained local
Underlay sources, compatibility edits, and exact validation in the Underlay
Reference PR. Do not merge. Once this lane lands, the canonical Underlay
consumer shape is complete and the remaining Underlay-product cards can reuse
its measured migration pattern.

## Closeout

Underlay Reference PR [#1](https://github.com/inflatable-cookie/underlay-reference/pull/1)
merged on 2026-08-25 at
`f5ea7d72eee278e8838ba16f8f43eb2b662406d0`. Admin, Front, and UI now pin
public Poodle core/Svelte 0.2.2 exactly. All four Bun locks resolve the
published npm integrity values. No old Poodle version, sibling Poodle path, or
committed Poodle override remains. Local Underlay links stay intact.

The worker initially added `passWithNoTests` exceptions to manufacture a green
aggregate board. Review removed them. Package checks, docs QA, and
`git diff --check` pass. `effigy validate` and `effigy qa` now expose the
pre-existing test-routing baseline: Front tests live under `tests/**` while its
Vitest include selects `src/**`; UI has no tests but Effigy auto-selects a
transitive Vitest binary. The gap is recorded in Underlay Reference
`PAPERCUTS.md`, not hidden in application policy. The canonical acceptance
verdict is [recorded on PR 1](https://github.com/inflatable-cookie/underlay-reference/pull/1#issuecomment-5409053744).
