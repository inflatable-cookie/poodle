# g16.053 — Repository Security-Audit Boundary Repair

Status: complete — implementation on `fix/g16-053-security-audit-boundary`;
orchestrator review and merge pending
Type: implementation — Papercuts
Opened: 2026-09-01
Completed: 2026-09-02
Depends on: the recorded `PAPERCUTS.md` false-positive entry and the accepted
HistoryCenter sequence recorded in
`../../handoffs/20260901-234025-post-triage-canonical-runway.md`
Governing refs: `../../contracts/001-working-rules.md`, `../../../PAPERCUTS.md`
execution log: `../../logs/2026-09/20260902-g16-053-repository-security-audit-boundary.md`

## Goal

Make the OpenAI-key repository audit match real token boundaries without
matching `sk-` inside ordinary hyphenated words. Restore a green release-gate
input before any `0.3.0` candidate freeze.

## Outcome

The OpenAI matcher is now `/\bsk-(?:proj-)?[A-Za-z0-9_-]{20,}/`. Interior
`sk-` in `mask-plus-translated-highlight` and `task-backed-agent-workflow-...`
is not a token. Real key shapes at whitespace, quote, `=`, `:`, and start of
string still match through the production `secretPatternHits` seam. Scan
enumeration is still `git ls-files`. Reproduction counted 4691 files with six
false positives; the repaired run is clean at 4692 after adding this lane's
execution log. Other secret classes are unchanged.

## Fixed Envelope

- Repair the existing matcher at its owning audit surface. The recorded
  plausible fix is a left word boundary; choose the smallest equivalent rule
  that preserves every real-key fixture.
- Add positive and negative focused tests: real `sk-` / `sk-proj-` shapes at
  allowed boundaries still match; embedded English compounds do not.
- Do not weaken other secret classes, suppress files, remove evidence prose,
  or waive the gate.
- This is a Papercuts worker lane. Its future workspace label is exactly
  `Papercuts`. This card does not itself launch that workspace.

## Acceptance

- The known English-compound false positive no longer fails
  `effigy audit:security`.
- Real key-shaped fixtures still fail detection at whitespace, quote, `=`, and
  `:` boundaries. Near misses and embedded substrings are explicit negatives.
- The audit remains fail-closed and scans the same repository denominator.
- No workflow, release, version, package, consumer, or unrelated scanner change
  enters the diff.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Real secrets remain detected | quoted `sk-proj-` fixture | audit/test still fails on it |
| Embedded prose is not a secret | `mask-plus-...` compound | no OpenAI-key finding |
| Denominator is unchanged | exclude the containing docs path | changed-scope audit fails |
| Repair is local | broad matcher/scanner rewrite | diff-scope gate fails |

### Oracle falsification

- Restored the unanchored `/sk-(?:proj-)?[A-Za-z0-9_-]{20,}/`: the English-compound test failed because it received `OpenAI token`; the production audit failed on the six tracked prose files plus the new test file.
- Planted an untracked quoted `sk-proj-` + 20-char body: production audit failed on that file only. Fixture removed.
- Planted `docs/` + `PAPERCUTS.md` path skips on the unanchored matcher: audit exited 0 (false green) while the matcher was still wrong. Restored. The shipped diff has no path exclusion.
- Diff is the OpenAI matcher, `secretPatternHits` extraction, focused tests, this card, the papercut, and the execution log. No other scanner, workflow, or package change.

## Writable Scope

The existing repository-security audit matcher, its focused tests/fixtures,
the exact `PAPERCUTS.md` entry, this card, and one log if repository convention
needs it. Do not edit workflows, releases, versions, packages, unrelated
scanners, evidence prose, or sibling repositories.

## Validation

Run focused matcher tests, `effigy audit:security`, `effigy docs:check`, the
narrow relevant QA selector discovered through Effigy, and `git diff --check
origin/main...HEAD`. No release or windowed selector is authorized.

## Stop Conditions

Stop if the fix requires excluding repository content, weakening real-key
detection, changing another secret class, touching workflows, or treating a
new unrelated audit failure as part of this papercut.

## Continuation

Accepted merge clears only the security-audit gate on `g16.054`. The separately
promoted compiled-JS/declarations prerequisite still blocks candidate work. No
release mutation authority is granted.
