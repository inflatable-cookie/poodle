# 007 — Ordinary changelog maintenance scope

Status: ready — operator selected structural repair for the g18.005 CI blocker
Owner: Poodle distribution certification
Created: 2026-09-10
Governing refs: `../../contracts/001-working-rules.md`,
`../../specs/044-deprecation-change-control-and-release-channel-operations.md`,
`../../../test/package-install/scope.ts`,
`../../../test/package-install/README.md`
Depends on: none; unblocks g18.005 task `b02f77de-5ab0-4123-8229-0001a940b9f6`

## Outcome

Let ordinary web CI admit a changelog syntax-normalization PR only when the
range is maintenance-only and preserves release semantics. Continue rejecting
new or changed releases, versions, dates, links, Unreleased content, release
transport, registry surfaces, and mixed release preparation before builds.

After merge, retry validation on the retained g18.005 PR #238. Do not copy,
replace, close, or merge that PR from this task.

## Ready-State Rubric

- [x] PR #238 is retained at reviewed clean head
  `40f9e0a3d44f485fd24b4aa6ba55dff2bc759a51`.
- [x] Its only current CI failure is ordinary scope rejecting `CHANGELOG.md`
  before the otherwise-green 3819-test web run can complete.
- [x] Existing tests deliberately reject every ordinary changelog change, so
  retry alone cannot clear the blocker.
- [x] The operator rejected a one-off exception and selected the structural lane.
- [x] The existing g18.005 worker/handoff cannot own classifier changes; a
  separate Queue task preserves its PR, thread, workspace, and accepted review.

## Decisions

- Add a content-aware maintenance classification. Do not globally allow
  `CHANGELOG.md` in ordinary mode and do not weaken strict/candidate modes.
- Maintenance admission requires a docs-only range: `CHANGELOG.md` plus the
  task's execution log. Any product, manifest, lock, workflow, registry,
  release-note, publish-script, or other release surface keeps the range red.
- Preserve the complete release-semantic inventory: version identifiers,
  release dates, comparison/reference links, normalized entry payloads, and an
  empty Unreleased section. Any addition, removal, or changed meaning is a
  release mutation and remains forbidden.
- The classifier may use a narrow changelog-maintenance parser/canonicalizer,
  but it must fail closed on ambiguity or unparsable shapes. Do not encode PR,
  branch, commit, task, or historical-version exceptions.
- This task repairs CI policy only. It does not merge PR #238; Queue retries
  that retained task after the fix lands.

## Dispatch manifest

- **State:** ready for immediate Queue dispatch; independent of g18.003 and
  g18.004 implementation, serial before g18.005 validation retry
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** `test/package-install/scope.ts`, focused scope tests,
  the web-preview falsification harness only if required, package-install
  contract documentation, one g18.007 execution log
- **Reserved closeout surfaces:** g18 README, generation index, dispatch
  projection, g18.005 PR/branch/workspace/worker/reviewer, changelog, workflows,
  release candidate surfaces
- **Worker:** distribution-policy worker comfortable with adversarial
  content-derived classifiers and fail-closed Git-range tests
- **Excluded:** `CHANGELOG.md`; PR #238 edits or merge; workflow changes;
  package versions; candidate mode redesign; tags; publication; Desktop edits;
  product code; windowed selectors
- **Escalation:** Chatterbox for any admission wider than semantic-preserving
  docs-only maintenance or any need to edit PR #238/workflows/release state

## Work

1. Reproduce the exact ordinary-mode rejection from PR #238 and bind the
   accepted before/after changelog shapes as a test fixture without depending
   on that PR's branch at runtime.
2. Implement a fail-closed changelog maintenance classifier that compares the
   semantic inventory across the Git range and applies only to the bounded
   docs-only path set.
3. Prove the PR #238 normalization shape passes while additions, deletions, or
   edits to versions, dates, links, entries, and Unreleased content remain red.
4. Prove mixed ranges and all existing workflow, manifest-version, registry,
   publish, strict, and candidate falsification laws remain red.
5. Run focused package-install scope tests, the ordinary web-preview
   falsification surface, relevant web CI selectors, docs QA, and
   `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Only maintenance passes | any changelog edit becomes ordinary | planted real entry/version/date/link/Unreleased mutations still reject as release surfaces |
| Meaning is preserved | formatting hides a removed or rewritten historical claim | canonical before/after inventory compares normalized entry payloads, versions, dates, and links |
| Range stays docs-only | source or manifest change rides beside normalization | mixed-range plants reject before build |
| Existing release guards survive | workflow, registry, publish, strict, or candidate law is weakened | full focused classifier/falsification suite remains green with negative cases intact |
| No identity exception | code names PR #238, its head, task, branch, or old releases | source inspection plus generic planted fixture |
| Retained task resumes | new fix merges but PR #238 stays red and orphaned | Queue retry-validation evidence on task b02f77de after this task closes |

## Stop conditions

- Stop if safe admission requires a global changelog allowlist, workflow edit,
  hard-coded task identity, or weakening strict/candidate certification.
- Stop if semantic preservation cannot be decided deterministically and
  fail-closed from the compared range.
- Stop before editing, closing, replacing, or merging PR #238; before any
  release mutation; or before touching Desktop.

## Evidence

- PR #238 web run `34531894294` passed 3819 Vitest tests, then failed only at
  `assertInstalledScope` with `forbidden release surface: CHANGELOG.md`.
- Focused source inspection confirms `CHANGELOG.md` is explicitly rejected in
  ordinary mode and exempted only by the historical release-candidate mode.
- The g18.005 independent reviewer accepted exact head `40f9e0a3...` before CI
  verification exposed the scope mismatch.

## Next task

After merge and closeout, Queue retries validation on retained g18.005 task
`b02f77de-5ab0-4123-8229-0001a940b9f6`. No release task auto-starts.
