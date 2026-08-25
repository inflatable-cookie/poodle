# g15.072 — Songsprout Underlay 0.9.2 and Poodle 0.2.2 adoption

Status: **complete — Songsprout PR 1 merged at `22f8ae78`**
Depends on: `g15.061`, `g15.063`, `g15.067`, Underlay tag `v0.9.2`
Target repository: `/Users/tom/Dev/projects/songsprout`
Target base: `b031c1a3c0dc197b1b082b706a4a134d7e5174ad`
Governing refs: `055-consumer-adoption-inventory.md`,
`067-underlay-reference-poodle-v022-adoption.md`, Songsprout root/Bloom/Greenhouse
`AGENTS.md`, and Trellis authority

## Outcome

Move Songsprout's active web and Rust Underlay dependencies from sibling paths
to tag `v0.9.2`, while moving Bloom and Greenhouse from Poodle 0.1.x plus
sibling overrides to public 0.2.2. Preserve the Trellis-owned workspace.

## Scope

- Pin Bloom and Greenhouse core/Svelte dependencies to exact `0.2.2`.
- Remove their committed Poodle overrides.
- Move Bloom, Greenhouse, and Stem Underlay dependencies to
  `git+ssh://git@github.com/inflatable-cookie/underlay.git#v0.9.2`.
- Move both public-config generators from their sibling source-file import to
  Underlay's published `@inflatable-cookie/underlay/server/config-stack`
  subpath.
- Move every active Nursery Underlay crate from sibling paths to
  `ssh://git@github.com/inflatable-cookie/underlay.git`, tag `v0.9.2`, retaining
  existing features.
- Regenerate all affected web and Rust locks narrowly; inspect other locks for
  mechanically propagated metadata only.
- Update Songsprout-owned dependency-resolution documentation and comments
  that still describe sibling Underlay/Poodle packages as application
  dependencies. Preserve sibling mounts used only by explicit cross-repository
  QA; they must not override installed package resolution.
- Repair only Songsprout-owned compatibility failures caused by the bump.

## Out Of Scope

- Do not edit Underlay/Poodle, change Trellis contracts, Nursery/API behavior,
  or product behavior beyond dependency compatibility repairs.
- Do not add aliases, app exceptions, or unrelated dependency upgrades.
- Do not launch the live stack.

## Acceptance

- Bloom and Greenhouse resolve public core/Svelte 0.2.2 with one identity.
- Every active Underlay dependency resolves tag `v0.9.2`
  (`ddba26400f480638829917cf72eecc62be4b978d`), with no sibling path.
- No config-generator import or other source bypass loads Underlay/Poodle from
  a sibling checkout during application validation.
- No active old Poodle version, sibling Poodle source, or override remains.
- Web and Rust lock churn is bounded to the coupled dependency upgrade.
- Both apps and the root workspace validate or a baseline is reproduced.

## Validation

- Use the Effigy-owned prepare/install path; inspect web/Rust manifests and all
  affected locks.
- Run `effigy stem/validate`, `effigy bloom/validate`,
  `effigy greenhouse/validate`, `effigy nursery/validate`, `effigy validate`,
  and `effigy qa`.
- Run `git diff --check`.

## Stop Conditions

- Adoption needs an Underlay/Trellis/public API decision.
- Install resolves duplicate or local Poodle.
- Lock churn is unrelated or validation needs an app exception.
- Evidence requires a visible runtime.

## Evidence And Continuation

Record the exact Underlay tag/commit, Poodle registry identities, lock review,
compatibility edits, and validation in the Songsprout PR. Do not merge.
Independent of the other product cards.

## Closeout

Songsprout PR [#1](https://github.com/inflatable-cookie/songsprout/pull/1)
merged on 2026-08-25 at
`22f8ae78077bdf8c3bb0841edc5066e90994a2d6`. Bloom, Greenhouse, and Stem now
resolve Underlay tag `v0.9.2` at
`ddba26400f480638829917cf72eecc62be4b978d`; Bloom and Greenhouse resolve
registry Poodle 0.2.2. Nursery's 27 Underlay packages resolve the same tagged
revision. No active sibling Underlay/Poodle source or Poodle 0.1 identity
remains.

Review caught stale local Underlay/Poodle metadata nested through Stem in the
two application locks. The repair refreshed only Stem's package metadata and
nested tagged graph, removing development packages inherited from the former
local Underlay checkout without changing a surviving unrelated package
version. Independent `effigy bloom/validate`,
`effigy greenhouse/validate`, and `git diff --check` passed. The pre-existing
root reorder-conflict rollout baseline remains recorded rather than hidden.
The canonical acceptance verdict is
[recorded on PR 1](https://github.com/inflatable-cookie/songsprout/pull/1#issuecomment-5411887134).
