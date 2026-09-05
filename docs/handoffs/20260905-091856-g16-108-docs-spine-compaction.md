---
title: g16.108 Docs spine compaction worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: complete
owner: Poodle docs spine
created: 2026-09-05
updated: 2026-09-05
handoff_path: /Users/tom/.paseo/worktrees/1ugbsx1t/g16-108-docs-spine-compaction/docs/handoffs/20260905-091856-g16-108-docs-spine-compaction.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g16, docs, compaction]
---

## What This Thread Was Doing

g16.108 — docs spine compaction (card
`docs/roadmaps/g16/108-docs-spine-compaction.md`, dispatch manifest
`docs/roadmaps/dispatch.md` **revision 7**, 2026-09-04). One PR from this
dedicated worktree (`worker/g16.108-docs-spine-compaction`) archives dead
weight agents keep reading, fixes consumer-facing guides that teach removed
APIs, and adds a gate that makes the snippet class of defect impossible to
reintroduce. Five boundaries: handoffs archive + retention rule, parity move
behind a pointer, unreferenced-spec archive + honest index, guide truth
repairs, and `docs:snippet-check` in `docs:check`.

## Why It Matters

Closed lanes left 175 finished worker briefs and a 141-file historical parity
corpus in the live docs paths agents walk first, g16 cards still instructed
workers to edit the now-archived audits, 56 specs no current surface cites
still sat in the normative set, and guides taught removed APIs (Tabs
`underline`, two-member `ButtonTone`, pre-state Popover trigger, HistoryCenter
v1) with nothing to stop regressions.

## Current State

PR open from this branch at base `9481cc95dbd65c1dff8c73a6b74b9504cf19b077`
(origin/main, promoted). Execution log:
`docs/logs/2026-09/20260905-g16-108-docs-spine-compaction.md`.

- **Handoffs**: 175 closed-lane files archived to
  `docs/handoffs/archive/2026-{08,09}/` by filename month; retention rule
  added to `docs/README.md`; 9 briefs kept in place (listed in the log) —
  orchestrator thread briefs, g16.052 planning feeds, held VL-1/visual/
  Jetstream-adjacent files, and one indeterminate papercuts file.
- **Parity**: 141 files moved to `docs/archive/parity/`; `docs/parity/`
  keeps a pointer README; archived README records the move and re-anchors
  links. Parity-edit instructions removed from g16 cards 001, 010, 034, 035.
- **Specs**: 56 unreferenced specs moved to `docs/specs/archive/` with
  one-line `index.md` entries; 14 kept; kept-spec dependency headers repointed
  at the archive; spec 001 marked `active`; `specs/README.md` rewritten to the
  kept set. Note: the card quoted an audit count of 28; a reproducible scan
  (filename/path or explicit "spec NNN" wording in the four named sources)
  yields 56 — g16 card numbers 001–061 collide one-for-one with spec numbers,
  which explains the audit's undercount. Evidence per spec available from the
  log author.
- **Guides**: Tabs/ButtonTone/Popover/StatusIndicator/RadioGroup/
  SegmentedControl/Stack fixed in `svelte-developer-guide.md`; the
  HistoryCenter block in `component-docs.ts` rewritten to the v3
  pages/continuations surface; duplicate `token-input.md` index line removed;
  snippet-check-driven repairs in the recipe guides (lang="ts", IconButton
  ariaLabel, MenuItem typing, Card density, ToastHost store).
- **Snippet check**: `docs:snippet-check` compiles every self-contained
  fenced svelte block from `docs/guides/*.md` against the shipped surface
  (60 snippets green, 32 app-context fragments skipped with reasons); wired
  into `docs:check`; planted `variant="underline"` fails with the exact union
  diagnostic and reverts green.
- Supporting: `scripts/check-recipe-only-surface.ts` historical-prefix list
  gains `docs/archive/` so the retired-Treatment drift gate follows the
  archived corpus (required for `docs:check` green).

**Open cards (unchanged, nothing archived against them):** g16.051 (held),
g16.052 (held), g16.097 (coordinator-executed release certification), g16.106,
g16.107 (concurrent workers), g16.108 (this lane). Held non-card directions
(Jetstream admission, VL-1 lab, visual tranche, citations/nested menus)
likewise untouched. No handoff files exist for 051/052/097/106/107, so no
open-lane brief was moved; the 9 kept files above are the full
indeterminate-or-open remainder.

## Stop Conditions

- Handoff lane undeterminable → left in place and listed (papercuts wave 2
  plus the eight open/held-adjacent files; see log table).
- Spec move breaking a link needing a content decision → none: no
  link-checked file referenced any moved spec; kept-spec `Depends on:` headers
  were repointed without content changes.

## Next Move

Reviewer verifies against dispatch manifest revision 7 and the open-card list:
nothing open archived; `effigy docs:check` and `effigy docs:snippet-check`
green; parity grep clean outside card 108's own text; specs README diff lists
no archived spec as active. Merge; coordinator closes out front-door lines at
generation rollover per the card's out-of-scope note.
