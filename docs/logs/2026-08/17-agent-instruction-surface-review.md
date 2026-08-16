# Agent Instruction Surface Review

Date: 2026-08-17
Posture: strict-ready
Scope: root `AGENTS.md` and `CLAUDE.md`
Authority: Northstar agent-instruction audit and content-class contract

## Outcome

Poodle's always-loaded agent instructions are compact and provider-neutral.
Safety, authority, parity, worktree, compatibility, and validation boundaries
remain in the root file. Detailed procedures now resolve through Northstar,
the repo-local Effigy skill, contract 005, and `PAPERCUTS.md`.

`CLAUDE.md` is now the exact one-line `@AGENTS.md` bridge rather than a second
copy of repository rules.

## Measurements

| Surface | Before | After |
| --- | --- | --- |
| `AGENTS.md` non-blank lines | 75 | 26 |
| `AGENTS.md` encoded bytes (`wc -c`) | 5,413 | 2,335 |
| Audit approximate tokens | 1,350 | 584 |
| `CLAUDE.md` | 18 lines / 710 bytes | 1 line / 11 bytes |

The post-change audit remains below Northstar's 100-line / 12 KiB target. It
reports two possible scoped lines, three possible procedural lines, and two
possible historical lines. Manual review retained them:

- the scoped hits are short pointers to the repo-local Effigy skill and local
  worktree contract;
- the procedural hits are verified common validation commands and the manual
  worktree safety boundary;
- the historical hits are lexical false positives from dynamic authority
  pointers, not embedded task status or completed-work narrative.

## Disposition

| Class | Disposition |
| --- | --- |
| Repository scope, docs authority, product boundary, parity, Underlay, Bits Svelte, headless conformance | retained as every-turn rules |
| Pre-1.0 compatibility boundary | added as an every-turn stop rule |
| Northstar worker and review procedure | compressed to the orchestrator/worker ownership boundary; detail remains in the Northstar skill |
| Manual worktree procedure | compressed to the no-guess safety boundary and `docs/contracts/005-agent-local-paths.md` pointer |
| Effigy routing and selector inventory | replaced with the repo-local skill pointer and common `tasks` / `qa` commands |
| Papercut procedure | replaced with one root-queue pointer |
| Active-generation reference | removed; `docs/roadmaps/README.md` owns currentness |
| Generated Effigy instruction block and optional command list | removed from the always-loaded surface; surviving detail is in `.agents/skills/effigy/SKILL.md` |
| Claude-specific duplicate architecture and style rules | removed; `CLAUDE.md` now imports `@AGENTS.md` |

## Validation

- Northstar `effigy check:agent-instructions ../poodle/AGENTS.md`: exit 0
- exact Poodle Claude bridge check: `@AGENTS.md`
- every retained canonical path exists
- `effigy tasks`: retained commands and selectors present
- `git diff --check`: clean

Two audit defects were recorded in Northstar's `PAPERCUTS.md`: supplied
consumer targets still check Northstar's own Claude bridges, and the reported
byte metric counts characters rather than encoded bytes. Neither weakens this
manual review; both need repair in the audit implementation.
