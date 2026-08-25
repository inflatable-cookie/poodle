# g16.015 — Compli Me Underlay 0.9.2 and Poodle 0.2.2 adoption

Status: **worker in flight — coupled Underlay/Poodle lane**
Depends on: `g16.007`, `g16.009`, `g16.013`, Underlay tag `v0.9.2`
Target repository: `/Users/tom/Dev/projects/compli-me`
Target base: `ee27d84964c61d406f90a5b9f8d4ed96e059d5b7`
Governing refs: `001-consumer-adoption-inventory.md`,
`013-underlay-reference-poodle-v022-adoption.md`, Compli Me root/admin/front/UI
`AGENTS.md`, and Compli Me working rules

## Outcome

Move every active Compli Me Underlay dependency from sibling paths to tag
`v0.9.2`, while moving Admin and Front from Poodle 0.1.x plus sibling overrides
to public 0.2.2. Keep the Underlay-owned application shape unchanged.

## Scope

- Pin existing direct Admin and Front core/Svelte dependencies to exact `0.2.2`.
- Remove committed Poodle core/Svelte overrides.
- Move Admin, Front, UI, and API-client Underlay dependencies to
  `git+ssh://git@github.com/inflatable-cookie/underlay.git#v0.9.2`.
- Move every active API Underlay crate from sibling paths to
  `ssh://git@github.com/inflatable-cookie/underlay.git`, tag `v0.9.2`, retaining
  the existing feature set.
- Regenerate all Bun and Rust locks narrowly; inspect every lock for
  mechanically propagated metadata.
- Repair only Compli Me-owned compatibility failures caused by the bump.

## Out Of Scope

- Do not edit Underlay/Poodle, change app APIs, deployment, containers, or
  templates beyond dependency compatibility repairs.
- Do not add app exceptions, aliases, or new direct dependencies without a
  demonstrated package requirement.
- Do not launch the live stack.

## Acceptance

- Admin and Front resolve public Poodle 0.2.2 with registry integrity.
- Every active web and Rust Underlay dependency resolves tag `v0.9.2`
  (`ddba26400f480638829917cf72eecc62be4b978d`), with no sibling Underlay path.
- No active old Poodle version, sibling Poodle source, or committed Poodle
  override remains.
- Underlay/Poodle lock churn is bounded and unrelated dependency versions stay
  unchanged.
- Admin, Front, UI, root validation, and broad QA are green or a reproduced
  pre-existing baseline is separated from the adoption result.

## Validation

- Use the Effigy-owned prepare/install path; inspect all web/Rust manifests and
  every Bun/Cargo lock.
- Run `effigy admin/validate`, `effigy front/validate`, `effigy ui/validate`,
  `effigy api/validate`, `effigy validate`, and `effigy qa`.
- Run `git diff --check`.

## Stop Conditions

- Adoption needs an Underlay/template/public API decision.
- Install resolves duplicate or local Poodle.
- Lock churn is unrelated or validation requires an application exception.
- Evidence requires the visible stack.

## Evidence And Continuation

Record the exact Underlay tag/commit, Poodle registry identities, lock review,
compatibility edits, and validation in the Compli Me PR. Do not merge.
Independent of `014`, `016`-`022`, and `025`.
