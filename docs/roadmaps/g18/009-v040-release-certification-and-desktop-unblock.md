# 009 — v0.4.0 release certification and Desktop unblock

Status: complete — `v0.4.0` published 2026-09-13; Desktop capsule returned
Owner: Poodle release operations
Created: 2026-09-11
Governing refs: `../../contracts/001-working-rules.md`,
`../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`../../specs/044-deprecation-change-control-and-release-channel-operations.md`,
`../../specs/070-compiled-web-distribution-contract.md`,
`../../../packages/release-manifest.json`,
`../../../packages/release-operations.json`
Depends on: g18.006 queue task `17ac3fee-de90-4b32-9672-1134770bb086`
complete with accepted exact candidate on pushed main

## Outcome

Certify and publish the exact merged Poodle `0.4.0` candidate. Publish only
`@inflatable-cookie/poodle-core` and `@inflatable-cookie/poodle-svelte`, prove a
fresh ordinary registry install of Svelte `./editor`, and return the immutable
release capsule that lets Desktop resume retained g02.058 and PR #215.

## Ready-State Rubric

- [x] Release scope, version and public package set are frozen by g18.006.
- [x] Tom explicitly authorized candidate and release execution on 2026-09-11.
- [x] g18.006 is complete and its accepted candidate is clean pushed main.
- [x] Candidate commit, package trees and archives are exact; branch dry run is
  the first hosted step after dependency close.
- [x] Operator ruling 2026-09-13 replaced the aggregate GPUI/native gate with
  the accepted g18.006 npm/web certificate through a bounded workflow wrapper
  repair before the hosted dry run.

## Decisions

- Tag the release commit: the accepted g18.006 candidate content carried by
the operator-authorized npm/web wrapper commit. Do not rebuild or amend the
candidate package trees.
- Consume g18.006's exact stable local-gate proof without repeating it. Run the
  mandatory hosted branch dry run against the unchanged merged candidate
  before tagging.
- Create one immutable lightweight `v0.4.0` tag only after every pre-tag gate is
  green. Never retag a failed or published release.
- Run the tag dry run before publication. Stop on every red or mismatched run.
- Publish core and Svelte only. React remains private packed evidence; Rust
  crates remain source/tag distribution.
- Verify registry metadata and tarball integrity, then prove `./editor` from a
  fresh unlinked install in type, SSR and browser modes.
- Record release evidence on a separate queue branch and PR after publication.
  That evidence PR cannot alter the candidate or release-bearing package inputs.

## Dispatch manifest

- **State:** dependency-queued behind g18.006 with no manual hold; g18.006 stays
  paused until all product work, the four-surface sweep, g18.012, all blocking
  repairs, and operator acceptance are complete
- **Completion:** exact candidate local gate evidence consumed and branch dry run green;
  `v0.4.0` at that commit; tag dry run and publish green; npm core/Svelte 0.4.0
  metadata and tarballs verified; fresh ordinary install proves `./editor`;
  evidence PR independently reviewed and merged; Desktop capsule returned
- **Owned mutable paths:** release evidence/log, g18.009 task and closeout
  surfaces; no candidate package input after g18.006
- **Worker:** release-capable high-reasoning worker with hosted workflow, npm
  trusted publishing, package integrity and cross-repo evidence experience
- **Excluded:** candidate reconstruction; workflow edits; React/crates
  publication; Desktop edits; source links; retries after a red tag/publication
- **Escalation:** operator through Chatterbox for any identity mismatch, failed
  gate/run, registry discrepancy or need to alter the accepted candidate

## Work

1. Resolve the exact accepted g18.006 candidate on clean pushed main. Recompute
   version, tree, archive and release-note identities without changing them.
2. Verify g18.006's stable local-gate and exact-head PR evidence, then run the
   release workflow branch dry run against that exact merged commit. Require
   the reported head to match; do not repeat the equivalent local board.
3. Create and push `v0.4.0` only after the branch dry run passes.
4. Run the tag dry run, then publication. Stop on any failure; never retag.
5. Query npm until exact core/Svelte 0.4.0 metadata and tarballs are available.
   Verify integrity against the publication evidence.
6. In a fresh source-free consumer, install exact registry 0.4.0 and prove the
   Svelte `./editor` export in declared type, SSR and browser modes.
7. Record immutable evidence in one PR without changing release-bearing inputs.
8. Return version, candidate/tag commits, final package trees, tarball integrity,
   workflow run IDs and installed result to Desktop Chatterbox. Direct the same
   g02.058 worker/workspace/PR to repin, unlink and finish Phase B.

## Acceptance and review oracle

| Invariant | Counterexample | Required proof |
| --- | --- | --- |
| Candidate is exact | release runs against later main | tag, workflow head and g18.006 accepted candidate are identical |
| Publication is ordered | tag or publish precedes a green prerequisite | timestamped branch dry run, tag, tag dry run and publish IDs |
| Public set stays bounded | React or crates appear in a registry | registry and workflow package-set inspection |
| Registry is usable | metadata exists but `./editor` is absent | tarball integrity plus fresh source-free import proof |
| Evidence cannot mutate release | evidence PR changes manifests or package bytes | exact path diff and candidate/tag tree comparison |
| Desktop remains consumer-owned | release worker edits PR #215 | zero Desktop mutation; capsule only |

## Stop Conditions

- Stop until g18.006 closes with the exact accepted candidate on pushed main.
- Stop on any dirty tree, identity mismatch, red gate, failed workflow or
  registry discrepancy. Do not bypass, retry a one-shot step or retag.
- Stop before workflow edits, candidate mutation, React/crates publication or
  Desktop changes.

## Next Task

Desktop resumes its retained g02.058 task and PR #215. Poodle returns to
Chatterbox for GPUI tranche compilation; no release successor auto-starts.

## Closeout evidence (2026-09-13)

Executed and recorded in
[`docs/logs/2026-09/20260913-g18-009-v040-release-certification.md`](../../logs/2026-09/20260913-g18-009-v040-release-certification.md).

- The accepted g18.006 candidate `9d18a21b` (merge `567fe01c`) was certified by
  a bounded workflow-only wrapper repair at `4a39055f3`: the aggregate
  `effigy release gates` step was replaced by `effigy svelte:package`,
  `effigy check:release-automation` and `effigy test:web-pack-install`, and the
  release-automation guard now forbids the wrapper regaining the aggregate,
  native, or GPUI selectors. Package trees stayed byte-identical to `a797ce413`.
- Ordered evidence: branch dry run `34743528777`, tag `v0.4.0` at `4a39055f3`,
  tag dry run `34743709181`, publish `34743884234` — all green at the same commit.
- npm now serves `@inflatable-cookie/poodle-core@0.4.0` and
  `@inflatable-cookie/poodle-svelte@0.4.0` as `latest` with attested provenance;
  registry tarballs are byte-identical to the publish artifact; React, the
  internal tooling packages, and every crate remain unpublished.
- A fresh source-free consumer installed the exact registry versions and proved
  the Svelte `./editor` entry in declared-type, SSR, and browser modes with the
  `svelte 5.56.8` floor. This is the entry `0.3.0` lacked.
- The first branch dry run (`34728955353`) was cancelled after six hours in the
  obsolete aggregate gate and was never retried; the operator ruling supersedes
  that step for this release.
- Evidence PR #266 merged 2026-09-13 as `4c0bb57410584da944f781eb8064805521fba054` (now `origin/main`) on independent exact-head review (comment `5651878364`, verdict `ready_to_merge`); `rust` + `web` checks green at merge. Recorded in [`docs/logs/2026-09/20260913-g18-009-v040-release-merged.md`](../../logs/2026-09/20260913-g18-009-v040-release-merged.md).
