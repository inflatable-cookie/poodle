---
title: g16.031 continuous audio machine and web lifecycle worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle component parity
created: 2026-08-29
updated: 2026-08-29
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260829-161419-g16-031-continuous-audio-web-lifecycle.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, audio, knob, fader, xy-pad]
---

## Assignment

Implement `g16.031` only. Align the paired TypeScript/Rust continuous-audio
machines for Knob, Fader, and XYPad, then repair the Svelte and React pointer
and entry lifecycles around that shared model. Do not begin native mounting,
payload drag-and-drop, or `g16.032`.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Planning branch: `main`
- Required planning ancestor: `aa801d4441f2f7ef9caa6f3cd2c866f8b13415f0`
- Assigned card:
  `docs/roadmaps/g16/031-continuous-audio-machine-and-web-lifecycle.md`
- Governing architecture:
  `docs/architecture/008-audio-control-family.md`
- Governing contracts:
  `docs/contracts/components/knob.md`,
  `docs/contracts/components/fader.md`, and
  `docs/contracts/components/xy-pad.md`
- Prior dependency: `g16.030` merged in PR #98
- Worker branch: use the launcher-provided clean non-`main` branch; suggested
  manual fallback is `t3code/g16-031-continuous-audio-web-lifecycle`
- Worktree: harness-managed. Do not create a second worktree when the launcher
  already supplied one.
- Required sibling links: none
- PR base: `main`
- Merge authority: worker must not merge

No open PR currently owns the audio machine, vector, or web component surfaces.
The orchestrator is separately compiling the public API for `g16.022`; that
planning work must not edit this worker's branch.

## Locked Outcome

- One shared `audioControls` semantic corpus runs through the existing
  TypeScript and Rust machine-conformance runners.
- Knob, Fader, and XYPad return identical contexts and ordered effects for the
  cases named by the card.
- A pointer gesture has exactly one accepted begin and one terminal end.
  Repeated begin, stale pointer ids, lost capture, cancel, teardown, and
  repeated termination cannot strand or duplicate it.
- Coarse/fine changes rebase without jumps.
- Knob vertical/circular modes, Fader detents and axes, and XYPad coarse press,
  fine travel, independent laws, and atomic pair updates follow their existing
  contracts.
- Svelte and React callback traces match. Svelte Knob/Fader Enter commits once;
  Escape commits nothing; following blur cannot reverse or duplicate either.
- Existing public web props and callbacks remain intact. Add no alias,
  fallback, DOM-event public type, or payload drag-and-drop dependency.
- The parity ledger does not move. Native mounted proof belongs to `g16.032`.

## Scope

Follow the card's five batches and writable scope exactly. In particular:

- paired pure audio modules, focused tests, exact exports, and the bounded
  shared machine-vector section;
- Svelte/React Knob, Fader, and XYPad implementations and focused mounted
  component tests;
- specimens only for a proven stateful-example defect;
- exact contract/architecture reconciliation, the card, one August execution
  log, and truthful g16/front-door closeout.

Do not edit Node vocabulary, poodle-render audio builders, GPUI/Jetstream
adapters, drag-and-drop files, other audio components, recipes/tokens,
workflows, versions, releases, sibling repositories, or consumers.

Stop if correctness requires a public API change, DOM/native behavior inside a
pure machine, a global singleton, or scope beyond this card.

## Startup

1. Before broad reads, run only `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. Accept a clean launcher-provided dedicated non-`main` worktree. If the
   launcher supplied `main` or a dirty checkout, stop and report it. Do not
   guess a worktree path or use `/tmp`.
3. Fetch origin; confirm `HEAD == origin/main`; confirm the required planning
   ancestor is present; load this tracked handoff with `git show HEAD:`. The
   committed blob is canonical.
4. Read `AGENTS.md`, this handoff, g16's README, card 031, architecture 008,
   the three component contracts, working rules, and the repo-local Effigy
   skill before implementation.
5. Run `effigy tasks` and resolve the focused selectors named by the card.

## Validation And Handoff Back

Run every focused and broad headless check named by card 031, including paired
machine vectors, focused web components, core/components/contracts, web and
Rust CI, docs, final `effigy qa`, and
`git diff --check origin/main...HEAD`. Never run `*-windowed`, native visual,
Jetstream preview/QA, release, publication, or workflow-mutation selectors.

Keep commits in coherent batches. Push the worker branch and open a PR against
`main`. The PR must link card 031 and its contracts, report paired vector and
callback evidence, list validation actually run, state that no ledger cell
moved, and link the execution log. Return the PR URL to the operator. Do not
merge or continue into card 032.
