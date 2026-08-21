# g15.048 Packed Roster Reachability

Date: 2026-08-22
Card: `../../roadmaps/g15/048-packed-roster-reachability.md`
Governing refs: `../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`../../roadmaps/g15/release-baseline-roster.md`,
`../../roadmaps/g15/013-v020-release-certification.md`
Handoff: `../../handoffs/20260821-234034-g15-048-packed-roster-reachability.md`
Worker branch: `t3code/improve-packed-roster-reachability`

## Outcome

`test:web-pack-install` now proves the complete packed web boundary in the
existing clean-consumer seam. It packs core, Svelte, and React, installs only
those tarballs plus declared framework/test dependencies, rejects sibling
source resolution and workspace dependencies, and runs the retained runtime
fixtures.

The new checkable inventory parser derives the 175-name denominator from the
frozen Svelte roster and checks both package-root indexes. The generated clean
consumer proof imports both installed roots as namespaces, compares exact
component and runtime export sets, and reports missing or extra names. It does
not mount the full roster. React's non-roster root exports are an explicit,
bounded authority, and the runner includes an in-memory regression that rejects
`AccidentalExtraComponent` by its exact name.

## Packed Boundary Evidence

The actual archives and installed roots passed these checks:

| Package | Version | Tarball entries | Declared wildcard matches |
| --- | ---: | ---: | --- |
| `@inflatable-cookie/poodle-core` | 0.1.0 | 391 | 166 styles; 106 generated icon modules |
| `@inflatable-cookie/poodle-svelte` | 0.1.0 | 214 | 176 `src/*.svelte` entries |
| `@inflatable-cookie/poodle-react` | 0.1.0 | 208 | none |

Manifest-derived exact export targets were checked for every package,
including package manifests, READMEs, licences, source entry points, type
entry points, styles, generated tokens, generated icons, and the core's 32
exact public subpaths. The installed Svelte `./types` subpath was also
resolved. The clean roots expose exactly 175 Svelte and 175 React component
names.

The same selector was rerun against a temporary v0.2.0 candidate mutation of
all three release package manifests and their core dependency pins. It packed,
installed, and emitted `0.2.0` tarballs for all three packages with the same
20-test pass and exact roster proof. The manifests were restored before the
worker commit.

## Representative Runtime Evidence

The existing representative fixture set remains bounded and separate from
root-import evidence. Its declared proof lists 9 Svelte and 13 React
components, covering licence and model-connection styles, core token/icon
helpers, DockRegion external drag and reorder, MeterSurface/AudioMeter,
overlay geometry, the state-aware Popover/Button trigger, Pill appearance,
React render props, IconProvider, and AgentPlan/AgentPlanRecord. Existing
supplementary Popover, Pill, and late-g15 package fixtures remain in the
consumer suite; none is presented as full-roster behavior evidence.

## Package Impact (spec 022)

- Changed packages: none. Package metadata, versions, dependencies, exports,
  and publication posture are unchanged.
- Changed surfaces: `test/package-install/` and this release evidence only.
- Public-entry-point impact: none; the proof validates existing public entry
  points and does not add compatibility aliases or fallbacks.
- Downstream re-check: consumers should continue to install the preview
  tarballs and use the declared package-root/subpath exports; this batch adds
  no migration requirement.

## Validation

| Check | Result |
| --- | --- |
| `effigy test:web-pack-install` | pass — 10 files, 20 tests; 175/175 Svelte and 175/175 React |
| `effigy test:web-pack-install` with temporary v0.2.0 package mutation | pass — 10 files, 20 tests; emitted `0.2.0` tarballs for core, Svelte, and React |
| Tarball archive and installed-root boundary checks | pass — exact targets and required files |
| `effigy check:svelte-components` | pass — 0 errors, 4 existing warnings |
| `effigy react:build` | pass |
| `effigy docs:check` | pass |
| `git diff --check origin/main...HEAD` | pass |

No workflow, release, version, publication, specimen, component, native,
windowed, Jetstream, or GPUI preview surface changed or ran.
