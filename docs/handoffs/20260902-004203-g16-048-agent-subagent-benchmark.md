---
title: g16.048 AgentSubagent ownership and shimmer benchmark worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
review_authority: orchestrator
merge_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-02
updated: 2026-09-02
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260902-004203-g16-048-agent-subagent-benchmark.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g16, agent-subagent, benchmark]
---

## What This Thread Was Doing

The Poodle orchestrator compiled the accepted post-triage runway and made
`g16.048` ready. This lane first reconciles AgentSubagent's stale contract to
the component's current static active-runtime truth, then runs the fixed
disposable web-only shimmer benchmark.

This dispatches one bounded implementation/benchmark lane. No transcript or
second prompt is part of the authority chain.

## Why It Matters

AgentSubagent is the chosen semantic host for a possible finite running-line
sweep, but Poodle must not ship an effect based on unmeasured assumptions or
let a benchmark become contract authority. The lane makes current ownership
truthful, measures the candidate against fixed lifecycle and performance gates,
and returns a pass/fail/inconclusive verdict while production stays static.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `7f59ae42f4917c675968819eb23a5e41dc90013c`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `7f59ae42f4917c675968819eb23a5e41dc90013c` before this handoff was drafted
- **Planning checkout:** clean
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** accepted canonical runway and
  ready `docs/roadmaps/g16/048-agent-subagent-ownership-and-shimmer-benchmark.md`
- **Worker branch:** `benchmark/g16-048-agent-subagent`
- **Worker worktree:** launcher-provided; manual fallback is
  `<AGENTS_WORKTREE_CONTAINER_DIR>/poodle-g16-048-agent-subagent`
- **Worktree creation command:** launcher-managed; manual fallback only after
  reading `.agents.local.env`: `git worktree add <resolved-container>/poodle-g16-048-agent-subagent -b benchmark/g16-048-agent-subagent origin/main`
- **Worker worktree policy:** follow `Completion Protocol`; launcher worktree
  first, named/manual fallback only when required.
- **Required sibling worktree links:** none
- **Active spec lane:** `docs/architecture/012-semantic-motion-policy.md`
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready cards, in order:** `docs/roadmaps/g16/048-agent-subagent-ownership-and-shimmer-benchmark.md`
- **Allowed runway:** `g16.048` only
- **Remaining card budget:** one card with an internal serial phase gate
- **Dispatch topology:** may run beside `g16.047`; other ready g16 lanes may
  also exist, but none shares this lane's AgentSubagent contract or disposable
  benchmark surface
- **Parallel safety check:** g16.048 owns only the AgentSubagent contract,
  ignored benchmark artifacts, its card, and its execution log; g16.047 owns
  Toast runtime surfaces. Global g16 front doors remain orchestrator-owned.
- **Surfaces this lane owns:** `docs/contracts/components/agent-subagent.md`;
  one disposable ignored web benchmark harness and external/ignored artifacts;
  the g16.048 card; one compact g16.048 execution log; new relevant
  `PAPERCUTS.md` entries only
- **Integration ownership:** the orchestrator owns g16 README, generation index,
  continuation register/runway, triage, global logs/indexes, merge ordering, any
  production-shimmer planning, and post-merge closeout
- **Merge ordering:** same-repository PRs merge one at a time; the orchestrator
  refreshes this head against current `main` and re-reviews it if a sibling lane
  merges first
- **Canonical refs:** `docs/architecture/012-semantic-motion-policy.md`;
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/components/agent-subagent.md`, and
  `docs/contracts/components/agent-transcript.md`
- **Review oracle:** `docs/roadmaps/g16/048-agent-subagent-ownership-and-shimmer-benchmark.md#review-oracle`
- **Model capability profile:** mechanical implementation/benchmark worker —
  long bounded evidence collection and shipped-truth documentation projection;
  no unsettled public API or product decision
- **Frontier-worker justification:** none
- **Tool/runtime restrictions:** use the repo-local Effigy skill; browser traces
  are web-only and non-windowed; no native visual or `*-windowed` selector; no
  production runtime effect, package/export change, permanent benchmark
  selector/corpus, release command, workflow edit, registry mutation,
  sibling-repository write, or Jetstream behavior change
- **Required validation:** existing focused AgentSubagent Svelte, React, Rust
  render, and headless GPUI checks; benchmark-local deterministic checks;
  `effigy docs:lint`; `effigy docs:check`; `git diff --check
  origin/main...HEAD`
- **PR base/head:** current pushed `main` / `benchmark/g16-048-agent-subagent`
- **PR URL:** pending
- **Review state:** awaiting implementation/benchmark PR
- **Merge path:** orchestrator after accepted review of the current head and
  passing required checks

## Boundaries

Please keep this run inside the named runway:

- **In scope:** phase one, reconcile the AgentSubagent contract only to current
  shipped static truth across Svelte, React, shared Rust, and GPUI; phase two,
  after that truth is coherent, run the disposable web-only benchmark exactly
  as fixed by g16.048 and record its compact evidence and verdict.
- **Out of scope:** production shimmer or CSS; a shimmer motion role, lifecycle,
  mask/fallback law, web-only effect promise, or future implementation promise
  in the contract; generic Text/TextShimmer/AgentMessage effects; public pause,
  stop, or hide props; status mutation; native masks; package exports; permanent
  benchmarks; releases, workflows, registries, sibling repositories; native
  visual runs; Jetstream behavior.
- **Outcome shape:** truthful static AgentSubagent contract plus one disposable
  benchmark verdict. A pass permits only a later separately accepted web
  implementation card. A fail or inconclusive result closes on static output.
- Do not invent architecture, widen the roadmap, or choose an unresolved
  product/API/performance threshold. The fixed candidate lifecycle and hard
  budgets are benchmark inputs, not component promises.
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

- **Planning lineage:** `g16.034` established full/reduced/frozen host motion
  policy; the accepted AgentSubagent packet chose the running activity line as
  the only candidate host; the canonical runway split contract truth from
  production-effect admission and made their reconciliation/benchmark sequence
  `g16.048`.
- **Why this card is ready:** current runtime ownership, benchmark lifecycle,
  fixed axes, hard budgets, retained evidence, API non-goals, oracle, and stop
  conditions are explicit. Production admission remains a later gate.
- **Decisions and preferences:** AgentTranscript is the sole live-region owner;
  AgentSubagent currently renders readable static text in every active runtime;
  Jetstream stays deferred. The disposable candidate gets one non-looping 2.0 s
  sweep per eligible running epoch after the first committed frame. Same-epoch
  text replacement does not restart or queue. Loss of eligibility, reduced or
  frozen policy, forced colors, print, unsupported path, inactivity/offscreen,
  or unmount cancels to ordinary readable static text. Re-entry creates a new
  epoch.
- **Open tensions:** the current contract/runtime audit may reveal a material
  factual disagreement, or the benchmark may lack a complete engine path. Both
  are stop conditions, not permission to promise or ship an effect.
- **Report after:** finish and validate the static contract reconciliation, then
  report before creating or running the disposable benchmark. This is the
  card's serial phase gate.
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Run the `Completion Protocol` preflight before broad reads. Then read
`AGENTS.md`, the g16 milestone, the full g16.048 card, the AgentSubagent and
AgentTranscript contracts, architecture 012, and the live Svelte/React/Rust/
GPUI implementations. Reconcile only facts already shipped. Validate that
contract-only chunk and report at the phase gate before constructing the
disposable benchmark.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad reads,
   run: `git rev-parse --show-toplevel`, `git branch --show-current`, `git
   status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch; do not compare its generated path/branch with the planned
   path/branch or create another worktree merely because they differ.
3. If current context is `main`, dirty, unregistered, or unusable, inspect the
   named worktree. If unusable, read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the operator when absent. Create a
   unique worktree/branch there from pushed `origin/main`. Never use `/tmp`,
   `TMPDIR`, or a guessed path; never clean, reset, stash over, or discard dirty
   state. Report a launcher-supplied dirty or `main` worktree instead of creating
   another.
4. From the selected worktree, record this handoff's repository-relative path:
   `docs/handoffs/20260902-004203-g16-048-agent-subagent-benchmark.md`. Run
   `GIT_SSH_COMMAND="ssh -o ConnectTimeout=10 -o BatchMode=yes" git fetch origin`.
   Confirm `git rev-parse HEAD` equals `git rev-parse origin/main`, confirm
   `git merge-base --is-ancestor 7f59ae42f4917c675968819eb23a5e41dc90013c HEAD`
   succeeds, and confirm that relative path exists in the selected `HEAD`. Load
   the tracked handoff with `git show HEAD:<relative-path>`. If the absolute
   dispatch file is readable and differs from that tracked blob, stop and
   report. The committed `HEAD` copy is the canonical execution input.
5. Required sibling links: none.
6. Read the active milestone, assigned card, `AGENTS.md`, and canonical refs.
7. Use `effigy tasks` only if selector discovery is needed; run the cheapest
   relevant orientation check and record what actually ran.

### While you work

- Execute the contract reconciliation before any benchmark construction or run.
- Keep candidate lifecycle, thresholds, and visual-effect language out of the
  component contract. The contract records current static behavior only.
- Keep every harness and raw artifact disposable and ignored. Retain only the
  compact manifest, digests, summaries, lifecycle evidence, and verdict named
  by the card.
- Exercise fixed content, scale, width, DPR/theme, engine, repetition, pre/post-
  roll, lifecycle, selection/copy, DOM/accessibility, geometry, paint, layer,
  memory, and frame axes without weakening a hard cell or hiding it in an
  aggregate.
- After each meaningful chunk, report through the active control plane or the
  operator with changed files, validation actually run, remaining work, risks,
  and blockers.
- Stop and say so if current facts disagree materially, a complete engine path
  is absent, text becomes transparent or unselectable, any hard budget fails,
  the benchmark needs a permanent/public surface, or native mask support becomes
  necessary.

### When the assigned runway is complete

1. Run the required final validation listed in **Current State**.
2. Falsify every exact, negative, and universal claim in the g16.048 review
   oracle: real AgentSubagent host, one sweep per epoch, terminal cancellation,
   fresh re-entry, singular source text, trace-honest claims, and a contract
   that remains static even when every candidate fails.
3. Update the g16.048 card and one compact execution log with the actual
   contract evidence, immutable benchmark manifest/digests, per-cell results,
   and mechanical verdict. Do not edit global roadmap front doors or
   continuation maps.
4. Push the selected worker branch. If a sibling lane merged first, rebase onto
   current `main`, rerun the required validation, and report the new exact head.
5. Open a reviewable PR against current pushed `main`. This handoff's planning
   base precedes the commit that contains the handoff and is not the PR base
   after later integration.
6. Link the card, milestone, contracts, changed surfaces, evidence, validation,
   and unresolved items in the PR body.
7. Report the PR URL, exact head, and pass/fail/inconclusive verdict. Do not
   merge.

### Review and merge path

The orchestrator reviews the exact PR head against the canonical refs, diff,
checks, and every g16.048 oracle row. Current review state: awaiting
implementation/benchmark PR. Requested changes: none.

When the exact reviewed head is current, checks pass, the PR is mergeable, and
no stricter rule or operator pause applies, the orchestrator merges. If another
same-repository lane merged first, integration and exact-head re-review are
mandatory.

- **Closeout refs:**
  `docs/roadmaps/g16/048-agent-subagent-ownership-and-shimmer-benchmark.md`;
  one new `docs/logs/2026-09/*g16-048*.md`; g16 README, generation index,
  continuation maps, triage, and any production-shimmer card remain
  orchestrator-owned

### Handoff closeout

Before calling the runway complete, leave the card, execution log, verdict, and
next-task state honest. If blocked, record the blocker and stop rather than
making the handoff look more complete than it is.
