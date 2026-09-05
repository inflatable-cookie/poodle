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

Closed lanes left 175 finished worker briefs and a 140-file historical parity
corpus in the live docs paths agents walk first, g16 cards still instructed
workers to edit the now-archived audits, 56 specs no current surface cites
still sat in the normative set, and guides taught removed APIs (Tabs
`underline`, two-member `ButtonTone`, pre-state Popover trigger, HistoryCenter
v1) with nothing to stop regressions.

## Current State

PR open from this branch, rebased onto promoted main `954a025222aeb2d7f126ed7f620c2948de58efb9`
(manifest revision 14; the commit that last touched `docs/roadmaps/dispatch.md`,
which amended this card with item 6 "Historical-prefix gates" authorizing the
drift-gate prefix update and planted test). Execution log:
`docs/logs/2026-09/20260905-g16-108-docs-spine-compaction.md`.

- **Handoffs**: 175 closed-lane files archived to
  `docs/handoffs/archive/2026-{08,09}/` by filename month; retention rule
  added to `docs/README.md`; 9 briefs kept in place (listed in the log) —
  orchestrator thread briefs, g16.052 planning feeds, held VL-1/visual/
  Jetstream-adjacent files, and one indeterminate papercuts file.
- **Parity**: 140 files renamed at 100% into `docs/archive/parity/` (139
  component audits plus `TEMPLATE.md`); `docs/parity/README.md` was rewritten
  as the pointer and the archive README records the move. Parity-edit
  instructions removed from g16 cards 001, 010, 034, 035.
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

**Open cards per `origin/main` `954a0252` (manifest revision 14):** g16.051
(held), g16.052 (held), g16.107 (concurrent worker), g16.108 (this lane),
g16.109 (consumer adoption wave), g16.110 (feasibility spike), g16.111–116
(Nucleus A1 accessibility programme). g16.097 and g16.106 are **closed**
(097: v0.3.0 published at `1eadc581a`; 106: merged in PR #211 at
`94febafad`). Held non-card directions (Jetstream admission, VL-1 lab,
visual tranche, citations/nested menus) likewise untouched. No open-lane
brief was moved. Count check: 185 pre-existing handoff files (184 at the
original base plus g16.106's brief added at its merge); this head archives
176 of them (g16.106's closed-lane brief included, per the retention rule),
keeps 9 top-level briefs, and adds this g16.108 handoff — 10 top-level
files, 186 total.

## Stop Conditions

- Handoff lane undeterminable → left in place and listed (papercuts wave 2
  plus the eight open/held-adjacent files; see log table).
- Spec move breaking a link needing a content decision → none: no
  link-checked file referenced any moved spec; kept-spec `Depends on:` headers
  were repointed without content changes.

## Revision 3 — Operator Decision Execution (2026-09-05)

Card amendment `954a0252` (item 6, manifest revision 14) authorizes the
historical-prefix gate work in this lane. Executed: `docs/archive/` added to
`HISTORICAL_PREFIXES` in `scripts/check-recipe-only-surface.ts`;
`drift:recipes` now composes the gate scan with planted tests
(`scripts/check-recipe-only-surface.test.ts`) proving an active-path
reference under `docs/guides/` still fails while `docs/archive/parity/`
stays exempt; other prefix-hardcoded gates audited (none required the
analogous change — each named in the execution log). Branch rebased onto
`954a0252`; g16.106's closed-lane handoff archived under the retention rule.
Archived parity content was never edited for a gate; the relocation stands.

## Revision 2 — Review Response (blockers from exact-head review of `eeb8fa2f`)

1. Stack directions in `svelte-developer-guide.md` fixed to `"column"` /
   `"row"` (the fence remains context-skipped by the snippet checker because
   it references earlier Icon state; the invalid values are gone).
2. Stale/broken archive and spec links repaired: `docs/archive/parity/
   README.md` pointer link depth; archive specs 064/065 links to kept spec
   066 (`../066-…`); archive `../architecture`/`../roadmaps` depth; archive
   specs' bare mentions of kept specs (`../` prefix); kept spec 008 body
   cites of archived 002/007; archive-internal root-relative `docs/specs/`
   cites of archived specs. Same-class mentions in `docs/research/` were
   left untouched (out of owned scope, not link-checked).
3. `scripts/check-recipe-only-surface.ts` reverted to `origin/main` —
   removing the unauthorized historical-prefix edit. Consequence: `effigy
   docs:check` now fails at `drift:recipes` exactly on the 13 archived
   parity lines that mention the retired treatment CSS-variable pattern; the 4-line
   `docs/archive/` exemption extension is **requested for explicit
   planning/manifest authorization** (the card's own oracle requires
   `docs:check` green after the sanctioned parity relocation, which is
   unsatisfiable without the gate following the corpus).
4. Coordination records refreshed to `origin/main` `da8c9c37` (see headers
   above): g16.097 closed, g16.109/110 open, counts clarified.
5. Parity wording corrected to the diff's real shape: 140 pure renames, a
   rewritten README pointer, and the archive README — not "141 files moved".

## Next Move

Reviewer verifies against dispatch manifest revision 7 and the open-card list:
nothing open archived; `effigy docs:check` and `effigy docs:snippet-check`
green; parity grep clean outside card 108's own text; specs README diff lists
no archived spec as active. Merge; coordinator closes out front-door lines at
generation rollover per the card's out-of-scope note.
