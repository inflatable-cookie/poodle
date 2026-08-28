---
title: g16.021 drag-and-drop semantic kernel worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle core
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260828-223046-g16-021-drag-drop-semantic-kernel.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, drag-drop]
---

## What This Thread Was Doing

The orchestrator compiled the dependable drag-and-drop programme into eight
bounded cards. This worker owns only the first: the pure TypeScript/Rust
semantic kernel and its shared transition vectors. It establishes lifecycle,
identity, intent, cancellation, target arbitration, and exactly-once effects
before any runtime adapter or component migration begins.

This is a self-contained worker handoff. Do not rely on the planning-thread
transcript or continue into `g16.022`.

## Why It Matters

Poodle has several independent drag implementations that are consistently
fragile across mouse, touch, keyboard, nested targets, window boundaries, and
cleanup. The programme fixes the shared semantics first so later Svelte,
React, GPUI, cross-window, inbound-file, and drag-out work all consume one
predictable lifecycle instead of inventing another local controller.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `fc0d1f10f288a47c606a826404435a62a3351579`
- **Pushed main verification:** `origin/main` equalled the planning base after
  the runway commit was pushed.
- **Planning checkout:** clean after the planning commit; the handoff itself is
  a later docs-only commit.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** architecture 011, spec 069,
  `g16.021`–`g16.028`, the component continuation runway, and the public
  migration triage gate.
- **Worker branch:** `t3code/g16-021-drag-drop-semantic-kernel` is the suggested
  manual fallback name; use the launcher-provided non-`main` branch when one
  exists.
- **Worker worktree:** harness-managed; record the actual launcher path. A
  manual fallback may only use the operator-selected container from
  `.agents.local.env`.
- **Worktree creation command:** none by default; use the supplied worktree.
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these handoff placeholders. Record the actual path and
  branch and never create a second worktree for that reason. If the current
  context is unusable, only then read `.agents.local.env`, require
  `AGENTS_WORKTREE_CONTAINER_DIR`, and create a unique manual worktree/branch
  under that container from `origin/main`. Ask the operator first if the file
  or key is absent; never use `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** `docs/specs/069-dependable-drag-and-drop-substrate.md`
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g16/021-drag-drop-semantic-kernel.md`
- **Allowed runway:** `g16.021` only
- **Remaining card budget:** one card
- **Dispatch topology:** serial foundation; later drag cards remain planned
- **Parallel safety check:** no other worker may edit the shared machine-vector
  corpus or the new drag kernel while this card is active.
- **Canonical refs:**
  `docs/architecture/011-drag-and-drop-substrate.md`,
  `docs/architecture/006-headless-core-and-machine-model.md`;
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/005-agent-local-paths.md`
- **Model capability profile:** paired TypeScript/Rust semantic implementation,
  immutable transition design, and fixture-driven conformance
- **Tool/runtime restrictions:** follow repo-local Effigy routing; all checks
  remain headless. Never run `*-windowed`, native visual, Jetstream preview/QA,
  release, tag, publication, or workflow-mutation selectors.
- **Required validation:** focused TypeScript and Rust drag-kernel tests; both
  shared conformance-vector runners; `effigy test:core`,
  `effigy test:contracts`, `effigy check:parity-evidence-ledger`,
  `effigy ci:web`, `effigy ci:rust`, `effigy docs:check`, one final headless
  `effigy qa`, and `git diff --check origin/main...HEAD`
- **PR base/head:** `main` / actual worker branch
- **PR URL:** pending
- **Review state:** awaiting implementation and orchestrator review
- **Merge authorisation:** worker must not merge; the operator authorises merge
  only after orchestrator review

## Boundaries

Keep this run inside `g16.021`:

- **In scope:** one new pure TypeScript drag semantic module and export; one
  paired pure Rust module and export; a hand-authored `dragDrop` section in the
  existing shared machine vectors; focused tests/runners; one August execution
  log; honest card and front-door closeout.
- **Out of scope:** DOM, Svelte, React, Node vocabulary, poodle-render, GPUI,
  Jetstream, component migrations, geometry measurement, pointer/touch/keyboard
  adapters, cross-window transport, files, drag-out, old Tabs/DockRegion export
  changes, tokens, package versions, releases, workflows, and sibling repos.
- Do not invent architecture, change the approved lifecycle, widen the roadmap,
  or choose the open public migration boundary during implementation.
- Do not add compatibility shims, aliases, silent fallbacks, runtime registries,
  code generation, a scene IR, or a second evidence ledger.
- This handoff represents one worker lane. If another active lane touches the
  same vector corpus or kernel files, stop and report the collision.
- Work only in the selected clean worker worktree. Never edit the orchestrator's
  `main` checkout or an unrelated dirty checkout.
- Do not merge the PR.

## Important Context

- **Planning lineage:** g16.020 audited all 175 components without inventing
  repairs. Architecture 011 and spec 069 then became the authority for the
  separate drag-and-drop programme. The orchestrator compiled `g16.021`–`028`
  and left only the semantic foundation ready.
- **Why this card is ready:** the renderer-neutral lifecycle and ownership
  boundaries are approved; the work can be proved against one existing
  cross-language fixture mechanism without making a runtime or product choice.
- **Decisions and preferences:** semantic inputs, states, ordered effects, and
  cleanup parity matter before renderer syntax. Runtime adapters own I/O and
  geometry. Longhorn owns cross-window transaction authority. Poodle sees only
  opaque capabilities later. Jetstream remains outside the active cohort.
- **Open tensions:** exact public type names may follow local conventions, but
  the paired distinctions must remain recognizable. If the existing vector
  runner cannot express ordered effects without a new IR or generator, stop.
  Do not solve the separate public-export migration gate.
- **Report after:** paired types plus fixture schema are settled, after each
  language kernel passes the shared vectors, and at final validation/PR
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Read this file from the top. Before broad repository reads, run the startup
worktree-safety preflight below. Accept a clean launcher-provided non-`main`
worktree regardless of its generated name. Then read `AGENTS.md`, the g16 front
door, card 021, architecture 011, architecture 006, spec 069, working rules,
the Effigy skill, and the existing machine-vector runners.

Start with Batch 1 from the card: settle paired semantic types and the bounded
shared vector cases. Stop if that exposes a missing contract decision rather
than designing around it. Report meaningful chunks, not turn-sized updates.

## Completion Protocol

### Before you start

1. This file's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Run one quick
   read-only probe before broad reads: `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as launcher-provided. Record its actual root
   and branch; do not create another worktree because the generated names differ
   from this handoff.
3. If the launcher supplied a dirty or `main` worktree, stop and report it. If
   no launcher worktree exists, inspect the suggested named worktree; only then
   read `.agents.local.env` as data and require a valid absolute
   `AGENTS_WORKTREE_CONTAINER_DIR`. Ask the operator if it is absent. Never use
   `/tmp`, `TMPDIR`, a repo child, a guessed sibling, reset, clean, or stash over
   another checkout.
4. From the selected worktree, fetch `origin`, confirm `HEAD` equals
   `origin/main`, confirm
   `git merge-base --is-ancestor fc0d1f10f288a47c606a826404435a62a3351579 HEAD`
   succeeds, and confirm this handoff file exists in `HEAD`.
5. Read the milestone, assigned card, `AGENTS.md`, canonical refs, and
   `.agents/skills/effigy/SKILL.md` completely.
6. Use `effigy tasks` to confirm supported selectors. Run cheap orientation
   checks and record what actually ran.

### While you work

- Execute only `g16.021`, in its four named batches. Keep commits aligned with
  meaningful implementation chunks.
- After each meaningful chunk, report changed files, validation actually run,
  remaining work, risks, and blockers through the operator.
- Stop if the approved semantic contract must change, platform/runtime state
  enters the kernel, the shared corpus demands a generated authority, an old
  public export must migrate, or another component/evidence cell changes.
- Do not quietly turn an open question into new architecture.

### When the assigned runway is complete

1. Run the card's full required validation: focused paired tests and vector
   runners; `effigy test:core`; `effigy test:contracts`;
   `effigy check:parity-evidence-ledger`; `effigy ci:web`; `effigy ci:rust`;
   `effigy docs:check`; one final headless `effigy qa`; and
   `git diff --check origin/main...HEAD`.
2. Update the card, one August execution log, and g16/front-door next-task state.
   Record exact paired APIs, vector coverage, validation, unchanged ledger
   totals, and non-claims.
3. Push the actual worker branch.
4. Open a PR against the current pushed `main`. The pinned planning base above
   predates this handoff commit and is intentionally not self-referential.
5. Link the spec, milestone, card, changed surfaces, evidence, validation, and
   unresolved items in the PR body.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator reviews the PR against the canonical refs, diff, and checks.
Because the worker and orchestrator may share a GitHub identity, the canonical
review verdict may be a PR comment rather than formal self-approval. Make only
requested changes on the same branch, push, and report back. The operator must
explicitly authorise any merge.

- **Requested changes:** none yet
- **Closeout refs:** `docs/roadmaps/g16/021-drag-drop-semantic-kernel.md`,
  `docs/roadmaps/g16/README.md`, `docs/roadmaps/README.md`,
  `docs/roadmaps/generation-index.md`, and one August execution log

### Handoff closeout

Before calling the card complete, leave the card, roadmap, log, ledger, and next
task honest. If blocked, record the blocker and stop instead of making the
handoff look complete.
