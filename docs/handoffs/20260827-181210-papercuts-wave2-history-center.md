---
title: Papercuts wave 2 HistoryCenter worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom / papercuts orchestrator
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260827-181210-papercuts-wave2-history-center.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts]
---

## What This Thread Was Doing

Wave 1 closed Loophole-local renderer tests. The remaining HistoryCenter
and Select papercuts live in this repo and were filed from Loophole /
Figmatic. The operator approved papercuts wave 2.

You are the Poodle implementation worker for this lane. Do not edit
Loophole or Figmatic. Do not invent a generation card.

## Why It Matters

After checkout inside a fork run, HistoryCenter keeps the OPEN-time copy,
stale levels spin on "Loading…", and a single-fork picker greys out
Checkout. Operators think checkout failed. Select `ghost` still renders
default chrome on the native path.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `c9c511f612a5f1bcd8bde155deb621640fe1efaf`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved
  to that SHA before this handoff was created.
- **Planning checkout:** clean before this handoff file was created.
- **Worker mode:** implementation worker dispatched by the orchestrator.
- **Planning artifacts included at the base:** this handoff. Evidence
  titles live in Loophole `PAPERCUTS.md` (open HistoryCenter entries).
- **Worker branch:** `worker/papercuts-wave2-history-center`
- **Worker worktree:** prefer the launcher worktree. Named fallback under
  `AGENTS_WORKTREE_CONTAINER_DIR=/Users/tom/.t3/worktrees/poodle`.
- **Worktree creation command:** only if preflight permits a unique
  worktree under that container from `origin/main`.
- **Worker worktree policy:** use a clean dedicated non-`main` launcher
  worktree regardless of generated path.
- **Active spec lane:** none for this papercuts lane. Do not join g16
  component workers.
- **Roadmap milestone:** none.
- **Ready work items, in order:**
  1. HistoryCenter machine never re-reads props after OPEN
  2. HistoryCenter stale-level reconcile needs an event that never comes
  3. HistoryCenter: single-fork picker disables its actions menu
  4. Poodle-svelte `types.ts` still exports v2 HistoryEntry
  5. Poodle Select ignores ghost variant in native mode
- **Allowed runway:** those five items only, one PR.
- **Remaining card budget:** five papercuts.
- **Dispatch topology:** serial inside Poodle; parallel with other wave-2
  repos.
- **Parallel safety check:** HistoryCenter files are one seam; keep this
  lane serial. Do not overlap g16 component PRs.
- **Canonical refs:** `AGENTS.md`; `PAPERCUTS.md`;
  `docs/contracts/001-working-rules.md`;
  `packages/core/src/history-center.ts`;
  HistoryCenter Svelte machine; Select native root.
- **Model capability profile:** capable coding model, medium reasoning.
- **Tool/runtime restrictions:** use Effigy selectors; do not edit
  Loophole or Figmatic.
- **Required validation:** focused HistoryCenter / Select component tests.
  Pages identity change while open hydrates or dispatches a no-op-safe
  PAGES_CHANGED. Single-fork Checkout is enabled when `picked.preferred`.
  Svelte package does not export v2 `branchCount` HistoryEntry as the
  live shape. Native Select root carries `data-variant`.
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting orchestrator review after worker completion
- **Merge authorisation:** absent; do not merge

## Boundaries

- **In scope:** the five items. Capture/close matching entries in this
  repo's `PAPERCUTS.md`.
- **Out of scope:** Keyboard pitch-row geometry; delete-rejection code
  widening; Longhorn AlreadyAtTarget wire codes; empty branch-head
  checkout; licence.ts drift unless it blocks these tests.
- Prop sync: machine consumes pages/continuations updates while open, or
  the component re-sends OPEN hydration. Reconcile is already idempotent;
  add a pages watch-effect.
- Picker: menu enablement independent of the Select's auto-chosen
  disable. Keep `picked.preferred` as the checkout gate.
- Types: delete or replace the v2 `HistoryEntry` export; re-export core
  shapes.
- Select: `data-variant={variant}` on the native root; cover with a
  component test. Do not force `native={false}` on consumers.
- Do not merge the PR.

## Important Context

- **Planning lineage:** papercuts wave 2. Loophole filed the HistoryCenter
  items 2026-08-12; Figmatic filed Select ghost 2026-08-14.
- **Do not edit Loophole.** After this PR merges, the orchestrator will
  close the Loophole copies.
- **Report after:** pages sync; picker menu; types; Select ghost; then PR.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Read this file from the top. Run the worktree-safety preflight. Use the
launcher worktree if it is clean, dedicated, and not `main`.

Start with the pages watch-effect; it unblocks both "stale list" and
"Loading forever".

## Completion Protocol

### Before you start

1. Read this handoff. Then run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and the
   branch is not `main`, accept it.
3. Only if unusable, use `.agents.local.env`
   (`AGENTS_WORKTREE_CONTAINER_DIR=/Users/tom/.t3/worktrees/poodle`).
   Never use `/tmp`.
4. Confirm `HEAD == origin/main`, confirm
   `git merge-base --is-ancestor c9c511f612a5f1bcd8bde155deb621640fe1efaf HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md` and the HistoryCenter sources.

### While you work

- Commit in meaningful chunks.
- Report through the operator after each item.

### When the assigned runway is complete

1. Run focused HistoryCenter / Select tests.
2. Close or add-and-close the five items in this repo's `PAPERCUTS.md`.
3. Push the worker branch and open a PR against current pushed `main`.
4. Report the PR URL. Do not merge.

### Review and merge path

Awaiting orchestrator review. Merge is operator-authorised only.

- **Closeout refs:** `PAPERCUTS.md`; this handoff; the PR.

### Handoff closeout

If a HistoryCenter item is already fixed on this SHA, close it with
evidence.
