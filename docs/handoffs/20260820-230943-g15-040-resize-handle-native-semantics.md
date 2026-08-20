---
title: g15.040 ResizeHandle native semantics worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-20
updated: 2026-08-20
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260820-230943-g15-040-resize-handle-native-semantics.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, resize-handle, gpui]
---

# 1. What This Thread Was Doing

Poodle is completing the g15 human specimen review before visual conformance
and v0.2.0 certification. PR #55 completed the nine-page foundation-layout
review and made ResizeHandle specimens genuinely interactive, but it also
exposed a real shared-native component gap: pointer drag works while focus,
keyboard resize, and value-range declaration do not.

Execute `g15.040` as one bounded native repair. Close the renderer-neutral and
GPUI behavior gap, return the audit row to an honest completed state, then stop
for orchestrator review.

# 2. Why It Matters

ResizeHandle is a primitive used by higher-level split layouts. If its native
implementation cannot receive focus or respond to the same keys as the web
reference, every composite built on it inherits the drift. The specimen review
cannot resume at `g15.031`, and visual conformance cannot begin, while that
semantic blocker remains open.

# 3. Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Planning checkout: `main`
- Planning base: `763ff922f08f1eb025955a066cdc26fccfa5f448`
- Pushed-main verification: local `HEAD` and `origin/main` matched that SHA
  before this handoff commit; the planning checkout was clean.
- Posture: `strict-ready`.
- Planning state: `g15.030` is complete; `g15.027` is paused on this routed
  repair; `g15.031`–`g15.033` remain serial behind it.
- Worker branch: `t3code/g15-040-resize-handle-native-semantics`
- Worker worktree: use the clean, registered, dedicated non-`main` worktree
  supplied by the launcher. Do not create another when that condition holds.
- Manual worktree creation command: none. If the launcher context is invalid,
  stop and report it; do not guess a path or use `/tmp`.
- Ready card: `docs/roadmaps/g15/040-resize-handle-native-semantics.md`
- Parent method: `docs/roadmaps/g15/027-screen-clear-human-review.md`
- Allowed runway: `g15.040` only.
- Work budget: one native primitive repair, one PR, then stop.
- Topology: serial. This card mutates the shared native node/render substrate
  and the catalogue audit, so no review child runs beside it.
- Model guidance: frontier coding model, high reasoning.
- PR target: `main` from the worker branch. The worker pushes but never merges.
- Merge authority: absent. Only the operator can authorise merge after review.

Canonical context:

- `AGENTS.md`
- `.agents/skills/effigy/SKILL.md`
- `docs/contracts/001-working-rules.md`
- `docs/contracts/003-native-accessibility.md`
- `docs/contracts/components/resize-handle.md`
- `docs/roadmaps/g15/040-resize-handle-native-semantics.md`
- `docs/roadmaps/g15/027-screen-clear-human-review.md`
- `docs/roadmaps/g15/specimen-catalogue-audit.md`
- `docs/logs/2026-08/20260820-g15-030-foundation-layout-review.md`
- `packages/contracts/components/src/resize_handle.rs`
- `packages/contracts/node/src/lib.rs`
- `packages/render/src/resize_handle.rs`
- `packages/gpui/node-backend/src/interaction.rs`
- `packages/gpui/preview/src/specimens/resize_handle.rs`

The open `docs/triage/20260820-205249-transitions-dev-motion-learning.md`
note is unrelated and remains open; do not pull it into this card.

# 4. Boundaries

Writable scope:

- ResizeHandle shared Rust spec/render composition, focused tests, and GPUI
  specimen
- the smallest reusable `NodeA11y` numeric-range extension required to carry
  current/minimum/maximum values
- focused GPUI node-backend or mounted-preview evidence needed to prove the
  existing key route
- ResizeHandle contract wording only where renderer-neutral/native behavior
  needs clarification
- the ResizeHandle audit row, mechanically affected totals, one August batch
  log, and root `PAPERCUTS.md` for newly found execution friction

Out of scope:

- Svelte or React implementation/specimen changes
- a new public resize callback, compatibility alias, dual API, or
  component-specific backend input route
- claiming GPUI platform assistive-technology projection is fixed; GPUI 0.2.2
  still carries the accepted contract-003 upstream gap
- any other primitive/composite repair, later screen-clear child, visual
  conformance, motion-learning triage, Jetstream, release work, or workflow
  edit
- roadmap/card status, generation front doors, or dispatch-ledger edits; those
  remain orchestrator-owned
- merging the PR

Stop if the existing node key and resize-delta seams cannot express the repair,
if numeric range declarations require a broad accessibility redesign, if a
visible focus treatment requires changing ResizeHandle geometry, or if the
same defect proves to be a family-wide repair outside this component.

# 5. Important Context

The contract already fixes the behavior: horizontal Left/Right and vertical
Up/Down emit `±8`; Home/End emit `±9999`; cross-axis arrows do nothing. Reuse
the existing native resize callback for those deltas. Do not create a separate
keyboard-only public handler when the current seam is sufficient.

The current substrate has most of what is needed:

- `ResizeHandleSpec` already carries `aria_value_now`, `aria_value_min`, and
  `aria_value_max`.
- `NodeA11y` carries a current numeric `value`, but no numeric minimum/maximum;
  add the smallest general range vocabulary if required.
- `NodeKey` and the GPUI backend already route Arrow/Home/End through
  `Interaction::on_key`.
- `poodle-render::resize_handle` currently wires drag only, sets role/label,
  and leaves the node unfocusable with no key handler or value declaration.
- GPUI 0.2.2 does not expose platform accessibility attributes. This card must
  carry the correct renderer-neutral declarations without overstating that
  accepted backend limitation.

Keep pointer drag and its per-frame delta semantics unchanged. Enabled and
disabled behavior must be explicit. A focusable control also needs an
observable focus treatment through the existing style/focus vocabulary; do not
accept a keyboard handler on an effectively unreachable or invisible focus
target.

The audit currently records ResizeHandle as `A / A / B` with disposition
`contract/runtime-blocker`. On successful closure, update the row and recount
all 175 rows mechanically rather than editing totals by intuition.

`effigy doctor` on the planning base reports the repository's existing scan
baseline (`generated-in-src`, `god-files`, and `stale-suppressions`) plus a
stale graph index. Those broad scan findings did not block the known task
routes or this planning change. Do not repair them in this card; use the named
selectors below and report any new failure that changes the plan.

# 6. Suggested Next Move

1. Read this handoff, the ready card, ResizeHandle contract, native
   accessibility contract, current render path, GPUI key routing, specimen,
   and audit row.
2. Run the worker preflight below and confirm a clean registered non-`main`
   worktree at the pushed main tip.
3. Write focused failing renderer tests for focusability, orientation/value
   declaration, axis-key filtering, exact Arrow/Home/End deltas, disabled
   suppression, and retained drag behavior.
4. Add only the reusable node range vocabulary and ResizeHandle composition
   needed to make those tests pass.
5. Add mounted headless GPUI evidence that the real focused key route changes
   specimen pane state and renderer-neutral current value.
6. Recount the audit, write one August batch log, run the complete headless
   validation once, push one PR, and stop.

# 7. Completion Protocol

## Startup

Read this handoff first. Then run only:

```sh
git rev-parse --show-toplevel
git branch --show-current
git status --porcelain
git worktree list --porcelain
```

The launcher-provided worktree is authoritative when it is clean, registered,
and on a dedicated non-`main` branch, even if its generated path or branch name
differs from this handoff. Record the actual values and do not create another
worktree. If it is dirty, on `main`, or unregistered, stop and report the
preflight output. Do not edit the main planning checkout. A manual fallback is
allowed only under the operator-selected `AGENTS_WORKTREE_CONTAINER_DIR` in
ignored `.agents.local.env`; never guess a repository-adjacent or temporary
path.

From the selected worktree, confirm `HEAD == origin/main`, confirm
`763ff922f08f1eb025955a066cdc26fccfa5f448` is an ancestor of `HEAD`, and
confirm this handoff exists in `HEAD`. Then read the canonical refs above and
run `effigy tasks` for selector orientation.

## While working

- Keep the repair on the existing renderer-neutral node and key seams.
- Work in one coherent implementation/evidence batch, not atomic micro-edits.
- Preserve pointer drag behavior while adding keyboard behavior.
- Report a stop condition before widening the node vocabulary or component
  family.
- Do not edit `.github/workflows/` or perform release mutations.

## Final validation

Run:

- focused `poodle-node`, `poodle-render`, and GPUI preview/backend tests
- `effigy ci:rust`
- `effigy check:gpui`
- `effigy regressions:native`
- `effigy probe:gpui-specimens`
- `effigy docs:check`
- `effigy qa`
- `git diff --check origin/main...HEAD`

Headless only. Never run a `*-windowed`, `test:native-visual`, Jetstream, or
release selector.

## Handoff back

Push the worker branch and open one PR targeting `main`. Report:

- PR URL and pushed head SHA
- exact spec/node/render/specimen/test files changed
- focused evidence for focus, axis keys, Home/End, range declaration,
  disabled suppression, and retained drag
- validation commands and outcomes
- audit grade/disposition and mechanically recounted totals
- any stop condition, intentional limitation, or residual risk

Do not claim platform AT projection, change card or roadmap status, edit the
dispatch ledger, merge the PR, start `g15.031`, or widen the card after the PR
is opened.
