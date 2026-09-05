---
title: g16.001 parity evidence ledger worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-25
updated: 2026-08-25
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260825-221330-g16-001-parity-evidence-ledger-worker.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, parity, evidence]
---

## What This Thread Was Doing

Poodle shipped v0.2.2 after two rejected consolidated-conformance attempts and
a release-first gap-closing runway. The active cohort is structurally complete,
but current parity reports still mix old g04/g09/g10 artifacts with g15
evidence. This run executes `g16.001`: build one truthful component-level
evidence ledger and repair the active reports that contradict it.

This is one bounded implementation thread. Start from this file; no copied
transcript or second prompt is required.

## Why It Matters

Poodle cannot choose the next conformance programme while implementation
presence, specimen construction, focused tests, mounted behaviour,
accessibility, and visual comparison are reported as if they were the same
claim. The ledger must show what each runtime actually proves today and expose
the measured gaps that should shape `g16.002`.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `d169cc053e6eeaf0fd8d0b20417a8891e0f6c44f`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `d169cc053e6eeaf0fd8d0b20417a8891e0f6c44f` before this handoff was created
- **Planning checkout:** clean at the recorded base
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** g16 aim, generation index, and
  ready card `docs/roadmaps/g16/001-active-cohort-parity-evidence-ledger.md`
- **Worker branch:** `t3code/g16-001-parity-evidence-ledger`
- **Worker worktree:** launcher-provided clean non-`main` worktree
- **Worktree creation command:** none in the normal path; use the launcher
  worktree. Manual fallback follows `docs/contracts/005-agent-local-paths.md`
- **Worker worktree policy:** use the clean, dedicated, non-`main` registered
  worktree supplied by the launcher even if its path or branch differs from
  these placeholders. Record the actual path/branch and never create a second
  worktree for that reason. If the current context is unusable, use the named
  worktree when it matches; only then read `.agents.local.env`, require
  `AGENTS_WORKTREE_CONTAINER_DIR`, and create a unique manual worktree/branch
  under that container from `origin/main`. Ask the operator if the file or key
  is absent; never use `/tmp`, `TMPDIR`, or a guessed path
- **Active spec lane:** existing parity-evidence rules in specs 008 and 025;
  no new conformance spec
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready cards, in order:** `g16.001` only
- **Allowed runway:** execute the complete evidence-ledger and stale-report
  repair in `g16.001`
- **Remaining card budget:** one card
- **Dispatch topology:** serial; no parallel lane is ready
- **Parallel safety check:** the ledger owns shared report generators,
  generated artifacts, task comments, and roadmap status. A parallel parity
  lane would overlap the same authority and is unsafe
- **Canonical refs:** `docs/architecture/001-poodle-system-shape.md`,
  `docs/architecture/003-component-docs-ia-and-implementation-substrates.md`;
  `docs/contracts/001-working-rules.md`,
  `docs/specs/008-parity-evidence-documented-delta-and-downstream-extension-rules.md`,
  `docs/specs/025-parity-automation-and-harness-boundary.md`
- **Model capability profile:** capable implementation model with medium or
  stronger reasoning; escalate ambiguity rather than inventing evidence
- **Tool/runtime restrictions:** Effigy-first; fully headless. Never run
  `*-windowed`, native visual, Jetstream preview/QA, release mutation, or edit
  `.github/workflows/`
- **Required validation:** focused ledger/report tests,
  `effigy probe:gpui-specimens`, `effigy regressions:native`,
  `effigy test:visual-fixtures`, `effigy docs:check`, `effigy ci:web`,
  `effigy ci:native`, one final `effigy qa`, and
  `git diff --check origin/main...HEAD`
- **PR base/head:** `main` ← worker branch
- **PR URL:** pending
- **Review state:** awaiting worker implementation and PR
- **Merge authorisation:** not granted; the operator must explicitly authorise
  merge after orchestrator review

## Boundaries

Keep the run inside `g16.001`.

- **In scope:** one current 175-component evidence ledger; deterministic
  denominator/reference checks; repair or retirement of active stale
  parity/accessibility reports; the bounded g15 capture supersession note;
  Longhorn lab capture correction; one execution log; honest closeout state.
- **Out of scope:** component contracts, public APIs, component or runtime
  implementations, specimen content, visual fixtures or thresholds, new
  behaviour cases, new runtime adapters, new comparators, Jetstream admission,
  downstream repositories, releases, and workflows.
- Do not invent a universal component schema, shared executable corpus,
  normalized observation model, or new component authority.
- Explicit evidence mapping is allowed when semantics cannot be inferred, but
  the live roster must drive participation so no manual registry can omit a
  component silently.
- A specimen route proves construction only. A focused web test proves only
  its named claim. Svelte axe evidence does not transfer to React. Shared Rust
  composition does not make Jetstream pass.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's planning checkout or an unrelated dirty checkout.
- Do not merge the PR.

## Important Context

- **Planning lineage:** g13's Rust-authored IR was retired after high cost and
  no replacement value. g14's executable conformance plane was rejected after
  manual registration omitted HistoryCenter and the corrected comparison found
  1,205 differences. g15 deliberately shipped through layered evidence, not a
  third parity architecture.
- **Why the card is ready:** the operator accepted the current-state stocktake.
  The denominator, active cohort, Jetstream deferral, capture boundary,
  evidence vocabulary, writable scope, acceptance, and stop conditions are
  fixed in `g16.001`.
- **Decisions and preferences:** Svelte is reference. Current expected counts
  are 175 public web components and 174 portable native components, with
  `MeterSurface` the sole native `not-applicable`. GPUI route construction is
  broad; mounted behaviour is bounded; three-runtime visual comparison is
  Button-only; GPUI pixels are windowed/non-activating and operator-owned.
- **Open tensions:** some explicit evidence claims cannot be derived safely
  from names alone. Keep those claims auditable and roster-complete without
  building another conformance data model. Stale active specs may require a
  narrow truth correction; stop if changing their normative meaning becomes
  necessary.
- **Report after:** the ledger/checker and first artifact-disposition pass,
  then the coherent final batch
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Read it from the top. Before
broad repository reads, run the startup worktree-safety preflight below. If the
current context is a clean, dedicated, non-`main` registered worktree, use it
immediately and record its actual path and branch. Do not create another
worktree because a launcher-generated name differs from this handoff.

Then read `AGENTS.md`, `g16.001`, the g16 milestone, the governing refs, the
g15 release roster/gap register, g14's conformance estate, and the active report
generators/artifacts named by the card. Inventory active consumers of each
report before changing or retiring it.

Take the first coherent chunk: derive the live roster, define the non-inflating
evidence cells, add the deterministic checker, and produce the first ledger.
Report the actual counts and any evidence source that cannot be classified
without a planning decision before repairing reports around an assumption.

## Completion Protocol

### Before you start

1. Run this quick read-only probe before broad repository reads:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch and do not compare them with the placeholders above.
3. If the launcher supplied a dirty or `main` worktree, stop and report it.
   Do not silently create a second worktree. If no usable launcher worktree
   exists through the normal launch path, follow
   `docs/contracts/005-agent-local-paths.md`: read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, ask the operator if absent, and create one
   unique worktree below that container from `origin/main`. Never use `/tmp`,
   `TMPDIR`, or a guessed path. Never clean, reset, stash, or discard another
   checkout's changes.
4. From the selected worktree, run `git fetch origin`; confirm `HEAD` equals
   `origin/main`; confirm
   `git merge-base --is-ancestor d169cc053e6eeaf0fd8d0b20417a8891e0f6c44f HEAD`
   succeeds; confirm this handoff exists in `HEAD`.
5. Read the active milestone, assigned card, `AGENTS.md`, and the canonical
   refs named above.
6. Use `effigy tasks` for selector discovery and `effigy doctor` only where
   routing or health ambiguity matters. Record what actually ran.

### While you work

- Execute only `g16.001`. Keep commits aligned with the ledger/checker batch
  and the report-repair/closeout batch.
- After each meaningful chunk, report changed files, measured counts,
  validation, remaining work, risks, and blockers through the operator.
- Stop if the denominator differs, a contract contradicts current runtime
  evidence, a runtime defect appears, or truthful repair needs architecture,
  runtime code, a workflow, a windowed run, or sibling Jetstream access.
- Do not make a missing evidence class green by inference, suppression, or
  borrowing another runtime's proof.

### When the assigned runway is complete

1. Run the final validation named in `## Current State`. Keep it headless.
2. Update the ledger, g16 card/milestone, one August execution log, report
   artifacts, and next-task state with the actual result.
3. Revisit the Longhorn conformance-lab triage note and leave its ownership and
   capture constraints honest. Do not close it unless this card resolves its
   open decision.
4. Push the worker branch and open a reviewable PR against current `main`.
5. Link the card, milestone, changed report surfaces, evidence counts,
   validation, and unresolved gaps in the PR body.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator reviews the PR independently against the card, live source,
diff, reports, and checks. With a shared GitHub identity, the orchestrator will
post the evidence-backed verdict as a PR comment. Make only requested changes
on this branch, push, and report back through the operator.

Current requested changes: none. Merge remains operator-authorised only.

- **Closeout refs:** `docs/roadmaps/g16/001-active-cohort-parity-evidence-ledger.md`,
  `docs/roadmaps/g16/README.md`, `docs/roadmaps/generation-index.md`, the new
  ledger, and the card's August execution log

### Handoff closeout

Leave the card, roadmap, log, triage disposition, and next-task state honest.
If blocked, record the exact blocker and stop rather than making the ledger look
complete.
