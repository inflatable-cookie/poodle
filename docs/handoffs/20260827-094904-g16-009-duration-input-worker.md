---
title: g16.009 DurationInput single-source worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle core
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260827-094904-g16-009-duration-input-worker.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, gpui, duration-input]
---

## What This Thread Is Doing

Execute ready card `g16.009`. Replace DurationInput's duplicated Rust value
model with the contract's three segment fields, derive formatting and bounds
validation from that one value, migrate every in-repo Rust caller, and prove
the result through mounted GPUI editing and focus dispatch.

This is an operator-approved pre-1.0 breaking migration. Start from this file;
no copied transcript or second prompt is required.

## Why It Matters

The current mounted fixture can only work by writing both a formatted string
and three numeric segments. Real hosts should never have to synchronize two
representations of one control. Removing that split makes the shared Rust
component contract dependable and turns the next evidence cell into proof of
the public DurationInput behavior rather than proof of a tailored fixture.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `b30f9fc59debae374129156608901f92184055b7`
- **Pushed main verification:** local `HEAD` and `origin/main` both equalled the
  planning base before this handoff was created
- **Planning checkout:** clean at the recorded base before the planning
  artifacts in this handoff were authored
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included after the base:** ready `g16.009` card,
  promoted lane decision, and updated g16/front-door status
- **Worker branch:** `t3code/g16-009-duration-input-single-source`
- **Worker worktree:**
  `/Users/tom/.t3/worktrees/poodle/g16-009-duration-input-single-source`
- **Worktree creation command:** `git worktree add -b t3code/g16-009-duration-input-single-source /Users/tom/.t3/worktrees/poodle/g16-009-duration-input-single-source origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path/branch and
  never create a second worktree for that reason. If the current context is
  unusable, inspect the named worktree, then read `.agents.local.env`, require
  `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the operator if it is absent. Never
  use `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** DurationInput's existing component contract; no new
  architecture or master spec
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready card:**
  `docs/roadmaps/g16/009-duration-input-single-source-and-mounted-behaviour.md`
- **Allowed runway:** execute `g16.009` only
- **Remaining card budget:** one card
- **Dispatch topology:** serial; spec migration, shared transition total type,
  renderer behavior, mounted fixture, and ledger row overlap directly
- **Parallel safety check:** no parallel lane is authorized against the shared
  Rust specs, renderer, mounted regression file, or ledger
- **Current ledger:** 37 mounted / 137 missing; only DurationInput may move to
  38 mounted / 136 missing
- **Canonical refs:** `AGENTS.md`,
  `docs/architecture/001-poodle-system-shape.md`,
  `docs/architecture/product-guardrails.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/components/duration-input.md`, and
  `docs/triage/20260827-094214-post-g16-008-native-lane-decision.md`
- **Model capability profile:** capable coding model, high reasoning; this is a
  bounded public Rust-spec migration with real mounted focus and editing proof
- **Tool/runtime restrictions:** use the repo-local Effigy skill and selectors;
  everything stays headless; never run `*-windowed`, native visual, Jetstream
  preview/QA, release, tag, or publication tasks
- **Required validation:** focused specs/headless/render and paired web tests;
  mounted DurationInput plus retained routing regressions;
  `effigy regressions:native`; `effigy probe:gpui-specimens`; ledger test/check;
  `effigy ci:native`; `effigy ci:web`; `effigy docs:check`; one final
  `effigy qa`; and `git diff --check origin/main...HEAD`
- **Known orientation finding:** `effigy doctor` is already red on the planning
  base from generated-in-src, oversized-file, and stale/broad suppression scans
  recorded in `PAPERCUTS.md`; report that baseline without absorbing cleanup
- **PR base/head:** `main` <- worker branch
- **PR URL:** pending
- **Review state:** awaiting worker implementation and orchestrator review
- **Merge authorization:** worker must not merge; explicit operator authority
  is required after orchestrator review

## Boundaries

- Remove `DurationInputSpec.value`, `with_value`, `validation_state`, and
  `with_validation_state`. Do not retain compatibility aliases, deprecated
  paths, parsing fallbacks, or dual-source synchronization.
- Use `hours`, `minutes`, and `seconds` as the sole controlled value. Format
  display text and derive total/bounds validation from them.
- Default `show_seconds` to `true`.
- Align shared total calculation and the native callback payload to `u64`;
  retain `u32` segment fields and existing carry/digit transition semantics.
- Treat min/max bounds as inclusive validation, not edit clamps.
- Update every in-repo Rust caller. Jetstream files may change only to compile
  against the shared spec; do not claim or test deferred backend behavior.
- Mounted tests must use real GPUI focus and key dispatch plus host rebuilds.
  Direct handler invocation does not prove the card.
- Move only DurationInput's ledger cell. Do not promote accessibility or visual
  evidence.
- Keep web behavior, specimens outside DurationInput, NumberInput, TimeInput,
  EditableLabel, IconButton, date pickers, Jetstream admission, release, and
  downstream consumers out of scope.
- Work only in the selected clean worker worktree. Never edit the orchestrator
  planning checkout or clean/reset an unrelated checkout.
- Do not merge the PR.

## Important Context

- The contract already decided the public model: three bindable segments,
  `showSeconds=true`, carry/borrow/digit rules, and derived min/max invalid
  state. This worker is implementing existing authority, not designing it.
- `poodle_headless::duration` is the shared Rust port of the same core machine
  used by Svelte and React. Extend its total type; do not duplicate transitions
  in the renderer or mounted fixture.
- `g16.008` made each visible segment a real tab stop and removed the inert
  root stop. Retain `H → M → S → out` and the surrounding text-routing proof.
- `show_seconds=false` hides only the field. Seconds remain part of the host
  value and total payload.
- The GPUI and Jetstream preview fixtures contain most stale `with_value`
  callers. Migrate them mechanically to segments while preserving specimen
  content and layout.
- Report after the spec/headless/renderer migration, then after caller/mounted
  proof and closeout. Report immediately on a stop condition.
- Report to the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Run the worktree preflight below. Then read the ready card, DurationInput
contract, Rust spec, shared headless duration machine, shared renderer, GPUI
specimen, mounted driver fixture, paired web tests, and ledger generator.

Implement in two meaningful chunks. First establish the single-source spec,
`u64` total, renderer formatting/validation, and focused tests. Then migrate
callers, deepen the mounted regression, update the one ledger row, and close
the docs. Stop rather than widening if a caller depends semantically on the
removed stale fields.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before any
   broad repository read, run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root and branch. Do not create another worktree because generated
   names differ.
3. Only if the current context is `main`, dirty, unregistered, or unusable,
   inspect the named worktree. If it cannot be used, read `.agents.local.env`,
   require the absolute `AGENTS_WORKTREE_CONTAINER_DIR`, and create a unique
   worktree/branch there from `origin/main`. Ask the operator if the key is
   absent. Never use `/tmp`, `TMPDIR`, a repository child, or a guessed path.
   Never clean, reset, stash, or discard the original checkout.
4. From the selected worktree, run `git fetch origin`; confirm `HEAD` equals
   `origin/main`; confirm
   `git merge-base --is-ancestor b30f9fc59debae374129156608901f92184055b7 HEAD`;
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, the g16 README, assigned card, DurationInput contract,
   ledger, source triage note, and canonical architecture/working-rule refs.
6. Read `.agents/skills/effigy/SKILL.md`, then use `effigy tasks` and
   `effigy doctor` for orientation. Record the known doctor baseline without
   widening into unrelated cleanup.

### While you work

- Execute only `g16.009`. Keep commits aligned with the two meaningful chunks,
  not model turns.
- Verify the Svelte and React references before changing Rust behavior. Stop if
  they disagree on a rule named by the card.
- Use production shared transitions and backend dispatch in mounted evidence.
- Remove stale fields cleanly and update all callers in the same branch.
- Keep the ledger move exact and stop on any card stop condition.

### When the assigned runway is complete

1. Run the full validation named in `Current State`, entirely headlessly.
2. Mark the card complete, mark the source triage resolved, add one August
   execution log, and leave g16's next task as an orchestrator checkpoint. Do
   not compile or implement another card.
3. Confirm the generated ledger changes only DurationInput and totals read
   38 mounted / 136 missing. Run `git diff --check origin/main...HEAD` and
   confirm the worktree is clean after committing.
4. Push the worker branch and open a reviewable PR against current `main`.
5. The PR body must link the card and contract; name the removed fields and all
   migrated caller families; report total/bounds rules, mounted key/focus
   evidence, ledger delta, validation, and explicit non-claims.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect PR metadata, commits, changed files, clean API
removal, one-source formatting/validation, total type, caller migration,
mounted production dispatch, exact ledger delta, and checks independently.
Because worker and orchestrator share the GitHub identity, the orchestrator
will post the canonical verdict as a PR comment rather than formal
self-approval. The operator must explicitly authorize merge after a green
review.

### Handoff closeout

Leave the card, execution log, source triage, runway/front doors, ledger, and PR
body mutually consistent. Record exact failures rather than claiming visual,
accessibility, Jetstream, or broader date/time parity. End at the orchestrator
checkpoint.
