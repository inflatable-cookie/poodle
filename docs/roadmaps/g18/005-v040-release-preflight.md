# 005 — v0.4.0 release preflight

Status: verifying — PR #238 retained; blocked on g18.007 scope repair
Owner: Poodle release operations
Created: 2026-09-10
Governing refs: `../../contracts/001-working-rules.md`,
`../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`../../specs/044-deprecation-change-control-and-release-channel-operations.md`,
`../../../packages/release-manifest.json`,
`../../../packages/release-operations.json`
Depends on: g18.007 for CI validation only; implementation and review complete

## Outcome

Remove the known non-mutating release-preflight blockers before the final
`0.4.0` source exists. Normalize the existing changelog into Effigy's supported
Keep a Changelog grammar, prove release status and planning can inspect the
repository, and record the exact candidate-time checks still owed by g18.006.

Do not version packages, create release notes for `0.4.0`, tag, publish, edit a
workflow, or mutate Desktop.

## Ready-State Rubric

- [x] Live npm latest remains `0.3.0` for core and Svelte.
- [x] `effigy release status --check-gates` currently stops at existing
  `CHANGELOG.md` grammar errors before it can report useful release state.
- [x] Changelog normalization is independent of the active rich-text and Tabs
  implementation paths.
- [x] The operator approved doing bounded preflight work in parallel.
- [x] Publication remains separately gated in g18.006.

## Decisions

- Preserve every historical release fact while translating headings and prose
  into the parser-supported Keep a Changelog shape.
- Fix repository content, not Effigy or CI. Do not weaken, skip, or special-case
  the release parser.
- Treat `0.4.0` as a provisional target confirmed by the additive public editor
  entries. Final version classification remains g18.006's responsibility after
  both component tasks merge.
- Read-only status, plan, package-set, tag, and registry inspection is allowed.
  Release mutation is not.

## Dispatch manifest

- **State:** ready for independent queue dispatch alongside g18.003 review and
  g18.004 implementation
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** `CHANGELOG.md`; one g18.005 execution log; narrowly
  required docs references if changelog normalization changes an exact heading
- **Reserved closeout surfaces:** g18 README, generation index, dispatch
  projection, g18.006 card, release notes, manifests, locks, tags, registry,
  Desktop repository/task/PR
- **Worker:** release-aware documentation/tooling worker comfortable with
  Effigy release inspection and historical changelog preservation
- **Excluded:** product code; package/Cargo versions; dependency changes;
  `0.4.0` release notes; workflow edits; full candidate certification; tag or
  publication; Desktop edits; windowed selectors
- **Escalation:** Chatterbox for any lost historical meaning, parser/tool defect,
  version decision, workflow change, or required release mutation

## Work

1. Capture the current parser failures and inspect the repository's release
   contracts, manifest, operations file, workflow, tags, and live registry
   package set without mutation.
2. Normalize `CHANGELOG.md` to the supported Keep a Changelog category grammar.
   Preserve dates, versions, links, and the meaning of every historical entry.
3. Prove Effigy release status and plan can parse the repository and report the
   expected package set. Stop before any command that tags or publishes.
4. Record a concise execution log: repaired grammar, read-only commands,
   provisional `0.4.0` rationale, current tags/registry, and the candidate-time
   gates intentionally deferred to g18.006.
5. Run docs QA and `git diff --check`. Open one non-draft PR and report the
   exact head for independent review.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| History is preserved | parser passes because old entries were dropped or flattened into vague prose | semantic before/after inventory binds every version, date, link, and entry |
| Parser is honestly repaired | worker changes Effigy, skips validation, or suppresses errors | only repository changelog/docs change; ordinary status/plan parses cleanly |
| Package set remains unchanged | preflight edits manifests or prepares versions | diff contains no package/Cargo manifest, lock, or release-note mutation |
| No premature release | tag, workflow dispatch, or registry mutation occurs | git tags, workflow history, and npm versions remain unchanged |
| Final checks remain final | preflight claims certification against source missing g18.003/g18.004 | log explicitly defers candidate trees, packed artifacts, gates, dry runs, tagging, and publish to g18.006 |

## Stop conditions

- Stop if parser compatibility requires changing Effigy, a workflow, or release
  policy rather than normalizing the changelog.
- Stop if any historical release meaning cannot be preserved unambiguously.
- Stop before package versioning, a release workflow dispatch, tag, publish,
  Desktop mutation, or full candidate certification.

## Evidence

Planning intake on 2026-09-10 reproduced the changelog parse failure through
Effigy before any release mutation. Live registry inspection reported core and
Svelte `latest=0.3.0`.

PR #238 reached reviewed head `40f9e0a3d44f485fd24b4aa6ba55dff2bc759a51`.
Web CI then exposed the deterministic ordinary-scope mismatch. The operator
selected g18.007's structural classifier repair; PR #238 and all attached Queue
threads/workspace remain retained for validation retry afterward.

## Next task

After g18.003, g18.004, and this preflight close, return to Chatterbox for the
final-source review and explicit release decision on g18.006.
