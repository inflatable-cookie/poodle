---
title: g16.026 cross-window drag bridge worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: poodle
created: 2026-08-31
updated: 2026-08-31
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260831-145018-g16-026-cross-window-drag-bridge.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator reconciled the landed web and GPUI drag controllers with the
next cross-window migration. It promoted a split per-source/per-window bridge,
an opaque bounded receipt, and a window-owned GPUI provider-unmount seam into
spec 069 and card g16.026.

This dispatches one bounded implementation lane. No transcript or second prompt
is part of the authority chain.

## Why It Matters

Poodle needs one semantic drag transaction across local web, same-application
cross-window, and GPUI paths before Tabs and DockRegion can leave their older
DOM-shaped and global-session implementations. The boundary must remain host
agnostic and must not leave native drag state alive when a GPUI provider
unmounts.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `b8fd86eb740d00a111b656f32a095e4decfe4cdd`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `b8fd86eb740d00a111b656f32a095e4decfe4cdd` before this handoff was written.
- **Planning checkout:** clean after the ready-state commit.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** spec 069, g16.026, the g16
  milestone front, and the generation index contain the promoted decision.
- **Worker branch:** `codex/g16-026-cross-window-drag-bridge` for named fallback;
  accept a clean launcher-provided non-`main` branch under the protocol below.
- **Worker worktree:**
  `$AGENTS_WORKTREE_CONTAINER_DIR/poodle-g16-026-cross-window-drag-bridge` for
  named fallback only.
- **Worktree creation command:** after validating the configured container,
  `git worktree add "$AGENTS_WORKTREE_CONTAINER_DIR/poodle-g16-026-cross-window-drag-bridge" -b codex/g16-026-cross-window-drag-bridge origin/main`.
- **Worker worktree policy:** follow `Completion Protocol`; launcher worktree
  first, named/manual fallback only when required.
- **Required sibling worktree links:** none.
- **Active spec lane:** `docs/specs/069-dependable-drag-and-drop-substrate.md`
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g16/026-drag-drop-cross-window-bridge-and-dock-region.md`
- **Allowed runway:** g16.026 only.
- **Remaining card budget:** one card.
- **Dispatch topology:** serial single-worker lane; g16.027 remains behind this
  card's merge.
- **Parallel safety check:** Tabs, DockRegion, shared web/Rust drag contracts,
  and GPUI host ownership overlap one mutable migration seam, so they stay in
  one lane.
- **Canonical refs:** `docs/architecture/011-drag-and-drop-substrate.md`;
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/components/tabs.md`, and
  `docs/contracts/components/dock-region.md`.
- **Review oracle:** the `Review Oracle` table in g16.026.
- **Model capability profile:** frontier, high-reasoning implementation across
  TypeScript, Svelte, React, Rust, GPUI lifecycle, and adversarial tests.
- **Tool/runtime restrictions:** use repo-local Effigy routing; keep the lane
  serial; never run `*-windowed`, native visual, Jetstream, release, or workflow
  mutations. Do not edit sibling repositories.
- **Required validation:** the card's focused bridge/kernel/DockRegion tests,
  headless Chromium/WebKit multi-context proof, mounted GPUI regressions,
  active-cohort drift and ledger checks, web/Rust/native/docs boards, one final
  headless `effigy qa`, absence searches, and `git diff --check`.
- **PR base/head:** current pushed `main` / selected worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation and orchestrator review.
- **Merge authorisation:** not granted; only the operator may authorise merge.

## Boundaries

Please keep this run inside the named runway:

- **In scope:** the exact writable scope and ordered work in g16.026, including
  the clean Tabs/DockRegion public migration and GPUI window-host proof.
- **Out of scope:** Longhorn, Loophole, Tauri/Electron, application window
  policy, file drag-out, package versions, releases, workflows, sibling repos,
  and g16.027 or later cards.
- **Outcome shape:** implementation and a reviewable PR. Complete the smallest
  contract-valid repair, cleanup, validation, evidence, and closeout owned by
  the card.
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product/API/persistence/security decision.
- Do not add compatibility aliases, shims, or silent fallbacks. The clean public
  replacement is approved, and the exact paired API in spec 069 is fixed.
- This handoff represents one worker lane. Do not edit another lane's assigned
  scope; if shared mutable scope or a hidden dependency appears, stop and report
  it to the operator.
- Work only in the clean worker worktree selected by `Completion Protocol`.
  Never edit the planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Planning lineage:** architecture 011 owns the generalized substrate; spec
  069 compiled g16.021-g16.028; g16.021-g16.025 are merged; the g16.025 log is
  `docs/logs/2026-08/20260831-g16-025-drag-drop-rust-gpui-substrate.md`.
- **Why this card is ready:** the operator chose split roles. Source preparation
  is per draggable source; target projection/commit is per document or native
  window; only `{ protocolVersion, token }` crosses the DataTransfer boundary.
  One `DragDropWindowHost` owns the GPUI controller census for one window.
- **Decisions and preferences:** keep semantic parity across TypeScript and
  Rust, preserve local Tabs/DockRegion behavior, delete the named old APIs only
  after mounted replacement proof, and leave the parity ledger unchanged.
- **Open tensions:** hostile timing around late prepare and stale target leases;
  native `dragend` must not imply host commit; live host geometry can move
  without local pointer input; one GPUI window must never sweep another; provider
  unmount must stop GPUI's real native drag and preview, not only semantic state.
- **Report after:** each ordered-work chunk that leaves a coherent tested seam,
  and immediately on a card stop condition.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Run the `Completion Protocol` preflight before broad reads. Then read
`AGENTS.md`, the g16 milestone, g16.026, spec 069's cross-window section, and the
canonical architecture/contracts from the selected worker worktree. Start with
the paired renderer-neutral contracts and deterministic lifecycle mapping; do
not begin component deletion before replacement proof exists.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad reads,
   run `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch; do not compare its generated path/branch with the named
   fallback or create another worktree merely because they differ.
3. If current context is `main`, dirty, unregistered, or unusable, inspect the
   named worktree. If unusable, read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the operator when absent. Create a
   unique worktree/branch there from pushed `origin/main`. Never use `/tmp`,
   `TMPDIR`, or a guessed path; never clean, reset, stash over, or discard dirty
   state. Report a launcher-supplied dirty or `main` worktree instead of
   creating another.
4. From the selected worktree, record this handoff's repository-relative path.
   Run `GIT_SSH_COMMAND="ssh -o ConnectTimeout=10 -o BatchMode=yes" git fetch origin`.
   Confirm `HEAD` equals `origin/main`, confirm
   `git merge-base --is-ancestor b8fd86eb740d00a111b656f32a095e4decfe4cdd HEAD`,
   and confirm the relative handoff path exists in selected `HEAD`. Load it with
   `git show HEAD:<relative-path>`. If the absolute dispatch file is readable
   and differs from that tracked blob, stop and report. The committed `HEAD`
   copy is the canonical execution input.
5. Required sibling links are `none`; make no sibling-path mutation.
6. Read the active milestone, assigned card, `AGENTS.md`, and canonical refs.
7. Run `effigy tasks` for selector orientation and record what you actually run.

### While you work

- Execute g16.026's ordered work in coherent chunks. Keep commits aligned with
  those chunks, not arbitrary model turns.
- Use the card's review oracle as an adversarial test matrix throughout, not as
  a final prose checklist.
- After each meaningful chunk, report to the operator with changed files,
  validation actually run, remaining work, new risks, and blockers.
- Stop and say so if a contract is missing, intent is ambiguous, scope expands,
  authority/access is missing, a card stop condition fires, or validation
  changes the plan.
- Do not quietly turn an open question into a new architecture.

### When the assigned runway is complete

1. Run the required final validation recorded in `Current State`. Never run a
   windowed/native visual or Jetstream selector.
2. Try to falsify the diff against the card. Exercise every review-oracle row,
   map it to proof, and reconcile the card, roadmap, log, handoff, and front
   doors. Return any new product threshold, contract choice, or acceptance rule
   to planning.
3. Update g16.026, one g16.026 log, the g16 milestone, generation index, and any
   current contract/front-door evidence the completed migration changes. Keep
   the ledger honest and record the actual worktree/branch when fallback was
   used.
4. Push the selected worker branch.
5. Open a reviewable PR against the current pushed `main` tip. The planning base
   above precedes this handoff commit; it is not a self-referential hash.
6. In the PR body, link the spec, milestone, card, changed surfaces, evidence,
   validation, and unresolved items.
7. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will review the PR against canonical refs, diff, checks, and
the card's review oracle. Current review state: awaiting implementation and
orchestrator review.

When orchestrator and worker share a GitHub identity, formal self-approval is
unavailable, so the orchestrator posts the verdict as a PR comment. That comment
is the canonical review record. If changes are requested, change only this
branch, push again, and report back through the operator. Blocking findings use
`execution-miss`, `oracle-gap`, `planning-change`, `validation-gap`, or
`integration-drift`; a `planning-change` returns to planning before revision.
Requested changes: none yet. The operator must explicitly authorise any merge.

- **Closeout refs:** g16.026; `docs/roadmaps/g16/README.md`; one g16.026 log;
  `docs/roadmaps/generation-index.md`; affected component contracts/front doors.

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, log, and next-task
state honest. If blocked, record the blocker and stop rather than making the
handoff look more complete than it is.
