# g16.010 — Soundcheck Library Poodle 0.2.2 follow-up

Status: **complete — PR 6 merged at `7f5ff0b9`**
Depends on: `g16.004`, `g16.007`, published npm `0.2.2`
Target repository: `/Users/tom/Dev/projects/soundcheck-library`
Target base: `a720f22f5bb08ae465ba3dd46873855fec9b7c72`
Governing refs: `004-soundcheck-library-poodle-v021-adoption.md`,
`007-v022-release-certification.md`, Soundcheck Library package READMEs and
public peer contracts

## Outcome

Move Soundcheck Library's development dependency and published peer lines from
Poodle 0.2.1 to 0.2.2. Preserve the repository's existing baseline honestly
while proving the corrected registry packages introduce no new failure.

## Scope

- Pin the root Poodle Svelte dependency to exact `0.2.2`.
- Move both published library peer requirements from `^0.2.1` to `^0.2.2`.
- Regenerate both Bun and npm locks without unrelated upgrades; preserve the
  platform-specific lock constraints restored during `g16.004` review.
- Repair only compatibility failures caused by Poodle 0.2.2. Update package
  docs/tests only when their peer or import guidance is stale.

## Out Of Scope

- Do not publish or version Soundcheck Library.
- Do not expand its public component surface, add Poodle wrappers, change Rust
  library behavior, or edit Poodle.
- Do not claim the pre-existing type/test baseline is fixed by this card.

## Acceptance

- The root installs exact Poodle core/Svelte 0.2.2 from npm.
- Both library peers are `^0.2.2`.
- No active manifest or lock resolution uses Poodle 0.2.1 or a sibling path.
- `npm ci` and the repository's Poodle-using package tests/build checks show no
  adoption regression.
- Any pre-existing failures are reproduced on target-base main and separated
  from the adoption result; no unrelated lockfile churn remains.

## Validation

- Run `npm ci`, `npm test`, and `npm run check`; use the repository's Effigy
  surface where it provides the same task.
- Run focused package build/type checks if the root commands do not compile the
  publishable package surfaces.
- Run `git diff --check` and inspect both lockfiles completely, especially the
  previously restored platform `libc` constraints.

## Stop Conditions

- The peer change needs a broader Soundcheck Library release-policy decision.
- Lock regeneration materially changes unrelated packages or drops platform
  constraints again.
- Adoption introduces a new failure that indicates a Poodle 0.2.2 defect
  rather than a bounded consumer migration.

## Evidence And Continuation

Record resolved versions/sources, changed manifests/locks, baseline comparison,
and exact validation in the Soundcheck Library PR. Do not merge. Once this PR
lands, Soundcheck itself still waits for both this lane and Longhorn.

## Closeout

Soundcheck Library PR 6 merged at `7f5ff0b9`. The root development dependency
now installs exact Poodle Svelte 0.2.2, both published peer lines require
`^0.2.2`, and the Bun and npm locks resolve the published Poodle core/Svelte
0.2.2 packages with matching registry integrity values. All ten npm platform
`libc` constraints remain unchanged and no stale 0.2.1 or sibling-path Poodle
reference remains.

Independent review passed `npm ci`, frozen Bun resolution, dependency-tree
inspection, and `git diff --check`. `npm test` reproduced the existing
179-pass/one-failure taxonomy baseline; `npm run check` reproduced the existing
18-error/one-warning baseline. The adoption introduced no new failure.
