---
title: g16.032 continuous audio native mounted parity worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle component parity
created: 2026-08-29
updated: 2026-08-29
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260829-231553-g16-032-continuous-audio-native-mounted-parity.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, audio, gpui, native]
---

## What This Thread Was Doing

The orchestrator has closed the paired continuous-audio machine and web
lifecycle work in `g16.031`. This worker owns the second half only: mount Knob,
Fader, and XYPad as real interactive GPUI controls over those landed Rust
machines, add the smallest renderer-neutral Node input seam they need, and
prove the three controls through production dispatch.

This is one bounded implementation lane. You do not need the originating
conversation or another prompt.

## Why It Matters

Poodle cannot claim active-cohort parity for these core audio controls while
the GPUI examples are static pictures. This card turns the already-paired Rust
semantics into real native behavior without coupling continuous values to the
separate payload drag-and-drop programme or reviving a renderer-specific
component architecture.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `13c620ff36dbd5a901cb6f881046b2e1647316d8`
- **Pushed main verification:** local `HEAD` equalled `origin/main` at that SHA
  before this handoff commit.
- **Planning checkout:** clean before the handoff and readiness edits.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** merged `g16.031`, architecture
  008, the three detailed component contracts, and the compiled `g16.032`
  card. The tracked handoff commit promotes `g16.032` to ready.
- **Worker branch:** use the launcher-provided clean non-`main` branch;
  suggested manual fallback is
  `t3code/g16-032-continuous-audio-native-mounted-parity`.
- **Worker worktree:** harness-managed. Do not create a second worktree when
  the launcher already supplied one.
- **Worktree creation command:** none on the normal launcher path. If manual
  fallback is genuinely required, first parse `.agents.local.env`, require an
  absolute `AGENTS_WORKTREE_CONTAINER_DIR`, then create a unique worktree under
  that container from `origin/main`. Never guess a path.
- **Required sibling worktree links:** none.
- **Active spec lane:**
  `docs/architecture/008-audio-control-family.md` and the three component
  contracts; no provisional spec remains.
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`.
- **Ready cards, in order:**
  `docs/roadmaps/g16/032-continuous-audio-native-mounted-parity.md`.
- **Allowed runway:** card 032, all five batches, then stop.
- **Remaining card budget:** one card.
- **Dispatch topology:** serial Node/GPUI lane.
- **Parallel safety check:** no open PR owns `g16.025`; card 025 is still
  dependency-blocked. Never overlap it with this run because both edit Node and
  GPUI interaction routing.
- **Canonical refs:** `docs/architecture/006-headless-core-and-machine-model.md`,
  `docs/architecture/008-audio-control-family.md`,
  `docs/contracts/components/knob.md`,
  `docs/contracts/components/fader.md`,
  `docs/contracts/components/xy-pad.md`, and
  `docs/contracts/001-working-rules.md`.
- **Model capability profile:** frontier coding/review capability with high
  reasoning; this lane touches input lifecycle, accessibility projection, and
  a public renderer-neutral event seam.
- **Tool/runtime restrictions:** headless only. Never run `*-windowed`, native
  visual, Jetstream preview/QA, release, publication, tag, or workflow-mutation
  selectors.
- **Required validation:** every focused and broad selector in card 032,
  including retained Slider/RangeSlider/ResizeHandle regressions,
  `probe:gpui-specimens`, contracts, Rust/native CI, docs and drift checks, the
  ledger check, one final headless `effigy qa`, and diff hygiene.
- **PR base/head:** `main` to the selected worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation, then orchestrator review.
- **Merge authorisation:** none. The worker must not merge.

## Boundaries

Please keep this run inside card 032:

- **In scope:** one renderer-neutral continuous-value event; the bounded
  wheel, double-activation, and missing physical-key routes; Node/GPUI capture
  lifetime; handler-backed Knob, Fader, and XYPad render paths; focused
  accessibility; stateful GPUI specimens; three mounted regressions; exactly
  three mounted-behavior ledger moves; closeout docs.
- **Out of scope:** TypeScript, Svelte, or React behavior; paired machine
  semantics and vectors; payload drag-and-drop; other audio controls; broad
  accessibility or visual programmes; Jetstream admission; tokens/recipes
  without a proven semantic need; releases, workflows, siblings, and
  consumers.
- **Outcome shape:** smallest complete contract-valid implementation, with any
  temporary diagnostics removed, validation recorded, evidence updated, and a
  reviewable PR. Do not stop at a capability probe or root-cause report unless
  a card stop condition is met.
- Keep existing `on_scrub` for Slider/RangeSlider and `on_drag` for
  ResizeHandle. Do not migrate unrelated controls or add compatibility aliases.
- The new Node event must remain renderer-neutral: normalized local position,
  logical-pixel delta, phase, and modifiers only. No GPUI type, runtime
  coordinate, pointer id, payload, file, window, or application type may leak.
- One primary gesture is admitted; repeated press is inert; release or cancel
  is exactly once, including outside bounds and lost-host paths.
- Disabled controls reject user mutations but retain the host, presentation,
  entry-cancellation, and accepted-gesture terminal routes locked by g16.031.
- Work only in the selected clean worker worktree. Never edit the orchestrator's
  planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge remains a separate operator-authorised action.

Stop and return to the operator if crates.io GPUI 0.2.2 cannot provide the
required captured release/cancel or wheel facts without a fork, private patch,
focus-stealing windowed harness, or application-owned global listener; if
XYPad needs a wider accessibility programme; if the public value/callback
contracts or paired machines must change; or if more than the three named
ledger cells would move.

## Important Context

- **Planning lineage:** `g16.020` audited the component roster and identified
  continuous audio as a bounded repair. `g16.031`, merged in PR #99, aligned
  the TypeScript/Rust machines and hardened all six web adapters. This card
  consumes those landed transitions and pointer-mapping helpers; it must not
  revisit them.
- **Why this card is ready:** its dependency is merged, the public behavior is
  already locked in architecture/contracts, the current native gap is measured,
  the exact ledger delta is known, and card 032 names acceptance, validation,
  writable scope, stop conditions, and continuation.
- **Decisions and preferences:** build on the existing scrub/capture lesson,
  not the payload drag session kernel. VisualState stays the sole drawing
  input. Runtime geometry, capture, focus, keyboard, wheel, entry, callbacks,
  and host-owned rebuilds stay in adapters and handlers. GPUI specimens are
  human examples, not an exhaustive hidden conformance matrix.
- **Open tensions:** GPUI's crates.io input surface must be proven sufficient
  inside this lane. Mechanical Jetstream compile maintenance is allowed only
  when the shared Node vocabulary requires it, with no behavior or evidence
  claim.
- **Report after:** the Node/backend lifetime seam is implemented and its
  retained interaction tests pass; then again after all three controls are
  mounted or immediately when a stop condition is reached.
- **Report to:** the operator, who will relay progress and the final PR URL to
  the orchestrator.

The two open triage notes about motion learning and a future Longhorn-backed
conformance lab remain separate programme choices. Do not pull either into this
card.

## Suggested Next Move

Start by reading this file from the top. Before any broad repository read, run
the four-command worktree safety preflight below. Once the selected worktree is
verified against pushed `main`, read `AGENTS.md`, card 032, g16's README,
architecture 006/008, the three component contracts, working rules, and the
repo-local Effigy skill.

Then inspect the existing `on_scrub`, Slider/RangeSlider mounted regressions,
ResizeHandle `on_drag`, Node interaction vocabulary, and GPUI backend capture
cleanup. Implement Batch 1 as one coherent tranche and prove its exact
lifecycle before mounting Fader, Knob, and XYPad in the card's order.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before
   broad reads run only: `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and the
   branch is not `main`, accept it as the launcher-provided worktree. Record
   its actual root and branch. Do not compare them with suggested placeholders
   or create another worktree because they differ.
3. If the launcher supplied `main` or a dirty checkout, stop and report it. Do
   not silently create a second worktree. A manual fallback is allowed only
   when the current context is otherwise unusable outside that launcher
   failure: parse `.agents.local.env` as data, require an absolute
   `AGENTS_WORKTREE_CONTAINER_DIR`, ask the operator if it is absent, and create
   a unique worktree/branch under it from `origin/main`. Never use `/tmp`,
   `TMPDIR`, or a guessed path; never clean, reset, stash over, or discard an
   existing checkout.
4. From the selected worktree, fetch origin and confirm `HEAD == origin/main`.
   Confirm `git merge-base --is-ancestor
   13c620ff36dbd5a901cb6f881046b2e1647316d8 HEAD` succeeds. Confirm this
   repository-relative handoff exists in the selected `HEAD`, load it with
   `git show HEAD:docs/handoffs/20260829-231553-g16-032-continuous-audio-native-mounted-parity.md`,
   and compare it with the absolute dispatch file. Stop if they differ; the
   committed `HEAD` copy is canonical.
5. Required sibling links are `none`; make no sibling-path mutation.
6. Read the authority and card refs named above, then run `effigy tasks` and
   select the focused checks from card 032.

### While you work

- Execute the five batches in order. Keep commits aligned with those coherent
  tranches, not model turns.
- Own capability inspection, diagnosis, implementation, cleanup, tests,
  evidence, and PR creation within the card. A capability probe alone does not
  finish the lane.
- Report after Batch 1 and after the three mounted controls, naming changed
  files, checks actually run, ledger state, remaining work, and blockers.
- Stop on any card stop condition, missing authority, scope expansion, public
  contract change, or validation result that changes the plan. Do not invent a
  wider architecture.

### When the assigned runway is complete

1. Run every check in card 032. Everything stays headless.
2. Update only the three mounted-behavior ledger cells, card 032, one August
   execution log, g16/front-door continuation state, and any exact
   contract/architecture names that landed. Record actual validation and
   non-claims.
3. Run `git diff --check origin/main...HEAD` and leave the worktree clean after
   committing.
4. Push the selected worker branch and open a reviewable PR against current
   `main`.
5. The PR body must link card 032, architecture 006/008, the three contracts,
   the Node/backend lifetime evidence, mounted callback/accessibility traces,
   retained regressions, the exact ledger delta, validation, and the execution
   log.
6. Return the PR URL and evidence to the operator. Do not merge and do not
   continue into payload drag-and-drop, another audio control, accessibility,
   visual comparison, motion, Longhorn-lab, or Jetstream work.

### Review and merge path

The orchestrator will review the PR independently against the current head,
card, canonical refs, diff, checks, and mounted evidence. Because the worker
and orchestrator share a GitHub identity, the orchestrator will post the
verdict as a PR comment. Requested changes: none yet. The operator must
explicitly authorise any merge.

- **Closeout refs:** card 032, its August execution log, g16 README, component
  continuation runway, roadmap front door, parity ledger/checker, and the
  single next-task state.

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, log, ledger, and
next-task state honest. If blocked, record the exact blocker and stop rather
than making the handoff look complete.
