# Request to Longhorn: `ForkTreePage` projection and host-stamped timestamps

Status: open
Raised: 2026-08-11
Raised by: Poodle core (HistoryCenter v2, batch cards `023` / `024`)
Blocking: no — Poodle ships a client-side stitcher meanwhile

Two asks, from building HistoryCenter v2's unified history tree. Neither
blocks Poodle; both would remove work from every host.

## 1. A topological `ForkTreePage` projection

v2 renders every entry in the fork graph exactly once at its true position:
a spine (the current branch) with other branches' entries indented as runs
where they truly diverge.

Poodle ships the stitcher as a pure tested function so hosts do not
reimplement it. It takes branch records plus a per-branch entry path and
merges shared ancestor prefixes. That part is fine.

**The fetching is the problem.** `ForkHistoryController` holds a single path:

- `#path?: ForkPathPageSnapshot` — one, not many
  (`packages/longhorn/src/history-tree/controller.ts:15`)
- `selectBranchPath(branchId)` sets `#pathTarget` and calls `refresh()`, so it
  **replaces** the visible path rather than adding to a set (`:73`)
- `refresh()` re-fetches only the current `#pathTarget` (`:58`)

So a host wanting the whole tree must call `selectBranchPath` once per branch,
accumulate the pages itself, and then keep that accumulation alive across
refreshes that only ever refresh one of them. The accumulated set goes stale
silently.

A single `ForkTreePage` projection — all reachable entries in topological
order, each tagged with the branch that owns it — would make one request
serve the whole view and stay coherent under refresh.

Failing that, the smaller version: let the controller hold several paths at
once and refresh them together.

## 2. Host-stamped `recordedAtMs` on entry and branch records

Fork-run captions want a relative time ("2m ago"). The history-tree domain has
no clock: `ForkBranchProjection` carries `branch_id`, `head_entry_id`,
`divergence_entry_id`, `name`, `annotation`, `pinned`, `current`, and the only
`Instant` in the crate is in `src/bin/measure.rs`.

Poodle will not invent one client-side — a timestamp derived at render time
describes when the popover opened, not when the edit happened, and would drift
between runtimes.

Poodle models `recordedAtMs?: number` on both record types now and renders
nothing when it is absent, so supplying it later is additive on both sides.

## Note on divergence ids

`divergence_entry_id` is documented as relative to the current branch
(`prototypes/history-tree/src/projection.rs:108`). That is what made v1's
fork-expander UI unreadable — several distinct forks project onto one visible
entry, so the popover said "6 branches off the root edit" and described
nothing.

**No change requested.** v2 does not key off divergence ids at all; it attaches
each run at the last entry the paths actually share. Recorded so the constraint
is not mistaken for a defect later.
