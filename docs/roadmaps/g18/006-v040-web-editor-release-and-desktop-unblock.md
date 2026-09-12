# 006 — v0.4.0 web editor release candidate

Status: blocked — retained Queue task/workspace; resume after g18.028 repairs the Rust release gate
Owner: Poodle release operations
Created: 2026-09-10
Governing refs: `../../contracts/001-working-rules.md`,
`../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`../../specs/044-deprecation-change-control-and-release-channel-operations.md`,
`../../specs/070-compiled-web-distribution-contract.md`,
`../../../packages/release-manifest.json`,
`../../../packages/release-operations.json`
Depends on: `g18.003`, `g18.004`, `g18.005`, `g18.008`, `g18.010`, `g18.011`,
`g18.012`, `g18.013`, `g18.014`, `g18.015`, `g18.016`, `g18.017`, `g18.018`,
`g18.019`, `g18.020`, `g18.021`, `g18.022`, `g18.023`, `g18.024`, `g18.025`,
`g18.026`, `g18.027`; current continuation gate: `g18.028`

## Outcome

Prepare and certify the immutable Poodle `0.4.0` candidate after the rich-text,
Tabs and preview work has merged. Merge one independently reviewed candidate
tree that g18.009 can tag and publish. Do not release from this task.

Do not publish before all dependencies close. Do not mutate Desktop.

## Ready-State Rubric

- [x] Desktop's source-linked `./editor` consumer proof is green and its only
  adoption blocker is npm `latest=0.3.0` without the editor entry.
- [x] The g18.002 merge introduced additive public `./editor` functionality.
- [x] The operator directed release work to wait for both rich text and Tabs.
- [x] `0.4.0` follows the repository's SemVer posture: new public `./editor`
  and `./rich-text` entries are additive minor-version functionality.
- [x] Current g18.002 package trees are verified on main as core
  `ffb16fed0a8cec7f8a2448bed2f5e680d60a5366` and Svelte
  `de3a5f55d435689f2886e524613364fa2abc785d`.
- [x] The release and Desktop proof boundaries are explicit.
- [x] g18.003, g18.004, g18.005, and g18.008 have merged with
  accepted exact-head review.
- [x] Final source package trees and the full v0.3.0-to-current-main delta are
  known. Candidate-time trees remain worker evidence after version preparation.
- [x] The operator explicitly authorized candidate execution and public release
  mutation on 2026-09-11 to unblock Desktop g02.058.

## Decisions

- Use `0.4.0`. `0.3.1` is rejected because this release adds public package
  entry points rather than only correcting compatible implementation defects.
- Release the accumulated public-intent delta since immutable `v0.3.0`, not an
  editor-only synthetic commit.
- Public npm publication remains core plus Svelte. React stays private and
  packed; Rust crates remain source/tag distribution under the common version.
- The release candidate must preserve the g18.002 CodeEditor seam while also
  including the final g18.003 RichTextEditor/Renderer and g18.004 Tabs result.
- The final core and Svelte tree hashes will therefore supersede Desktop's
  g18.002-only hashes. Evidence must preserve those hashes as lineage, report
  final candidate/tag/package trees, and prove installed `./editor` behavior.
- g18.028 repins the 29 source-bound Nucleus M1/A1 receipts so its own web gate
  remains green. Repin the complete cohort again in this task after the `0.4.0`
  Cargo manifest changes, then require the whole parity-ledger, GPUI-census and
  release gate to pass against the final candidate identity.
- Include the g18.022 pre-v1 Slider-family migration. Release notes must name
  the new default `variant="block"`, retained `variant="embedded"`, and removed
  `appearance`, `standard`, `track`, and combined RangeSlider visible-range
  formatting surface. Do not add compatibility aliases in the candidate.
- Include the accepted g18.023 dual syntax palettes. Do not release the
  temporary accent/status mapping from g18.021 as the final CodeEditor theme.
- Consume the accepted g18.027 public-surface freeze report from the exact
  post-g18.026 main head. Candidate preparation must recheck its ancestry and
  delta identity before changing any release-bearing input.
- Repair the current Keep-a-Changelog parser incompatibility before treating
  Effigy's release status/plan as healthy. Preserve historical meaning; do not
  bypass the parser or release gates.
- Preserve the repository's two-step `release.yml` protocol. This candidate
  task owns the exact-candidate branch dry run only; g18.009 owns the later tag,
  tag dry run and publication sequence.

## Dispatch manifest

- **State:** blocked in retained dispatched Queue task/workspace after rebasing
  onto accepted g18.026/g18.027 main. Its release gate found two stale Rust
  accessibility assertions. Resume only after separate g18.028 merges. The
  task's dependency list froze on first dispatch, so preserve the same task,
  worker and workspace; do not replace them or pretend a later dependency
  mutation is available.
- **Completion:** one reviewed release-candidate PR merged to main; exact
  candidate local gates and branch dry run green; final version set, package
  trees, packed archives and release notes recorded for g18.009
- **Owned mutable paths:** package and Cargo version manifests; intra-repo
  version requirements; lockfiles; `CHANGELOG.md`; `docs/release-notes/0.4.0.md`
  and release-notes index; release evidence/log; required generated package
  artifacts and release metadata; the complete source-bound Nucleus receipt,
  ledger and GPUI-census repin required by the final `packages/render` tree
- **Reserved closeout surfaces:** g18 README, generation index, dispatch
  projection, task status/evidence, Desktop repository/task/PR
- **Worker:** release-candidate high-reasoning worker comfortable with lockstep
  TypeScript/Rust versioning, packed-library certification and hosted dry-run
  evidence
- **Excluded:** Desktop edits; React npm publication; crates.io publication;
  stable-channel claims; feature implementation; workflow edits; gate bypasses;
  windowed selectors; unrelated dependency updates
- **Escalation:** operator through Chatterbox for any failed candidate gate,
  package-set change, version change or unclassified public delta

## Work

1. After g18.027 closes, consume its full public-intent delta from `v0.3.0` to
   current main. Record final core/Svelte tree hashes and verify the
   g18.002 editor-bearing hashes occur in their lineage.
2. Repair `CHANGELOG.md` into the supported Keep a Changelog category grammar
   without losing historical release meaning. Prove `effigy release status
   --check-gates` and release plans parse rather than bypassing them.
3. Prepare lockstep `0.4.0` manifests, intra-repository requirements, locks,
   release notes, and package evidence. Classify CodeEditor and rich-text as
   additive, Tabs card fill as behavioral, and the g18.022 Slider-family API
   replacement as an explicit pre-v1 breaking migration.
4. Regenerate and repin all 29 Nucleus M1/A1 receipts, the parity ledger and
   GPUI census against the final candidate `packages/render` source identity.
   Refuse a partial cohort, mixed source commits or a second repin source.
5. Prove source-free packed core/Svelte archives, private packed React, root
   isolation, SSR/browser imports, declarations, licenses, exact dependencies,
   and a fresh source-free consumer installed from the packed Svelte archive.
6. Open one non-draft candidate PR. Independent review must bind the exact head,
   version set, final package trees, package contents, release notes, and
   Desktop unblock oracle. The worker never merges, tags, or publishes.
7. Return the exact candidate identity for queue review and merge. Closeout
   passes the merged identity to g18.009. Do not tag, publish or mutate npm.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Release waits for complete web admission | candidate omits rich text, Tabs, or editor preview admission | all four task merge commits are ancestors of the exact candidate |
| Version reflects additive API | manifests prepare `0.3.1` or disagree | every release-bearing TS/Rust manifest and internal requirement is `0.4.0` |
| Editor is in the candidate | source link passes but packed `./editor` is absent | source-free packed install imports Svelte `./editor` in browser/SSR/type modes |
| Rich text is isolated | root or `./editor` pulls TipTap/ProseMirror | packed graph/archive isolation checks on final tarballs |
| Desktop proof is honest | report repeats obsolete g18.002 whole-tree hashes as final | lineage records old hashes; final candidate/tag/package trees and integrity are reported separately |
| Candidate is exact | CI success belongs to another head | branch dry-run `headSha` equals clean pushed candidate commit |
| Publication stays separate | candidate task creates a tag or changes npm | tag absence and no release workflow dispatch; g18.009 owns release mutation |
| Changelog is valid | release proceeds by skipping the parser | Effigy status/plan parses and local release gates remain green |
| Nucleus evidence binds the candidate once | receipts remain at the predecessor commit or mix repair/version identities | all 29 receipts, ledger and census resolve to the final candidate source commit and the complete release gate is green |
| Public set stays bounded | candidate config admits React or crates | manifest, pack and workflow package-set inspection show core/Svelte only |
| Desktop remains consumer-owned | Poodle worker edits PR #215 or its checkout | Poodle diff has no Desktop path; handoff contains evidence and resume instruction only |

## Stop conditions

- Stop if g18.003, g18.004, g18.005, or g18.008 is not merged and accepted.
- Stop if the final delta contains an unclassified breaking public change or
  requires a version other than `0.4.0`.
- Stop if local gates, candidate dry run or packing fails. Do not bypass.
- Stop before every tag, publication or registry mutation; g18.009 owns them.
- Stop before editing workflows, publishing React/crates, or mutating Desktop.

## Evidence

Planning intake verified on 2026-09-10:

- npm core and Svelte `latest` are both `0.3.0`; published versions are
  `0.1.0`, `0.2.1`, `0.2.2`, and `0.3.0`.
- current main preserves the g18.002 package trees: core
  `ffb16fed0a8cec7f8a2448bed2f5e680d60a5366`, Svelte
  `de3a5f55d435689f2886e524613364fa2abc785d`.
- Svelte source manifest contains `./editor`; immutable `v0.3.0` does not.
- g18.005 plus g18.007 repaired the existing Keep-a-Changelog parser conflict;
  PR #238 merged after release status/planning and the bounded preflight passed.

Pre-correction promotion recheck on 2026-09-11:

- current pushed main is `70b59890cb3f069d9f43139bf1072dbcc2ff7364`;
- g18.002, g18.003, g18.004, g18.005 and g18.008 accepted merges are all
  ancestors;
- source trees before the g18.010 focus correction were core
  `86412853ee4b490f5630e9573ebaa0abf3f77c17`, Svelte components
  `a76b9b5163469aadedab78ba8c471086df777d1b`, and React components
  `3a662d815efc0e158943579f4a9b4d0c8070eb96`;
- the accumulated public-intent delta from immutable `v0.3.0` contains the
  accepted CodeEditor, rich-text, Tabs and paired preview work; and
- Tom explicitly authorized this release lane to unblock Desktop g02.058.

These identities are no longer final after the operator-required g18.010 UX
repair. The retained worker must recompute them after rebasing onto its merge.

## Next task

Complete g18.028's test-only Rust accessibility assertion repair. Resume this
retained task only after that PR merges and the full library/release gate is
green. The Queue cannot add dependencies to this already-dispatched task, so
its blocked state is the explicit serial gate; preserve its worker and workspace.
After the candidate merges and closes, g18.009 dispatches from its existing
dependency. Desktop then resumes retained g02.058 and PR #215.
