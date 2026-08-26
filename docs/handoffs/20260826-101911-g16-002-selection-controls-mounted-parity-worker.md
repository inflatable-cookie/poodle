---
title: g16.002 selection controls mounted parity worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle core
created: 2026-08-26
updated: 2026-08-26
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260826-101911-g16-002-selection-controls-mounted-parity-worker.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, parity, gpui]
---

## What This Thread Was Doing

Poodle has replaced its stale parity claims with a checked component-level
evidence ledger. The first measured follow-up is a five-component headless
behaviour batch: `Checkbox`, `Switch`, `RadioGroup`, `SegmentedControl`, and
`ToggleGroup` all construct in GPUI but lack named mounted behaviour proof.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. Start from this file; no copied transcript or second
prompt is required.

## Why It Matters

Poodle needs dependable contract-backed parity, not another broad conformance
experiment. This batch exercises binary, exclusive, multiple, disabled,
readonly, callback, and roving-focus semantics through the real mounted GPUI
tree. It is deliberately headless and reuses the shipped specs, renderer, node
backend, and test driver. A clean result gives the operator evidence strong
enough to decide whether the same family should proceed to visual comparison.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `2bded7a3e2591aa0df795d36c18a8eb28623b0cc`
- **Pushed main verification:** local `HEAD` and `origin/main` both equalled the
  planning base before this handoff was created
- **Planning checkout:** clean at the recorded base
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:**
  `docs/roadmaps/g16/002-selection-controls-mounted-parity.md` and the updated
  g16/front-door runway
- **Worker branch:** `t3code/g16-002-selection-controls-mounted-parity`
- **Worker worktree:**
  `/Users/tom/.t3/worktrees/poodle/g16-002-selection-controls-mounted-parity`
- **Worktree creation command:** `git worktree add -b t3code/g16-002-selection-controls-mounted-parity /Users/tom/.t3/worktrees/poodle/g16-002-selection-controls-mounted-parity origin/main`
- **Worker worktree policy:** use a clean launcher-provided non-`main`
  registered worktree first, whatever its generated name. Only fall back to
  the named path and then `.agents.local.env` when the current context is
  unusable. Never create a second worktree merely because names differ.
- **Active spec lane:** existing component contracts; no new spec or contract
  is authorised
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g16/002-selection-controls-mounted-parity.md`
- **Allowed runway:** execute `g16.002` only
- **Remaining card budget:** one card
- **Dispatch topology:** serial; no parallel Poodle worker lane is authorised
- **Parallel safety check:** this card owns the shared GPUI headless regression
  file and parity ledger generator, so overlapping native/parity work would not
  be independent
- **Canonical refs:** `AGENTS.md`, `docs/architecture/001-poodle-system-shape.md`,
  `docs/architecture/product-guardrails.md`;
  `docs/contracts/001-working-rules.md` and the five contracts named by the card
- **Model capability profile:** capable coding model, medium reasoning; stop for
  frontier/orchestrator review on a contract, public API, node vocabulary, or
  runtime-architecture decision
- **Tool/runtime restrictions:** use the repo-local Effigy skill and selectors;
  everything stays headless; never run `*-windowed`, native visual, Jetstream,
  preview-window, release, tag, or publication tasks
- **Required validation:** focused changed-module tests,
  `effigy regressions:native`, `effigy probe:gpui-specimens`,
  `effigy test:parity-evidence-ledger`,
  `effigy check:parity-evidence-ledger`, `effigy ci:native`,
  `effigy ci:web`, `effigy docs:check`, one final `effigy qa`, and
  `git diff --check origin/main...HEAD`
- **PR base/head:** `main` <- worker branch
- **PR URL:** pending
- **Review state:** awaiting worker implementation and orchestrator review
- **Merge authorisation:** worker must not merge; explicit operator authority is
  required after orchestrator review

## Boundaries

Keep the run inside the exact five-component mounted-behaviour card.

- **In scope:** real mounted GPUI input/focus/callback/host-rebuild proofs for
  `Checkbox`, `Switch`, `RadioGroup`, `SegmentedControl`, and `ToggleGroup`;
  contract-backed shared-Rust or GPUI repairs measured by those proofs; exact
  ledger regeneration and one execution log.
- **Out of scope:** Svelte/React component changes, component contracts, public
  Rust specs/APIs, specimens, visual fixtures or thresholds, screenshots,
  accessibility promotion, generic schemas/corpora/adapters, Jetstream,
  workflows, releases, and downstream repositories.
- Do not turn the selected controls into a universal component case language.
  The mounted tests are owner-local evidence with explicit test names.
- Do not resolve the deferred Jetstream `SegmentedControl` re-selection note.
  The active-cohort contract says same-value selection is inert. Stop if an
  active runtime contradicts that rule.
- Do not infer native accessibility parity from node role/state assertions.
  The ledger's GPUI accessibility cells remain `manual`.
- Work only in the selected clean worker worktree. Never edit the orchestrator's
  planning checkout or clean/reset an unrelated checkout.
- Do not merge the PR.

## Important Context

- **Planning lineage:** g14 proved that a shared executable case authority was
  too costly and removed it. g15 shipped full structural coverage and one
  Button-only visual comparator. g16.001 now records the honest denominator:
  175 public components, 174 portable native components, only 29 with mounted
  GPUI evidence, and Button as the only three-runtime visual comparison.
- **Why this card is ready:** every selected component already has a current
  contract, web implementation/tests, shared Rust spec/render path, GPUI route,
  and a `missing` mounted cell. The required headless driver already powers 70
  native regressions.
- **Decisions and preferences:** semantics and behaviour come before pixels;
  Svelte remains the reference; tests must drive real mounted input and host
  rebuilds; small component-specific evidence is preferred to a generic
  cross-runtime authority.
- **Open tensions:** real mounted input may expose a contract contradiction or a
  missing public Rust capability. Those are stop conditions, not permission to
  alter the contract or API. GPUI 0.2.2 has no broad assistive-technology seam,
  so this batch cannot close native accessibility.
- **Report after:** the first coherent mounted family pass or the first stop
  condition, whichever comes first
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Read this handoff from the top and run the quick four-command worktree preflight
below before broad reads. Once the worktree is established, read `AGENTS.md`,
the g16 runway, the card, and the five contracts. Inspect the existing headless
driver and current node/handler shapes before choosing test structure.

Implement the smallest coherent batch. A useful order is Checkbox and Switch,
then RadioGroup and SegmentedControl, then ToggleGroup, because each step adds
one behaviour dimension while reusing the mounted host-rebuild pattern. Report
real defects instead of normalising them away in test construction.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before any
   broad repository read, run:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and the
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root and branch. Do not compare generated names with this handoff or
   create another worktree because they differ.
3. Only if the current context is `main`, dirty, unregistered, or unusable,
   inspect the named worktree. If that cannot be used, read
   `.agents.local.env`, require the absolute
   `AGENTS_WORKTREE_CONTAINER_DIR`, and create a unique worktree/branch there
   from `origin/main`. Ask the operator if the key is absent. Never use `/tmp`,
   `TMPDIR`, a repository child, or a guessed path. Never clean, reset, stash,
   or discard the original checkout. If the launcher itself supplied a dirty
   or `main` worktree, stop and report it instead of silently creating another.
4. From the selected worktree, run `git fetch origin`; confirm `HEAD` equals
   `origin/main`; confirm
   `git merge-base --is-ancestor 2bded7a3e2591aa0df795d36c18a8eb28623b0cc HEAD`;
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `docs/roadmaps/g16/README.md`, the assigned card, the five
   component contracts, and the canonical architecture/working-rule refs named
   above.
6. Read `.agents/skills/effigy/SKILL.md`, then use `effigy tasks` and
   `effigy doctor` for the required orientation. Record known baseline findings
   without absorbing unrelated cleanup.

### While you work

- Execute only `g16.002`. Keep commits aligned with meaningful behaviour
  chunks, not model turns.
- Drive the real mounted backend/input path. Direct handler calls, transition
  helper calls, spec inspection, and specimen construction do not count.
- After each meaningful chunk, report changed files, validation actually run,
  remaining acceptance, defects, and blockers through the operator.
- Stop on any card stop condition. Do not quietly invent a contract, public
  capability, node vocabulary, generic action language, or visual mechanism.

### When the assigned runway is complete

1. Run the full validation named in `Current State`, entirely headlessly.
2. Update the card status, regenerate the ledger through its source, add one
   August execution log, and leave g16's next task as an orchestrator review
   checkpoint. Do not create `g16.003`.
3. Run `git diff --check origin/main...HEAD` and confirm the worktree is clean
   after committing.
4. Push the worker branch and open a reviewable PR against current `main`.
5. The PR body must link the g16 milestone/card, name all mounted tests and any
   repairs, report the ledger before/after count, list validation, and preserve
   unresolved evidence gaps.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect PR metadata, commits, changed files, diff, tests,
ledger lineage, and checks independently. Because worker and orchestrator share
the GitHub identity, the orchestrator will post the canonical verdict as a PR
comment rather than formal self-approval. Requested changes are currently none.
The operator must explicitly authorise merge after a green review.

- **Closeout refs:** the assigned card, g16 README/front doors, generated parity
  ledger, one August log, and the single next-task checkpoint

### Handoff closeout

Before calling the runway complete, leave the card, log, ledger, roadmap, and
next-task state honest. If blocked, record the exact blocker and stop rather
than making the handoff appear complete.
