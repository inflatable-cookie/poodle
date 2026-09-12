# g18.029 — v0.4.0 candidate-scope admission

Status: complete — awaiting orchestrator review
Date: 2026-09-12
Branch: `ns-cbbe9d2d-0d1b-49f1-9d24-6af92c6de3ad`
Card: `docs/roadmaps/g18/029-v040-candidate-scope-admission.md`
Handoff: `docs/handoffs/20260912-210450-g18-029-v040-candidate-scope-admission.md`
Base: pushed `main` at `78133c584386c3b67499afd8f3b204e0be10e783` (planning
promotion that carries this handoff)

## Outcome

The installed-package guard now carries two closed candidate policies instead
of one historical mode. `g16.054-candidate` is preserved byte-for-byte in
behaviour: its `0.2.3` → `0.3.0` proof, its direct one-commit candidate tree,
its writable set and its Cargo/JS content rules. A distinct
`g18.006-candidate` policy admits only the exact `0.3.0` → `0.4.0` release.

Ordinary CI, which carries no candidate environment variable, now recognizes
that candidate: when a range still contains release-bearing changes after the
existing content-aware ordinary checks, ordinary scope admits it only when the
complete base-to-head diff validates against the closed g18.006 policy. A
closed admission stays `ordinary` and never emits a certification receipt.

Policy data stays together in `CandidatePolicy`: mode, source/target versions,
the exact release inputs, the generated/evidence families, the content rules,
and whether the candidate needs a direct source (g16.054) or a frozen
release-input commit with an evidence suffix (g18.006). The same
`assertClosedCandidateRange` implementation backs both the explicit
`g18.006-candidate` mode and ordinary recognition, so a PR lane and a local
certification run cannot disagree about admission. No workflow, release
input, version, lock, changelog, receipt, tag or registry surface was touched.

## Closed g18.006 policy

Admitted release inputs: the complete lockstep JS and Cargo manifests, their
intra-repository requirements, the tracked Cargo locks, `bun.lock`,
`CHANGELOG.md`, the `0.4.0` release notes and the release-notes index.
Admitted generated/evidence families: the codegen version stamps, the complete
Nucleus receipt/manifest/ledger cohort, the GPUI census cohort and mounted
receipts, and one fixed `g18.006` execution record. Component source,
workflows, publish/registry transport, React admission, Cargo retargeting and
arbitrary documentation stay outside the allowlist.

Identity rules:

- the complete release-input set must change from `0.3.0` to `0.4.0`;
- exactly one commit in the range may change those release inputs, so the final
  inputs are byte-identical to that frozen commit and both hidden prior
  candidate commits and later input drift are rejected;
- changed evidence that records a `source_commit` must name that frozen
  commit;
- the head changelog must parse and carry a linked `0.4.0` release and staged
  notes;
- React must stay `private: true` and Cargo requirements may only change
  version, never identity or path.

## Acceptance and falsification

The focused suite `test/package-install/scope.test.ts` exercises the closed
policy directly, and `test:web-pack-install` records the same plants through
the production guard. All observed results:

| Invariant | Adversarial counterexample | Result |
| --- | --- | --- |
| Ordinary CI admits only the exact candidate | complete `0.3.0` → `0.4.0` plant | admitted as `ordinary` with 28 release-bearing paths, no receipt |
| Ordinary CI admits only the exact candidate | partial set (two Cargo manifests omitted) | ordinary rejection; explicit mode reports the missing release inputs |
| Ordinary CI admits only the exact candidate | one `Cargo.toml` at `0.3.1` | `candidate scope rejected unauthorized Cargo manifest change` |
| Ordinary CI admits only the exact candidate | lone `docs/release-notes/0.4.0.md` addition | `rejected a partial release-note change without the closed g18.006-candidate candidate` |
| Candidate identity is frozen once | a later commit re-bumps a release input | `requires exactly one frozen 0.4.0 release-input commit; found 2` |
| Candidate identity is frozen once | evidence bound to the base commit | `rejected evidence ... bound to <base> instead of the frozen release-input commit <frozen>` |
| Release transport stays protected | `.github/workflows/release.yml` | `forbidden workflow surface` |
| Release transport stays protected | `scripts/publish/release.ts` | `forbidden release surface`, `forbidden registry surface` |
| Release transport stays protected | `.npmrc` | `forbidden registry surface` |
| React and Rust publication stay bounded | React `private: false` | `rejected unauthorized packages/react/components/package.json changes: private` |
| React and Rust publication stay bounded | retargeted intra-repository Cargo requirement | `rejected unauthorized Cargo manifest change` |
| Precursor is not a candidate | arbitrary source path | `paths outside writable allowlist` |
| Historical proof remains immutable | `0.3.0` manifest under g16.054 | `candidate scope requires packages/core/package.json` (still `0.2.3`-bound) |

The unchanged g16.054 laws remain green: `g16.054-candidate` output routing,
its `0.3.0` package-manifest requirement, its Cargo publication/registry/
retarget rejection, and its direct-source evidence-head rejection all still
pass.

## Validation

- `effigy test:core-build` — pass; 75 tests across 6 files, including the
  focused `scope.test.ts` laws (40 scope tests).
- `effigy test:web-pack-install` (ordinary, unset scope mode) — pass; the
  clean checkout certified the ordinary range and recorded every closed
  0.4.0 plant as a falsification receipt, including the positive ordinary
  admission.
- `effigy docs:check` — pass on the built package trees; the package-install
  README documents the closed policy, ordinary recognition and the unchanged
  g16.054 contract.
- `git diff --check` — pass.
- `bunx tsc -p tsconfig.json --noEmit` — no `test/package-install` diagnostics
  (the repository-wide board has pre-existing unrelated errors and is not a
  supported selector).

## Limits

This PR only admits the policy. It does not prepare the `0.4.0` candidate,
edit versions, locks, changelog or receipts, resume retained g18.006, run the
release gate, tag, publish, mutate Desktop or edit workflows. No release
mutation, windowed selector or `*-windowed` conformance selector was run. The
hosted branch dry run remains post-merge g18.009 work.
