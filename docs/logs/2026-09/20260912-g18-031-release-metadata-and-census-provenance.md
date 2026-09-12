# g18.031 — Release metadata and GPUI census provenance

Status: ready for review — implementation and focused validation complete;
Queue owns CI, review and merge
Date: 2026-09-12
Branch: `ns-e58c6d92-5f97-4263-8bc6-7756f749727a`
Card: `docs/roadmaps/g18/031-release-metadata-and-census-provenance.md`
Handoff: `docs/handoffs/20260912-230744-g18-031-release-metadata-and-census-provenance.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/specs/022-packaging-versioning-and-release-channel-rules.md`,
`docs/roadmaps/g18/006-v040-web-editor-release-and-desktop-unblock.md`
Base: pushed `main` at `086e8b506f958309b6797d92cb2e704140ff0374`
(planning promotion that carries this handoff)

## Outcome

The two false release identities are removed before `0.4.0` preparation.

`effigy.toml` now carries an explicit `[release]` block: `version-file =
"packages/core/package.json"`, `changelog = "CHANGELOG.md"`, `tag-format =
"v{version}"`, `pre-1-0 = true`. The existing
`[release.gates.headless]` (`effigy qa`) gate is preserved byte-for-byte. Effigy
no longer auto-detects the private root workspace manifest, so release status
reports the public lockstep core package instead of `poodle@0.1.0`. Root
`package.json` is untouched and stays private workspace tooling.

`scripts/gpui-functionality-census.ts` no longer embeds `package_version:
"0.3.0"`. The generator parses `[package].version` from
`packages/gpui/preview/Cargo.toml` and injects it into every mounted
expected-test receipt through an extracted, pure
`expectedTestReceiptContent(...)` builder. Checked-in receipts are validated
against the live manifest version, so a stale receipt fails `--check` with its
own provenance message. No receipt schema field, claim, source commit, lock
hash, run ID or test body changed.

## Release authority proof

Read-only probe, `effigy release status --json` (exit non-zero only because the
pre-prepare `[Unreleased]` section is empty, which is the stage-correct state):

- `current_version`: `0.3.0`
- `version_source`: `packages/core/package.json`, format `package.json`, path
  `version`
- `changelog`: root `CHANGELOG.md`, valid
- `gates`: 1 configured gate, unchanged
- sole blocker: `unreleased changelog section has no entries` (no
  version-mismatch blocker; the former
  `version file reports 0.1.0 but latest changelog release is 0.3.0` is gone)

No root `package.json`, package version, Cargo version, lock, changelog,
release note, g18.029 policy, workflow, tag or registry surface was changed.

## Census provenance proof

- `parsePreviewPackageVersion` reads exactly one `[package]` table and exactly
  one quoted-semver `version` key inside it. Missing, duplicate `[package]`,
  duplicate `version`, unquoted and non-semver values all reject before any
  receipt is produced.
- `loadPreviewPackageVersion` is the generator's single version source; the
  planted manifest `version = "0.4.0"` yields receipt content with
  `"package_version": "0.4.0"` (focused law).
- `validateReceiptPackageVersion` binds every checked-in receipt to the live
  manifest; a planted stale `Button` receipt (`0.2.0` against live `0.3.0`)
  fails `--check` with
  `Mounted receipt for Button records package version 0.2.0 but the preview manifest is 0.3.0; regenerate.`
- A deterministic regeneration at the current `0.3.0` identity produced a
  **zero-line diff**: 176 rows, 73 rows with admitted capabilities, 65 mounted
  receipts, byte-identical to the checked-in cohort. Deriving the version
  changed provenance only — no capability claim, source commit, lock hash, run
  ID or test body moved.
- A live planted `Cargo.toml` version is additionally caught upstream by the
  Nucleus receipt authority (`manifest version 0.3.0 does not match Cargo.toml
  0.4.0`), so the manifest is not the only independent release check; the
  manifest was restored and `Cargo.toml` is unmodified.

## Validation

Budget: `test:gpui-census`, `check:gpui-census`, one read-only release
status/config probe, `docs:lint`, `git diff --check`. No mounted tests, full
`qa`, release gates or windowed conformance selectors were run.

- `effigy test:gpui-census` — 21 pass / 0 fail (17 existing oracles plus 4 new
  g18.031 release-provenance laws).
- `effigy check:gpui-census` — checked-in artifacts match the generator and all
  oracles hold.
- `effigy release status --json` — resolves core `0.3.0` (read-only).
- `effigy docs:lint` — pass (185 component contracts and the surrounding docs
  surfaces). The clean worktree first needed the gitignored distribution
  prerequisites for the lint's export checks: `core:build`, then
  `svelte:package` and `react:package`. These are bootstrap builds, not an
  additional board; `docs:check` was not run.
- `git diff --check` — clean.

## Limits

This PR does not prepare the `0.4.0` candidate, bump any version, resume
retained g18.006, run the release gate, mutate Desktop, edit workflows, tag or
publish. It also does not change `scripts/parity-evidence-ledger.ts`; its
`0.3.0` fallback only applies when no Nucleus row exists and is outside this
card's census scope. The retained task
`17ac3fee-de90-4b32-9672-1134770bb086` remains preserved with no candidate
commit or PR.
