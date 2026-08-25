# g16.016 — Composer Underlay 0.9.2 and Poodle 0.2.2 adoption

Status: **complete — PR 1 merged at `74e5a7a7`**
Depends on: `g16.007`, `g16.009`, `g16.013`, Underlay tag `v0.9.2`
Target repository: `/Users/tom/Dev/projects/composer`
Target base: `29a32c17d4a5e6f75da311c09776db08f118244a`
Governing refs: `001-consumer-adoption-inventory.md`,
`013-underlay-reference-poodle-v022-adoption.md`, Composer root/admin/front
`AGENTS.md`, and Composer workspace authority

## Outcome

Move Composer's active web and Rust Underlay dependencies from sibling paths to
tag `v0.9.2`, while moving Admin and Front from Poodle 0.1.x plus sibling
overrides to public 0.2.2.

## Scope

- Pin every existing direct Admin/Front Poodle dependency to exact `0.2.2`.
- Remove committed core/Svelte Poodle overrides. Do not add an unused direct
  Svelte dependency merely because the override existed.
- Move Composer Admin, Front, and API-client Underlay dependencies to
  `git+ssh://git@github.com/inflatable-cookie/underlay.git#v0.9.2`.
- Remove Composer Admin's SvelteKit/Vite aliases to the sibling Underlay source
  tree so the installed tagged package is the code under validation.
- Move both public-config generators from the sibling source-file import to
  Underlay's published `@inflatable-cookie/underlay/server/config-stack`
  subpath. Remove Front's stale `ensure-local-js-links.sh` Effigy calls; the
  script is absent and a tagged dependency must not be relinked locally.
- Move every active Composer API Underlay crate from sibling paths to
  `ssh://git@github.com/inflatable-cookie/underlay.git`, tag `v0.9.2`, retaining
  existing features.
- Regenerate web and Rust locks narrowly; prove transitive Svelte converges on
  0.2.2 through Underlay `v0.9.2`.
- Update Composer-owned dependency-resolution documentation that still says the
  frontends load Underlay/Poodle from sibling source. Preserve sibling mounts
  used only by the workspace's explicit cross-repository QA; they must not
  override package resolution.
- Repair only Composer-owned compatibility failures caused by the bump.

## Out Of Scope

- Do not edit Underlay/Poodle, change APIs, containers, or product behavior
  beyond dependency compatibility repairs.
- Do not add compatibility aliases, app exceptions, or unrelated dependency
  upgrades.
- Do not launch the live stack.

## Acceptance

- All active Composer Poodle identities resolve from the registry at 0.2.2.
- Every active Underlay dependency resolves tag `v0.9.2`
  (`ddba26400f480638829917cf72eecc62be4b978d`), with no sibling path.
- No SvelteKit/Vite alias, generator import, or local-link helper bypasses the
  tagged Underlay package or registry Poodle packages.
- No old Poodle version, sibling Poodle source, or committed override remains.
- Web and Rust lock diffs are bounded to the coupled dependency upgrade.
- Admin/Front and root validation pass or a reproduced baseline is isolated.

## Validation

- Use Composer's Effigy-owned install path; inspect all web/Rust manifests and
  locks.
- Run `effigy composer-api-client/validate`, `effigy composer-admin/validate`,
  `effigy composer-front/validate`, `effigy composer-api/validate`,
  `effigy validate`, and `effigy qa`.
- Run `git diff --check`.

## Stop Conditions

- Adoption needs an Underlay/template/public API decision.
- Install resolves duplicate or local Poodle.
- Lock churn is unrelated or validation needs an app exception.
- Evidence requires a visible runtime.

## Evidence And Continuation

Record the exact Underlay tag/commit, Poodle identities, transitive peer
convergence, lock review, compatibility edits, and validation in the Composer
PR. Do not merge. Independent of the other product cards.

## Closeout

Composer PR [#1](https://github.com/inflatable-cookie/loophole-composer/pull/1)
merged on 2026-08-25 at
`74e5a7a79ad4b5d4b57d881520934a5c75e1d6ca`. Admin, Front, and API-client
resolve Underlay tag `v0.9.2` at
`ddba26400f480638829917cf72eecc62be4b978d`; direct web Poodle dependencies
resolve exact registry 0.2.2. The Rust workspace moves exactly 24 Underlay
packages from local 0.8.0 paths to the same tagged revision.

Review caught and removed three kinds of lock drift before merge: a stale local
API-client Underlay identity in Front, broad unrelated Front lock regeneration,
and 62 unrelated Cargo package upgrades. The final web lock changes are bounded
to the declared dependency migration, and Cargo has no unrelated added or
removed package.

Independent `cargo check --workspace --all-features --all-targets --locked`
and `git diff --check` passed. Composer's Effigy container selector remains
blocked by the repository's pre-existing mounted-volume permission baseline;
host validation passed and no exception was added. Canonical approval evidence
is recorded in the
[PR review comment](https://github.com/inflatable-cookie/loophole-composer/pull/1#issuecomment-5412113581).
