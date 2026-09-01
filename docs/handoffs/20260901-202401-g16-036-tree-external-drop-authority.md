---
title: g16.036 Tree external drop authority worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260901-202401-g16-036-tree-external-drop-authority.md
base_required: pushed-main
tags: [coordination, handoff, worker, tree, drag-drop]
---

## Outcome

Implement ready card `g16.036`: one generic paired Svelte/React Tree
`reorderAuthority` adapter over the existing drag substrate. The host projects
an ordered moving set, synchronously accepts/rewrites/refuses the resolved Tree
candidate before accepted presentation, and returns the real sync/async commit
result. Preserve the ordinary `onReorder` path.

This is Poodle work. Do not read or edit Figmatic implementation source. Its
two planning files are evidence only and the orchestrator will send the merged
artifact back after review.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Planning commit: `169651f51b3443ffad08f4dd435198384deacebd`
- Planning commit is pushed to `origin/main` and includes architecture 011,
  spec 069, the amended Tree contract, and ready card g16.036.
- PR #125 is merged at
  `a980cb7748fdf9751dd4ca64b02903111a44d59f`; it is the fixed Tree
  interaction, geometry, indicator, focus, auto-scroll, and revalidation
  baseline.
- Worker workspace:
  `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-036-tree-external-drop-authority`
- Worker branch: `feature/g16-036-tree-external-drop-authority`
- Workspace id: `wks_a90aeb42c7a21e74`
- Active card:
  `docs/roadmaps/g16/036-tree-external-drop-authority.md`
- Roadmap: `docs/roadmaps/g16/README.md`
- Worker class: day-to-day. This is a bounded paired-web implementation over
  settled public types and a landed controller. No frontier-worker
  justification applies; frontier review remains with the orchestrator because
  the API controls external mutation authority.
- Selected profile: Cursor Grok Worker (`cursor/grok-4.6`, agent mode,
  `auto_accept=true`). Its profile is the configured day-to-day fallback while
  the main Grok account is exhausted.

## Public Shape Is Fixed

Shared core owns and both component packages re-export:

- `TreeReorderSubject { sourceValue, movingValues }`;
- `TreeReorderCandidate { subject, intent }`; and
- `TreeReorderAuthority { projectMovingValues, canDrop, onDrop }`.

Tree adds `reorderAuthority`. Its public prop union makes it mutually exclusive
with `onReorder`; `reorderable` remains the explicit enable switch. Reuse
`DropIntent`, `DropEligibility`, and `DragDropCommitResult`. Do not add parallel
destination/decision/result types.

The card and contract fix the remaining laws: one latched valid moving set;
synchronous pre-paint policy; only `intent.destination` may be rewritten;
accepted destination drives depth, announcement, revalidation, and commit;
live authority at release; exact sync/async terminal; full cleanup; unchanged
convenience path.

## Boundaries

- Implement Svelte and React as one paired semantic surface.
- Keep the generic internal `DragSubject` as the source row id. The richer Tree
  subject is component-owned session context, not an encoded payload.
- Use the existing Tree source, targets, outline resolver, pointer/keyboard
  routes, controller, focus, auto-scroll, and terminal machine.
- Do not add a second controller, dwell controller, timer, coordinate callback,
  DOM ancestry API, application record, revision, or consumer target enum.
- Do not change PR #125's geometry policy.
- Do not edit Rust, GPUI, or Jetstream source. The documented native boundary
  is real: local Node commit is synchronous, intent presentation lacks the full
  rewritten intent, and the session subject has no durable multi-row payload.
  Stop rather than widening the Node substrate or encoding moving values into
  an id.
- Do not edit Figmatic, versions, releases, tags, workflows, or sibling repos.
- The temporary `reorderAuthority` contract/spec drift entries are planning
  scaffolding. Delete them as the implementation makes each allowance stale.
- Work only in the named worker worktree. Do not merge.

## Parallel And Serial Edges

- PR #124 (`g16.034`) is a separate in-flight motion repair. It may overlap
  only g16 front doors, which this worker may not edit.
- Production Tree/core source is independently writable from #124's motion
  surfaces. Continue without waiting.
- Same-repository merge order is serial. If #124 merges first, rebase this PR
  onto current `main`, rerun proportional checks, and report the new exact
  head. If this PR is ready first, the orchestrator still owns which PR merges.
- Do not create or coordinate a Figmatic consumer worker. Figmatic `016-22`
  starts only after the Poodle artifact is merged and reported.

## Ordered Work And Evidence

Execute the card in order. In particular:

1. Add the shared-core types, validity helper, and public exports.
2. Latch the projected subject at actual semantic activation for pointer and
   one-shot keyboard routes. Clear it on every terminal and teardown.
3. Route Svelte and React pointer and logical keyboard eligibility through the
   same current authority, then through live structural validation after any
   rewrite.
4. Pass the revalidated accepted candidate to `onDrop` and return its Promise
   or result unchanged. Never call `onReorder` in authority mode.
5. Derive accepted indicator depth from the controller's accepted intent and
   `dropCommitDestination`, not a private second pointer resolve.
6. Prove the eight oracle rows in focused and mounted evidence. Commit real
   proofs before falsification plants; restore from the commit, not an
   unstaged index.
7. Reconcile the card and one September log. Push one reviewable PR.

Required proof includes two-shell subject latching and reset, refusal before
accepted paint, rewritten depth/announcement/commit agreement, live release
revalidation, async terminal/stale answer behavior, pointer and Alt+↑/↓,
invalid projection refusal, and unchanged `onReorder` convenience.

## Validation

Use the repo-local Effigy skill and discover exact selectors. At minimum run:

- focused core Tree geometry/eligibility tests;
- focused Svelte and React Tree tests;
- the mounted drag-drop browser cases in Chromium and WebKit;
- installed-package type checks for the three shared types and mutually
  exclusive prop union;
- relevant contract/API/drag inventory drift checks;
- `effigy ci:web`;
- `effigy docs:check`;
- `git diff --check origin/main...HEAD` and exact writable-scope checks.

Do not run `*-windowed`, native visual, release, tag, publication, workflow
mutation, or sibling-repository commands. No Rust source change means broad
Rust/native boards are not required.

## Completion Protocol

1. Before broad reads, verify the current root, branch, clean status, and
   registered worktree. Fetch `origin` and fast-forward the clean worker branch
   to current `origin/main` if the launcher has not already done so.
2. Confirm planning commit `169651f51b3443ffad08f4dd435198384deacebd`
   is an ancestor and load this handoff from the committed worktree HEAD.
3. Read `AGENTS.md`, the card, Tree contract, architecture 011, spec 069, and
   the Effigy skill. Do not reinterpret the fixed public shape.
4. Implement coherent batches and report material findings, not micro-updates.
5. Stop on a missing decision, generic controller lifecycle change, native
   substrate expansion, shared mutable source, or evidence that cannot bite.
6. Commit, push, and open a PR against current `main`. Include exact head,
   public exports, oracle falsifications, validation, and the explicit native
   delta. Do not merge.

## Review And Merge

The Poodle orchestrator reviews the exact PR head against g16.036 and the
contract, sends repair rounds to this same worker, rebases after any earlier
same-repo merge, and merges only an accepted current head. After merge, the
orchestrator sends Figmatic the exact merge SHA, public API/import surface, and
validation receipt. The worker does not dispatch Figmatic `016-22`.

## Next Move

Run the worktree preflight, then start with the shared-core types and a failing
paired type/subject-latch proof before changing Tree components.
