---
title: g16.029 TimeInput semantic model and native parity worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle component parity
created: 2026-08-29
updated: 2026-08-29
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260829-000238-g16-029-time-input-native-parity.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, time-input]
---

## What This Thread Was Doing

The orchestrator closed the drag-and-drop semantic foundation in PR #96 and
returned to Poodle's serial component-continuation lane. TimeInput is next. Its
editing decision is already promoted into the component contract and g16.029,
so this worker owns implementation rather than further product design.

This run replaces the unconstrained native Rust text substitute with one paired
TypeScript/Rust time-entry model, aligns the Svelte and React native-input
commit boundary, performs the clean pre-1.0 `TimeField` to `TimeInput` Rust
rename, and mounts a real segmented GPUI editor. Do not continue into
NumberInput, continuous audio, or the drag adapter cards.

## Why It Matters

TimeInput currently exposes the right public string contract on the web but a
different editing model in GPUI. Closing that split gives consumers one
predictable value, draft, validation, stepping, focus, and accessibility
contract across the active cohort. It also removes a legacy Rust name before it
becomes a compatibility burden.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `a40983ce65e48cf7adb3f26b553a66df410218b3`
- **Pushed main verification:** local `HEAD` and `origin/main` both equalled the
  planning base before this handoff commit.
- **Planning checkout:** clean before this docs-only dispatch commit.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** the promoted TimeInput contract,
  resolved editing decision, g16.029 card with its dependency satisfied, merged
  g16.021 foundation, compiled g16.030–032 continuation lane, and current
  parity ledger. This handoff commit marks g16.029 ready.
- **Worker branch:** `t3code/g16-029-time-input-native-parity` is the suggested
  manual fallback name; use the launcher-provided non-`main` branch when one
  exists.
- **Worker worktree:** harness-managed; record the actual launcher path. A
  manual fallback may only use the operator-selected container from
  `.agents.local.env`.
- **Worktree creation command:** none by default; use the supplied worktree.
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path and branch;
  never create a second worktree for a naming difference. If the current
  context is unusable, only then read `.agents.local.env`, require
  `AGENTS_WORKTREE_CONTAINER_DIR`, and create a unique manual worktree/branch
  under that container from `origin/main`. Ask the operator if the file or key
  is absent; never use `/tmp`, `TMPDIR`, or a guessed path.
- **Required sibling worktree links:** none. Jetstream remains deferred and no
  sibling checkout is part of this card's validation.
- **Active spec lane:** none; the decision is promoted into
  `docs/contracts/components/time-input.md`.
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g16/029-time-input-semantic-model-and-native-parity.md`
- **Allowed runway:** `g16.029` only
- **Remaining card budget:** one card in five coherent batches
- **Dispatch topology:** serial; g16.030 follows only after this PR merges
- **Parallel safety check:** no open PR currently owns the shared core/headless
  exports, domain vectors, TimeInput surfaces, or GPUI input routing. Stop if a
  new overlapping worker appears.
- **Canonical refs:**
  `docs/architecture/006-headless-core-and-machine-model.md`;
  `docs/contracts/components/time-input.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/005-agent-local-paths.md`
- **Decision lineage:**
  `docs/triage/20260828-224148-time-input-native-editing-decision.md`
- **Model capability profile:** cross-language TypeScript/Rust state-machine
  work, native input semantics, GPUI mounted interaction, accessibility, and a
  clean public Rust rename
- **Tool/runtime restrictions:** follow repo-local Effigy routing; all checks
  remain headless. Never run `*-windowed`, native visual, Jetstream preview/QA,
  release, tag, publication, or workflow-mutation selectors.
- **Required validation:** the focused paired model/vector, Svelte, React,
  contract, renderer, Node, GPUI, and mounted TimeInput checks from g16.029;
  `effigy probe:gpui-specimens`, `effigy test:core`,
  `effigy test:components`, `effigy test:contracts`, the named drift and ledger
  checks, `effigy ci:web`, `effigy ci:rust`, `effigy ci:native`,
  `effigy docs:check`, one final headless `effigy qa`,
  `git diff --check origin/main...HEAD`, and an active-surface search proving
  the legacy Rust names are gone
- **PR base/head:** `main` / actual worker branch
- **PR URL:** pending
- **Review state:** awaiting implementation and orchestrator review
- **Merge authorisation:** worker must not merge; the operator authorises merge
  only after orchestrator review

## Boundaries

Keep this run inside g16.029:

- **In scope:** the paired pure time-entry semantics and shared domain vectors;
  Svelte/React native-input commit boundary; the clean in-repository Rust
  `TimeFieldSpec` / `time_field` to `TimeInputSpec` / `time_input` migration;
  segmented GPUI editing, focus, accessibility, specimens, mounted proof; the
  one TimeInput ledger-cell move; and honest card/log/front-door closeout.
- **Out of scope:** NumberInput, EditableLabel, DurationInput behavior,
  DateTimePicker-family behavior beyond mechanical TimeInput renames,
  drag-and-drop, continuous-audio controls, token work without a proven gap,
  Jetstream behavior or preview admission, workflows, versions, releases,
  sibling repositories, and downstream consumers.
- Preserve the public canonical `HH:MM` / `HH:MM:SS` string-or-null contract.
  Do not add raw-draft callbacks, locale/timezone/date ownership, picker
  overlays, compatibility aliases, shims, fallbacks, or silent normalization.
- Do not replace the web native time input unless the approved portable
  callback boundary is impossible; if that stop condition is reached, report it
  rather than redesigning the component.
- This handoff represents one worker lane. If another active lane touches its
  shared vectors, exports, Node input routing, or TimeInput surfaces, stop and
  report the collision.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's `main` checkout or another dirty checkout.
- Do not merge the PR.

## Important Context

- **Planning lineage:** g16.020 audited the component roster. The operator then
  approved TimeInput's committed-value versus adapter-owned-draft model and the
  clean pre-1.0 Rust rename. g16.021 had to land first because both cards touch
  shared exports and fixture runners; PR #96 now satisfies that dependency.
- **Why this card is ready:** the external value shape, segment model,
  validation, stepping, overnight behavior, seconds visibility, revert rules,
  web/native ownership, rename policy, acceptance, evidence move, validation,
  and stop conditions are all explicit in the contract and card.
- **Decisions and preferences:** Svelte and React keep native
  `input[type=time]`; GPUI uses one labelled group with separately focusable
  hour, minute, and conditional-second SpinButton segments. A direct edit emits
  only when complete, in bounds, and step-aligned. Partial or invalid drafts
  stay local, show invalid while focused, and revert on blur or Escape. External
  controlled replacement discards the draft. Clearing the whole control emits
  `null`; clearing one segment does not.
- **Open tensions:** browser native-input event details and GPUI segment focus
  mechanisms are adapter-owned, but their observable results must match. Stop
  if the current crates.io GPUI backend cannot express the required
  focus/text/SpinButton semantics without a wider input or accessibility
  programme.
- **Report after:** paired semantics plus shared vectors pass; web adapters and
  the clean rename pass; mounted GPUI behavior is proven; final QA and PR are
  ready
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Read this file from the top. Before broad repository reads, run the startup
worktree-safety preflight below and accept a clean launcher-provided non-`main`
worktree regardless of its generated name. Then read `AGENTS.md`, g16's front
door, card 029, the TimeInput contract and decision note, architecture 006,
working rules, and the repo-local Effigy skill.

Start with Batch 1: inspect the existing TypeScript/Rust domain-vector pattern,
settle the paired time types and transition boundary, and add the bounded
`timeInput` corpus. Do not start adapter work until both languages agree on the
shared cases. Report at the meaningful checkpoints above, not after tiny edits.

## Completion Protocol

### Before you start

1. This file's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Run one quick
   read-only probe before broad reads: `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as launcher-provided. Record the actual root
   and branch; do not create another worktree because names differ.
3. If the launcher supplied a dirty or `main` worktree, stop and report it. If
   no launcher worktree exists, inspect the suggested named worktree; only then
   read `.agents.local.env` as data and require a valid absolute
   `AGENTS_WORKTREE_CONTAINER_DIR`. Ask the operator if it is absent. Never use
   `/tmp`, `TMPDIR`, a repository child, or a guessed sibling; never reset,
   clean, or stash over another checkout.
4. From the selected worktree, fetch `origin`, confirm `HEAD` equals
   `origin/main`, confirm
   `git merge-base --is-ancestor a40983ce65e48cf7adb3f26b553a66df410218b3 HEAD`
   succeeds, and confirm
   `docs/handoffs/20260829-000238-g16-029-time-input-native-parity.md` exists in
   that `HEAD`. Load it with
   `git show HEAD:docs/handoffs/20260829-000238-g16-029-time-input-native-parity.md`.
   If the absolute file differs from the tracked blob, stop. The tracked copy
   is canonical.
5. Required sibling links are `none`; make no sibling-path changes.
6. Read the milestone, assigned card, `AGENTS.md`, canonical refs, decision
   note, and `.agents/skills/effigy/SKILL.md` completely.
7. Use `effigy tasks` to confirm supported selectors, run cheap orientation,
   and record what actually ran. A fresh worktree may need `bun install` before
   web checks; do not alter `bun.lock`.

### While you work

- Execute only g16.029, in its five named batches. Keep commits aligned with
  paired semantics, web/rename convergence, mounted native behavior, and
  closeout—not model turns.
- After each meaningful chunk, report changed files, validation actually run,
  remaining work, risks, and blockers through the operator.
- Stop if the approved value/draft/step/overnight/seconds/rename contract must
  change; native web cannot preserve the callback boundary; GPUI needs a wider
  input or accessibility programme; a compatibility layer becomes necessary;
  another component's behavior or more than one ledger cell would move; or an
  overlapping worker owns shared files.
- Do not quietly turn a stop condition into new architecture.

### When the assigned runway is complete

1. Run g16.029's full required headless validation: focused paired domain,
   Svelte, React, contract, render, Node, GPUI, and mounted TimeInput tests;
   `effigy probe:gpui-specimens`; `effigy test:core`;
   `effigy test:components`; `effigy test:contracts`; contract, callback,
   value-domain, capability, and parity-ledger checks; `effigy ci:web`;
   `effigy ci:rust`; `effigy ci:native`; `effigy docs:check`; one final
   `effigy qa`; `git diff --check origin/main...HEAD`; and the legacy-name
   removal search. Never run a windowed selector.
2. Update g16.029, one August execution log, the TimeInput contract where it
   requires exact landed names, the one ledger/checker cell, and g16/front-door
   next-task state. Record exact paired APIs, vector coverage, web/native
   behavior, clean-rename proof, mounted evidence, ledger delta, validation,
   and non-claims.
3. Push the actual worker branch.
4. Open a reviewable PR against current pushed `main`. The pinned planning base
   predates this handoff commit and is intentionally not self-referential.
5. Link the milestone, card, contract, changed surfaces, evidence, validation,
   and unresolved items in the PR body.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator reviews the PR independently against the canonical refs,
diff, checks, and evidence. Because worker and orchestrator may share a GitHub
identity, the canonical verdict may be a PR comment rather than formal
self-approval. Make only requested changes on the same branch, push, and report
back. The operator must explicitly authorise any merge.

- **Requested changes:** none yet
- **Closeout refs:**
  `docs/roadmaps/g16/029-time-input-semantic-model-and-native-parity.md`,
  `docs/contracts/components/time-input.md`, the one August execution log,
  `docs/roadmaps/g16/parity-evidence-ledger.md`,
  `docs/roadmaps/g16/README.md`, `docs/roadmaps/README.md`, and
  `docs/roadmaps/generation-index.md`

### Handoff closeout

Before calling the card complete, leave the contract, card, log, ledger,
front-door currentness, and single next task honest. If blocked, record the
blocker and stop instead of making the handoff look complete.
