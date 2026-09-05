---
title: g16.107 validation hygiene bundle worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: awaiting-review
owner: Poodle Northstar orchestrator
created: 2026-09-05
updated: 2026-09-05
handoff_path: /Users/tom/.paseo/worktrees/1ugbsx1t/g16-107-validation-hygiene-bundle/docs/handoffs/20260905-090600-g16-107-validation-hygiene-bundle.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g16, g16.107]
---

## What This Thread Was Doing

Worker loop for g16.107: close the 2026-09-01 validation-hygiene remainder so
boards stop dirtying checkouts, doctor stops failing on accepted source shape,
and red-but-ungated checks join a board or leave. Eight independent items; one
stopped.

This dispatches one bounded implementation lane. No transcript or second prompt
is part of the authority chain.

## Why It Matters

Doctor, pack-install, and parallel worktrees were lying or colliding. That
blocks every worker's orientation and makes `ci:web` / `docs:check` flaky
across lanes. This card is tooling only.

## Current State

Here is the state after the worker run:

- **Repository:** `inflatable-cookie/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `9481cc95dbd65c1dff8c73a6b74b9504cf19b077`
- **Pushed main verification:** `HEAD == origin/main` at that SHA before mutation
- **Planning checkout:** not used; worker stayed in the dedicated Paseo worktree
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** card `docs/roadmaps/g16/107-validation-hygiene-bundle.md` on that SHA
- **Worker branch:** `worker/g16.107-validation-hygiene-bundle`
- **Worker worktree:** `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-107-validation-hygiene-bundle`
- **Worktree creation command:** launcher-provided Paseo worktree; reused as-is
- **Worker worktree policy:** follow `Completion Protocol`; launcher worktree
  first, named/manual fallback only when required.
- **Required sibling worktree links:** none
- **Active spec lane:** none
- **Roadmap milestone:** g16
- **Ready cards, in order:** `docs/roadmaps/g16/107-validation-hygiene-bundle.md`
- **Allowed runway:** g16.107 only
- **Remaining card budget:** this card; item 5 follow-up is Chatterbox
- **Coordinator agent ID:** chat-dispatched worker; report through this PR
- **Delivery route:** coordinator-attached child with `notifyOnFinish: true`;
  the coordinator records scoped creation and returned child/workspace identity.
  Use manual operator relay only when automatic transport is unavailable and
  the handoff explicitly declares it. A workspace or thread name is not linkage.
- **Dispatch topology:** concurrent with g16.106 and g16.108
- **Parallel safety check:** g16.108 also edits `tasks/effigy.tasks.toml`
  (`docs:check` + snippet-check). This lane touched other hunks in that file
  (test:contracts, advisory selector, value-domain on docs:check, test:core-build
  file list). Expect 108 to rebase. No shared component or contract files.
- **Surfaces this lane owns:** listed on the card; execution log; `PAPERCUTS.md`
- **Integration ownership:** orchestrator merge, then g16 closeout front doors
- **Merge ordering:** same-repository PRs merge one at a time; the orchestrator
  refreshes this head against current `main` and re-reviews it if a sibling lane
  merges first
- **Canonical refs:** `docs/architecture/001-poodle-system-shape.md`; `docs/contracts/001-working-rules.md`
- **Review oracle:** all rows in the g16.107 card; item 5 is the documented stop
- **Model capability profile:** capable coding model, high reasoning
- **Worker provider/model identity:** Cursor Grok 4.6
- **Frontier-worker justification:** none
- **Tool/runtime restrictions:** no windowed selectors; no workflow edits; no
  component or contract changes
- **Required validation:** `effigy qa`, `effigy doctor`, `effigy docs:check`,
  `git diff --check origin/main...HEAD`
- **PR base/head:** `main` / `worker/g16.107-validation-hygiene-bundle`
- **PR URL:** pending
- **Review state:** awaiting orchestrator review of the pushed head
- **Merge path:** orchestrator after accepted review of the current head and
  passing required checks
- **Key files:**
  - `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-107-validation-hygiene-bundle/docs/roadmaps/g16/107-validation-hygiene-bundle.md`
  - `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-107-validation-hygiene-bundle/docs/logs/2026-09/20260905-g16-107-validation-hygiene-bundle.md`

## Boundaries

Please keep this run inside the named runway:

- **In scope:** the eight g16.107 items, tests, log, papercuts, this handoff
- **Out of scope:** workflows, components, contracts, windowed selectors, merge
- **Outcome shape:** issue-fix. Item 5 is diagnostics-plus-selector: the
  typecheck exists; putting it on `ci:web` needs the React type backlog cleared.
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product/API/persistence/security decision.
- This handoff represents one worker lane, and sibling lanes may be running
  concurrently. Write only inside **Surfaces this lane owns**. Leave any
  closeout or front-door surface assigned to **Integration ownership** to its
  named owner. If shared mutable scope, a hidden dependency, or another lane's
  write appears, stop and report it through the active control plane or the
  operator instead of resolving it yourself.
- Work only in the clean worker worktree selected by `Completion Protocol`.
  Never edit the planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge belongs to the orchestrator after its accepted
  review/check gate.

## Important Context

- **Planning lineage:** 2026-09-01 holistic posture remainder, promoted as
  g16.107 on 2026-09-05
- **Why these cards are ready:** tooling repair, no component/contract/workflow
  change, independent items
- **Decisions and preferences:** machine-shape went to `advisory:` because its
  20 findings need vector/machine work. Value-domain joined `docs:check` with
  a ratchet. `#[allow(` is warning, not high: Effigy 0.12 has no adjacent-rationale
  scorer. God-files fail only above 3200 code lines.
- **Open tensions:** item 5 — `check:react` is red (12 component errors + preview
  specimen `string`/`ControlSize` backlog). Including it would red `ci:web`.
- **Report after:** the eight-item batch
- **Report to:** the owning coordinator through the linked child result. Do
  not require operator relay or message Chatterbox during automatic dispatch;
  manual relay applies only to the explicitly declared fallback.

## Suggested Next Move

Review the PR against the card oracle. Item 5 is the only stop: keep
`check:react` off `ci:web` until a React type-backlog card clears it, or
accept that call. If g16.108 merged first, rebase this head and re-run
`docs:check` / `qa`.

Take a moment to read the named canonical files before changing anything. If
one of the open questions changes the shape of the work, pause and bring that
question back rather than quietly choosing for the user.

## Completion Protocol

### Before you start

Launcher worktree
`/Users/tom/.paseo/worktrees/1ugbsx1t/g16-107-validation-hygiene-bundle`
on `worker/g16.107-validation-hygiene-bundle` was clean and not `main`; reused.
`HEAD` was `9481cc95dbd65c1dff8c73a6b74b9504cf19b077` == `origin/main`. No
sibling links. This handoff was written at closeout (chat dispatch had no
pre-committed handoff blob).

### While you work

Eight items executed independently. Item 5 stopped on the type backlog.

### When the assigned runway is complete

1. Run the required final validation: `effigy qa`, `effigy doctor`,
   `effigy docs:check`, `git diff --check origin/main...HEAD`.
2. Dirty-tree `docs:check` failed at the parity ledger because the source
   commit no longer matched; re-run on the committed head.
3. Log: `docs/logs/2026-09/20260905-g16-107-validation-hygiene-bundle.md`
4. Push this worker branch. If a sibling lane merged first, rebase onto
   current `main`.
5. Open a reviewable PR against current pushed `main`.
6. Do not merge.

### Review and merge path

The orchestrator launches an independent review child in this worker workspace
under a serial clean exact-head lease. Current review state: awaiting review.
Requested changes are: none yet.

- **Closeout refs:** card, this log, g16 front doors (orchestrator)

### Handoff closeout

Item 5 is the honest stop. Everything else in the card landed.
