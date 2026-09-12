# g18.029 — v0.4.0 candidate-scope admission

Status: complete — PR #261 merged as `01bb0dd84236f69bf55d5556c56b46513191a586` on 2026-09-12 after exact-head independent review
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
  version, never identity or path;
- lockstep is exact in content: every internal JS dependency and every
  version-carrying intra-repository Cargo requirement in a changed manifest
  must move `sourceVersion` → `targetVersion`, path-only requirements must
  keep their identity and path, and added or removed internal requirements
  fail closed.

## Review repair

Independent review (PR #261 comment `5648573763`) found one blocking
fail-closed defect at head `330c899583dbad1143711026d4c1e078d86b1246`: the
candidate policy validated only *changed* requirement lines, so a lockstep
`[package]`/JS version bump could leave a stale `0.3.0` internal requirement
in place, and an allowed JS dependency leaf could carry an arbitrary
specifier. Ordinary CI could therefore admit a non-lockstep `0.4.0`
candidate.

The repair adds two content checks on the candidate branch and base:

- `internalJsDependencies` reads every `@inflatable-cookie/poodle-*` entry in
  `dependencies`, `devDependencies`, `peerDependencies` and
  `optionalDependencies` of each lockstep JS manifest. The dependency identity
  set must be identical at base and head, every specifier must be exactly
  `sourceVersion` at base and `targetVersion` at head, and a non-string or
  added/removed internal dependency fails closed.
- `cargoIntraRepoRequirements` reads every `poodle-*` inline requirement in
  `[dependencies]`, `[dev-dependencies]` and `[build-dependencies]`. Requirement
  names must match base to head, paths must be preserved, and every
  version-carrying requirement must move `sourceVersion` → `targetVersion`;
  path-only requirements must stay version-free and unchanged.

Both checks run for every changed manifest, not only the changed lines, so
stale or arbitrary requirements can no longer ride a version bump in either
the explicit `g18.006-candidate` mode or ordinary recognition.

Re-review (PR #261 comment `5648655527`) found one remaining blocking defect at
head `5983509e9b158497be330163fbd40f6d16c6fb21`: Cargo requirements were keyed
by crate name only, so a correct later `[dev-dependencies]` entry shadowed a
stale runtime `[dependencies]` entry with the same crate name (a legal Cargo
shape already present in `packages/contracts/components/Cargo.toml`), and a
synthetic candidate was admitted in both ordinary and explicit modes. The
repair keys every intra-repository Cargo requirement by owning dependency
table and crate name, validates every occurrence independently, and fails
closed on a duplicate requirement in the same table.

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
| Ordinary CI admits only the exact candidate | non-lockstep candidate (stale internal JS dependency) | ordinary rejection through the closed range validator |
| Candidate identity is frozen once | a later commit re-bumps a release input | `requires exactly one frozen 0.4.0 release-input commit; found 2` |
| Candidate identity is frozen once | evidence bound to the base commit | `rejected evidence ... bound to <base> instead of the frozen release-input commit <frozen>` |
| Release transport stays protected | `.github/workflows/release.yml` | `forbidden workflow surface` |
| Release transport stays protected | `scripts/publish/release.ts` | `forbidden release surface`, `forbidden registry surface` |
| Release transport stays protected | `.npmrc` | `forbidden registry surface` |
| React and Rust publication stay bounded | React `private: false` | `rejected unauthorized packages/react/components/package.json changes: private` |
| React and Rust publication stay bounded | retargeted intra-repository Cargo requirement | `rejected unauthorized Cargo manifest change` |
| Lockstep is exact in content | stale internal JS dependency left at `0.3.0` | `requires internal JS dependency dependencies:@inflatable-cookie/poodle-core ... to move 0.3.0 -> 0.4.0` |
| Lockstep is exact in content | arbitrary internal JS dependency (`^0.4.0`) | same exact specifier requirement rejects the range |
| Lockstep is exact in content | removed internal JS dependency | `rejected added or removed internal JS dependency` |
| Lockstep is exact in content | unchanged stale Cargo requirement | `requires intra-repository Cargo requirement poodle-ir in [dependencies] of packages/contracts/tokens/Cargo.toml to move 0.3.0 -> 0.4.0, found 0.3.0 -> 0.3.0` |
| Lockstep is exact in content | stale runtime requirement shadowed by a correct `[dev-dependencies]` entry | same `[dependencies]` requirement rejection in ordinary and explicit modes |
| Precursor is not a candidate | arbitrary source path | `paths outside writable allowlist` |
| Historical proof remains immutable | `0.3.0` manifest under g16.054 | `candidate scope requires packages/core/package.json` (still `0.2.3`-bound) |

The unchanged g16.054 laws remain green: `g16.054-candidate` output routing,
its `0.3.0` package-manifest requirement, its Cargo publication/registry/
retarget rejection, and its direct-source evidence-head rejection all still
pass.

## Validation

Validation budget for the second repair: focused `scope.test.ts` plus one
`test:web-pack-install` proof, with `docs:lint` because this log changed.

- `bun test test/package-install/scope.test.ts` — pass; 43 focused scope
  tests, including the duplicate-section Cargo requirement regression (the
  new plant fails against the pre-repair `scope.ts`, stash-verified).
- `effigy test:web-pack-install` (ordinary, unset scope mode) — pass; the
  clean checkout certified the ordinary range and recorded every closed
  0.4.0 plant as a falsification receipt, including the positive ordinary
  admission and the new shadowed-requirement plant.
- `effigy docs:lint` — pass.
- `git diff --check` — pass.
- `effigy test:core-build` and `effigy docs:check` from the first repair are
  carried forward for unaffected surfaces; they were not re-run under the
  second-repair validation budget.

## Limits

This PR only admits the policy. It does not prepare the `0.4.0` candidate,
edit versions, locks, changelog or receipts, resume retained g18.006, run the
release gate, tag, publish, mutate Desktop or edit workflows. No release
mutation, windowed selector or `*-windowed` conformance selector was run. The
hosted branch dry run remains post-merge g18.009 work.

## Merge

PR #261 merged `01bb0dd84236f69bf55d5556c56b46513191a586` on 2026-09-12 with parents `b6a8dd7a4dff2933b73696292b3a1abb772779da` (main) and `e569e83acc32c42e64d4af2a0e3d1901f5727232` (reviewed head): the merge matches the reviewed head exactly, no refresh was needed. Accepted review is issue comment `5648702922` (`ready_to_merge`, no blocking findings remain at the exact head). Exact-head GitHub `web` and `rust` checks were green at merge. Closeout reran no suites; validation above is the worker's recorded evidence plus the coordinator's exact-head verification. No failures deferred. Next: resume retained Queue task `17ac3fee-de90-4b32-9672-1134770bb086` (g18.006) onto the admitted policy, per the card.
