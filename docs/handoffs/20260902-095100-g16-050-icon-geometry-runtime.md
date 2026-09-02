---
title: g16.050 icon geometry internal runtime worker handoff
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
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260902-095100-g16-050-icon-geometry-runtime.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g16, icon-geometry]
---

## What This Thread Was Doing

Poodle has merged the deterministic icon-geometry foundation from `g16.049`.
This handoff dispatches only `g16.050`: the private planning/lifecycle runtime,
resolved geometry node, shared Rust and GPUI headless substrate, and private
Svelte/React shells. Candidate geometry is fixture input only.

This is one bounded implementation lane. No transcript or second prompt is
part of the authority chain.

## Why It Matters

The geometry foundation is useful only when every active renderer can consume
the same resolved frame and lifecycle without changing the existing Icon API.
This card proves that internal substrate before native visual admission. It
does not approve a pair, expose IconMorph, or claim pixels.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `a809792c62399523830068082454445123754879`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `a809792c62399523830068082454445123754879` before this handoff was finalized
- **Planning checkout:** clean; canonical promotion and front-door updates are
  already committed and pushed
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** merged `g16.049` foundation and
  its accepted exact-head evidence; the card and front-door readiness update
  are committed with this handoff before launch
- **Worker branch:** `feat/g16-050-icon-geometry-runtime`
- **Worker worktree:**
  `/Users/tom/.t3/worktrees/poodle/g16-050-icon-geometry-runtime`
- **Worktree creation command:** fallback only:
  `git worktree add /Users/tom/.t3/worktrees/poodle/g16-050-icon-geometry-runtime -b feat/g16-050-icon-geometry-runtime origin/main`
- **Worker worktree policy:** follow `Completion Protocol`; launcher worktree
  first, named/manual fallback only when required
- **Required sibling worktree links:** none
- **Active spec lane:**
  `docs/roadmaps/g16/component-continuation-runway.md`
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g16/050-icon-geometry-internal-runtime-substrate.md`
- **Allowed runway:** `g16.050` only
- **Remaining card budget:** one card
- **Dispatch topology:** parallel ready frontier with `g16.056` web
  distribution contract; the lanes have no intended shared mutable scope
- **Parallel safety check:** this lane owns only private icon-geometry runtime,
  node, shared-render, GPUI, shell, fixture, test, card, and log surfaces.
  `g16.056` owns distribution architecture/contracts. Stop if either lane needs
  the other's files or a global front door.
- **Surfaces this lane owns:** private icon-geometry plan/lifecycle modules;
  the distinct resolved geometry node payload; shared Rust composition; GPUI
  production backend and headless probes; private Svelte/React shells and
  bounded browser fixtures; focused tests; implementation evidence in
  `docs/architecture/013-icon-geometry-substrate.md` when required;
  `docs/roadmaps/g16/050-icon-geometry-internal-runtime-substrate.md`; one
  `g16.050` execution log; new `PAPERCUTS.md` entries only for newly observed
  execution friction
- **Integration ownership:** the orchestrator owns
  `docs/roadmaps/g16/README.md`, `docs/roadmaps/generation-index.md`,
  `docs/roadmaps/g16/component-continuation-runway.md`, continuation/register
  front doors, cross-lane status, review, merge, and `g16.051` readiness
- **Merge ordering:** same-repository PRs merge one at a time; the orchestrator
  refreshes this head against current `main` and re-reviews it if a sibling lane
  merges first
- **Canonical refs:** `docs/architecture/013-icon-geometry-substrate.md`,
  `docs/architecture/012-semantic-motion-policy.md`;
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/components/icon.md`,
  `docs/contracts/components/icon-button.md`
- **Review oracle:** `g16.050` `## Review Oracle`
- **Model capability profile:** `day-to-day` non-frontier implementation
  worker; this is bounded cross-runtime implementation over settled planning
- **Frontier-worker justification:** none
- **Tool/runtime restrictions:** no `*-windowed`, native-visual, public API,
  pair-status/eligibility promotion, release/workflow, consumer, sibling-repo,
  or Jetstream work
- **Required validation:** focused paired lifecycle vectors; node/render
  assertions; focused GPUI headless probes; web SSR/hydration/focus/layout and
  controlled browser tests; relevant drift/audit selectors;
  `effigy ci:web`; `effigy ci:rust`; `effigy ci:native`;
  `effigy docs:check`; one final headless `effigy qa`; and
  `git diff --check origin/main...HEAD`
- **PR base/head:** current pushed `main` at dispatch / worker branch head
  pending
- **PR URL:** pending
- **Review state:** awaiting worker implementation and PR, then exact-head
  orchestrator review
- **Merge path:** orchestrator after accepted exact-head review and passing
  required checks

## Boundaries

Please keep this run inside the named runway:

- **In scope:** deliver every ordered-work, acceptance, evidence, and review-
  oracle row in `g16.050`: pure plan/lifecycle traces; distinct resolved
  geometry node; shared Rust construction; production GPUI path and headless
  budget proof; private Svelte/React shells and controlled browser proof; one
  execution log and reviewable PR.
- **Out of scope:** changing any `g16.049` pair status or runtime-eligibility
  state; public IconMorph or raw geometry; public package exports; Icon,
  IconProvider, or IconButton behavior changes; `g16.051`; native pixel or
  windowed evidence; visual-ledger admission; Jetstream; release/workflow;
  consumers or sibling repositories; global g16 README, generation index, or
  continuation-runway edits.
- **Outcome shape:** private internal runtime substrate over candidate geometry
  fixtures. Full/reduced/frozen behavior remains owned by architecture 012;
  existing static Icon behavior and public surfaces remain unchanged.
- Do not invent architecture, change public contracts, widen the roadmap, or
  choose a new pair-admission, paint, policy, API, or release rule.
- This handoff represents one worker lane, and `g16.056` may run concurrently.
  Write only inside **Surfaces this lane owns**. Leave global closeout and
  front-door surfaces to **Integration ownership**. If shared mutable scope, a
  hidden dependency, or another lane's write appears, stop and report it.
- Work only in the clean dedicated worker worktree selected by `Completion
  Protocol`. Never edit the planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Review and merge belong to the orchestrator.

## Important Context

- **Planning lineage:** `g16.049` merged a deterministic TS/Rust geometry
  foundation with zero accepted/runtime-eligible pairs, six candidates, and six
  rejected pairs. The operator kept `g16.050` fixture-only until a later visual
  gate.
- **Why this card is ready:** architecture 013, architecture 012, the merged
  geometry registry, lifecycle laws, budgets, public-surface exclusions,
  acceptance, review counterexamples, validation, and stop conditions are all
  settled in the card.
- **Decisions and preferences:** NodeKind::Icon remains static; a distinct
  resolved geometry leaf carries a compact validated frame; shared composition
  owns pair meaning; renderers receive resolved geometry only; latest state
  wins; reversal rebases from the current sample; repeated targets are inert;
  reduced/frozen snap; teardown leaves no live handle.
- **Open tensions:** headless evidence may expose an unsupported paint,
  allocation, scheduler, or lifecycle requirement. Stop rather than adding a
  public path, backend semantic lookup, duplicate clock, or visual claim.
- **Report after:** the pure lifecycle plus resolved-node batch is coherent,
  then after GPUI and web shell evidence, then after final validation.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Run the `Completion Protocol` preflight before broad reads. Then read
`AGENTS.md`, the continuation runway, `g16.050`, architectures 012 and 013, the
Icon/IconButton contracts, the merged `g16.049` log, and the current generated
registry. Start with the pure plan/lifecycle traces and resolved node boundary.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad reads,
   run `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch; do not compare its generated path/branch with the planned
   fallback or create another worktree merely because they differ.
3. If current context is `main`, dirty, unregistered, or unusable, inspect the
   named worktree. If unusable, read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the operator when absent. Create a
   unique worktree/branch there from pushed `origin/main`. Never use `/tmp`,
   `TMPDIR`, or a guessed path; never clean, reset, stash over, or discard dirty
   state. Report a launcher-supplied dirty or `main` worktree instead.
4. From the selected worktree, record this handoff's repository-relative path.
   Run `GIT_SSH_COMMAND="ssh -o ConnectTimeout=10 -o BatchMode=yes" git fetch
   origin`. Confirm `HEAD` equals `origin/main`, confirm
   `git merge-base --is-ancestor a809792c62399523830068082454445123754879 HEAD`,
   and confirm this relative path exists in `HEAD`. Load it with
   `git show HEAD:docs/handoffs/20260902-095100-g16-050-icon-geometry-runtime.md`.
   If the absolute dispatch file differs from that tracked blob, stop.
5. Required sibling links are `none`.
6. Read the active milestone, assigned card, `AGENTS.md`, and canonical refs.
7. Use Effigy where it fits the job. Run cheap orientation checks and record
   what actually ran.

### While you work

- Execute `g16.050` as one ordered runway. Keep commits aligned with the pure
  plan/node batch and renderer evidence batch, not model turns.
- Candidate geometry is fixture-only. Do not edit its candidate status,
  runtime eligibility, manifest authority, or reviewer/acceptance fields.
- Keep shared composition authoritative for pair meaning and send only resolved
  geometry to backends. Do not add renderer lookup or raw SVG transport.
- After each meaningful chunk, report changed files, validation actually run,
  what remains, risks, and blockers.
- Stop if a contract is missing, scope expands, authority is absent, another
  lane owns a surface, or validation changes the plan.

### When the assigned runway is complete

1. Run the required final validation exactly as listed in **Current State**.
   Do not run any local `*-windowed` or native-visual selector.
2. Falsify the diff against every `g16.050` review-oracle row. At minimum plant
   and restore: A→B→A before completion; unrelated pair replacement;
   full→reduced→frozen tightening; missing shared lookup; existing static Icon;
   and teardown during scheduled browser/native work. Record why each proof
   bites.
3. Update `g16.050`, architecture implementation evidence when required, and
   one execution log. Do not edit the g16 README, generation index,
   continuation runway/register, `g16.051`, or pair status.
4. Push the worker branch. If a sibling lane merged first, rebase onto current
   `main`, rerun the required validation, and report the new exact head.
5. Open one PR against current pushed `main`. The planning base above is not a
   self-referential hash for the handoff commit.
6. Link the card, architectures, changed surfaces, evidence, validation, and
   unresolved items. State explicitly that candidate geometry is fixture-only,
   pair status is unchanged, and no public/native-visual claim was made.
7. Report the PR URL and exact head. Do not merge and do not start `g16.051`.

### Review and merge path

The orchestrator reviews the current PR head against the card, full diff,
shared lifecycle vectors, node/backend boundary, browser/headless evidence,
budgets, and validation. Shared-identity review is posted as the canonical PR
comment when formal self-approval is unavailable. Requested changes stay on
this branch. Blocking classes are `execution-miss`, `oracle-gap`,
`planning-change`, `validation-gap`, and `integration-drift`. Requested changes:
none. The orchestrator alone merges a current, mergeable head after checks.

- **Closeout refs:**
  `docs/roadmaps/g16/050-icon-geometry-internal-runtime-substrate.md`,
  `docs/architecture/013-icon-geometry-substrate.md` implementation evidence
  when required, and one `docs/logs/2026-09/` g16.050 execution log. Global
  roadmap/front-door and `g16.051` readiness updates remain orchestrator-owned.

### Handoff closeout

Before calling the runway complete, leave the card, architecture evidence,
log, private substrate, and next-task state honest. Completion only unblocks
orchestrator assessment of `g16.051`; it does not admit a pair, public API,
native visual capability, Jetstream path, or release.
