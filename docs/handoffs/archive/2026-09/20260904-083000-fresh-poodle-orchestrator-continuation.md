---
title: Fresh Poodle orchestrator continuation
kind: northstar-handoff
handoff_mode: coordinator-continuation
worker_mode: orchestration
dispatch_authority: orchestrator
review_authority: orchestrator
merge_authority: orchestrator
status: ready
owner: Poodle Northstar orchestrator
created: 2026-09-04
updated: 2026-09-04
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260904-083000-fresh-poodle-orchestrator-continuation.md
base_required: current-pushed-main
tags: [coordination, handoff, orchestrator, poodle, g16, continuation]
---

## Task

Take over as the active Poodle Northstar coordinator in the existing main
Poodle workspace. Maintain the roadmap, compile bounded worker cards, dispatch
and supervise workers, arrange independent exact-head reviews, merge accepted
PRs, close roadmap front doors, and keep the programme moving. Pinning remains
manual; do not add automatic pinning machinery.

## Context

The previous coordinator completed the 29-component Nucleus GPUI-headless M1
receipt tranche and is handing off because its conversation became long. This
is a coordination transfer, not a request to reopen completed component work.
The previous coordinator must stop mutating or coordinating this lane after
launching you.

Repository: `/Users/tom/Dev/projects/poodle`

Paseo main workspace: `wks_a43cedb45e82cb05` (`Poodle doodle`)

Exact pushed main at handoff: `01135d461691b6dd901f637ab041bab93be4aa82`

Git state at handoff: clean `main`, synchronized with `origin/main`.

GitHub state at handoff: no open pull requests.

Poodle worker-workspace state: no active g16 worker worktrees remain. The
g16.092 and g16.093 workspaces and their agents were archived after merge.

## Current State

- Nucleus M1 mounted receipts are complete: 29/29 validated terminal receipts
  at runtime source `f0774a7d15a195cc6b8506c5da68db99807e5376` and preview
  lock digest
  `c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c`.
- Final MessageCenter PR #198 merged as
  `06de812f7037eeca204d89c72fb4c586723600eb`.
- Native fresh-consumer repair PR #199 merged as
  `17534f484665bbbdd93e2ec70bec521318201941`.
- Ordinary Cargo scope routing repair PR #200 merged as
  `f7ae38d9f7e644de6d39de43363dd77bbf75f842`.
- ToastHost PR #197 merged as
  `4a615e99046fa9e6dc14801ef1e6f60760336fc2`.
- Mounted-programme/front-door closeout is commit
  `01135d461691b6dd901f637ab041bab93be4aa82`.
- `effigy docs:check` passed on the closeout. Main was clean afterward.
- M1 completion does not imply A1, V1, lab V2, Nucleus M2, adoption, release,
  or publication authority.

## Relevant Files

- `AGENTS.md` — repository workflow, ownership, and hard constraints.
- `docs/roadmaps/generation-index.md` — current programme summary and next
  authority choice.
- `docs/roadmaps/g16/README.md` — g16 front door, complete through g16.094.
- `docs/roadmaps/g16/nucleus-gpui-parity-programme.md` — 29/29 M1 completion
  and the later separately owned phases.
- `docs/roadmaps/g16/nucleus-parity-manifest.json` — exact 29-row cohort.
- `docs/roadmaps/g16/parity-evidence-ledger.md` — generated evidence state.
- `docs/roadmaps/g16/component-continuation-runway.md` — canonical ready,
  serial, gated, and held frontier.
- `docs/roadmaps/g16/visual-lab-unblock-runway.md` — exact VL-0 through VL-3
  authority and execution route.
- `docs/roadmaps/g16/051-icon-geometry-native-visual-admission.md` — blocked
  until accepted VL-1 plus a separate VL-2A icon adapter/manifest extension.
- `docs/roadmaps/g16/052-contributor-design-guidance-pilot.md` — gated on
  named human reviewers, two freeze approvals, and orchestrator custody.
- `PAPERCUTS.md` — small inherited execution friction; do not silently turn it
  into unplanned programme work.

## What Was Tried And Resolved

- ToastHost initially introduced a thread-global focus sweep that hung a
  two-window drag regression. PR #197 repaired it with per-window ownership and
  proved the old head failed while the exact base passed.
- Rust 1.95 fresh consumers resolved crates.io `tinyvec 1.13.0` alloc-only and
  failed to find `vec!`. PR #199 unified the `std` feature at the node-backend
  dependency boundary and freshly re-emitted the receipt cohort.
- Ordinary installed smoke incorrectly treated every Cargo manifest/lock path
  as a release version surface. PR #200 made ordinary classification
  content-aware while leaving strict and candidate certification unchanged.
- MessageCenter preparation required production Popover focus, concurrent
  layer isolation, exact tokens, and real mounted wheel scrolling. PR #198
  proved those and completed receipt 29/29.

## Decisions

- Treat `docs/` as programme authority. Do not infer a new implementation card
  merely because capacity exists.
- Keep workers in dedicated Paseo worktree workspaces. Workers push PRs and
  never merge.
- Put PR reviewers in short-lived agent tabs attached to the main Poodle
  workspace, or use private subagents. Archive reviewer tabs immediately after
  the verdict. The operator explicitly dislikes permanent worker-plus-reviewer
  sidebar clutter.
- Archive completed worker workspaces after accepted PR merge and closeout.
- Parallelize only independent preparation lanes; shared receipt/evidence and
  front-door mutations remain serial.
- The orchestrator owns merges and roadmap closeout.
- Never run local `*-windowed` or native-visual selectors without explicit
  operator approval.
- Never edit workflows, tag, publish, release, or mutate sibling repositories
  without explicit authority.
- Pinning remains manual. Do not create schedules, automation, or implicit pin
  updates unless the operator asks.

## Next Authority Choice

There is no automatic next component receipt: the M1 mounted tranche is done.
Present the operator with the real gated choices before dispatching:

1. Visual lab VL-0: requires explicit authority to create or use the dedicated
   internal `poodle-conformance-lab` repository and a named maintainer. Only
   then compile/dispatch Button-only VL-1. After VL-1, VL-2A icon geometry and
   VL-2B Nucleus adapters may run in parallel if their files do not overlap.
2. GPUI accessibility evidence: programme hold until the operator selects the
   authority and manual/runtime evidence boundary.
3. Nucleus consumer journey/M2/V2/adoption: Nucleus/operator-owned; do not fake
   consumer evidence inside Poodle.
4. Contributor guidance g16.052: needs named human reviewers, Poodle-core and
   Northstar freeze approvals, and orchestrator-owned execution custody.
5. Release/publication: the v0.3.0 candidate exists, but tag, publish, workflow,
   registry, and sibling-adoption authority remain separate and absent.

## Acceptance Criteria

- [ ] Confirm main still equals or descends cleanly from the pinned handoff SHA.
- [ ] Read AGENTS, Northstar, repo-local Effigy, generation index, g16 front
      door, continuation runway, and relevant card before acting.
- [ ] Reconcile any new operator message or external PR state before dispatch.
- [ ] Ask for the missing authority when a choice above would materially expand
      scope; do not guess it.
- [ ] For new work, compile a bounded card and single-file worker handoff on
      main, validate docs/diff, then dispatch through a dedicated worktree.
- [ ] Use configured Paseo profiles. Use temporary main-workspace reviewers and
      archive them after verdicts.
- [ ] Keep pinning manual.

## Constraints

- Do not reopen completed g16.062–g16.094 receipt work without a new defect or
  explicit operator direction.
- Do not call 29/29 M1 full parity, accessibility parity, visual parity, or a
  shipping/adoption decision.
- Do not edit Figmatic, Nucleus, the future lab, or any sibling repository from
  Poodle authority alone.
- Preserve unrelated user changes and a dirty worktree if one appears.
- Use `effigy tasks`/focused selectors and `effigy qa` according to repo policy;
  do not invent shell rituals or run prohibited selectors.
- End meaningful turns with the current state and one logically scoped next
  move unless the work is fully finished.
