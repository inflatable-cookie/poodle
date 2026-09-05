---
title: g15.034 component-specific specimen axis domains worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-18
updated: 2026-08-18
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260818-215524-g15-034-component-axis-domains.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, breaking, specimens, parity]
---

## What This Thread Is Doing

Execute `g15.034`: remove the duplicate/dead EmptyState and Icon APIs, restore
their exact cross-runtime value domains, and harden the three specimen shells
so every advertised axis value is real and proved.

This is one clean pre-v1.0 breaking migration approved by the operator. It is
not a new conformance architecture and it must not retain compatibility twins.
Start from this file without a copied transcript or second prompt.

## Why It Matters

`g15.019` found that the catalogue could advertise an axis even when the
component could not render its values: EmptyState ignored its real native size
field, while native Icon collapsed five web sizes into three and carried a
dead density field. The same shell assumption can fabricate or omit evidence
for any component-specific domain.

Poodle v0.2.0 needs truthful component interfaces and specimens before the
native probe and visual-conformance lanes can be trusted. Fix the component
contracts and the admission hole once, without reviving the rejected g13/g14
parity mechanisms.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `636ee4da3416e10aff3549e7c66c99ed86fb4ee8`
- **Pushed main verification:** local `HEAD` and `origin/main` matched that
  commit before this handoff was created
- **Planning checkout:** clean `main`; implementation edits are forbidden there
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts at the base:** `g15.034`, exact-axis working rule,
  specimen-plan clarification, gap-register/audit/runway currentness
- **Worker branch placeholder:** `t3code/g15-034-component-axis-domains`
- **Worker worktree:** launcher-managed. If a manual fallback is required, the
  intended path is
  `${AGENTS_WORKTREE_CONTAINER_DIR}/poodle-g15-034-component-axis-domains`
- **Manual fallback command:** after validating the ignored local-path file,
  `git worktree add -b t3code/g15-034-component-axis-domains
  "$AGENTS_WORKTREE_CONTAINER_DIR/poodle-g15-034-component-axis-domains"
  origin/main`
- **Ready card:** `docs/roadmaps/g15/034-component-specific-specimen-axis-domains.md`
- **Allowed runway:** `g15.034` only
- **Remaining card budget:** one card
- **Dispatch topology:** serial. It touches shared Svelte, React, GPUI, codegen,
  and parity specimen substrates and blocks the next catalogue worker
- **Canonical refs:** `docs/contracts/001-working-rules.md`,
  `docs/contracts/components/empty-state.md`,
  `docs/contracts/components/icon.md`,
  `docs/roadmaps/g15/specimen-plan-outline.md`,
  `docs/specs/022-packaging-versioning-and-release-channel-rules.md`, and
  `docs/specs/044-deprecation-change-control-and-release-channel-operations.md`
- **Model capability profile:** frontier coding model, high reasoning; this is
  a public API and renderer-parity migration
- **Known doctor baseline:** generated-in-src, god-file, stale-suppression,
  stale-graph, and comment-ratio findings. Record them; do not absorb them
- **Required validation:** the exact headless board in the roadmap
- **PR base/head:** `main` <- selected worker branch
- **PR URL:** pending
- **Review state:** awaiting implementation
- **Merge authorisation:** none. Push one PR and stop

## Boundaries

- **In scope:** the exact `g15.034` Delivery and Writable Scope: EmptyState's
  single two-value size, Icon's five values and density removal, explicit
  ordered domains in all three specimen shells/adapters, generated-scene
  pass-through, four exact-domain proofs, release/migration evidence, and one
  batch log.
- **Out of scope:** curation of other specimen pages, the live GPUI page probe,
  screenshots or visual conformance, Jetstream parity, new IR/schema behaviour,
  release execution, and unrelated API cleanup.
- Update `empty-state.md` and `icon.md` first in the worker batch, then make the
  implementations follow. The roadmap records the approved target; the current
  component contracts still describe the pre-migration package surface so main
  remains internally valid until the breaking PR lands.
- Mechanical deferred-Jetstream source changes are allowed only where the Rust
  API removal must compile. Run no Jetstream selector and make no parity claim.
- Do not add aliases, deprecated twins, wrapper builders, or silent endpoint
  fallbacks. If a real downstream use appears, stop and report it.
- Do not edit another lane's pages, the generation runway, dispatch ledger, this
  handoff, `.github/workflows/`, or release automation.
- Work only in the selected clean worker worktree. Never clean, reset, or
  discard a dirty checkout and never edit the orchestrator's `main` checkout.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Planning lineage:** g13's generated component representation and g14's
  executable conformance corpus were rejected and removed. g15 uses focused
  component evidence and human-centred specimens. Explicit ordered axis values
  are a narrow truthfulness repair to the current shells, not a third shared
  renderer.
- **Why the card is ready:** the operator approved the clean break; the exact
  API targets, migration sites, special domains, evidence hole, package class,
  validation, and stop conditions are recorded.
- **Decisions:** EmptyState keeps `size: default | compact` and loses the Rust
  `compact` twin. Icon gains five native sizes and loses density everywhere.
  Text/Eyebrow stop at `md`. Authored scene axis values are authoritative.
- **Open tension:** `SpecimenLayout` may keep a standard-domain convenience for
  ordinary hand-written pages, but an explicit provided domain must be
  authoritative. GPUI may keep an equivalent standard `ControlSize`
  convenience; filtering is not how a smaller public domain is represented.
- **Downstream evidence:** orchestration found no direct removed-API use outside
  Poodle under `~/Dev/projects`. Re-run a properly excluded targeted search at
  the worker base and record the command/result. A hit is a stop condition.
- **Report after:** (1) contract/API migration plus focused component/Rust
  evidence, then (2) specimen/codegen/census migration plus the final board.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Read this handoff first. Before broad repository reads, run the quick worktree
preflight below. If the launcher already placed you in a clean, registered,
dedicated non-`main` worktree, use it regardless of its generated path or
branch name and do not create another one.

Then read `AGENTS.md`, the Northstar worker rules, the repo-local Effigy skill,
the ready card, and its canonical refs. Re-prove the measured call sites with
targeted `rg`. Start with the observable contracts and the component API
migration; do not begin from specimen styling.

## Completion Protocol

### Before you start

1. This file's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before any
   broad reads run only:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and branch is
   not `main`, accept it as the launcher-provided worktree. Record the actual
   root/branch. Do not compare them with the placeholders or create another
   worktree merely because they differ.
3. If the launcher supplied a dirty or `main` worktree, stop and report it. Do
   not silently create a second worktree behind the launcher. Only when the
   current context is otherwise not launcher-owned may you inspect the named
   fallback, then read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and create a unique worktree beneath it.
   Ask the operator if the key is absent. Never use `/tmp`, `TMPDIR`, or a
   guessed repository-adjacent path.
4. From the selected worktree fetch `origin`, confirm `HEAD == origin/main`,
   confirm `git merge-base --is-ancestor
   636ee4da3416e10aff3549e7c66c99ed86fb4ee8 HEAD`, and confirm this handoff
   exists in `HEAD`.
5. Read the ready card and canonical refs named above. Use `effigy tasks` for
   selector orientation. Run `effigy doctor` only when routing or repository
   health is ambiguous; its known findings are not card scope.
6. Check for an overlapping worker on the shared specimen layout/scene/codegen
   files. Stop if one exists.

### While you work

- Execute only `g15.034`. Keep commits aligned with the two meaningful chunks,
  not arbitrary model turns.
- Use `apply_patch` for edits and repository generators for generated output.
- Report each chunk with changed files, exact validation run, remaining work,
  risks, and blockers.
- Stop on a new non-standard domain, a real downstream removed-API use, a need
  for compatibility, scene behaviour/schema growth, or validation that changes
  the plan.
- Record small solvable friction in `PAPERCUTS.md`; do not absorb it.

### When the assigned runway is complete

1. Run the card's exact validation: focused tests; `effigy ir:check`;
   `effigy test:components`; `effigy check:svelte`; `effigy react:build`;
   `effigy test:parity`; `effigy check:gpui`;
   `effigy regressions:native`; `effigy test:web-pack-install`;
   `effigy docs:check`; and `git diff --check origin/main...HEAD`.
2. Never run `*-windowed`, `test:native-visual`, `qa:jetstream`, a Jetstream
   selector, or a release mutation.
3. Write one August batch log with the package change class, public entry-point
   effects, migration notes, downstream re-check, commands, counts, outcomes,
   and unresolved findings. Do not change roadmap/dispatch status.
4. Start the paired Svelte and React previews on strict, non-conflicting ports
   and give the operator the four exact routes for EmptyState, Icon, Text, and
   Eyebrow. The operator must accept their axis rows before merge.
5. Commit meaningful batches, push the selected branch, and open one reviewable
   PR against current `main`. Link the card, contracts, changed surfaces,
   validation, batch log, breaking migration, and any unresolved item.
6. Return the PR URL, head SHA, exact evidence, operator-review routes, and any
   deviation. Do not merge.

### Review and merge path

The orchestrator will independently inspect the PR metadata, commits, diff,
checks, package/API migration, downstream evidence, and preview checkpoint.
Because worker and orchestrator may share a GitHub identity, the canonical
review verdict may be a PR comment rather than formal self-approval.

If changes are requested, change only this branch and report back through the
operator. Merge requires explicit operator authorisation after the gate passes.

- **Closeout refs:** `g15.034`, one August batch log, `g15`/root roadmap
  currentness, generation index, release gap register, specimen audit, and the
  next-task decision for `g15.020`.

### Handoff closeout

Leave the worker branch and PR honest. The orchestrator—not this worker—will
mark the card complete, update the dispatch ledger/front doors, and select the
next lane after merge. If blocked, record the blocker and stop.
