---
title: g16.020 component continuation audit worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle core
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260828-205115-g16-020-component-continuation-audit.md
base_required: pushed-main
base_commit: 69118d83173d3d69b284b5ecf6d7315dc43ae5a8
recommended_model: gpt-5.6-luna
tags: [coordination, handoff, worker, pr, audit, components, luna]
---

## What This Thread Was Doing

Execute ready audit card `g16.020`. Catalog component work completed through
g15 and g16, account for the full live component roster, distinguish remaining
implementation work from evidence-only gaps and unresolved decisions, and
return a bounded continuation map.

This is a Luna reconnaissance and planning-evidence worker. Use
`gpt-5.6-luna`. It edits only the card, one continuation register, and one
audit log. It does not implement components or choose ready implementation
cards.

Start from this file. No copied transcript or second prompt is required.

## Why It Matters

Poodle must continue broad component work while the dependable drag-and-drop
programme proceeds. The long g15/g16 run repaired and certified many
components, but the current ledger records evidence levels rather than a clean
map of remaining work. A bounded audit prevents completed work from
reappearing, missing evidence from being mislabeled as defects, and seven
drag-dependent components from becoming seven unrelated implementations.

The result gives the orchestrator a trustworthy component continuation lane to
run alongside architecture 011/spec 069. It does not stop or absorb the
drag-and-drop programme.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `69118d83173d3d69b284b5ecf6d7315dc43ae5a8`
- **Pushed main verification:** local `HEAD` and `origin/main` both equalled
  the planning base before this handoff was created
- **Planning checkout:** clean at the planning base
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Worker model:** `gpt-5.6-luna`, selected explicitly by the operator for the
  audit/reconnaissance role
- **Worker branch:** `t3code/g16-020-component-continuation-audit`
- **Worker worktree:**
  `/Users/tom/.t3/worktrees/poodle/g16-020-component-continuation-audit`
- **Worktree creation command:** `git worktree add -b t3code/g16-020-component-continuation-audit /Users/tom/.t3/worktrees/poodle/g16-020-component-continuation-audit origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even when its generated path
  or branch differs from these placeholders. Record the actual path/branch and
  do not create another worktree for a naming mismatch. If the current context
  is unusable, inspect the named worktree; only then read
  `.agents.local.env` and require `AGENTS_WORKTREE_CONTAINER_DIR`. Ask the
  operator if that local path contract is absent. Never use `/tmp`, `TMPDIR`,
  or a guessed path.
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready card:**
  `docs/roadmaps/g16/020-component-continuation-audit.md`
- **Output register:**
  `docs/roadmaps/g16/component-continuation-register.md`
- **Output log:** one new August file under `docs/logs/2026-08/`
- **Allowed runway:** execute `g16.020` only
- **Remaining card budget:** one audit card
- **Dispatch topology:** parallel with PR #94 repair and the orchestrator's
  drag-and-drop planning; no shared writable implementation, ledger, or
  drag-and-drop authority files
- **Current main ledger:** 46 mounted / 128 missing; 115 known-delta present /
  60 not-applicable
- **PR #94:** active changes-requested external input for `g16.019`; its
  proposed 47 / 127 totals are not current until merged
- **Parallel programme:**
  `docs/architecture/011-drag-and-drop-substrate.md` and
  `docs/specs/069-dependable-drag-and-drop-substrate.md`
- **Canonical refs:** `AGENTS.md`, `README.md`, `docs/README.md`,
  `docs/contracts/001-working-rules.md`, architecture 001/003/011, specs
  008/025/069, g15 and g16 roadmaps, the parity evidence ledger, current
  August logs, and open triage notes
- **Tool/runtime restrictions:** use repo-local Effigy selectors; everything is
  headless; never run `*-windowed`, native visual, Jetstream preview/QA,
  release, tag, publication, or workflow-mutation tasks
- **Required validation:** deterministic roster accounting,
  `check:parity-evidence-ledger`, `docs:lint`, `docs:check`, one final `qa`, and
  diff check
- **PR base/head:** `main` <- worker branch
- **PR URL:** pending
- **Review state:** awaiting worker audit and orchestrator review
- **Merge authorisation:** worker must not merge; explicit operator
  authorization follows orchestrator review

## Boundaries

- **In scope:** derive the live 175/174 roster; reconcile named component work
  from g15/g16 cards and logs; classify explicit non-claims, evidence-only
  gaps, known repairs, blocked decisions, programme-owned work, and unknowns;
  account for every component once; write no more than eight coherent
  candidate continuation lanes; update the audit card; and write one execution
  log.
- **Out of scope:** component, contract, specimen, test, runtime, token,
  package, public API, parity-ledger, generated report, architecture 011, spec
  069, front-door, workflow, release, downstream, and sibling-repository
  changes.
- Main is evidence authority. If PR #94 remains unmerged, record Select as an
  active changes-requested lane and preserve 46 / 128. If it merges before
  closeout, audit merged main. Never treat the PR branch as current evidence.
- Do not infer defects from missing evidence cells. Preserve `evidence-only`,
  `known repair`, `decision-blocked`, `programme-owned`, and `unknown` as
  distinct classifications.
- Do not perform a whole-codebase code-quality audit. Inspect live source only
  to resolve a named contradiction or classify a bounded candidate.
- Do not create a new schema, conformance corpus, component authority,
  generated runtime adapter, or competing evidence matrix.
- Drag-dependent Tabs, EditableList, Tree, ModelCatalogueEditor, OrderBy,
  BlockEditor, and DockRegion point to architecture 011/spec 069 as one
  programme-owned family. Non-drag work in those components may be recorded
  separately when observed.
- Keep NumberInput/editing decisions, accessibility programmes, visual
  comparison, motion research, and Jetstream admission distinct rather than
  presenting them as ordinary ready component fixes.

## Plan

1. **Prove the denominator.** Derive the public Svelte roster and match it to
   the checked g16 ledger. Stop on a count other than 175/174 or on duplicate,
   missing, or extra identity.
2. **Build deterministic indexes.** Index current g15/g16 cards and August logs
   by component name. Index explicit non-claims, open triage notes, and current
   programme authorities. Use `rg` for exact references and Effigy graph
   queries only for ownership/impact questions.
3. **Classify the roster.** Write
   `docs/roadmaps/g16/component-continuation-register.md`. Account for every
   component once. Point to the live ledger row instead of copying evidence
   cells. Mark confidence as observed, inferred, or unknown.
4. **Derive candidate lanes.** Return no more than eight family-level
   continuation candidates with exact gaps, dependencies, decision authority,
   evidence gain, file overlap, coherent batching, and readiness. Include one
   drag-and-drop programme-owned lane, not seven replacements.
5. **Close the audit honestly.** Update only `g16.020`, add one August log,
   validate, push the branch, and open a PR. Leave g16/front-door integration
   to the orchestrator because PR #94 owns overlapping closeout files.

After proving the denominator, report one meaningful checkpoint through the
operator: exact counts, indexing method, and any contradiction that could
change the audit. Report again only when the register/candidate map is ready or
a stop condition fires.

## Next Step

Run the worker preflight below, then execute `g16.020` in the order above. Do
not wait for PR #94 unless its state changes the denominator or audit method.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before
   broad repository reads, run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record
   its actual root/branch and do not create another worktree merely because it
   differs from the named placeholders.
3. If the launcher supplied a dirty or `main` worktree, stop and report it.
   Only when the current context is otherwise unusable may you inspect the
   named worktree, then read `.agents.local.env` and require
   `AGENTS_WORKTREE_CONTAINER_DIR` for a unique manual fallback from
   `origin/main`. Ask the operator when absent. Never use `/tmp`, `TMPDIR`, or
   a guessed path; never clean, reset, stash-over, or discard another checkout.
4. From the selected worktree, run `git fetch origin`; confirm `HEAD` equals
   `origin/main`; confirm
   `git merge-base --is-ancestor 69118d83173d3d69b284b5ecf6d7315dc43ae5a8 HEAD`;
   and confirm this handoff and ready card exist in `HEAD`.
5. Read `AGENTS.md`, the repo-local Northstar and Effigy skills, g16 README,
   g16.020, parity evidence ledger, architecture 001/003/011, specs
   008/025/069, current g15/g16 roadmaps and August logs, and open triage
   notes. Inspect exact component contracts/source only as the card permits.
6. Run `effigy tasks` and `effigy doctor`. Record the accepted doctor scan
   baseline already present in `PAPERCUTS.md`; do not widen into cleanup.

### While you work

- Execute only `g16.020` and keep the work docs-only.
- Use deterministic roster/card/log indexing before classification prose.
- Keep observed facts, inference, and unknowns visibly separate.
- Do not change the ledger to make the audit easier.
- Do not update roadmap front doors; they overlap PR #94 and remain
  orchestrator-owned.
- Stop on a denominator change, authority contradiction, required operator
  decision, implementation repair, or pressure to create a new evidence
  authority.
- Report only meaningful checkpoints, not component-by-component progress.

### When the assigned runway is complete

1. Confirm the register accounts for exactly 175 public Svelte components and
   174 portable native components, with MeterSurface explicit and Jetstream
   program-deferred.
2. Confirm no more than eight candidate lanes and that every drag-dependent
   migration points to architecture 011/spec 069.
3. Run the card's final validation, entirely headlessly:
   - the worker's deterministic roster/accounting proof;
   - `effigy check:parity-evidence-ledger`;
   - `effigy docs:lint`;
   - `effigy docs:check`;
   - one final `effigy qa`; and
   - `git diff --check origin/main...HEAD`.
4. Mark only `g16.020` complete and add one August audit log. Do not update
   g16 README, roadmap front door, generation index, parity ledger, generated
   reports, architecture, or specs.
5. Push the selected worker branch and open a reviewable PR against current
   `main`. The planning base above predates this handoff commit; it is not a
   self-referential hash.
6. In the PR body, link the card, register, audit log, denominator proof,
   historical sources, candidate lanes, PR #94 treatment, drag-and-drop
   boundary, unknowns, validation, and non-claims.
7. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator independently checks roster accounting, historical
reconciliation, classification truth, programme boundaries, candidate-lane
quality, PR #94 treatment, file scope, and validation. Because worker and
orchestrator share one GitHub identity, the orchestrator posts the canonical
verdict as a PR comment rather than formal self-approval. If changes are
requested, make only those docs-only changes on this branch, push, and report
through the operator. The operator must explicitly authorize any merge.

- **Requested changes:** none yet
- **Closeout refs:** `g16.020`, component continuation register, one August
  audit log; front-door reconciliation remains orchestrator-owned

### Handoff closeout

Before calling the audit complete, leave the card, register, log, and PR state
honest. If the work is blocked, record the exact blocker and stop rather than
inventing a classification or widening into implementation.

