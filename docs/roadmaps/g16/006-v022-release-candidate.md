# g16.006 — v0.2.2 release candidate

Status: **blocked — waits for accepted `g16.005` recovery**
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

## Scope

- Advance all release-bearing TypeScript and Rust manifests,
  intra-repository requirements, generated version stamps, and lockfiles from
  `0.2.1` to `0.2.2` in lockstep.
- Add `0.2.2` release notes and changelog history naming the GPUI
  source-identity defect, restored crates.io boundary, and explicit
  non-activating window diagnostic.
- Preserve the existing publish set: core and Svelte publish; React remains
  source-only.
- Run the complete headless release board and package the same three web
  artifacts as `v0.2.1`.
- Prove a clean consumer can combine crates.io GPUI 0.2.2 with Poodle's GPUI
  node backend from the candidate without duplicate crate identities.
- Record one exact candidate SHA, package versions, artifact digests, and
  validation receipt.

## Acceptance

- [ ] All release-bearing manifests, generated version stamps, and lockfiles
      agree on `0.2.2` without unrelated upgrades.
- [ ] Release notes state the `v0.2.1` defect and the windowed diagnostic
      limitation without claiming true headless GPUI pixels.
- [ ] `effigy qa`, `effigy release gates`, `effigy docs:check`,
      `effigy ir:check`, and `effigy catalogue:check` pass from the exact
      candidate.
- [ ] Core, Svelte, and React pack checks and clean-install proof pass; only
      core and Svelte remain publishable.
- [ ] The candidate receipt pins one clean SHA and expected artifact digests.

## Stop Conditions

- Any source or capture defect remains open from `g16.005`.
- Version or lockfile regeneration introduces unrelated dependency upgrades.
- A release gate is weakened, bypassed, or moved to a windowed path.
- Tag creation, publication, or workflow editing becomes necessary. Stop at
  the candidate and return to `g16.007`.

