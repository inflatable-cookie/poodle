# g16.053 — Repository Security-Audit Boundary Repair

Status: complete — awaiting orchestrator review
Date: 2026-09-02
PR: https://github.com/inflatable-cookie/poodle/pull/150
Card: `docs/roadmaps/g16/053-repository-security-audit-boundary-repair.md`
Handoff: `docs/handoffs/20260902-004205-g16-053-security-audit-boundary.md`
Governing refs: `docs/contracts/001-working-rules.md`, `PAPERCUTS.md`
Branch: `fix/g16-053-security-audit-boundary`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-053-security-audit`
Base: `origin/main` at `c1a527898e7425853359bd72b7113a8cf38b8d97`
Planning base `7f59ae42f4917c675968819eb23a5e41dc90013c` is an ancestor.

## Outcome

The repository OpenAI-token matcher requires a left word boundary. Interior
`sk-` in ordinary English compounds is not a secret. Real `sk-` and `sk-proj-`
shapes at whitespace, quote, `=`, `:`, and start of string still match. Other
secret classes, scan enumeration, and the fail-closed gate are unchanged.

## Reproduction

`bun scripts/audit-repository-security.ts` on the unanchored matcher failed
with OpenAI-token findings in:

- `PAPERCUTS.md` (`mask-plus-translated-highlight`)
- g16.033 / g16.034 / g16.053 handoff and log prose (same compound)
- `docs/research/value-tracks/agent-task-list.md`
  (`task-backed-agent-workflow-contract`)

No credential was present. Path count at that HEAD: 4691. After this log
exists the same enumerator reports 4692.

## Repair

Moved the secret-pattern table into `scripts/repository-security-policy.ts` as
`secretPatternHits`, the production path the tests call. OpenAI pattern is
`/\bsk-(?:proj-)?[A-Za-z0-9_-]{20,}/`. Fixtures are concatenated at runtime so
tracked test source does not itself contain a live key shape.

## Oracle

| Row | Plant | Result |
| --- | --- | --- |
| Unanchored matcher | removed `\b` | English-compound test failed (`OpenAI token`); audit failed on the six prose files plus the test file |
| Quoted `sk-proj-` | untracked `"sk-proj-` + 20 `A`s | production audit failed on that file |
| English compound | same unanchor plant | `mask-plus-translated-highlight` matched; restored matcher does not |
| Path exclusion | skip `docs/` and `PAPERCUTS.md` on the unanchored matcher | audit exited 0 (false green) |

All plants restored. Shipped tree has no exclusion.

## Validation

- `bun test scripts/audit-repository-security.test.ts` — 17 pass
- `bun scripts/audit-repository-security.ts` — clean, 4692 files
- `effigy audit:security` — matcher, `bun audit`, and four `cargo deny` advisory/source checks pass
- `effigy docs:check` — pass
- Narrow QA selector: no dedicated scripts-test task; `audit:security` is the board that owns this matcher. `effigy graph affected` hung on index refresh and was not used as a gate.
- `git diff --check` on the worktree — pass

No workflow, release, package, or unrelated scanner change.
