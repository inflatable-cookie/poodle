---
title: g16.055 drag-source browser suppression worker handoff
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
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260902-002950-g16-055-drag-source-selection-suppression.md
base_required: pushed-main
tags: [Papercuts, drag-drop, tree, web, selection]
---

## Objective

Implement
`docs/roadmaps/g16/055-drag-source-preactivation-selection-suppression.md`.
Fix the browser text-selection leak and trailing compatibility click at the
shared DOM drag-controller boundary. Prove both through paired Tree exposure
and real Chromium/WebKit behavior, and return one reviewable PR. Do not merge.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Planning branch: `main`
- Planning base before this card/handoff commit:
  `74f612d6b5dadb3b91bb62961d5396c64d7c1a95`
- Worker branch: `fix/g16-055-drag-source-selection-suppression`
- Ready card:
  `docs/roadmaps/g16/055-drag-source-preactivation-selection-suppression.md`
- Allowed runway: g16.055 only
- Required sibling links: none
- Public API decision: none; no new prop or compatibility surface
- Release posture: merge to source first; ship in the separately gated compiled
  `0.3.0` candidate, never a new `0.2.x` lane

## Settled Boundary — Do Not Re-Ask

- This is one shared web-substrate fix, not a Tree CSS workaround.
- Start suppression only after a primary pointerdown resolves to an accepted,
  enabled registered source inside the connected root.
- Existing `sourceFromEvent` interactive/no-drag exclusions stay authoritative.
- Suppress during the pre-threshold candidate and active drag; restore the exact
  prior inline root declaration on every exit.
- Consume an activated pointer gesture's compatibility click at the shared
  capture boundary before Tree row selection sees it, regardless of eventual
  commit/reject/fail/cancel outcome. Expire the one-shot guard without eating a
  later unrelated click.
- Click row selection, rename/input selection, and non-reorderable text
  selection must remain. A tap or pre-threshold abandonment still clicks and
  selects normally; keyboard reorder is unchanged.
- Svelte and React consume the shared controller behavior. Rust/GPUI does not
  emulate a browser Selection lifecycle.

## Worker Shape

- Workspace label: `Papercuts` (capitalized), applied before launch.
- Worker class: day-to-day. This is a bounded lifecycle repair with an explicit
  shared boundary and oracle; no frontier-worker justification applies.
- Ready-frontier shape: independent of the post-triage planning PR and its docs
  front doors.
- Named serial edge: same-repository merge/rebase ordering only. The
  orchestrator reviews and merges this PR before handing Figmatic an adoption
  SHA.

## Write Scope

Follow the card exactly. Own the shared controller, its focused tests, paired
Tree exposure tests, the smallest existing browser probe extension, spec/Tree
contract text only when needed, the card evidence, and one execution log.

Do not edit g16 README/generation index, post-triage cards or runway, Tree CSS,
Tree geometry or external authority, Rust/GPUI, versions, changelog/release
notes, package build/distribution, workflows, Figmatic, or any sibling repo.
`PAPERCUTS.md` is allowed only for newly found execution friction.

## Required Evidence

- Focused controller proofs for accepted pointerdown suppression, exact authored
  style restoration, pre-threshold release/cancel, source loss,
  disconnect/destroy, interactive exclusions, activated compatibility-click
  consumption, stale-guard expiry, and repeated sessions.
- Paired Svelte and React Tree proof that click selection and rename/input
  behavior remain, while an activated drag causes one reorder request and no
  trailing selection request for committed, rejected, failed, and cancelled
  outcomes.
- Mounted Chromium and WebKit Tree proof that a pointer drag through multiple
  labels creates no Selection range and its compatibility click never reaches
  the row, plus non-reorderable/tap counterexamples proving the probe observes
  normal selection and clicks.
- Falsification by moving selection suppression back to activation and removing
  the compatibility-click guard. Commit proof first; restore from a clean
  commit.
- Relevant drag inventory/contract drift checks, `effigy ci:web`,
  `effigy docs:check`, and `git diff --check origin/main...HEAD`.

Never run `*-windowed`, native visual, release, tag, publication, workflow
mutation, or sibling-repository commands.

## Completion Protocol

1. Before broad reads, confirm the launcher supplied a clean non-main worktree,
   fetch origin, and prove the committed handoff is present in `HEAD`.
2. Read AGENTS.md, the card, spec 069, the Tree contract, the controller source,
   and existing controller/Tree/browser drag tests.
3. Use Effigy to discover the narrow selectors. Reproduce before repair.
4. Implement one coherent fix and its biting evidence. Stop on a public API,
   gesture-semantics, or cross-runtime architecture decision.
5. Run the required validation, update card/log evidence, commit, push, and open
   a PR against current `main`.
6. Report PR URL, exact head, falsification result, validation, and precise
   release/adoption guidance. Do not merge.

The Poodle orchestrator owns exact-head review, requested-change loops, merge,
front-door closeout, and the final Figmatic receipt.
