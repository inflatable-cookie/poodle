---
title: g15.039 DateTimeZonePicker nested-layer pointer commit worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-20
updated: 2026-08-20
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260820-210052-g15-039-date-time-zone-picker-nested-layer.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, web, overlays, date-time-zone-picker]
---

# 1. What This Thread Was Doing

Poodle is in the g15 release-evidence and specimen-review generation. The
foundation date/time review landed in PR #53 and found one real component
blocker: pointer selection from DateTimeZonePicker's nested portalled
TimeZoneSelect closes the outer picker before the timezone commits.

Implement `g15.039` as one bounded paired-web repair. This is not a specimen
curation card and does not reopen the rest of `g15.029`.

# 2. Why It Matters

The primary pointer workflow is dead in both active web runtimes while
keyboard selection still works. That makes the current A-level specimen and
component evidence dishonest. The repair blocks `g15.030`, the remaining
screen-clear review runway, and therefore the v0.2.0 release path.

# 3. Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Planning checkout: `main`
- Planning base: `260ca8a9919406b9c95692ddc79472e12b7b56cb`
- Pushed-main verification: local `HEAD` and `origin/main` matched that SHA
  before this handoff commit; the planning checkout was clean.
- Planning state: PR #53 is merged, `g15.029` is complete, `g15.039` is ready,
  and `g15.030` is explicitly blocked behind this repair.
- Worker branch: `t3code/g15-039-date-time-zone-picker-nested-layer`
- Worker worktree: use the clean, registered, dedicated non-`main` worktree
  supplied by the launcher. Do not create another when that condition holds.
- Manual worktree creation command: none. If the launcher context is invalid,
  stop and report it; do not guess a path or use `/tmp`.
- Active lane: g15 human specimen review returned a component blocker.
- Ready card: `docs/roadmaps/g15/039-date-time-zone-picker-nested-layer.md`
- Allowed runway: that card only.
- Work budget: one paired-web implementation/evidence batch, one PR, then
  stop.
- Topology: serial. `g15.030` shares the audit and must not begin until this
  card is accepted.
- Model guidance: capable coding model, medium reasoning.
- Required validation is the exact headless set in the card. Never run a
  `*-windowed`, native-visual, Jetstream, or release selector.
- PR target: `main` from the worker branch. The worker pushes but never merges.
- Review state: implementation, orchestrator review, and operator live pointer
  sign-off are pending.

Canonical context:

- `AGENTS.md`
- `.agents/skills/effigy/SKILL.md`
- `docs/contracts/001-working-rules.md`
- `docs/contracts/components/date-time-zone-picker.md`
- `docs/contracts/components/time-zone-select.md`
- `docs/roadmaps/g15/039-date-time-zone-picker-nested-layer.md`
- `docs/roadmaps/g15/specimen-catalogue-audit.md`
- `docs/logs/2026-08/20260820-g15-029-foundation-date-time-review.md`

# 4. Boundaries

In scope:

- Svelte and React DateTimeZonePicker nested-layer ownership and dismissal
  logic
- TimeZoneSelect or shared internal layer plumbing only when required to make
  the nested portal an explicit part of its owning composite
- paired focused tests using a real portalled timezone option
- the DateTimeZonePicker contract only if the ownership guarantee needs
  explicit wording
- the DateTimeZonePicker audit row, mechanically recounted totals, and one
  August batch log

Out of scope:

- specimen-page changes
- picker value shape, public props, aliases, compatibility shims, or breaking
  API changes
- timezone data, date/time math, shared Rust, GPUI, Jetstream, native work, or
  unrelated overlays
- global CSS-selector special cases, DOM-shape coupling, or a fix that makes a
  genuine outside dismissal take two gestures
- the open `transitions.dev` triage item
- roadmap front doors, card status, or dispatch-ledger edits; those remain
  orchestrator-owned
- merging the PR

Stop and report before expanding scope if a general overlay-stack redesign,
new public API, divergent Svelte/React behavior, or repair of another
composite becomes necessary.

# 5. Important Context

Both DateTimeZonePicker implementations install an outer document-level
`mousedown` dismiss handler and currently consider only the trigger/root and
outer picker surface to be inside. TimeZoneSelect renders its options in a
second portal, so a real option press reaches the outer dismiss path as an
outside interaction before the option commit completes.

TimeZoneSelect already forwards `onOpenChange`. Open-state tracking alone is
not proof of containment: blindly ignoring outside presses while the nested
list is open would leave the whole composite open or require a second click.
The required outcome is explicit nested-layer ownership plus one-gesture
outside dismissal.

Use a real option click as the regression seam. Directly invoking a callback
does not exercise the event ordering that caused the defect. Preserve Escape,
controlled and uncontrolled open state, calendar selection, time entry, focus,
and disabled behavior.

Svelte remains the reference behavior, but the paired React surface must have
the same observable contract. After this card lands, the orchestrator can
promote `g15.030`.

# 6. Suggested Next Move

1. Read this handoff and the ready card before exploring.
2. Run the worker preflight below and confirm the launcher supplied a clean,
   registered, non-`main` worktree.
3. Inspect the paired DateTimeZonePicker implementations, TimeZoneSelect/Select
   portal seam, shared layer helpers, and existing focused tests.
4. First encode the failure with paired tests that click a real portalled
   option and separately exercise one-gesture outside dismissal while the
   nested list is open.
5. Implement the smallest internal ownership mechanism that satisfies both
   cases without changing public API.
6. Update the contract only if needed, then update the audit row/totals and
   write `docs/logs/2026-08/20260820-g15-039-date-time-zone-picker-nested-layer.md`.
7. Validate the complete batch once, push the PR, and stop for orchestrator
   review.

If diagnosis shows the fix crosses a stop condition, report the evidence and
proposed boundary change instead of improvising a larger solution.

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
and on the dedicated non-`main` branch, even if a generated path or branch
hint differs. If it is dirty, on `main`, or unregistered, stop and report the
preflight output. Do not edit the main planning checkout. A manual fallback is
allowed only under the operator-selected `AGENTS_WORKTREE_CONTAINER_DIR` in
ignored `.agents.local.env`; never guess a repository-adjacent or temporary
path.

## While working

- Use Effigy task routing and supported selectors.
- Keep changes inside the stated boundaries.
- Work as one coherent implementation and evidence batch rather than a chain
  of micro-commits.
- Record small execution friction in root `PAPERCUTS.md` only when genuinely
  encountered; keep it append-only.
- Do not edit `.github/workflows/` or perform release mutations.

## Final validation

Run:

- focused Svelte and React DateTimeZonePicker tests
- `effigy check:svelte`
- `effigy react:build`
- `effigy catalogue:check`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Headless only. Do not run a `*-windowed`, native-visual, Jetstream, native, or
release selector.

## Handoff back

Push the worker branch and open one PR targeting `main`. Report:

- PR URL and pushed head SHA
- exact files and contract behavior changed
- validation commands and outcomes
- audit grade/totals before and after
- live-review routes for both previews:
  `#components/date-time-zone-picker`
- any residual risk or explicit stop-condition finding

Operator live pointer review remains pending until the orchestrator opens the
paired routes. Do not claim visual sign-off, change roadmap/card status, edit
the dispatch ledger, merge the PR, or start `g15.030`.
