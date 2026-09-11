# 009 — v0.4.0 release certification and Desktop unblock

Status: held — operator requires g18.010–g18.016 editor acceptance work first
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
- [ ] g18.006 is complete and its accepted candidate is clean pushed main.
- [ ] Candidate commit, package trees, archives and branch dry run are exact.

## Decisions

- Tag the exact accepted g18.006 candidate commit. Do not rebuild or amend it.
- Run local release gates and the mandatory branch dry run before tagging.
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

- **State:** queue-held behind g18.006; release only after g18.010–g18.016 and
  all other blocking repairs are operator-accepted and the retained candidate
  is rebuilt/reviewed from that source
- **Completion:** exact candidate local gates and branch dry run green;
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
2. Run local release gates and the release workflow branch dry run against that
   exact commit. Require the reported head to match.
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
