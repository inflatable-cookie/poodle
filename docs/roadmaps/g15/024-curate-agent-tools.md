# g15.024 — Overloaded Examples: agent and tools

Status: **planned** — orchestrator review required before dispatch
Parent: `018-overloaded-examples-curation.md` (method, acceptance, stop
conditions — this card does not restate them)
Depends on: `g15.011`
Sequenced after `g15.015`, which restores the 52 blank captions. Curating
before that would be cutting examples nobody can read.
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`

## Scope

Agent-surface pages. Their authored captions do not currently render, so
their real example structure is not visible until `g15.015` lands.

Catalogue families: `agent-tools`.
### Pages this card owns (6)

- `AgentChatInput`
- `AgentMessage`
- `AgentQuestion`
- `AgentQuestionRecord`
- `AgentTranscript`
- `ChangedFiles`

This list is exact and exhaustive: it is every page in these families whose
`Examples` view the audit measured as overloaded (10+ captioned examples) or
long (7–9). No other card owns these pages, and this card owns no others. If a
prerequisite card changes a page's count before this one runs, re-measure and
record the change — do not silently widen or narrow the set.

No component, contract, or public API change.

## Goals

- [ ] Every page in the group meets the parent's method.
- [ ] Svelte and React stay identical; GPUI teaches the same set.
- [ ] Removals are named, with contract coverage checked first.

## Acceptance

Per the parent, including its operator-review checkpoint: **the changed pages
are reviewed live in the Svelte and React previews before this card is called
complete.** Unreviewed pages remain an explicit PR item.

## Writable Scope

- the specimen files for these families across Svelte, React, and GPUI
- one August batch log
