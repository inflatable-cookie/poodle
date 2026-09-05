---
title: g15.052 native focus-ring parity worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-22
updated: 2026-08-22
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260822-215150-g15-052-native-focus-ring-parity.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, focus, gpui, button, stepper]
---

## What This Thread Is Doing

Execute `g15.052` only. Add one reusable focus-ring capability to the existing
shared Rust node vocabulary, project it faithfully in GPUI, then use it to
close the measured Button focus-ring mismatch and the retained Stepper
keyboard-entry/focus gap.

This file and the ready card are the complete worker prompt. Do not depend on
conversation history.

## Why It Matters

The first real primitive visual comparison succeeded as a diagnostic and found
a release blocker: web Button declares a 2px ring with a 2px offset, while
GPUI only recolours the existing 1px border. Stepper has the other half of the
same missing primitive: borderless controls declare `focusable`, but without a
focus treatment the backend creates no tracked handle, so keyboard entry only
works after pointer focus.

The operator rejected tolerance widening and rejected classifying the Button
result as a known delta. This is a real native substrate repair before v0.2.0.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:**
  `62b4da1337c356aac8918a5b3977e29a542bff54`
- **Pushed base:** local `HEAD` and `origin/main` matched that SHA; the planning
  checkout was clean before this handoff commit.
- **Posture:** `strict-ready`.
- **Worker branch:** `t3code/g15-052-native-focus-ring-parity`.
- **Worker worktree:** use the clean, registered, dedicated non-`main`
  worktree supplied by the launcher. Do not create another worktree.
- **Ready card:**
  `docs/roadmaps/g15/052-native-focus-ring-parity.md`.
- **Allowed runway:** `g15.052` only.
- **Dispatch topology:** serial. `g15.043`, `g15.050`, and `g15.013` stay out.
  No overlapping second lane is authorised.
- **Canonical refs:** `AGENTS.md`, `.agents/skills/effigy/SKILL.md`,
  `docs/contracts/001-working-rules.md`, Button and Stepper contracts,
  `docs/roadmaps/g15/{042-stepper-native-interaction-parity,047-primitive-visual-comparison,052-native-focus-ring-parity}.md`,
  `docs/logs/2026-08/20260822-g15-047-primitive-visual-comparison.md`,
  `packages/contracts/node/src/lib.rs`, `packages/render/src/{button,stepper}.rs`,
  `packages/gpui/node-backend/`, and the existing headless GPUI regressions.
- **Runtime restriction:** headless only. Never run a `*-windowed` selector,
  `test:native-visual`, GPUI preview, Jetstream selector, workflow, release,
  tag, or publication path.
- **PR base/head:** `main` <- worker branch.
- **Merge authority:** absent. Push a PR and stop.

## Fixed Design

The roadmap has already made the architecture decision. Implement it; do not
reopen it casually.

Add a small typed focus-ring value to `NodeStyle`, with resolved `color`,
`width`, and `offset`. It is separate from `StylePatch` because it is an
out-of-flow affordance that must preserve the resting border and work on a
borderless control. `StylePatch::focus` remains available for other focused
visual changes, and the two compose.

Shared component renderers opt into the ring and supply resolved values. The
GPUI backend observes the real focus handle and paints the ring. Do not infer
one for every focusable node, put GPUI objects in `poodle-node`, duplicate
Button/Stepper rules in the backend, or widen this into a focus-style sweep.

## Exact Work

1. **Node and backend capability**
   - introduce the resolved ring value with absent default;
   - make a declared ring sufficient for focus tracking;
   - paint it only while the real handle is focused;
   - preserve resting border, shadow stack, radius, bounds, and hover
     composition;
   - project width and offset without layout inflation or a wider replacement
     border;
   - prove bordered and borderless nodes, focus/blur, and existing-shadow
     composition in focused backend tests.

2. **Button closure**
   - replace the current focus-time border recolour in
     `packages/render/src/button.rs` with the ring channel;
   - resolve the existing focus color, `border.width.focus`, and contracted
     2px offset;
   - change no idle/hover/active/pressed/disabled/loading/layout behavior;
   - run the unchanged 18-case comparator into disposable output. The 16
     focus-ring findings must become zero with no policy, tolerance, fixture,
     receipt identity, or known-delta change. The 16 accepted shadow findings
     may remain annotated and blocking.

3. **Stepper closure**
   - apply the channel only to the contracted trigger, rerun, and summary
     controls;
   - ensure stable tracked focus identities;
   - prove keyboard entry reaches the controls without any prior pointer
     press and that `Enter`/`Space` activates the focused action;
   - preserve the selection, rerun, collapse, and pointer behavior from
     `g15.042`.

4. **Evidence**
   - write one August `g15.052` execution log with the exact paint mechanism,
     pre/post comparator role verdict, tests, and remaining shadow delta;
   - retain a small headless focused-state image/contact sheet for one
     bordered Button and one borderless Stepper control under
     `docs/logs/2026-08/assets/g15-052/`;
   - keep it point-in-time evidence, never a baseline.

## Hard Boundaries

Do not change:

- Svelte/React components or CSS;
- public component props or tokens;
- specimen catalogue pages;
- the accepted 18-fixture inventory;
- comparator thresholds, policy, known-delta classification, or batch
  completeness rules;
- existing Button shadows as an incidental repair;
- unrelated component focus treatments;
- package versions, release notes, workflows, Jetstream, Longhorn, tags,
  publication, or merge state.

Stop if GPUI cannot reproduce the declared width and offset without changing
layout, replacing the resting border, or erasing shadows. Stop if Stepper needs
a general focus-order architecture beyond the three named controls. Stop if a
public API, token, GPUI fork, or new contract decision is needed. Do not solve
a red comparison by changing its judge.

## Important Source Context

- `StylePatch` currently carries only background, border color, text color,
  and opacity. Button's current `style.focus` recolours the resting border.
- `tracks_focus` currently registers a handle when a node asks for
  `on_focus_change` or combines `focusable` with `style.focus`. The new
  dedicated ring must participate without making every focusable node tracked.
- GPUI already projects multi-layer shadows. Choose the smallest faithful
  focus-time paint mechanism, but the observable result—not a specific GPUI
  trick—is the requirement.
- PR #68's committed evidence remains historical. Generate `g15.052` evidence
  separately; do not overwrite `assets/g15-047/`.
- Before repair, the Button batch reports 32 blocking findings: 16 focus-ring
  width mismatches and 16 annotated `gpui-omits-box-shadow` findings. After
  repair, the expected red aggregate is the 16 shadow findings only.
- The GPUI fork remains pinned to
  `inflatable-cookie/zed@87d9afbe71ef06ea0634499dc35d104bb29dc020`.
- The operator has already accepted the shadow limitation for v0.2.0. Keep it
  visible; do not absorb it into this card.

Report after three meaningful chunks:

1. the node/backend ring paints correctly on focused bordered and borderless
   proof nodes;
2. Button comparison loses all 16 focus findings under the unchanged policy;
3. Stepper keyboard entry/activation, evidence, validation, and PR are ready.

## Validation

Use `effigy tasks` to confirm exact selector names, then run the narrow board:

- focused `poodle-node`, `poodle-render`, and
  `poodle-gpui-node-backend` tests;
- focused Stepper mounted headless regressions and specimen probe;
- `effigy test:visual-button-comparison` with disposable output;
- `effigy smoke:gpui-offscreen-capture`;
- `effigy check:gpui`;
- `effigy docs:check`;
- `git diff --check origin/main...HEAD`.

The comparator is expected to exit non-zero on the accepted shadow findings;
record the exact channel counts and prove focus findings are zero. Do not turn
that expected diagnostic red into a skipped validation.

## Worker Protocol

### Before editing

1. Run only:
   - `git rev-parse --show-toplevel`
   - `git branch --show-current`
   - `git status --porcelain`
   - `git worktree list --porcelain`
2. Require a clean registered non-`main` worktree. Never clean, reset, stash,
   or discard someone else's checkout.
3. Fetch `origin`; require `HEAD == origin/main`; confirm the planning base
   above is an ancestor and this handoff exists in `HEAD`.
4. Read `AGENTS.md`, the Effigy skill, the ready card, the named contracts and
   prior cards/log, then the narrow source surfaces above.
5. Run `effigy tasks` and the smallest clean starting tests. Do not start with
   the full repository QA board.

### While working

- Work in the three chunks above, not micro-commits.
- Add focused regressions before broadening from the proof nodes to Button and
  Stepper.
- Keep all captures headless and deterministic.
- Record measured before/after evidence; do not assert visual closure from
  source inspection.
- If a stop condition fires, push the evidence and report the blocker instead
  of improvising a new architecture.

### Completion

1. Run the validation board and record exact results.
2. Write the execution log and small `g15.052` evidence set.
3. Update `g15.052` and the release-gap register only with landed worker
   evidence; do not alter other roadmap status.
4. Run `git diff --check origin/main...HEAD`; inspect scope for every forbidden
   surface.
5. Commit, push, and open one PR against `main`.
6. Link the ready card, prior `g15.047` evidence, new log/assets, validation,
   exact comparator counts, and unresolved items in the PR body.
7. Report the PR URL and stop. Never merge.

## Review And Merge

The orchestrator independently reviews the new node vocabulary, GPUI
projection, real focus tracking, Button comparison counts, Stepper keyboard
entry, scope, assets, and checks. The operator reviews the focused-state
evidence. Only the orchestrator merges after explicit operator authority.

After merge, the orchestrator closes the two focus-gap rows and advances to
`g15.043`. The release candidate remains blocked until both are complete.
