# g16.054 — HistoryCenter v0.3.0 Release Candidate

Status: ready — `g16.053` and serial `g16.056`–`g16.059` are complete; later
release certification and Loophole adoption remain separate authority gates
Type: release candidate — no release mutation
Opened: 2026-09-01
Depends on: completed `g16.053`; completed `g16.059`, which certifies the
compiled core/Svelte/private-React distribution from installed tarballs;
merged `g16.033`; merged `g16.036`; and the accepted sequence recorded in
`../../handoffs/20260901-234025-post-triage-canonical-runway.md`
Governing refs: `../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`../../specs/044-deprecation-change-control-and-release-channel-operations.md`,
`../../architecture/014-compiled-web-package-distribution.md`,
`../g15/060-v022-release-candidate.md`, `../../contracts/001-working-rules.md`

## Goal

Prepare, validate, and evidence one immutable Poodle `0.3.0` candidate from
current main after both serial prerequisites are accepted.
Record exact package, lock, generated, tarball, and headless-gate evidence. Do
not tag, publish, dispatch release workflows, edit Loophole, or imply that a
green candidate is a released package.

## Fixed Release Shape

- Target version is `0.3.0`. `0.2.3` remains prepared but unpublished;
  `0.2.4` is skipped. The breaking v3 HistoryEntry surface requires a pre-1.0
  minor under current authority.
- Lockstep-bump the three public TypeScript manifests and all 17
  `packages/**/Cargo.toml` files plus intra-repository requirements, locks, and
  generated stamps. Private/internal zero-version packages stay put.
- Publish set remains core and Svelte only. React is packed and validated only;
  Rust stays source/tag distribution. Jetstream lockstep identity is not
  admission.
- Core, Svelte, and React package inputs must already emit compiled JavaScript
  and declarations under architecture 014 and completed `g16.056`–`g16.059`.
  This card consumes their accepted receipt; it does not redesign the build,
  export map, CSS delivery, `sideEffects`, or dependency isolation.
- Release notes inventory public intent from immutable `v0.2.2`, name the
  HistoryEntry breaking migration and five-code rejection surface, explain
  unpublished `0.2.3`, and correct READMEs that advertise it.
- Two commits: candidate tree, then evidence receipt naming the candidate SHA.
  The receipt must not repin or mutate the candidate tree.
- Candidate work is headless and read-only with respect to release transport.
  No `release prepare/execute/simulate`, tag, workflow dispatch, npm publish,
  registry mutation, workflow edit, or sibling write.

## Ordered Work

1. Reconcile current main after `g16.053` and `g16.059` are complete. Prove the
   installed-distribution receipt names the landed mainline outputs before
   freezing candidate inputs, then verify `v0.3.0` /
   `v0.2.4` are absent locally and remotely.
2. Apply lockstep versions, requirements, locks, generated stamps, changelog,
   `0.3.0` notes, and README/unpublished-`0.2.3` honesty edits.
3. Commit the candidate tree. From a clean checkout of that exact commit, run
   every packet-required headless gate, pack core/Svelte/React, and record names,
   byte sizes, SHA-256, content verification, and packed HistoryEntry proof.
4. Commit the evidence-only receipt. Open one candidate PR; do not tag or
   publish.

## Acceptance

- Lockstep manifests, requirements, `bun.lock`, generated IR/catalogue stamps,
  release notes, changelog, and package READMEs all name the same `0.3.0`
  candidate truth.
- Installed tarballs prove v3 `HistoryEntry` on both Svelte public paths and
  one unsuppressed `branchCount` failure per path; React public-root/types proof
  remains honest.
- Core/Svelte/React tarball identities and SHA-256 receipts come from the exact
  candidate commit. Core/Svelte pack-content verification matches workflow
  rules without editing the workflow.
- Packed core/Svelte/React surfaces contain compiled JavaScript and declarations
  supplied by the accepted prerequisite. Candidate work does not design or
  alter its build, export, CSS, or dependency mechanics.
- `effigy release gates` executes the one configured headless gate from the
  exact candidate and all required boards pass.
- Local/remote tag checks remain absent. npm, registries, workflows, releases,
  Loophole, Longhorn, React publication, crates.io, and Jetstream admission are
  unchanged.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Candidate identity is immutable | evidence commit changes package tree | candidate-tree hash stays unchanged |
| Version truth is lockstep | one Cargo manifest or bun workspace stays 0.2.3 | version gate fails |
| Notes describe shipped delta | notes omit HistoryEntry break | release-doc review fails |
| Packed surface is v3 | `branchCount` compiles on either Svelte path | unsuppressed TS2339 proof fails |
| Compiled package prerequisite is real | a tarball exposes raw source or lacks declarations | prerequisite receipt and pack proof fail |
| React is validate-only | workflow/manifest gains React publish | automation audit fails |
| Green candidate is not a release | tag/workflow/npm changes appear | scope gate fails |

## Writable Scope

Release-bearing manifests and intra-repository requirements; locks and
generated version stamps; changelog, `0.3.0` notes, unpublished-`0.2.3`
clarification, package README honesty; ignored tarballs; one candidate receipt;
this card, one log, and new papercuts. Do not edit `.github/workflows/`, tags,
registries, sibling repositories, component behavior, Jetstream admission, or
windowed/native-visual routes. Do not choose or change the prerequisite's
build, export, CSS, `sideEffects`, or dependency-isolation mechanics.

## Validation

Run the exact packet list: lockstep check, `effigy ir:build` / `ir:check`,
`catalogue:build` / `catalogue:check`, `test:web-pack-install`, local packs and
content hashes, `check:release-automation`, `audit:licenses`, `audit:security`,
`drift:gpui-consumer-identity`, `docs:check`, `qa`, read-only `effigy release
gates`, tag-absence checks, and `git diff --check origin/main...HEAD`. Never run
windowed/native-visual or mutating release commands.

## Stop Conditions

Stop on any red non-flake gate, unproved flake, lock/stamp drift, missing packed
negative proof, absent compiled JavaScript/declarations prerequisite, incomplete
public-intent notes, a candidate-bearing mainline change after freeze,
workflow/tag/registry mismatch, requested compatibility shim, sibling mutation,
or release/windowed action.

## Continuation

After accepted candidate merge, the orchestrator must separately authorize and
perform certification: tag the exact candidate SHA, dispatch the release
workflow, prove npm `latest=0.3.0`, then route Loophole-owned adoption. None of
those actions is ready or authorized by this card.
