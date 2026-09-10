# g18.005 — v0.4.0 release preflight

Status: ready_for_review
Date: 2026-09-10
Branch: `ns-b02f77de-5ab0-4123-8229-0001a940b9f6`
Card: `docs/roadmaps/g18/005-v040-release-preflight.md`
Handoff: `docs/handoffs/20260910-g18-005-v040-release-preflight.md`
Planning commit: `70c1b8c8a3235863d97dd549d413bffda4fe9dff`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/specs/022-packaging-versioning-and-release-channel-rules.md`,
`docs/specs/044-deprecation-change-control-and-release-channel-operations.md`,
`packages/release-manifest.json`, `packages/release-operations.json`,
`docs/roadmaps/g18/006-v040-web-editor-release-and-desktop-unblock.md`

## Outcome

`CHANGELOG.md` now parses and validates under Effigy's Northstar Changelog
Profile with every historical release fact preserved, and ordinary Effigy
release status/plan inspection reports repository release state instead of
stopping on changelog grammar errors. No version, tag, workflow, registry,
manifest, lock, release-note, or Desktop mutation occurred. Final `0.4.0`
candidate work remains owned by g18.006.

## What changed

Only `CHANGELOG.md` plus this execution log. Normalizations (structure only;
entry text otherwise verbatim):

- Removed the `Nothing yet.` placeholder under `## [Unreleased]`; an empty
  Unreleased section carries the identical meaning and is what the profile
  expects before entries accumulate.
- Converted release-level prose into category entries with bold lead-ins that
  preserve the removed heading/paragraph labels: 0.3.0 and 0.2.3 status
  paragraphs became `**Release status.**` entries and the 0.2.0 denominator
  paragraph became a `**Release posture.**` entry, all under `### Changed`;
  the 0.2.2 trailing web-package paragraph became a `### Changed` bullet.
- Split `### Added and changed` (0.3.0) into `### Added` (the public-intent
  delta entry) and `### Changed` (the versioning-set and web-package-boundary
  entries).
- Renamed `### Downstream checks` (0.3.0) into two `**Downstream checks.**`
  entries under `### Changed`; both bullets are verbatim.
- `## [0.2.3] - 2026-08-30 (prepared — unpublished)` became
  `## [0.2.3] - 2026-08-30`; the strict profile date cannot carry a suffix,
  and the prepared-but-unpublished status was already carried by the new
  `**Release status.**` entry, so no fact was lost.

No release-notes files, manifests, locks, workflows, or product code were
touched; no doc references pointed at the changed changelog headings.

## Semantic inventory proof

An automated before/after comparison bound every content unit of the old
file (bullets with continuations, prose paragraphs, link references) against
the new file: all matched verbatim except the five label-carrying
conversions above. Version set and dates are unchanged: Unreleased,
0.3.0 (2026-09-05), 0.2.3 (2026-08-30), 0.2.2 (2026-08-24), 0.2.1
(2026-08-23), 0.2.0 (2026-08-23), 0.1.0 (2026-07-24), descending, no
duplicates. All seven link references and inline release-note links are
unchanged.

## Read-only commands and parser proof

- `effigy release status --check-gates` (before): stopped on 31 changelog
  parse errors — lines 10, 14, 34, 37–49, 51, 54–57, 61–62, 114–117, 131–137
  (prose outside categories, unknown categories `Added and changed` and
  `Downstream checks`).
- `effigy changelog validate CHANGELOG.md`: `CHANGELOG.md: valid ✓`.
- `effigy changelog analyze CHANGELOG.md`: Unreleased empty, suggested bump
  none.
- `effigy changelog extract CHANGELOG.md --version 0.2.3`: extracts the
  release body including the preserved release-status entry.
- `effigy release status`: parses; `Changelog valid: yes`; reports the
  honest pre-candidate blockers recorded below.
- `effigy release prepare --plan` (plan-only, non-destructive): parses;
  reports the same blockers plus `no next version could be derived from
  changelog content` (correct while Unreleased is empty).
- `effigy docs check links|paths|forbidden CHANGELOG.md`: pass.
- `git diff --check`: clean.
- Read-only registry inspection: npm `@inflatable-cookie/poodle-core`
  latest `0.3.0`; `@inflatable-cookie/poodle-svelte` latest `0.3.0`.
- Local tags unchanged: `v0.1.0`, `v0.2.0`, `v0.2.2`, `v0.3.0`.

## Provisional 0.4.0 rationale

`0.4.0` remains the g18.006-planned target because g18.002 already added
additive public `./editor` entries and g18.003/g18.004 add further public
editor/tabs functionality; a minor bump fits the repository's pre-1.0 SemVer
posture. Classification of the complete post-`v0.3.0` delta — and the
decision to release — stays with g18.006 after g18.003 and g18.004 merge.

## Unchanged release state

- npm core and Svelte `latest` remain `0.3.0` (published versions `0.1.0`,
  `0.2.1`, `0.2.2`, `0.3.0`).
- Tags remain `v0.1.0`, `v0.2.0`, `v0.2.2`, `v0.3.0`; no new tag.
- `.github/workflows/` untouched; no workflow dispatch; no publication.
- Package manifests, Cargo manifests, lockfiles, release notes, and the
  release manifest/operations files show no diff.

## Release-status blockers at candidate time (honest state, pre-existing)

- `version file reports 0.1.0 but latest changelog release is 0.3.0` —
  Effigy resolves the root `package.json` (`poodle`, private workspace) as
  the version source; fixing that alignment is g18.006 versioning work.
- `unreleased changelog section has no entries` — correct until g18.006
  inventories the delta.

## Gates deferred to g18.006

Final candidate trees and hashes, full `v0.3.0`→candidate delta
classification, version alignment across the package set, `0.4.0` release
notes, local release gates (`effigy qa` headless board, configured as the
single release gate), branch and tag dry runs, tagging, publication, and
installed-registry consumer proof. Preflight certification is not claimed;
no gate was bypassed or suppressed.

## Validation

`effigy changelog validate` valid; `release status` and `release prepare
--plan` parse cleanly (plan-only); changelog link/path/forbidden checks
pass; semantic inventory proof above; `git diff --check` clean; only
`CHANGELOG.md` and this log changed.
