# g16.025 — Underlay Reference tagged Underlay adoption

Status: **review approved — correct stale PR metadata before merge**
Depends on: `g16.013`, Underlay tag `v0.9.4`
Target repository: `/Users/tom/Dev/projects/underlay-reference`
Target base: `f5ea7d72eee278e8838ba16f8f43eb2b662406d0`
Governing refs: `001-consumer-adoption-inventory.md`,
`013-underlay-reference-poodle-v022-adoption.md`, Underlay Reference root and
scoped `AGENTS.md`, reference implementation notes, and Underlay
adapter/template contracts

## Outcome

Move every active Underlay Reference web and Rust dependency from sibling paths
to Underlay tag `v0.9.4`, while preserving the exact public Poodle 0.2.2 result
already merged through `g16.013`.

## Scope

- Move Admin, Front, UI, and client Underlay dependencies to
  `git+ssh://git@github.com/inflatable-cookie/underlay.git#v0.9.4`.
- Move every active API Underlay crate from sibling paths to
  `ssh://git@github.com/inflatable-cookie/underlay.git`, tag `v0.9.4`, retaining
  its existing features.
- Regenerate all four Bun locks and Rust locks narrowly. Preserve exact Poodle
  0.2.2 registry identities and review the complete dependency-source diff.
- Repair only Underlay Reference-owned compatibility failures caused by moving
  from the sibling checkout to the released tag.

## Out Of Scope

- Do not edit Underlay or Poodle, change template/public APIs, refactor the
  reference structure, or add application exceptions.
- Do not reopen the completed Poodle migration or restore a sibling Poodle
  source.
- Do not launch the live stack.

## Acceptance

- Every active Underlay web and Rust dependency resolves tag `v0.9.4`
  (`7004af5b3461b6c89a7faa646575ff69576c73b8`), with no sibling Underlay path.
- Admin, Front, and UI still resolve exact public Poodle core/Svelte 0.2.2 with
  registry integrity and no duplicate runtime.
- Bun and Cargo lock churn is limited to the Underlay source migration and
  mechanically required metadata.
- Package checks, root validation, docs QA, and broad headless QA pass, or the
  pre-existing test-routing baseline recorded by `g16.013` is reproduced and
  kept separate from this adoption result.

## Validation

- Use the repository's Effigy-owned prepare/install flow; inspect every
  web/Rust manifest and all Bun/Cargo locks.
- Run `effigy validate`, `effigy qa`, and `effigy qa:docs`. Add package-local
  validation selected by `effigy tasks` where the aggregate board does not
  cover the changed package.
- Run `git diff --check`.

## Stop Conditions

- Adoption needs an Underlay adapter/template/public API decision.
- The released tag does not expose a crate, feature, or web export currently
  used by the reference estate.
- Install resolves duplicate/local Poodle, a sibling Underlay path, unrelated
  lock churn, or needs an application validation exception.
- Evidence requires the visible stack.

## Evidence And Continuation

Record the exact Underlay tag/commit, preserved Poodle registry identities,
manifest and lock review, compatibility edits, and validation in the Underlay
Reference PR. Do not merge. This lane is independent of `014`-`024` and closes
the fourth remaining Underlay consumer identified after the Poodle-only first
wave.

## Review Result

PR [#2](https://github.com/inflatable-cookie/underlay-reference/pull/2) has two
reviewed implementation head `25818824`. It rebases on current `main`, aligns
all active web and Rust dependencies with Underlay v0.9.4 at `7004af5b`, keeps
Poodle on registry 0.2.2, and repairs the canonical README. Empty-directory
frozen installs and fresh tag resolution pass; Admin, Front, UI, Client, API
build, and docs QA are green. Before merge, update the PR title/body/test plan,
which still describe the superseded v0.9.2 boundary. No implementation change
or further code review is needed if the head remains unchanged.
