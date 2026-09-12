# 031 — Release metadata and GPUI census provenance

Status: changes requested — operator corrected root version policy on PR #263
Owner: Poodle release and evidence infrastructure
Created: 2026-09-12
Governing refs: `../../contracts/001-working-rules.md`,
`../../specs/008-parity-evidence-documented-delta-and-downstream-extension-rules.md`,
`../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`006-v040-web-editor-release-and-desktop-unblock.md`,
`../../../effigy.toml`, `../../../packages/core/package.json`,
`../../../packages/gpui/preview/Cargo.toml`,
`../../../scripts/gpui-functionality-census.ts`
Depends on: `g18.030`; retained g18.006 has no candidate commit or PR

## Outcome

Remove the last two false release identities before `0.4.0` preparation.
Bring the root repository manifest into the existing lockstep version set and
make it Effigy's explicit release authority. Make GPUI census receipts read
their package version from the preview crate manifest. Repin the current
census once, then return to the preserved g18.006 task.

## Ready-State Rubric

- [x] Retained g18.006 is clean at current main with no candidate mutation.
- [x] Root `package.json` is private workspace tooling, but its `0.1.0` version
  is misleading beside the repository's accepted `0.3.0` release.
- [x] Effigy already auto-detects the root manifest as repository release
  authority; the defect is that it was omitted from lockstep versioning.
- [x] The GPUI census generator embeds `package_version: "0.3.0"` into 65
  receipts although the preview crate manifest is the live package authority.
- [x] Both defects can be removed before the candidate without widening its
  release-only allowlist.

## Decisions

- Set explicit Effigy release authority to root `package.json`, with
  `CHANGELOG.md`, `v{version}` and the repository's pre-1.0 policy.
- Align root `package.json` to the current accepted release `0.3.0` in this
  precursor. Keep it private and preserve every non-version field.
- Extend the closed g18.006 candidate policy so root `package.json` is a
  required version-only `0.3.0` to `0.4.0` input. Reject any other root
  manifest change.
- Keep the public lockstep set unchanged: core and Svelte publish; React stays
  private; Rust remains source/tag distributed.
- Derive census `package_version` from the `[package].version` in
  `packages/gpui/preview/Cargo.toml`. Parse narrowly and fail closed on missing,
  duplicate or malformed package version declarations.
- Validate every generated expected-test receipt against the live manifest
  version. A stale checked-in receipt must fail `--check`.
- Regenerate all 65 expected-test receipts and census projections once on the
  current `0.3.0` identity. Do not rerun mounted tests; this is deterministic
  metadata generation from already accepted execution evidence.
- Correct g18.006's validation contract: `release status` is a pre-prepare
  readiness view and an empty `[Unreleased]` is expected after candidate
  preparation. The stable candidate runs `effigy release gates --json` once.
  Do not interpret a deliberately empty post-prepare Unreleased section as a
  release-gate failure or bypass a real gate.

## Dispatch manifest

- **State:** ready; independent structural precursor before retained g18.006
- **Completion:** one independently reviewed PR merged with explicit release
  authority, manifest-derived census version, planted stale-version laws and
  one deterministic current census regeneration
- **Owned mutable paths:** `effigy.toml`; root `package.json` version only;
  `test/package-install/scope.ts` and focused candidate-scope tests;
  `scripts/gpui-functionality-census.ts` and its focused tests;
  `docs/evidence/gpui/**`; one focused execution log
- **Reserved closeout surfaces:** g18 README, generation index, dispatch
  projection, g18.006/g18.009 cards and handoffs, retained g18.006 task,
  other package/Cargo versions, locks, changelog, release notes, workflows, tags,
  registries and Desktop
- **Worker:** release-infrastructure worker comfortable with Effigy release
  config, narrow manifest parsing and generated-evidence validation
- **Excluded:** versions other than the root baseline alignment; release
  candidate preparation; candidate-scope changes beyond the exact root
  version-only rule; mounted-test execution; full `qa`; workflows; tags;
  publication; Desktop mutation
- **Escalation:** Chatterbox if explicit nested package version authority is not
  supported or census regeneration changes evidence beyond package-version
  provenance

## Work

1. Align root `package.json` from `0.1.0` to current `0.3.0`, select it
   explicitly in release config, and prove Effigy resolves that version.
2. Extend the g18.006 candidate admission and its focused plants so root
   `package.json` must move only `0.3.0` to `0.4.0` with all other content
   unchanged.
3. Add pure census laws for current and planted preview manifests plus
   missing, duplicate, malformed and stale checked-in receipt versions.
4. Replace the census literal with the derived preview package version and
   bind receipt validation to it.
5. Regenerate the 65 mounted expected-test receipts and affected census
   projections from existing execution evidence. Prove only the intended
   provenance changes.
6. Run the focused candidate-scope suite, `test:gpui-census`,
   `check:gpui-census`, a read-only Effigy release status/config probe,
   `docs:lint` and `git diff --check`. Open one non-draft
   PR and report immediately; Queue owns CI, review and merge.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Repository version is truthful | root remains `0.1.0` beside release `0.3.0` | root and Effigy status resolve `0.3.0` |
| Candidate scope stays closed | root scripts/dependencies change with version | plants admit only exact version-only lockstep transition |
| Census version is live | planted preview `0.4.0` still emits `0.3.0` | focused generator law emits the planted version |
| Parsing fails closed | missing or duplicate version silently falls back | negative laws reject before writing |
| Checked-in receipts cannot drift | one receipt keeps an old version and check passes | planted stale receipt makes `--check` fail |
| Regeneration is bounded | mounted execution is rerun or claims change | deterministic generator diff changes provenance only |
| Candidate validation is stage-correct | empty post-prepare Unreleased blocks real gates | g18.006 requires one explicit `release gates --json` run |

## Stop conditions

- Stop if root alignment changes any non-version field or requires publication.
- Stop if package-version derivation needs a new dependency or receipt schema
  change.
- Stop if deterministic regeneration changes capability claims, execution
  bodies or source identities.
- Stop before version preparation, full release gates, workflows, tags,
  publication or Desktop.

## Evidence

Retained g18.006 is clean at main
`9f30dc6277581aa451c326054bee621d4ad2c330` with no PR or unique commit.
Its release probe verified two independent hard-coded identities:

- Effigy auto-detects root `package.json@0.1.0`, while the latest accepted
  release and public packages are `0.3.0`. The operator ruled that leaving
  this repository metadata stale is misleading.
- `scripts/gpui-functionality-census.ts` writes literal `0.3.0` into 65
  mounted expected-test receipts although the candidate will change
  `packages/gpui/preview/Cargo.toml` to `0.4.0`.

The root package remains private and unpublished, but its version is repository
metadata and now moves with the lockstep release. The accepted v0.3.0 candidate
ran `effigy release gates --json`; it did not use post-prepare `release status`
as its gate.

## Next task

After this PR merges, resume Queue task
`17ac3fee-de90-4b32-9672-1134770bb086` in its retained workspace. Prepare the
immutable `0.4.0` candidate and run the complete local release gate once.
g18.009 remains dependency-queued behind that task and owns hosted branch/tag
proof, publication and the Desktop return.
