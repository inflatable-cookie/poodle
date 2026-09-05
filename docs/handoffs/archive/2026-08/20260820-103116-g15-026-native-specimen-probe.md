---
title: g15.026 headless native specimen probe worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-20
updated: 2026-08-20
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260820-103116-g15-026-native-specimen-probe.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, gpui, specimens, headless]
---

## What This Thread Was Doing

Poodle's human-centred catalogue audit measured every Svelte and React page
live, but its GPUI grades remained source-based and provisional. This worker
closes that native construction gap with one bounded headless probe over the
174 portable catalogue routes.

The original card proposed turning the GPUI preview into a library. Readiness
review found a smaller seam already in the binary: `main.rs` unit tests can
mount the production `PreviewRoot` on GPUI's in-memory test platform. Use that.
Do not create a `lib.rs` or another conformance system.

## Why It Matters

The specimen audit cannot complete while one active runtime has never mounted
its pages. Poodle also cannot afford a third broad parity architecture after
the g13 and g14 attempts. This card adds one durable release fact: every native
catalogue route constructs, avoids the fallback, and opens every axis pane it
advertises.

It deliberately does not claim visual parity, general component interaction,
or teaching quality. Those already have separate owners.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `ba51f3a7e2bcc0263914ebb7ee4379b64c9e6274`
- **Pushed main verification:** local `HEAD` and `origin/main` matched that
  commit before this handoff was created.
- **Planning checkout:** clean, orchestrator-owned, and unavailable for worker
  edits.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** `g15.026` is ready; the audit,
  release-gap register, generation front doors, and dispatch ledger reflect
  the bounded in-binary design.
- **Worker branch:** `t3code/g15-026-native-specimen-probe`
- **Worker worktree:** launcher-provided clean, dedicated, registered
  non-`main` worktree.
- **Worktree creation command:** none. The launcher owns normal creation. A
  manual fallback may only use the operator-selected
  `AGENTS_WORKTREE_CONTAINER_DIR` from `.agents.local.env`.
- **Active spec lane:** g15 human-centred specimen catalogue completion.
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g15/026-native-specimen-probe.md`
- **Allowed runway:** `g15.026` only.
- **Remaining card budget:** one card, one batch log, one PR, then stop.
- **Dispatch topology:** serial. The six screen-clear review children follow
  after this PR merges; `g15.012` and release certification remain later.
- **Parallel safety check:** this lane owns only the GPUI preview's private
  test observation, its Effigy task composition, the audit update, and one
  batch log. Stop if another active worker touches those surfaces.
- **Canonical refs:** `docs/contracts/001-working-rules.md`,
  `docs/roadmaps/g15/011-specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/019-gpui-specimen-structure.md`, and
  `docs/roadmaps/g15/034-component-specific-specimen-axis-domains.md`.
- **Model capability profile:** capable coding model, medium reasoning.
- **Tool/runtime restrictions:** headless only. Never run `*-windowed`,
  `test:native-visual`, browser, Jetstream, or release selectors.
- **Required validation:** `effigy probe:gpui-specimens`,
  `effigy ci:conformance`, `effigy check:gpui`,
  `effigy regressions:native`, `effigy catalogue:check`,
  `effigy docs:check`, and `git diff --check origin/main...HEAD`.
- **PR base/head:** `main` <- worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation and orchestrator review. No live
  window checkpoint is part of this card.
- **Merge authorisation:** absent. Push the PR and stop for review.

## Boundaries

Keep this run to one construction probe:

- mount the production `PreviewRoot` through `TestAppContext`;
- iterate `component_registry::CANONICAL_COMPONENTS` directly;
- distinguish the real specimen card and fallback with test-only GPUI debug
  selectors;
- click every rendered `Sizes` or `Densities` tab through pointer input and
  assert its pane paints;
- add the named selector and compose it into the existing headless native
  boards;
- replace the audit's provisional render claim with exact measured evidence.

Out of scope: `lib.rs`, public preview APIs, generated inventories, shared
fixtures, screenshots, pixel comparison, accessibility dumps, arbitrary
component interactions, caption extraction, overflow grading, component or
contract repairs, Jetstream, workflows, and release mutation.

If a route fails because of real component behaviour rather than construction
or axis-tab navigation, record the slug and stop for an orchestrator decision.
The known Stepper selection/re-run gap and UiPresentationProvider cascade gap
remain separate release blockers; do not absorb them here.

Work only in the selected worker worktree. Never edit, clean, reset, or stash
over the orchestrator's planning checkout. Do not edit
`docs/roadmaps/dispatch.md` or change roadmap/card status. Do not merge the
PR.

## Important Context

- The release denominator is 175 web entries: 174 portable routes plus the
  web-only `MeterSurface`. The probe must assert 174, not infer it from a new
  list.
- `packages/gpui/preview/src/main.rs` already contains `file_pick_tests`, which
  demonstrates the accepted `TestAppContext::single()` / `add_window_view`
  shape. Follow that private in-binary pattern.
- `packages/gpui/preview/src/specimens/mod.rs` owns both `specimen_card` and
  `missing_specimen`; `specimen_layout.rs` owns explicit axis admission and
  tab state.
- GPUI's `debug_selector` is available under `test-support` and is a no-op in
  ordinary builds. Prefer it over production IDs or a new result schema.
- The probe should run at 768px wide to exercise route construction at the
  audit's narrow width. It does not prove that content avoids horizontal
  overflow.
- Discover admitted native tabs from the mounted page. Do not duplicate the
  web axis census: `g15.019` and `g15.034` already own eligibility and exact
  domains.
- `effigy regressions:native` remains the component-interaction board. The new
  probe is page construction plus specimen-shell navigation only.
- Add `probe:gpui-specimens` as a binary test selector and compose it into
  `ci:conformance` and `ci:native`. The former name is legacy; update its
  comment without making a cross-runtime conformance claim.
- The existing GitHub workflow already calls `effigy ci:conformance`; editing
  `.github/workflows/` is neither necessary nor authorised.
- Record the test-body wall time. If it exceeds two minutes after compilation,
  stop rather than burying a slow sweep in QA.
- The GPUI preview lockfile is intentionally tracked after PR #49. Preserve
  it; do not regenerate or remove it without a concrete dependency change.

Report after the minimal seam works on an ordinary page, an axis page, and the
fallback sentinel. Report again when the 174-route sweep and validation are
PR-ready. Name changed files, exact counts, runtime, checks actually run, and
any failing slugs.

## Suggested Next Move

Read this handoff from the top, then run the four-command worktree preflight
below before broad repository reads. Once the worktree is accepted, read
`AGENTS.md`, the Effigy skill, `g15.026`, the audit, working rules, and the two
completed axis cards.

Start with the smallest proof: add test-only selectors, mount one ordinary
route, mount one route with an axis tab, and prove an unknown dispatch reaches
the fallback marker. Only then loop the canonical registry. This keeps any
GPUI test-platform limitation visible before the worker pays for the full
sweep.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before
   any broad read, run only:
   - `git rev-parse --show-toplevel`
   - `git branch --show-current`
   - `git status --porcelain`
   - `git worktree list --porcelain`
2. If the current root is a registered, clean, non-`main` worktree, accept it
   as the launcher-provided worktree regardless of generated path or branch
   name. Record the actual values and do not create another worktree.
3. If the launcher supplied a dirty, `main`, or unregistered context, stop and
   report it. Do not clean or reset it. A manual fallback is allowed only after
   reading `.agents.local.env`, finding a valid
   `AGENTS_WORKTREE_CONTAINER_DIR`, and creating a unique worktree there from
   `origin/main`; ask the operator if the key is absent. Never use `/tmp`,
   `TMPDIR`, or a guessed path.
4. From the accepted worktree, run `git fetch origin`, confirm `HEAD` equals
   current `origin/main`, confirm
   `git merge-base --is-ancestor ba51f3a7e2bcc0263914ebb7ee4379b64c9e6274 HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `.agents/skills/effigy/SKILL.md`, the g15 milestone,
   `g15.026`, `g15.011`, the audit, working rules, `g15.019`, and `g15.034`.
6. Use `effigy tasks` for the supported selector inventory. Do not run any
   windowed, native-visual, browser, Jetstream, or release path.

### While you work

- Keep all observation private to the binary test build. Add no published or
  generated API.
- Use the canonical registry as the only route list.
- Reset route-owned specimen state between entries and include the active slug
  in every failure.
- Drive axis tabs through real GPUI pointer input; do not call their handlers.
- Keep component interactions on the existing focused regression board.
- Append evidence to one new August `g15.026` batch log.
- Work in coherent chunks and commit meaningful results rather than model
  turns.
- Report through the operator after the three-case seam proof and at PR-ready
  state.
- Stop on any condition listed by the card.

### When the assigned runway is complete

1. Run every required selector named in Current State. Finish with
   `git diff --check origin/main...HEAD`.
2. Ensure the batch log records 174/174 routes, `MeterSurface` as the one
   native `n/a`, exact size/density tab counts, failures, test-body wall time,
   changed audit claims, changed files, and validation outcomes.
3. Confirm `ci:conformance` and `ci:native` remain headless and that no workflow
   file changed.
4. Push the worker branch and open a reviewable PR against current `main`.
5. Link `g15.011`, `g15.026`, the updated audit, batch log, changed surfaces,
   and validation in the PR body.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect the PR independently against the card, audit,
diff, task graph, checks, and batch log. Because the orchestrator and worker
may share a GitHub identity, the verdict may be recorded as a PR comment rather
than a formal approval.

If changes are requested, make only those changes on this branch, push again,
and report back. Merge requires explicit operator authorisation after the
review and check gates pass.

- **Requested changes:** none yet.
- **Closeout refs:** `docs/roadmaps/g15/026-native-specimen-probe.md`, the
  August batch log, `docs/roadmaps/g15/011-specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/README.md`, `docs/roadmaps/generation-index.md`,
  `docs/roadmaps/g15/release-gap-register.md`, and
  `docs/roadmaps/dispatch.md`.

### Handoff closeout

The worker owns implementation evidence and the batch log. The orchestrator
owns dispatch status, roadmap status, merge, and the next card. If the probe
is blocked or finds an out-of-scope component defect, leave those surfaces
open and report the exact slug and failure instead of widening the card.
