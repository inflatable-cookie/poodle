# g16.017 — Contact Patch Underlay 0.9.2 and Poodle 0.2.2 adoption

Status: **ready — independent Underlay-product lane**
Depends on: `g16.007`, `g16.009`, `g16.013`, Underlay tag `v0.9.2`
Target repository: `/Users/tom/Dev/projects/contact-patch`
Target base: `a6d2316f5c5248c1d27f7f595bfbf2b0de91127e`
Governing refs: `001-consumer-adoption-inventory.md`,
`013-underlay-reference-poodle-v022-adoption.md`, Contact Patch root/admin/front/UI
`AGENTS.md`, and Contact Patch workspace authority

## Outcome

Move Contact Patch's active web and Rust Underlay dependencies from sibling
paths to tag `v0.9.2`, while moving Admin and Front from Poodle 0.1.x plus
sibling overrides to public 0.2.2. Keep its UI architecture unchanged.

## Scope

- Pin every existing direct Admin/Front Poodle dependency to exact `0.2.2`.
- Remove committed Poodle core/Svelte overrides.
- Move Admin, Front, UI, and client Underlay dependencies to
  `git+ssh://git@github.com/inflatable-cookie/underlay.git#v0.9.2`.
- Move every active Contact Patch API Underlay crate from sibling paths to
  `ssh://git@github.com/inflatable-cookie/underlay.git`, tag `v0.9.2`, retaining
  existing features.
- Regenerate web and Rust locks narrowly and prove one Poodle and Underlay
  identity.
- Repair only Contact Patch-owned compatibility failures caused by the bump.

## Out Of Scope

- Do not edit Underlay/Poodle, change book APIs, UI templates, or product
  behavior beyond dependency compatibility repairs.
- Do not add unused direct dependencies, compatibility aliases, or app-specific
  validation exceptions.
- Do not launch the live stack.

## Acceptance

- Active Admin/Front Poodle dependencies resolve registry 0.2.2.
- Every active Underlay dependency resolves tag `v0.9.2`
  (`ddba26400f480638829917cf72eecc62be4b978d`), with no sibling path.
- No old Poodle version, sibling Poodle source, or committed override remains.
- Web and Rust lock changes are bounded to the coupled dependency upgrade.
- Admin, Front, UI, root validation, and QA pass or a baseline is reproduced.

## Validation

- Use the Effigy-owned prepare/install flow; inspect all Bun/Cargo manifests and
  locks.
- Run `effigy cp-admin/validate`, `effigy cp-front/validate`,
  `effigy cp-ui/validate`, the API validation selected by `effigy tasks`,
  `effigy validate`, and `effigy qa`.
- Run `git diff --check`.

## Stop Conditions

- Adoption needs an Underlay/template/public API decision.
- Install resolves duplicate or local Poodle.
- Lock churn is unrelated or validation needs an app exception.
- Evidence requires a visible runtime.

## Evidence And Continuation

Record the exact Underlay tag/commit, Poodle identities, lock review,
compatibility edits, and validation in the Contact Patch PR. Do not merge.
Independent of the other product cards.
