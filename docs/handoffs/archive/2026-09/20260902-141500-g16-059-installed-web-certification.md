---
title: g16.059 installed web distribution certification worker handoff
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
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260902-141500-g16-059-installed-web-certification.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g16, web-distribution, certification]
---

## Assignment

Implement only `g16.059`: turn `test:web-pack-install` into the sole permanent
installed certification for the compiled core, Svelte, and private React
packages. Work from merged PR #162. Produce the exact-main promotion receipt;
do not release anything.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Required base: pushed `main` containing PR #162 merge
  `b43481dff4e25b70fc0b0b076cee116f5e97d93b`
- Assigned card:
  `docs/roadmaps/g16/059-installed-web-distribution-certification.md`
- Governing refs: architecture 014, spec 070, cards g16.056–g16.058, and the
  current package-install harness
- Worker class: mechanical implementation. The contract and oracles are frozen;
  the work is long evidence/harness integration, not frontier reasoning.
- Integration ownership: orchestrator owns exact-head review, merge, closeout,
  and any later dispatch of `g16.054`

## Boundary

In scope:

- complete every ordered-work and acceptance row in `g16.059`;
- certify clean temporary-checkout builds and packs from one exact commit;
- inspect archive members, export targets, declarations, conditions, CSS,
  parser edges, notices, receipts, and canonical 176-name roster identity;
- make clean-checkout selector ordering explicit: build contracted `dist/`
  outputs before export-target/docs audits rather than relying on stale local
  artifacts;
- repair the merged clean-main declaration bootstrap so Bun cannot substitute
  root TypeScript 7 for the pinned nested TypeScript 6.0.3 toolchain. This is an
  explicitly reviewed minimal build repair; preserve the exact 6.0.3 gate;
- install archive `file:` references into fresh no-workspace consumers;
- prove browser, Node SSR, worker-like default resolution, direct-client SSR
  rejection, Svelte 5.56.8 success, and a visible below-floor failure;
- retain all existing HistoryEntry, Slider, Tree, shell, and React negative
  proofs while removing source-oriented assumptions;
- compare two clean builds and packs byte-for-byte and write the permanent
  deterministic certification receipt;
- update only the assigned card, one execution log, package-install docs, and
  new papercuts.

Out of scope:

- versions, changelog, release notes/history, tags, npm publication, registry
  mutation, workflow dispatch/editing, sibling repositories, public React
  admission, component behavior, native, Jetstream, or `g16.054`;
- compatibility shims, aliases, raw-source fallbacks, suppressed diagnostics,
  workspace links, source paths, or hand-edited archives.

The two named clean-main bootstrap/order repairs above are explicitly reviewed.
Any other package/build repair requires orchestrator review. Stop rather than
redesign g16.056–g16.058.

## Required Proof

- Run the upgraded `test:web-pack-install` from a clean temporary checkout of
  one exact commit; no workspace or source resolution may survive.
- Prove installed browser and SSR paths for root/direct Button/Select and all
  five `./markdown` components, plus direct-client SSR rejection.
- Prove Svelte 5.56.8 success and a named below-floor expected failure.
- Compile declarations under Bundler and NodeNext with unsuppressed negative
  fixtures; preserve the existing HistoryEntry and Tree/Slider proofs.
- Build and pack twice; compare output inventories, file hashes, archive hashes,
  dotfile receipt membership, notices, provenance, artifact-set ID, roster, and
  exact source commit.
- Falsify each card oracle from a committed clean proof point, restore from that
  commit, and record the failing proof.
- Run relevant web build/declaration selectors, roster drift, CSS/parser and
  notices audits, `effigy docs:check`, final headless `effigy qa`, and `git diff
  --check origin/main...HEAD`. Never run windowed, release, or workflow selectors.

## Completion Protocol

1. Confirm the launcher-provided root is a clean registered non-`main` worktree.
   Do not create another worktree, reset, stash, or discard state.
2. Fetch origin; require `HEAD == origin/main` and the PR #162 merge above as an
   ancestor; load this exact handoff from `HEAD`. Stop on mismatch.
3. First reproduce and repair the two clean-main red baselines without relying
   on prebuilt `dist/` or a warmed declaration-tools cache. Then work in
   meaningful harness/build/receipt batches. Keep package behavior and
   release surfaces unchanged unless the orchestrator explicitly accepts a
   minimal repair.
4. Remove every disposable checkout, consumer, archive, and cache before
   handoff. Reconcile the card and one execution log with actual evidence.
5. Push one worker branch and open one PR against current `main`. Rebase and
   proportionally revalidate if unrelated work lands first.
6. Return PR URL, exact head, receipt identity, validation, falsification
   receipts, and limits. Do not merge or start `g16.054`.

## Next Move

Inspect the current compiled pack harness and spec 070 receipt schema first.
Design the clean-checkout/repeated-pack flow before editing individual probes,
then migrate the existing proofs without reducing their count or strength.
