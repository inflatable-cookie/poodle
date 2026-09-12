# 027 — `v0.4.0` public-surface freeze audit

Status: dependency-ready behind g18.026
Owner: Poodle release operations
Created: 2026-09-12
Governing refs: `../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`../../specs/044-deprecation-change-control-and-release-channel-operations.md`,
`../../specs/070-compiled-web-distribution-contract.md`,
`../../../packages/release-manifest.json`
Depends on: g18.026 complete and merged

## Outcome

Bind the final pre-release public delta from immutable `v0.3.0` to accepted
post-g18.026 main. Classify every export, package entry, prop/type removal,
default change, dependency/peer constraint and observable contract migration
before g18.006 prepares `0.4.0`.

This is an audit and freeze gate, not a feature or release task.

## Ready-State Rubric

- [x] The operator requires all remaining deliberate breaks before `0.4.0`,
  followed by compatible-only `0.4.1` work.
- [x] Known migrations include the Slider-family API replacement and Markdown
  safe-HTML default.
- [x] Additive editors, renderers and package entry points must be separated
  from behavioral and breaking changes in release evidence.
- [x] g18.006 already stops on an unclassified public break but must not be the
  first place one is discovered.
- [ ] g18.026 is merged and its exact main head is known.

## Decisions

- Compare immutable tag `v0.3.0` with the exact accepted post-g18.026 main
  head. Do not use an intermediate task branch or stale package tree.
- Audit public core and Svelte packages, private packed React parity, Rust
  source/tag contracts, public CSS custom properties, component defaults and
  package dependency/peer constraints.
- Classify each delta as additive, compatible behavioral, breaking pre-v1,
  internal-only or evidence/docs-only. Every breaking row needs a migration
  statement for g18.006 release notes.
- Known breaking rows start with: Slider/RangeSlider `variant` replacement and
  removed presentation/combined-range vocabulary; Markdown safe HTML default;
  removed dead exported Slider fallback helpers. Verify rather than assume the
  list is complete.
- Tabs inactive card fill is behavioral unless the audit proves a public
  contract removal. CodeEditor, RichTextEditor/Renderer, MarkdownRenderer,
  language-provider adapters and syntax palettes are expected additive rows.
- Do not repair product code here. A newly found required breaking change stops
  the audit and returns a bounded decision capsule to Chatterbox. Compatible
  defects may be deferred to the post-release sweep.
- The accepted report freezes `0.4.0`: no further public break may enter the
  candidate. Later desired breaks target `0.5.0`.

## Dispatch manifest

- **State:** dependency-ready; submit only after g18.026 has a Queue identity
- **Completion:** one clean pushed audit PR with exact-head independent review,
  merged report/log and no product or release mutation
- **Owned mutable paths:** one `0.4.0` public-delta evidence report under
  `docs/evidence/releases/`; required evidence index update; focused audit
  scripts/tests only if no existing reproducible route can express the check;
  one execution log
- **Reserved closeout surfaces:** component/product code; package versions;
  locks; changelog and release notes; workflows; g18 README/index/dispatch/task
  state; retained g18.006/g18.009; Desktop
- **Worker:** release-aware high-reasoning auditor comfortable with TypeScript
  declarations, Svelte/React package exports, Rust APIs and SemVer classification
- **Excluded:** feature implementation; compatibility shims; candidate prep;
  tag/publish; consumer mutation; broad post-release UX sweep
- **Escalation:** Chatterbox for every unclassified break or required source fix

## Work

1. Record immutable baseline and final main identities, including package-tree
   hashes and release-manifest scope.
2. Build/inspect declarations and packed export maps for core, Svelte and
   private React; compare props, types, helpers, defaults and subpaths.
3. Compare Rust public declarations and component contract defaults.
4. Inspect CSS custom-property removals and package dependency/peer changes
   that affect consumers.
5. Produce a complete classified delta table with consumer migration text and
   explicit known-safe additive rows.
6. Falsify the report with planted removed export/default/package-entry cases
   if existing tooling cannot already prove those classes.
7. Run the narrow audit selectors, package builds needed for declarations,
   docs QA and `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Baseline is immutable | report compares against moving main | exact `v0.3.0` tag and post-g18.026 main SHAs/tree hashes |
| Every public surface is covered | package JSON is checked but declarations/defaults are missed | package/export/type/default/CSS/Rust inventory with reproducible commands |
| Breaks are explicit | removed Slider helper or safe HTML default is called additive | row-level classification and migration text |
| Additions are not overstated | private React or web-only editor claims public native support | release-manifest and runtime-boundary classification |
| Audit cannot mutate the candidate | versions or release notes change here | exact path diff excludes release-bearing inputs |
| Freeze is enforceable | a later break silently enters g18.006 | candidate rechecks report head ancestry and delta identity |

## Stop conditions

- Stop on any unclassified public change or required product repair.
- Stop before editing versions, locks, changelog, release notes, workflows,
  Desktop or retained Queue state.
- Stop if the final comparison head does not contain accepted g18.026.

## Next task

After this audit merges without an open breaking decision, resume retained
g18.006 on current main. Its candidate evidence consumes this report, prepares
`0.4.0`, and hands the immutable candidate to g18.009 for publication and the
Desktop unblock.
