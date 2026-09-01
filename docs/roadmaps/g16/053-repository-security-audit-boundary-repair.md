# g16.053 — Repository Security-Audit Boundary Repair

Status: ready
Type: implementation — Papercuts
Opened: 2026-09-01
Depends on: the recorded `PAPERCUTS.md` false-positive entry and the accepted
HistoryCenter sequence recorded in
`../../handoffs/20260901-234025-post-triage-canonical-runway.md`
Governing refs: `../../contracts/001-working-rules.md`, `../../../PAPERCUTS.md`

## Goal

Make the OpenAI-key repository audit match real token boundaries without
matching `sk-` inside ordinary hyphenated words. Restore a green release-gate
input before any `0.3.0` candidate freeze.

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

Accepted merge clears the serial gate on `g16.054`. It grants no release
mutation authority.
