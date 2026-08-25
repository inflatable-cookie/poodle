# g15.071 — Contact Patch Underlay 0.9.2 and Poodle 0.2.2 adoption

Status: **complete — Contact Patch PR 1 merged at `c497547b`**
Depends on: `g15.061`, `g15.063`, `g15.067`, Underlay tag `v0.9.2`
Target repository: `/Users/tom/Dev/projects/contact-patch`
Target base: `a6d2316f5c5248c1d27f7f595bfbf2b0de91127e`
Governing refs: `055-consumer-adoption-inventory.md`,
`067-underlay-reference-poodle-v022-adoption.md`, Contact Patch root/admin/front/UI
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
- Remove `cp-client`'s TypeScript path mapping to the sibling Underlay source so
  client validation resolves the installed tagged package.
- Move the shared public-config generator from its sibling source-file import
  to Underlay's published
  `@inflatable-cookie/underlay/server/config-stack` subpath.
- Move every active Contact Patch API Underlay crate from sibling paths to
  `ssh://git@github.com/inflatable-cookie/underlay.git`, tag `v0.9.2`, retaining
  existing features.
- Regenerate web and Rust locks narrowly and prove one Poodle and Underlay
  identity.
- Update Contact Patch-owned dependency-resolution documentation that still
  describes sibling Underlay/Poodle packages as application dependencies.
  Preserve sibling mounts used only by explicit cross-repository QA; they must
  not override installed package resolution.
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
- No TypeScript path mapping, generator import, or other source bypass loads
  Underlay/Poodle from a sibling checkout during application validation.
- No old Poodle version, sibling Poodle source, or committed override remains.
- Web and Rust lock changes are bounded to the coupled dependency upgrade.
- Admin, Front, UI, root validation, and QA pass or a baseline is reproduced.

## Validation

- Use the Effigy-owned prepare/install flow; inspect all Bun/Cargo manifests and
  locks.
- Run `effigy cp-client/validate`, `effigy cp-admin/validate`,
  `effigy cp-front/validate`, `effigy cp-ui/validate`,
  `effigy cp-api/validate`, `effigy validate`, and `effigy qa`.
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

## Closeout

Contact Patch PR [#1](https://github.com/contact-patch/contact-patch/pull/1)
merged on 2026-08-25 at
`c497547bfb244f53b1f68f2f16e292103e9e756f`. Admin, Front, UI, and client now
resolve Underlay tag `v0.9.2` at
`ddba26400f480638829917cf72eecc62be4b978d`; Admin and Front resolve registry
Poodle 0.2.2. All 26 Rust Underlay packages resolve the same tagged revision.
The sibling TypeScript path map, config-stack source import, Poodle overrides,
and active sibling package paths are gone.

Independent review passed `effigy health`, `effigy cp-admin/validate`,
`effigy cp-front/validate`, `effigy cp-ui/validate`, and `git diff --check`.
Database-backed API tests still require Postgres. A pre-existing Admin contract
test searches the wrong client types file. Both baselines are recorded honestly
in the consumer PR and `PAPERCUTS.md`; neither was hidden with an application
exception. The canonical acceptance verdict is
[recorded on PR 1](https://github.com/contact-patch/contact-patch/pull/1#issuecomment-5411781380).
