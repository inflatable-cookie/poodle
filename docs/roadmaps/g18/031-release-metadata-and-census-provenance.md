# 031 — Release metadata and GPUI census provenance

Status: ready — structural precursor to retained g18.006
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
Make Effigy read the repository release version from the public core package,
and make GPUI census receipts read their package version from the preview
crate manifest. Repin the current census once, then return to the preserved
g18.006 task.

## Ready-State Rubric

- [x] Retained g18.006 is clean at current main with no candidate mutation.
- [x] Root `package.json` is private workspace tooling at `0.1.0`, not a
  published Poodle release input.
- [x] `packages/core/package.json` is public, lockstep-versioned and already in
  the closed candidate surface.
- [x] Effigy currently auto-detects the unrelated root manifest and reports a
  false `0.1.0`/`0.3.0` mismatch.
- [x] The GPUI census generator embeds `package_version: "0.3.0"` into 65
  receipts although the preview crate manifest is the live package authority.
- [x] Both defects can be removed before the candidate without widening its
  release-only allowlist.

## Decisions

- Set explicit Effigy release authority to
  `packages/core/package.json`, with `CHANGELOG.md`, `v{version}` and the
  repository's pre-1.0 policy. Do not bump the root private workspace version
  or add it to the release set.
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
- **Owned mutable paths:** `effigy.toml`;
  `scripts/gpui-functionality-census.ts` and its focused tests;
  `docs/evidence/gpui/**`; one focused execution log
- **Reserved closeout surfaces:** g18 README, generation index, dispatch
  projection, g18.006/g18.009 cards and handoffs, retained g18.006 task,
  package/Cargo versions, locks, changelog, release notes, workflows, tags,
  registries and Desktop
- **Worker:** release-infrastructure worker comfortable with Effigy release
  config, narrow manifest parsing and generated-evidence validation
- **Excluded:** version bumps; release candidate preparation; installed-package
  scope changes; mounted-test execution; full `qa`; workflows; tags;
  publication; Desktop mutation
- **Escalation:** Chatterbox if explicit nested package version authority is not
  supported or census regeneration changes evidence beyond package-version
  provenance

## Work

1. Add focused release-config proof that Effigy resolves current version
   `0.3.0` from `packages/core/package.json`, not root `package.json`.
2. Add pure census laws for current and planted preview manifests plus
   missing, duplicate, malformed and stale checked-in receipt versions.
3. Replace the census literal with the derived preview package version and
   bind receipt validation to it.
4. Regenerate the 65 mounted expected-test receipts and affected census
   projections from existing execution evidence. Prove only the intended
   provenance changes.
5. Run `test:gpui-census`, `check:gpui-census`, a read-only Effigy release
   status/config probe, `docs:lint` and `git diff --check`. Open one non-draft
   PR and report immediately; Queue owns CI, review and merge.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Release authority is public | Effigy still reads root private `0.1.0` | status/config probe resolves core `0.3.0` |
| Candidate scope stays closed | root manifest is bumped or admitted | no root manifest or g18.029 policy diff |
| Census version is live | planted preview `0.4.0` still emits `0.3.0` | focused generator law emits the planted version |
| Parsing fails closed | missing or duplicate version silently falls back | negative laws reject before writing |
| Checked-in receipts cannot drift | one receipt keeps an old version and check passes | planted stale receipt makes `--check` fail |
| Regeneration is bounded | mounted execution is rerun or claims change | deterministic generator diff changes provenance only |
| Candidate validation is stage-correct | empty post-prepare Unreleased blocks real gates | g18.006 requires one explicit `release gates --json` run |

## Stop conditions

- Stop if Effigy cannot use the nested core manifest as explicit version file.
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
  release and public packages are `0.3.0`.
- `scripts/gpui-functionality-census.ts` writes literal `0.3.0` into 65
  mounted expected-test receipts although the candidate will change
  `packages/gpui/preview/Cargo.toml` to `0.4.0`.

The root package is private workspace tooling. The public core manifest is
already a required lockstep candidate input, so selecting it explicitly fixes
release status without adding a new version surface. The accepted v0.3.0
candidate ran `effigy release gates --json`; it did not use post-prepare
`release status` as its gate.

## Next task

After this PR merges, resume Queue task
`17ac3fee-de90-4b32-9672-1134770bb086` in its retained workspace. Prepare the
immutable `0.4.0` candidate and run the complete local release gate once.
g18.009 remains dependency-queued behind that task and owns hosted branch/tag
proof, publication and the Desktop return.
