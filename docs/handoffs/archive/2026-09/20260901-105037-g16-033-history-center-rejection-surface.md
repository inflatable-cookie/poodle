---
title: g16.033 HistoryCenter rejection surface worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle core
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260901-105037-g16-033-history-center-rejection-surface.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, Papercuts]
---

## What This Thread Was Doing

The drag-and-drop runway closed in PR #118. The post-merge Northstar readiness
review classified `g16.033` `strict-ready` with coherent planning. This
dispatches the next bounded Poodle papercut: HistoryCenter rejection semantics
plus packed proof of the already-correct v3 `HistoryEntry` type surface.

This committed file is the only launch prompt. No transcript or sibling-repo
instruction is part of the authority chain.

## Why It Matters

HistoryCenter currently reduces every refused continuation deletion to
`UnknownEntry`, so a stale revision, protected current/checkpoint entry, and an
unavailable delete all tell the operator “Entry does not exist”. Poodle also
needs consumer-level proof that both packed Svelte type entry points expose v3
`continuationCount` and reject the removed v2 `branchCount` field.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `17a25d633b7f953aa0e9cf2e14b8e91a6074ffae`
- **Pushed main verification:** planning base matched `origin/main` before this
  handoff commit
- **Planning checkout:** clean before dispatch preparation
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Worker label:** `Papercuts` — capitalized, required before launch
- **Planning artifacts at the base:** merged `g16.028`, promoted and ready
  `g16.033`, HistoryCenter contract, portfolio ownership note, and current g16
  front doors
- **Worker branch:** `papercuts/g16-033-history-center-rejection-surface`
- **Worker worktree:** Paseo-managed worktree created from pushed `origin/main`
- **Worktree creation command:** Paseo `create_workspace`, worktree
  `branch-off`, base `main`, branch
  `papercuts/g16-033-history-center-rejection-surface`
- **Required sibling worktree links:** none
- **Roadmap card:**
  `docs/roadmaps/g16/033-history-center-rejection-surface.md`
- **Allowed runway:** `g16.033` only
- **Remaining card budget:** one card
- **Dispatch topology:** serial Poodle papercut after the closed drag runway;
  post-g16 motion/block-slider research remains behind this lane
- **Parallel safety:** no worker may edit Keyboard geometry, CS20 policy,
  Longhorn, Loophole, release/publication, or drag-and-drop surfaces
- **Canonical refs:** `docs/contracts/components/history-center.md`,
  `docs/architecture/006-headless-core-and-machine-model.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/triage/20260831-194043-history-papercut-ownership.md`, and the card
- **Review oracle:** `g16.033` Review Oracle
- **Model capability profile:** frontier worker with high reasoning; the lane
  changes one public semantic union across TypeScript, Svelte, React, shared
  Rust, and GPUI and must prove a packed consumer boundary
- **Tool/runtime restrictions:** route validation through Effigy; never run
  windowed/native-visual, Jetstream, release, publication, workflow mutation,
  or sibling-repository commands
- **Required validation:** the card's focused HistoryCenter suites and named
  GPUI proof; `effigy test:svelte-pack-install`; relevant drift checks;
  `effigy test:core`; `effigy test:components`; `effigy test:contracts`;
  `effigy ci:web`; `effigy ci:rust`; `effigy ci:native`;
  `effigy docs:check`; one final headless `effigy qa`; and
  `git diff --check origin/main...HEAD`
- **Known main baseline:** `effigy qa` currently reaches an unrelated
  `audit:security` false positive on `mask-plus-translated-highlight` in the
  post-g16 research triage note. It is recorded in `PAPERCUTS.md`. Do not widen
  this card into the security scanner; report the exact baseline if it remains
  the only red result.
- **PR base/head:** `main` /
  `papercuts/g16-033-history-center-rejection-surface`
- **PR URL:** pending
- **Review state:** awaiting worker implementation and PR
- **Merge path:** orchestrator after accepted exact-head review and required
  checks

## Promoted API — Do Not Reopen

Keep the structured, renderer-neutral Poodle rejection seam and exact
component-owned English copy. Preserve:

- `AlreadyAtTarget` → `Already at the requested target`
- `UnknownEntry` → `Entry does not exist`

Add exactly:

- `StaleHistory` → `History changed; this entry was not deleted`
- `ProtectedEntry` → `This history entry is protected`
- `DeletionUnavailable` → `History deletion is unavailable`

Current-line and pinned/checkpoint policy share `ProtectedEntry`. Do not add a
host message, override, locale input, compatibility alias, protocol string, or
general i18n system.

## Boundaries

- **In scope:** framework-free TypeScript rejection code/resolver; Svelte and
  React HistoryCenter props, focused tests, and only necessary curated
  specimens; Rust `HistoryCenterRejection` and `rejection_message`; shared
  rendering and exact GPUI specimen/regression evidence; installed-tarball v3
  type proof; contract/card/front-door/log closeout
- **Packed proof:** install the built Svelte tarball in the existing disposable
  consumer without workspace aliases. Positively typecheck `HistoryEntry` from
  both package root and `@inflatable-cookie/poodle-svelte/types` with
  `continuationCount`. Run one unsuppressed expected-failure compile per import
  path with `branchCount`; require each to fail with a named diagnostic. Keep
  tarball hash/realpath evidence. This proves the source candidate only.
- **Out of scope:** package version, npm publication, tags, release notes,
  Loophole pin/adoption, Longhorn wires, CS20 `groupId` policy, Keyboard
  vertical geometry, drag-and-drop, HistoryCenter data/navigation redesign,
  Jetstream admission, parity-ledger movement, or broad accessibility work
- **Outcome shape:** complete contract-valid implementation, falsified oracle,
  packed-candidate evidence, one September closeout log, and a reviewable PR.
  Diagnosis alone is not a result.
- Work only in the selected worker worktree. Never edit the planning checkout.
- Do not merge. Review and merge belong to the orchestrator.

## Stop Conditions

- The five-code union or exact copy cannot remain exhaustive and
  renderer-neutral.
- Correctness requires a Longhorn/Loophole change or protocol-detail sniffing.
- The installed-tarball proof exposes a release-build defect wider than the
  existing `HistoryEntry` export.
- Work requires package mutation/publication, Keyboard geometry, CS20 policy,
  unrelated HistoryCenter behavior, drag changes, or parity-ledger movement.
- More than HistoryCenter semantic evidence would move.

## Suggested Next Move

Run the completion-protocol preflight. Read the card and current implementations
before editing. Start with the contract and paired semantic unions/resolvers,
then wire web and Rust/GPUI shells, then build the installed-tarball positive
and expected-failure proof. Design every oracle proof so deleting one new
category, collapsing it to `UnknownEntry`, or reintroducing `branchCount`
actually fails.

## Completion Protocol

### Before you start

1. The handoff metadata activates worker mode. Before broad reads, run
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. Accept a clean registered non-`main` launcher worktree. Do not create a
   second worktree.
3. If the launcher context is dirty, `main`, unregistered, or unusable, report
   it. Never guess a fallback path or discard dirty state.
4. Fetch `origin` with the bounded SSH command from the Northstar worker
   contract. Confirm `HEAD == origin/main`, the planning base above is an
   ancestor, and this handoff exists in `HEAD`. The tracked blob is canonical.
5. Create no sibling links. Read `AGENTS.md`, the card, contract, architecture,
   working rules, ownership note, current implementations, and `PAPERCUTS.md`.
6. Read the repo-local Effigy skill, run `effigy tasks`, and use
   `effigy test --plan` where selector shape matters.

### While you work

- Change the contract before observable implementation.
- Preserve `AlreadyAtTarget`, `UnknownEntry`, replacement, duplicate,
  dismissal, open/closed, disabled, and live-region behavior.
- Keep copy in exhaustive Poodle resolvers; adapters map codes, never messages.
- Keep the package proof consumer-real: no source import, workspace alias,
  suppressed negative diagnostic, or declaration-text substitute.
- Report coherent chunks through Paseo with changed files, checks, remaining
  work, risks, and blockers.
- Stop on a missing contract, new semantic decision, broader packaging defect,
  authority failure, or scope expansion.

### When the assigned runway is complete

1. Run the required validation and record the known main QA baseline separately
   if it is the only failure.
2. Falsify every review-oracle row. Plant the smallest pre-fix behavior,
   confirm the named proof fails, restore, and record the exact result.
3. Reconcile the contract, card, one September closeout log, g16/front-door
   currentness, ownership note, and next-task state. Move no ledger cell.
4. Push the worker branch and open a PR against current `main`.
5. Link the card, changed surfaces, evidence map, installed-tarball receipt,
   validation, falsifications, and explicit publication non-claim in the PR.
6. Report the PR URL and exact head through Paseo. Do not merge.

### Review and merge path

The orchestrator reviews the exact PR head against the canonical refs, diff,
checks, installed consumer proof, and every oracle falsification. Findings are
posted on the PR and returned to this same worker. Blocking labels are
`execution-miss`, `oracle-gap`, `planning-change`, `validation-gap`, or
`integration-drift`. A planning change returns to the planning checkout.

When the reviewed head remains current, required checks pass, the PR is
mergeable into `main`, and no stricter rule or operator pause applies, the
orchestrator merges without another approval prompt.

- **Requested changes:** none
- **Closeout refs:** card 033, HistoryCenter contract, one
  `docs/logs/2026-09/` execution log, g16 front doors, ownership note, and the
  packed install fixture

### Handoff closeout

Leave the card, log, contract, packed proof, and next-task state honest. If
blocked, record the blocker and stop.
