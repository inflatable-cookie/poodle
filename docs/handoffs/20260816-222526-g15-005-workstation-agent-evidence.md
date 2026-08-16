---
title: g15.005 workstation and agent evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-16
updated: 2026-08-16
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260816-222526-g15-005-workstation-agent-evidence.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

Poodle's g15 runway is building an honest v0.2.0 release baseline for every
public Svelte component while keeping React tied to the same observable
contract cases. The first three evidence tranches are complete, and g15.006
has just closed the remaining React implementation and gallery gaps.

This worker takes the final web evidence tranche: 24 workstation and agent
components on Svelte, paired with the 23 React gaps that remain. The work is
component-local evidence and bounded fixes, not another shared conformance
system.

## Why It Matters

Longhorn and most projects under `~/Dev/projects` depend on Poodle. This card
is the last focused-evidence gap between the current web packages and the
v0.2.0 release baseline: it should leave both Svelte and React at 175/0 named
focused evidence without repeating the failed g13/g14 authority experiments.

## Current State

Here is the state this worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `c5f63180f83cc672da3e7ad0a8e1cea49a96bf15`
- **Pushed main verification:** local `HEAD` equalled `origin/main` at
  `c5f63180f83cc672da3e7ad0a8e1cea49a96bf15` before this handoff was written
- **Planning checkout:** clean before this handoff was written
- **Planning artifacts included at the base:** g15 front doors advanced after
  PR #28; g15.005 marked ready; g15.006 marked complete
- **Worker branch:** `t3code/g15-005-workstation-agent-evidence`
- **Worker worktree:** `/Users/tom/.t3/worktrees/poodle/g15-005-workstation-agent-evidence`
- **Worktree creation command:** `git fetch origin && git worktree add /Users/tom/.t3/worktrees/poodle/g15-005-workstation-agent-evidence -b t3code/g15-005-workstation-agent-evidence origin/main`
- **Worker worktree policy:** use the named clean non-`main` worktree when it
  matches; otherwise create a unique temporary worktree and branch from
  `origin/main` before editing
- **Active spec lane:** none; component contracts and working rules are
  canonical
- **Roadmap milestone:** `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/README.md`
- **Ready cards, in order:** `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/005-svelte-focused-evidence-workstation-agent.md`
- **Allowed runway:** g15.005 only, in batches A, B, then C
- **Remaining card budget:** one card, three meaningful batches
- **Dispatch topology:** serial final web-evidence lane
- **Parallel safety check:** no concurrent lane is assumed; the roster and gap
  register are shared final-count surfaces, so do not overwrite newer totals
  if another PR lands while this run is active
- **Canonical refs:** `/Users/tom/Dev/projects/poodle/docs/contracts/001-working-rules.md`, `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/release-baseline-roster.md`, `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/release-gap-register.md`, and the 24 named contracts under `/Users/tom/Dev/projects/poodle/docs/contracts/components/`
- **Model capability profile:** capable coding model, medium reasoning; stop
  rather than guess on public API or contract ambiguity
- **Tool/runtime restrictions:** never run a `*-windowed`,
  `test:native-visual`, native, GPUI, Jetstream, `ci:jetstream`, or
  `qa:jetstream` selector; do not create a Jetstream sibling link
- **Required validation:** narrow touched tests after each batch, then
  `effigy test:components`, `effigy check:svelte`, `effigy react:build`,
  `effigy docs:check`, and `git diff --check origin/main...HEAD`
- **PR base/head:** `main` ← `t3code/g15-005-workstation-agent-evidence`
- **PR URL:** pending
- **Review state:** awaiting implementation and orchestrator review
- **Merge authorisation:** worker must not merge; the operator retains merge
  authority

## Boundaries

Please keep this run inside g15.005:

- **In scope:** named focused Svelte and React tests for the card's 24
  workstation/agent components; bounded harness fixtures; bounded fixes to a
  scoped component when the new evidence exposes a real defect; contract-first
  documentation for any such observable fix; focused-evidence roster/register
  rows; one August batch log; new papercuts found during execution.
- **Out of scope:** specimens or preview redesign, native/GPUI/Jetstream work,
  new shared corpora or comparators, a new parity authority, broad refactors,
  public API changes not already required by a scoped contract defect, release
  mutations, later g15 cards, and roadmap/front-door status.
- Tests must assert load-bearing observable behavior. A renamed anatomy smoke,
  a mount-only assertion, or a selector-presence check does not close a gap.
- Pair Svelte and React around the same contract behavior, but keep the tests
  idiomatic and owner-local. Do not build a framework-neutral case corpus.
- If a test exposes an observable defect, update the relevant contract before
  changing behavior. If the contract is ambiguous or the fix would widen the
  public API, stop and report it through the operator.
- Work only in `/Users/tom/.t3/worktrees/poodle/g15-005-workstation-agent-evidence`
  on `t3code/g15-005-workstation-agent-evidence`, or in the recorded temporary
  fallback created by the startup preflight. Never edit the orchestrator's
  planning checkout or discard another checkout's dirty state.
- Do not merge the PR.

## Important Context

- **Planning lineage:** `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/001-release-baseline-roster-inventory.md` froze the 175-component denominator;
  g15.002–g15.004 established the paired evidence threshold; `/Users/tom/Dev/projects/poodle/docs/logs/2026-08/16-g15-006-react-mirror-closure.md` supplied the final React implementations and the current baseline.
- **Current counts:** 151/24 focused Svelte evidence and 152/23 focused React
  evidence. g15.005 should finish at 175/0 on both axes. AgentSubagent already
  has React evidence, which is why the card closes 24 Svelte gaps but 23 React
  gaps.
- **Why the card is ready:** its 24-component list, three batches, minimum
  evidence threshold, writable scope, final counts, validation, and stop
  conditions are explicit. The missing React AgentPlanRecord dependency landed
  in PR #28.
- **Decisions and preferences:** Svelte is the reference implementation;
  parity means observable semantics before renderer mechanics. Catalogue
  specimens are human-facing documentation and are not test matrices. The
  exhaustive shared case-corpus approach was retired and must not return here.
- **Known risks:** focused tests may expose genuine web drift. Keep fixes
  surgical and contract-first. Existing Svelte warnings and Effigy doctor
  findings are baseline health, not permission to refactor adjacent code.
- **Dependency setup:** a fresh worktree may lack `node_modules`; use the
  repo-owned setup path (`effigy bootstrap:deps`) if a selector fails only for
  missing dependencies. Do not improvise a Jetstream link.
- **Report after:** each of batches A, B, and C from the card
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Start by reading this handoff from the top. Before broad repository reads, run
the startup worktree-safety preflight below. Then read
`/Users/tom/Dev/projects/poodle/AGENTS.md`, the g15 milestone, g15.005, the
roster/register, and the eight contracts in batch A.

For each component, identify one observable contract behavior that can really
break, confirm the existing anatomy smoke does not already prove it, then add
the paired Svelte and React evidence. Run the batch's narrow touched tests
before moving to batch B.

## Completion Protocol

### Before you start

1. Read this handoff path, then run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain` before broad repository reads.
2. Run `git fetch origin`. Use the named worktree only if its path and branch
   match `/Users/tom/.t3/worktrees/poodle/g15-005-workstation-agent-evidence`
   and `t3code/g15-005-workstation-agent-evidence`, its status is empty, its
   branch is not `main`, and its `HEAD` is `origin/main`.
3. If any condition fails, do not edit the current checkout. Create a unique
   temporary worktree and branch from pushed `origin/main`, for example:
   `TEMP_SUFFIX="$(date +%Y%m%d%H%M%S)-$$"; TEMP_WORKTREE="${TMPDIR:-/tmp}/northstar-worker-${TEMP_SUFFIX}"; TEMP_BRANCH="t3code/g15-005-workstation-agent-evidence-tmp-${TEMP_SUFFIX}"; git worktree add -b "$TEMP_BRANCH" "$TEMP_WORKTREE" "$(git rev-parse origin/main)"`
   Record the actual fallback path and branch, and use only that worktree.
   Never clean, reset, stash over, or discard the original checkout's state.
4. From the selected worktree, confirm `git rev-parse HEAD` equals
   `git rev-parse origin/main`, confirm
   `git merge-base --is-ancestor c5f63180f83cc672da3e7ad0a8e1cea49a96bf15 HEAD`
   succeeds, and confirm this handoff file exists in `HEAD`.
5. Read `/Users/tom/Dev/projects/poodle/AGENTS.md`,
   `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/README.md`,
   `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/005-svelte-focused-evidence-workstation-agent.md`,
   the roster/register, working rules, and the contracts for the current batch.
6. Use Effigy for the repo's cheap orientation and record what actually ran.
   Do not refresh the graph unless code-navigation work genuinely needs it.

### While you work

- Execute batches A, B, and C in order. Keep commits aligned with meaningful
  batches, not model turns.
- After each batch, report changed files, validation actually run, remaining
  batches, new risks, and blockers through the operator.
- Stop if a contract is missing or ambiguous, scope expands, a public API
  decision appears, or validation changes the plan.
- Do not quietly turn an open question into a new architecture or shared test
  authority.

### When the assigned runway is complete

1. Run `effigy test:components`, `effigy check:svelte`,
   `effigy react:build`, `effigy docs:check`, and
   `git diff --check origin/main...HEAD`. Do not substitute a bare
   `git diff --check` after committing.
2. Update only the focused-evidence cells/counts in
   `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/release-baseline-roster.md`
   and `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/release-gap-register.md`,
   plus one honest August batch log under
   `/Users/tom/Dev/projects/poodle/docs/logs/2026-08/`. Do not change roadmap
   status, the generation front doors, or the dispatch ledger.
3. If current `origin/main` moved, rebase or merge it before final validation
   and preserve newer additive evidence rather than copying old totals over it.
4. Push the selected worker branch and open a reviewable PR against current
   `main`.
5. In the PR body, link this handoff, g15.005, the milestone, changed surfaces,
   named evidence, validation, and unresolved items.
6. Report the PR URL and final evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will review the PR independently against the contracts, card,
diff, and checks. Because worker and orchestrator share a GitHub identity, the
orchestrator records its verdict in a PR comment rather than self-approving.
Requested changes are none yet. The operator retains explicit merge authority.

- **Closeout refs:** `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/005-svelte-focused-evidence-workstation-agent.md`, `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/README.md`, `/Users/tom/Dev/projects/poodle/docs/roadmaps/generation-index.md`, `/Users/tom/Dev/projects/poodle/docs/roadmaps/README.md`, `/Users/tom/Dev/projects/poodle/docs/roadmaps/dispatch.md`, `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/release-baseline-roster.md`, `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/release-gap-register.md`, and the worker's log under `/Users/tom/Dev/projects/poodle/docs/logs/2026-08/`

### Handoff closeout

Leave the test evidence, roster, gap register, and batch log honest. If the
work blocks, record the blocker and stop. The worker does not advance g15.005
or choose the next roadmap card; the orchestrator owns that after PR review.
