# g15.054 — v0.2.1 Release Recovery

Status: **candidate green — direct orchestrator recovery complete; awaiting
exact commit and evidence-receipt pin**
Depends on: accepted `g15.050` candidate `7922a3a9`, immutable `v0.2.0` tag,
and explicit operator approval to repair `release.yml`
Governing refs: `013-v020-release-certification.md`,
`../../contracts/001-working-rules.md`, `../../../.agents/skills/effigy/references/release-protocol.md`

## Outcome

Prepare one exact `0.2.1` replacement candidate. Preserve the failed `v0.2.0`
tag, repair only the npm bootstrap failure, advance all release-bearing
manifests in lockstep, and rerun the complete headless release evidence before
returning to the operator gate.

## Failure Boundary

GitHub Actions run `32656225297` failed while
`npm install --global npm@12.0.2` replaced the npm tree executing that command.
The running CLI then could not load `promise-retry`. Dependency installation,
release gates, version checks, packing, publication, and artifact upload were
all skipped. npm remained at `0.1.0` for core and Svelte.

## Scope

- Install npm `12.0.2` into an isolated runner prefix and put its binary on
  subsequent-step `PATH`; reject in-place global npm replacement statically.
- Advance all 20 release-bearing TypeScript and Rust manifests, intra-repo
  requirements, and lockfiles from `0.2.0` to `0.2.1`.
- Regenerate version-stamped IR and catalogue artifacts.
- Record the failed `0.2.0` publication honestly and publish `0.2.1` notes.
- Pack core, Svelte, and experimental React; publish only core and Svelte.

## Acceptance

- [x] The isolated npm `12.0.2` bootstrap is locally reproduced and the
      automation check rejects the failed global-install shape.
- [x] All release-bearing manifests and lockfiles agree on `0.2.1`.
- [x] `effigy qa`, `effigy release gates`, `effigy docs:check`,
      `effigy ir:check`, and `effigy catalogue:check` pass from the exact
      candidate.
- [x] `npm pack` verifies the three expected tarballs; React remains outside
      the workflow publish set.
- [ ] A receipt pins the exact candidate SHA and artifact digests.
- [ ] `g15.013` returns to an explicit `v0.2.1` operator gate.

## Stop Conditions

- Never move, delete, or reuse `v0.2.0`.
- Do not rerun the failed workflow.
- Do not weaken or bypass a release gate.
- Stop if recovery changes public product behavior beyond the version and
  release-documentation correction.

## Validation

Use the repository's headless Effigy selectors only. Do not run windowed,
native-visual, or Jetstream selectors locally.
