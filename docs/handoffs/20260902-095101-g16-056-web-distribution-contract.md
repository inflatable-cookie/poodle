---
title: g16.056 web distribution contract worker handoff
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
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260902-095101-g16-056-web-distribution-contract.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g16, web-distribution]
---

## What This Thread Was Doing

Poodle has accepted a compiled, source-free web package architecture. This
handoff dispatches only `g16.056`: freeze the package inventories, export maps,
declaration shape, dependency ownership, deterministic receipts, migration
boundary, and review oracles that the later build cards must implement.

This is one bounded contract-and-migration implementation lane. It does not
build packages, install a permanent certification harness, or perform release
work. No transcript or second prompt is part of the authority chain.

## Why It Matters

The later core, Svelte, React, and installed-certification cards must not choose
package semantics while writing build code. `g16.056` turns architecture 014
into an exact, reviewable contract: every public target resolves to compiled
JavaScript and declarations, browser and SSR choose the correct Svelte lane,
raw source cannot leak into archives, CSS and `marked` ownership stay isolated,
and the root markdown break is explicit without a compatibility shim.

Accepted merge unlocks only `g16.057`. It does not produce an artifact, certify
an installation, or unblock the `0.3.0` candidate by itself.

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
- **Planning artifacts at dispatch:** accepted
  `docs/architecture/014-compiled-web-package-distribution.md`, ready
  `docs/roadmaps/g16/056-web-distribution-contract.md`, serial successor cards
  `g16.057`–`g16.059`, and the canonical continuation runway
- **Worker branch:** `docs/g16-056-web-distribution-contract`
- **Worker worktree:** dedicated launcher-managed non-`main` worktree; manual
  fallback only under the operator-selected `AGENTS_WORKTREE_CONTAINER_DIR`
- **Worker worktree policy:** follow `Completion Protocol`; reuse a clean,
  registered, launcher-provided non-`main` worktree regardless of its generated
  path or branch name; never create a second worktree behind it
- **Required sibling worktree links:** none
- **Active spec lane:**
  `docs/roadmaps/g16/component-continuation-runway.md`
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g16/056-web-distribution-contract.md`
- **Allowed runway:** `g16.056` only
- **Remaining card budget:** one card
- **Dispatch topology:** independent ready-frontier lane beside `g16.050`;
  `g16.057` remains serially blocked on this card
- **Parallel safety check:** this lane owns only architecture/package contract,
  inventory, migration, and card-local evidence surfaces. It must not edit icon
  geometry/runtime files or any global roadmap front door. Stop if a shared
  mutable surface appears.
- **Surfaces this lane owns:**
  `docs/architecture/014-compiled-web-package-distribution.md`;
  `docs/specs/022-packaging-versioning-and-release-channel-rules.md` only for
  the smallest non-release package-boundary clarification required by the card;
  focused package/export inventory and migration documentation; scoped package
  or roster documentation required to freeze one canonical denominator;
  `docs/roadmaps/g16/056-web-distribution-contract.md`; one `g16.056` execution
  log; new `PAPERCUTS.md` entries only for newly observed execution friction
- **Integration ownership:** the orchestrator owns
  `docs/roadmaps/g16/README.md`, `docs/roadmaps/generation-index.md`,
  `docs/roadmaps/g16/component-continuation-runway.md`, continuation-register
  and roadmap front doors, cross-lane status, dispatch state, review, and merge
- **Merge ordering:** same-repository PRs merge one at a time; if another lane
  merges first, refresh this branch against current `main`, revalidate, and
  expect exact-head re-review
- **Canonical refs:** `docs/architecture/014-compiled-web-package-distribution.md`;
  `docs/architecture/002-token-system-and-package-layout.md`;
  `docs/specs/022-packaging-versioning-and-release-channel-rules.md`;
  `docs/contracts/001-working-rules.md`; current core, Svelte, and React package
  manifests and public entry inventories
- **Review oracle:** `g16.056` `## Review Oracle`
- **Model capability profile:** `day-to-day` non-frontier implementation worker;
  the architecture and acceptance rules are settled, leaving bounded contract,
  inventory, and migration projection with ordinary engineering judgment
- **Frontier-worker justification:** none
- **Tool/runtime restrictions:** no build implementation, generated package
  output, permanent pack-install harness, version, changelog, release-note,
  candidate-history, workflow, tag, publish, registry, consumer, sibling-repo,
  compatibility alias, source fallback, `*-windowed`, or native-visual work
- **Required validation:** focused docs links and package/spec drift checks;
  the relevant roster/export drift selectors discovered through Effigy;
  `effigy docs:check`; and `git diff --check origin/main...HEAD`
- **PR base/head:** current pushed `main` at dispatch / worker branch head
  pending
- **PR URL:** pending
- **Review state:** awaiting worker implementation and PR, then exact-head
  orchestrator review
- **Merge path:** orchestrator after accepted exact-head review and passing
  required checks

## Boundaries

Please keep this run inside the named runway:

- **In scope:** complete every ordered-work, acceptance, evidence, and review-
  oracle row in `g16.056`; freeze exact core, public Svelte, and private React
  `dist` inventories; stable public/chunk naming; files and `sideEffects` laws;
  declaration suffixes; every token/icon/style/runtime target; Svelte
  `browser`/`default` resolution; core's single lane; Svelte floor; optional
  `marked` ownership; CSS ownership; deterministic build and installed receipt
  schemas; forbidden archive content; one canonical roster denominator; the
  root-to-`./markdown` breaking migration; successor-card writable scopes and
  review oracles; one execution log and reviewable PR.
- **Out of scope:** `g16.057`–`g16.059` implementation; build drivers or scripts;
  Vite/TypeScript/Svelte build configuration; compiled artifacts; archive
  creation; permanent installed-consumer fixtures or harness changes; package
  version edits; `0.3.0` changelog, release notes, or candidate history;
  workflows; release commands; tags; publishing; registries; consumer or
  sibling-repository mutation; component behavior; compatibility shims,
  aliases, or source fallbacks.
- **Outcome shape:** an exact normative contract and migration boundary that
  lets later cards implement without choosing package semantics. Do not make a
  disposable spike or generated output the durable contract.
- Preserve the accepted architecture: core and Svelte are public, React is
  compiled/certified but private; `dist` is the only code boundary; Svelte uses
  `browser` for client and non-browser `default` for server; `import` is not an
  environment selector; `./types` retains runtime and declaration reachability;
  markdown moves to `./markdown` with no compatibility path.
- Do not invent package topology, alter the release sequence, change the Svelte
  floor, make React publishable, or weaken fail-closed source/archive rules.
- This handoff represents one worker lane, and sibling lanes may run
  concurrently. Write only inside **Surfaces this lane owns**. Leave every
  global roadmap/front-door surface to **Integration ownership**. Stop on
  shared mutable scope or a hidden cross-lane dependency rather than resolving
  it yourself.
- Work only in the clean dedicated worker worktree selected by `Completion
  Protocol`. Never edit the planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge belongs to the orchestrator after exact-head
  review and the required check gate.

## Important Context

- **Planning lineage:** the holistic posture review made compiled JavaScript and
  declarations a prerequisite for `0.3.0`. A disposable evidence packet then
  proved a source-free dual Svelte shape, the `>=5.56.8 <6` floor, parser/CSS
  isolation, deterministic archives, and the need for runtime plus declaration
  reachability at `./types`. The orchestrator promoted that packet into
  architecture 014 and serial cards `g16.056`–`g16.059`.
- **Why this card is ready:** package membership, public/private ownership,
  client/server resolution, declaration/source laws, dependency policy,
  migration break, receipt requirements, acceptance, and stop conditions are
  already settled. This worker translates them into exact canonical contract
  surfaces; it does not discover or choose them.
- **Decisions and preferences:** export inventories are explicit and sorted;
  names are stable; declarations resolve under Bundler and NodeNext; CSS stays
  in core; ordinary Button/Select graphs remain parser-free; only
  `./markdown` owns `marked`; receipts exclude timestamps and absolute paths;
  raw Svelte, non-declaration TypeScript/TSX, maps, `src/`, workspace aliases,
  and source fallbacks fail closed.
- **Open tensions:** current manifests and inventories may expose drift from the
  accepted target. Record the target contract and successor migration boundary;
  do not repair manifests or build mechanics here. Stop if exact inventories
  reveal a new product/export choice rather than choosing it silently.
- **Report after:** the package/export and receipt inventories are coherent,
  then again after migration/roster/oracle projection and final validation.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Run the `Completion Protocol` preflight before broad reads. Then read
`AGENTS.md`, architecture 014, `g16.056`, packaging spec 022, working rules,
package-layout architecture 002, all three package manifests, and current public
entry/roster inventories. Build one exact target matrix before editing prose;
use it to reconcile every contract and successor-card boundary.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad reads,
   run `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`. Do not run
   `effigy tasks`, `effigy doctor`, discovery commands, or broad repository
   reads before this decision.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided dedicated worktree.
   Record the actual root and branch. Do not compare generated names with this
   handoff or create another worktree merely because they differ.
3. If the current context is `main`, dirty, unregistered, or otherwise
   unusable, stop and inspect the named launcher workspace first. Only when a
   manual fallback is required may you read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and create one unique branch/worktree under
   that operator-selected container from `origin/main`. Ask the operator if the
   variable is absent. Never use `/tmp`, `TMPDIR`, or a guessed path; never
   clean, reset, stash over, or discard dirty state. A launcher-supplied dirty
   or `main` worktree is a stop condition, not permission to create a duplicate.
4. From the selected worktree, run
   `GIT_SSH_COMMAND="ssh -o ConnectTimeout=10 -o BatchMode=yes" git fetch origin`.
   Confirm `HEAD == origin/main`, confirm
   `git merge-base --is-ancestor a809792c62399523830068082454445123754879 HEAD`,
   and confirm this repository-relative handoff exists in `HEAD`. Load it with
   `git show HEAD:docs/handoffs/20260902-095101-g16-056-web-distribution-contract.md`.
   If the absolute dispatch file differs from that tracked blob, stop. The
   committed `HEAD` copy is canonical.
5. Required sibling worktree links are `none`.
6. Read the active milestone, assigned card, `AGENTS.md`, and canonical refs.
   Use Effigy only where it fits the job and record what actually ran.

### While you work

- Execute `g16.056` as one contract-and-migration batch. Start from an exact
  package/export/receipt matrix; do not drift into build implementation to
  test-drive the prose.
- Inspect all three current manifests and public inventories. Distinguish
  current-state evidence from the accepted target. Keep unresolved mechanical
  migration in the scoped successor cards.
- After each meaningful chunk, report changed files, validation actually run,
  what remains, risks, and blockers.
- Stop if the work needs a new export/product choice, compatibility behavior,
  build code, permanent harness code, release/version/history edits, workflow
  authority, or another lane's mutable surface.

### When the assigned runway is complete

1. Run the required final validation exactly as listed in **Current State**.
   Do not run any `*-windowed` or native-visual selector.
2. Falsify every `g16.056` oracle row. At minimum plant and restore: an
   `import` condition before `default`; a declarations-only `./types`; a
   wildcard that can expose raw `.svelte`; `marked` in an ordinary root graph;
   and an attempted `0.3.0` release-note edit. Record why each proof bites.
3. Reconcile architecture 014, package/export inventories, migration/roster
   documentation, `g16.056`, and one execution log with the actual result. Do
   not edit the g16 README, generation index, continuation runway/register, or
   global roadmap front doors.
4. Push the worker branch. If a sibling lane merged first, refresh onto current
   `main`, rerun required validation, and report the new exact head.
5. Open one PR against current pushed `main`. The planning base above is not a
   self-referential hash for the handoff commit.
6. Link architecture 014, `g16.056`, the exact inventory/contract surfaces,
   migration boundary, validation, falsification evidence, diff-scope proof,
   and unresolved items.
7. Report the PR URL and exact head. Do not merge, start `g16.057`, build or
   pack packages, create the permanent harness, or perform release work.

### Review and merge path

The orchestrator reviews the current PR head against architecture 014,
`g16.056`, exact inventories, migration scope, full diff, falsification, and
required checks. Shared-identity review is posted as the canonical PR comment
when formal self-approval is unavailable. Requested changes stay on this
branch. Blocking classes are `execution-miss`, `oracle-gap`, `planning-change`,
`validation-gap`, and `integration-drift`. Requested changes: none. The
orchestrator alone merges an accepted, current, mergeable head after required
checks.

- **Closeout refs:**
  `docs/architecture/014-compiled-web-package-distribution.md`,
  `docs/roadmaps/g16/056-web-distribution-contract.md`, scoped package/export
  and migration documentation, and one `docs/logs/2026-09/` g16.056 execution
  log. Global roadmap/front-door state and `g16.057` readiness remain
  orchestrator-owned after merge.

### Handoff closeout

Before calling the runway complete, leave the target contract, inventory,
migration boundary, card, log, and next-task state honest. An accepted
`g16.056` merge unlocks `g16.057` only. It does not implement a build, create a
certification receipt, unblock `g16.054`, change `0.3.0`, or release anything.
If blocked, record the exact blocker and stop.
