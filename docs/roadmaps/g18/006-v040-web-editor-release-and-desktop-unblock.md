# 006 — v0.4.0 web editor release and Desktop unblock

Status: planned — g18.005 and g18.008 merged; publication requires explicit operator release authority
Owner: Poodle release operations
Created: 2026-09-10
Governing refs: `../../contracts/001-working-rules.md`,
`../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`../../specs/044-deprecation-change-control-and-release-channel-operations.md`,
`../../specs/070-compiled-web-distribution-contract.md`,
`../../../packages/release-manifest.json`,
`../../../packages/release-operations.json`
Depends on: `g18.003`, `g18.004`, `g18.005`, `g18.008`

## Outcome

Prepare, certify, and publish Poodle `0.4.0` after the rich-text and Tabs work
has merged. Publish `@inflatable-cookie/poodle-core` and
`@inflatable-cookie/poodle-svelte` from one exact candidate/tag, retain React
as private packed validation, keep Rust crates on source/tag distribution, and
return registry-installed evidence that lets Desktop resume its retained
`g02.058` task and PR #215.

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
- [ ] Final candidate package trees and full v0.3.0-to-candidate delta are known.
- [ ] The operator explicitly authorizes candidate execution and public release
  mutation after reviewing the final package set.

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
- Repair the current Keep-a-Changelog parser incompatibility before treating
  Effigy's release status/plan as healthy. Preserve historical meaning; do not
  bypass the parser or release gates.
- Follow the repository's two-step `release.yml` protocol: exact-candidate
  branch dry run, tag only after green evidence, tag dry run, then publication.
  Any red step stops. Never retag a failed published release.

## Dispatch manifest

- **State:** planned and serial after g18.003, g18.004, g18.005, and g18.008; not dispatchable
  until Chatterbox rechecks the final source and the operator grants explicit
  release authority
- **Completion:** one reviewed release-candidate PR merged to main; exact
  candidate local gates and branch dry run green; immutable `v0.4.0` tag; tag
  dry run and publish workflow green; npm core/Svelte `0.4.0` available; fresh
  ordinary installed consumer proves `./editor`; Desktop blocker capsule sent
- **Owned mutable paths:** package and Cargo version manifests; intra-repo
  version requirements; lockfiles; `CHANGELOG.md`; `docs/release-notes/0.4.0.md`
  and release-notes index; release evidence/log; required generated package
  artifacts and release metadata
- **Reserved closeout surfaces:** g18 README, generation index, dispatch
  projection, task status/evidence, Desktop repository/task/PR
- **Worker:** release-capable high-reasoning worker comfortable with lockstep
  TypeScript/Rust versioning, packed-library certification, hosted workflow
  evidence, npm trusted publishing, and cross-repo proof
- **Excluded:** Desktop edits; React npm publication; crates.io publication;
  stable-channel claims; feature implementation; workflow edits; gate bypasses;
  windowed selectors; unrelated dependency updates
- **Escalation:** operator through Chatterbox for the final release go, any
  failed gate/tag/publication, package-set change, version change, or registry
  discrepancy

## Work

1. After g18.003 and g18.004 close, inventory the full public-intent delta from
   `v0.3.0` to current main. Record final core/Svelte tree hashes and verify the
   g18.002 editor-bearing hashes occur in their lineage.
2. Repair `CHANGELOG.md` into the supported Keep a Changelog category grammar
   without losing historical release meaning. Prove `effigy release status
   --check-gates` and release plans parse rather than bypassing them.
3. Prepare lockstep `0.4.0` manifests, intra-repository requirements, locks,
   release notes, and package evidence. Classify CodeEditor and rich-text as
   additive and Tabs card fill as behavioral.
4. Prove source-free packed core/Svelte archives, private packed React, root
   isolation, SSR/browser imports, declarations, licenses, exact dependencies,
   and a fresh ordinary installed consumer of Svelte `./editor`.
5. Open one non-draft candidate PR. Independent review must bind the exact head,
   version set, final package trees, package contents, release notes, and
   Desktop unblock oracle. The worker never merges, tags, or publishes.
6. After merge and explicit operator release authority, run local release gates
   on the exact clean pushed candidate. Dispatch `release.yml` dry-run against
   that exact commit and require green evidence before tagging.
7. Create and push `v0.4.0` only after the candidate dry run passes.
   Run the tag dry run, then the tag publication workflow. Stop on any failure.
8. Query npm until exact core and Svelte `0.4.0` metadata and tarballs are
   available. Install them in a fresh unlinked consumer and prove `./editor`
   loads with the expected declarations and browser/SSR boundaries.
9. Return Desktop Chatterbox the exact version, release/tag/candidate commits,
   final package-tree hashes, tarball integrity/registry evidence, and installed
   `./editor` result. Direct it to resume the same g02.058 task/workspace/PR for
   repin, unlink, ordinary install, exact-head review, and closeout.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Release waits for complete web admission | candidate omits rich text, Tabs, or editor preview admission | all four task merge commits are ancestors of the exact candidate |
| Version reflects additive API | manifests prepare `0.3.1` or disagree | every release-bearing TS/Rust manifest and internal requirement is `0.4.0` |
| Editor is really published | source link passes but packed `./editor` is absent | fresh registry install imports Svelte `./editor` in declared browser/SSR/type modes |
| Rich text is isolated | root or `./editor` pulls TipTap/ProseMirror | packed graph/archive isolation checks on final tarballs |
| Desktop proof is honest | report repeats obsolete g18.002 whole-tree hashes as final | lineage records old hashes; final candidate/tag/package trees and integrity are reported separately |
| Candidate is exact | CI success belongs to another head | branch dry-run `headSha` equals clean pushed candidate commit |
| Publication is gated | tag or npm publish happens before dry runs pass | ordered workflow/run IDs show candidate dry run, tag, tag dry run, publish |
| Registry is usable | npm metadata exists but tarball or export is stale | exact registry metadata, integrity, fresh install, and runtime/type import proof |
| Changelog is valid | release proceeds by skipping the parser | Effigy status/plan parses and local release gates remain green |
| Public set stays bounded | React or crates are published accidentally | registry queries plus workflow/package-set inspection show core/Svelte only |
| Desktop remains consumer-owned | Poodle worker edits PR #215 or its checkout | Poodle diff has no Desktop path; handoff contains evidence and resume instruction only |

## Stop conditions

- Stop if g18.003, g18.004, g18.005, or g18.008 is not merged and accepted.
- Stop if the final delta contains an unclassified breaking public change or
  requires a version other than `0.4.0`.
- Stop if local gates, candidate dry run, tag dry run, packing, publication, or
  installed-registry proof fails. Do not bypass or retag.
- Stop before any release mutation without explicit operator release authority.
- Stop before editing workflows, publishing React/crates, or mutating Desktop.

## Evidence

Planning intake verified on 2026-09-10:

- npm core and Svelte `latest` are both `0.3.0`; published versions are
  `0.1.0`, `0.2.1`, `0.2.2`, and `0.3.0`.
- current main preserves the g18.002 package trees: core
  `ffb16fed0a8cec7f8a2448bed2f5e680d60a5366`, Svelte
  `de3a5f55d435689f2886e524613364fa2abc785d`.
- Svelte source manifest contains `./editor`; immutable `v0.3.0` does not.
- `effigy release status --check-gates` currently stops on existing
  `CHANGELOG.md` grammar errors. The release task must repair, not bypass, this
  preflight before certification.

## Next task

After registry and installed-consumer proof, Desktop resumes its retained
g02.058 task and PR #215. Poodle returns to Chatterbox for the next GPUI repair
tranche or release/adoption need; no successor auto-starts.
