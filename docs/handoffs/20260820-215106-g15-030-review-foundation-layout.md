---
title: g15.030 foundation-layout specimen review worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-20
updated: 2026-08-20
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260820-215106-g15-030-review-foundation-layout.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, specimens, layout]
---

# 1. What This Thread Was Doing

Poodle is completing the g15 human specimen review before visual conformance
and v0.2.0 certification. The first two screen-clear children are complete,
and the DateTimeZonePicker blocker they returned is repaired in PR #54.

Execute `g15.030`: review exactly nine foundation-layout catalogue pages in
Svelte and React, consume the existing headless GPUI evidence, keep good pages
unchanged, and repair only bounded specimen-teaching defects.

# 2. Why It Matters

The earlier audit only proved that these pages had no obvious mechanical
defect. It did not prove that a human can understand what each primitive is
for, see a realistic default composition, or compare the same teaching across
the active runtimes. These nine verdicts are part of the full 175-component
Svelte release denominator and must be honest before the visual-conformance
lane begins.

# 3. Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Planning checkout: `main`
- Planning base: `291af60a19fb12b67934d5803568909fb652a76c`
- Pushed-main verification: local `HEAD` and `origin/main` matched that SHA
  before this handoff commit; the planning checkout was clean.
- Posture: `strict-ready`.
- Planning state: `g15.039` is complete; `g15.030` is the single next ready
  child; `g15.031`–`g15.033` remain serial behind it.
- Worker branch: `t3code/g15-030-review-foundation-layout`
- Worker worktree: use the clean, registered, dedicated non-`main` worktree
  supplied by the launcher. Do not create another when that condition holds.
- Manual worktree creation command: none. If the launcher context is invalid,
  stop and report it; do not guess a path or use `/tmp`.
- Ready card: `docs/roadmaps/g15/030-review-foundation-layout.md`
- Parent method: `docs/roadmaps/g15/027-screen-clear-human-review.md`
- Allowed runway: `g15.030` only.
- Work budget: one nine-page review/repair batch, one PR, then stop.
- Topology: serial. This card and later children share the catalogue audit.
- Model guidance: capable coding model, medium reasoning.
- PR target: `main` from the worker branch. The worker pushes but never merges.
- Operator live review is required for every changed Svelte or React page and
  remains pending until the orchestrator opens the paired PR previews.

Canonical context:

- `AGENTS.md`
- `.agents/skills/effigy/SKILL.md`
- `docs/contracts/001-working-rules.md`
- `docs/roadmaps/g15/030-review-foundation-layout.md`
- `docs/roadmaps/g15/027-screen-clear-human-review.md`
- `docs/roadmaps/g15/specimen-plan-outline.md`
- `docs/roadmaps/g15/specimen-catalogue-audit.md`
- `docs/roadmaps/g15/026-native-specimen-probe.md`
- `docs/logs/2026-08/20260820-g15-026-native-specimen-probe.md`

The open `docs/triage/20260820-205249-transitions-dev-motion-learning.md`
note is unrelated and remains open; do not pull it into this card.

# 4. Boundaries

This card owns exactly:

- `Box`
- `Grid`
- `Region`
- `ResizeHandle`
- `ScrollShell`
- `Separator`
- `Spacer`
- `Stack`
- `Surface`

Writable scope:

- those nine specimen pages across Svelte, React, and GPUI
- focused preview/specimen tests needed for changed teaching or interaction
- those nine rows and mechanically affected totals in
  `docs/roadmaps/g15/specimen-catalogue-audit.md`
- one August batch log
- root `PAPERCUTS.md` only for newly encountered, small execution friction

Out of scope:

- component implementation, public props, contracts, tokens, shared CSS,
  Rust specs/render composition, or runtime semantics
- any catalogue page outside the exact nine-page list
- exhaustive prop matrices, a `Conformance` tab, shared fixture corpus,
  schema, codegen, or generated adapter
- visual screenshot comparison, native-window execution, Jetstream, release
  work, or the open motion-learning triage note
- roadmap/card status, generation front doors, or dispatch-ledger edits; those
  remain orchestrator-owned
- merging the PR

Stop and report rather than repairing when a page exposes a component,
contract, public API, interaction, or native-runtime defect. Also stop if the
review would need a page outside this list, an executable cross-runtime
specimen representation, or windowed/native visual evidence.

# 5. Important Context

All nine rows currently read A/A/A `keep`, but that is a screen-clear grade,
not a human verdict. Record a short verdict for every page; unchanged pages
still need evidence that they were actually inspected.

Review these as documentation, not coverage matrices. `Examples` should lead
with a realistic answer to “what is this for?” and use roughly three to six
meaningful sections where the component warrants them. Sizes and densities
belong only in their dedicated panes. Captions describe user-facing meaning,
not fixture identifiers or prop combinations.

Layout primitives are especially vulnerable to empty-looking swatches and
abstract geometry. Prefer small, recognisable compositions that make the
primitive's responsibility visible:

- Box, Stack, and Grid should teach containment and arrangement, not repeat
  every gap/alignment value.
- Region, Surface, and ScrollShell should make their boundary or scrolling
  role legible through realistic content.
- Separator and Spacer need enough surrounding context to reveal their
  effect without becoming a layout showcase.
- ResizeHandle must demonstrate the intended gesture with working controls;
  a caption over inert geometry is not evidence.

Svelte is the reference. React should match its section order, copy, and
fixture meaning. GPUI may use renderer-owned composition but must teach the
same important evidence. Consume `g15.026`'s 174-route headless probe; do not
rerun or replace it with a foreground preview.

If no page needs repair, a docs-only audit/log PR is valid. Do not create churn
to justify the worker run.

# 6. Suggested Next Move

1. Read this handoff, the card, parent method, shared outline, and the nine
   existing audit rows.
2. Run the worker preflight below and confirm a clean registered non-`main`
   worktree.
3. Locate the exact Svelte, React, and GPUI specimen files for the nine pages.
4. Inspect all nine Svelte and React routes live, including applicable axis
   panes and real ResizeHandle interaction. Consume the `g15.026` native probe
   and inspect the corresponding GPUI specimen source/evidence.
5. Write a verdict for every page before changing anything. Keep passing pages
   untouched.
6. Repair only bounded specimen defects, maintaining paired section/copy
   agreement and equivalent native teaching.
7. Update the nine audit rows, mechanically recount totals only if a grade or
   disposition changes, and write
   `docs/logs/2026-08/20260820-g15-030-foundation-layout-review.md`.
8. Validate the complete batch once, push one PR, and stop for orchestrator
   review.

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

## While working

- Use Effigy task routing and supported selectors.
- Keep changes inside the exact nine-page boundary.
- Work in one coherent review/repair batch rather than incremental micro-edits.
- Report any component-semantic finding as a stop condition before touching
  component or contract code.
- Do not edit `.github/workflows/` or perform release mutations.

## Final validation

Always run:

- focused preview/specimen tests for every changed page
- `effigy catalogue:check`
- `effigy check:svelte`
- `effigy react:build`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

If GPUI specimen code changes, also run:

- `effigy check:gpui`
- `effigy regressions:native`

Headless only. Never run a `*-windowed`, `test:native-visual`, Jetstream, or
release selector.

## Handoff back

Push the worker branch and open one PR targeting `main`. Report:

- PR URL and pushed head SHA
- a verdict for all nine pages, including unchanged pages
- exact changed pages/files and why each change improves human teaching
- validation commands and outcomes
- audit grades/dispositions and any mechanically recounted totals
- the exact changed Svelte/React routes the operator must review live
- any routed stop condition or residual risk

Do not claim operator sign-off, change card or roadmap status, edit the
dispatch ledger, merge the PR, start `g15.031`, or widen the card after the PR
is opened.
