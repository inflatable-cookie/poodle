# g16.006 — v0.2.2 release candidate

Status: **ready — `g16.005` accepted and merged**
Depends on: `g16.005`
Blocks: `g16.007` and all consumer adoption
Governing refs: `005-gpui-cratesio-recovery.md`,
`../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`../../contracts/001-working-rules.md`

## Outcome

Prepare one exact Poodle `v0.2.2` patch candidate from the accepted crates.io
GPUI recovery. Keep all release-bearing manifests in lockstep, document the
`v0.2.1` dependency-identity defect honestly, and return one green candidate
SHA to the operator gate.

This card must not create a tag, publish a package, or edit a release workflow.

## Fixed Decisions

- Version the same fixed denominator as `v0.2.1`: the three public-intent
  TypeScript packages and all 17 Rust crates under `packages/`. Private
  TypeScript tooling manifests and the private repository root keep their
  current versions.
- Preserve the publication set: core and Svelte publish; React is packed and
  certified but remains source-only; Rust remains source/tag distribution.
- Use two commits for exact-SHA evidence:
  1. one candidate commit containing every version, requirement, lockfile,
     generated stamp, changelog, release-note, and release-facing metadata
     change;
  2. one evidence-only commit adding the execution receipt for that candidate
     SHA.
- Run every candidate gate from a clean checkout of the candidate commit. Any
  later candidate-bearing edit replaces that SHA and requires a complete
  rerun; the evidence-only receipt does not repin the candidate.
- Keep tarballs outside the tracked tree. Record filenames, byte sizes, and
  SHA-256 digests in the receipt.
- `effigy release gates` is read-only and allowed. Do not run `effigy release
  simulate`: its accepted changelog-parser mismatch is recorded in
  `PAPERCUTS.md` and it is not the configured release path. `release prepare`,
  `release execute`, workflow dispatch, tag creation/push, GitHub release
  creation, `npm publish`, and registry mutation remain forbidden.

## Scope

- Advance all release-bearing TypeScript and Rust manifests,
  intra-repository requirements, generated version stamps, and lockfiles from
  `0.2.1` to `0.2.2` in lockstep.
- Add `0.2.2` release notes and changelog history naming the GPUI
  source-identity defect, restored crates.io boundary, and explicit
  non-activating window diagnostic.
- Re-derive the native licence and notice surface from the corrected final
  graph. Remove the obsolete `bzip2` / `libbz2-rs-sys` claims and licence
  allow-list entry when neither crate resolves in either GPUI lockfile.
- Preserve the existing publish set: core and Svelte publish; React remains
  source-only.
- Run the complete headless release board and package the same three web
  artifacts as `v0.2.1`.
- Prove a clean consumer can combine crates.io GPUI 0.2.2 with Poodle's GPUI
  node backend from the candidate without duplicate crate identities.
- Record one exact candidate SHA, package versions, artifact digests, and
  validation receipt.

## Writable Scope

- the three public-intent TypeScript manifests and relevant minimal `bun.lock`
  workspace entries;
- every Rust crate manifest below `packages/`, intra-repository version
  requirements, and Cargo lockfiles;
- generated IR/catalogue artifacts whose only candidate change is the
  `poodle-codegen 0.2.2` generator stamp;
- `CHANGELOG.md`, `docs/release-notes/0.2.2.md`, package READMEs and
  release-facing front doors only where existing `0.2.1` wording would
  contradict the candidate;
- the root and GPUI node-backend third-party notices, `deny.toml`, spec 022,
  and `scripts/audit-license-compliance.ts`, only for the bounded removal of
  stale `bzip2` / `libbz2-rs-sys` claims proved absent from the final graph;
- one August `g16.006` execution log/receipt and `PAPERCUTS.md` for new
  execution friction.

Do not edit component contracts or implementations, specimens, visual
baselines, Effigy tasks, dependency policy, release workflow files, downstream
repositories, or Jetstream admission state. A required change there means the
candidate is not ready: stop and report it.

## Acceptance

- [ ] All release-bearing manifests, generated version stamps, and lockfiles
      agree on `0.2.2` without unrelated upgrades.
- [ ] Release notes state the `v0.2.1` defect and the windowed diagnostic
      limitation without claiming true headless GPUI pixels.
- [ ] The final GPUI graphs and every active notice, licence-policy, and audit
      surface agree that `bzip2` / `libbz2-rs-sys` are absent.
- [ ] `effigy qa`, `effigy release gates`, `effigy docs:check`,
      `effigy ir:check`, and `effigy catalogue:check` pass from the exact
      candidate.
- [ ] Core, Svelte, and React pack checks and clean-install proof pass; only
      core and Svelte remain publishable.
- [ ] The candidate receipt pins one clean SHA and expected artifact digests.
- [ ] No tag, workflow dispatch, GitHub release, npm publish, registry
      mutation, or windowed/native-visual selector runs in the worker.

## Required Validation

From the clean candidate commit:

- exact manifest, intra-repository requirement, and lockfile agreement across
  the fixed release denominator;
- `effigy test:web-pack-install` plus clean local packs for core, Svelte, and
  React, with filenames, sizes, and SHA-256 digests recorded;
- `effigy check:release-automation`;
- `effigy audit:licenses` and `effigy audit:security`;
- an exact source/lock sweep proving no active release or notice surface still
  claims `bzip2` / `libbz2-rs-sys` as a current dependency;
- `effigy drift:gpui-consumer-identity`;
- `effigy qa`;
- read-only `effigy release gates`, with evidence that its configured headless
  gate ran;
- `effigy docs:check`, `effigy ir:check`, and `effigy catalogue:check`;
- `git diff --check` before the candidate commit and
  `git diff --check origin/main...HEAD` before PR handoff.

Use only supported headless selectors. Do not run `*-windowed`, native-visual,
Jetstream preview/QA, or any release mutation.

## Stop Conditions

- Any source or capture defect remains open from `g16.005`.
- Version or lockfile regeneration introduces unrelated dependency upgrades.
- A release gate is weakened, bypassed, or moved to a windowed path.
- Tag creation, publication, or workflow editing becomes necessary. Stop at
  the candidate and return to `g16.007`.

## Completion

Commit the complete candidate tree first. From that clean commit, run the full
required board and produce the external tarballs. Then add only the execution
receipt in a second commit, naming the candidate SHA, versions, artifact
digests, command results, known non-blocking warnings, and absence of release
mutations. Push one worker branch and open one PR against `main`; do not merge.
