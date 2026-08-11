# Longhorn history-tree requests — Poodle-side record

Status: filed upstream 2026-08-11
Raised by: Poodle core (HistoryCenter v2, batch cards `023` / `024`)
Blocking: no — Poodle ships a client-side stitcher meanwhile

The canonical request now lives on the Longhorn thread as five items from
Loophole field use, with a field priority of **2 > 1 > 4 > 3 > 5**. Two of
those are the ones Poodle raised: `recorded_at` (item 4) and `ForkTreePage`
(item 5). This file records what Poodle depends on and what the agreed shapes
mean for HistoryCenter, so the constraints are not rediscovered later.

## Item 5 — `ForkTreePage`, and why Poodle stitches meanwhile

v2 renders every entry in the fork graph exactly once at its true position: a
spine (the current branch) with other branches' entries indented as runs where
they truly diverge.

Poodle ships the stitcher as a pure tested function so hosts do not
reimplement it. **Fetching** is the awkward part upstream:

- `#path?: ForkPathPageSnapshot` — one, not many
  (`packages/longhorn/src/history-tree/controller.ts:15`)
- `selectBranchPath(branchId)` sets `#pathTarget` and calls `refresh()`, so it
  **replaces** the visible path rather than adding to a set (`:73`)
- `refresh()` re-fetches only the current `#pathTarget` (`:58`)

So a host assembling the tree makes N calls, accumulates the pages itself, and
watches that accumulation go stale on the next refresh.

`ForkTreePage` is last in the field priority order, which is reasonable —
items 1–3 delete live workaround code. **The practical consequence for Poodle
is that the client-side stitcher is the real path for some time, not a
stopgap.** It gets its own card and its own test list accordingly.

## Item 4 — `recorded_at` is entries-only, and that changes a caption

The agreed shape is an optional consumer-supplied `recorded_at` (epoch ms) on
`HistoryEntryMetadata`, carried inert through node → persistence envelope →
`ForkEntryRecord` → generated TS types. The tree never reads it; hosts with
clocks stamp it at `record_applied` time.

**It is on entry metadata only. There is no branch-level equivalent, and none
is proposed.**

Card `023` as dispatched models `recordedAtMs` on both `HistoryEntry` and
`HistoryBranch`. The branch field would never be populated. Correction to
apply at review:

- Keep `recordedAtMs?: number` on `HistoryEntry`.
- Drop it from `HistoryBranch`.
- A fork-run caption derives its relative time from **its own run's entries**
  — the most recent entry in the run. That is derivation from supplied data,
  not an invented clock, and it needs no field that will not exist.

## Item 3 — empty branch heads confirm the caption rule

`ForkNavigationTarget::Checkout` requires an entry id, so empty branch heads
(a nascent main, a root-only switch) cannot be expressed as a navigation
target at all today.

This independently confirms card `023`'s rule that **captions are focusable
for rename but never navigate**: for an empty-head branch there is no entry to
navigate to, and the protocol could not express the target even if the UI
offered it. Card `023`'s "empty branch head" stitcher test is therefore load
bearing, not an edge case.

## Divergence ids — no change requested

`divergence_entry_id` is documented as relative to the current branch
(`prototypes/history-tree/src/projection.rs:108`). That is what made v1's
fork-expander UI unreadable: several distinct forks project onto one visible
entry, so the popover said "6 branches off the root edit" and described
nothing.

v2 does not key off divergence ids at all — it attaches each run at the last
entry the paths actually share. Recorded so the constraint is not mistaken for
a defect later.

## Items 1 and 2 — no Poodle surface

Command-macro re-export and stale branches-page refresh are both host and
controller concerns. Poodle takes branch records as data and has no view of
either. Noted only so this file is a complete map of the five.
