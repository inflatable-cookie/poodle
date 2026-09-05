---
title: g15.011 specimen catalogue audit worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-17
updated: 2026-08-17
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260817-214451-g15-011-specimen-catalogue-audit.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, catalogue, specimens]
---

## What This Thread Was Doing

Poodle has finished the measured implementation and GPUI specimen gaps for
the v0.2.0 active cohort. The next problem is now visible in the previews:
many catalogue pages are poor documentation. Some still carry exhaustive
conformance-era matrices, some duplicate size and density examples in the
main view, and some runtimes teach materially different things.

This worker owns g15.011 only. Audit the complete frozen 175-component
catalogue, prove the human-centred standard on Button, RangeSlider, and Tabs,
then turn the remaining findings into a small set of bounded curation cards.
Start from this file without a copied transcript or second prompt.

## Why It Matters

The catalogue is the way maintainers and consumers understand Poodle's
surface. A green test suite does not help if the preview is noisy, misleading,
or impossible to navigate. v0.2.0 needs a complete Svelte roster and a useful
reference experience, while preserving the hard lesson from g13 and g14: the
specimen layer must not become another universal component language or an
exhaustive conformance report.

The three pilots are deliberately representative. Button exposes variant,
size, and density duplication; RangeSlider combines state and interaction;
Tabs is itself a documentation/navigation component and was not made good
merely by restoring its pre-conformance page.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `10c53abce26275b4eb79899e7f9b0daf7b924990`
- **Pushed-main verification:** local `HEAD` and `origin/main` both equalled
  the planning base before this handoff was created
- **Planning checkout:** clean `main`; implementation edits are forbidden there
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Worker branch placeholder:** `t3code/g15-011-specimen-catalogue-audit`
- **Worker worktree:** launcher-managed. No manual path is authorised in this
  handoff; use the clean registered non-`main` worktree supplied by T3 Code
- **Worktree creation command:** none. If the launcher did not supply a usable
  worktree, stop and ask the operator; `.agents.local.env` did not provide a
  manual worktree container at dispatch time
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready card:** `docs/roadmaps/g15/011-specimen-catalogue-audit.md`
- **Allowed runway:** g15.011 only
- **Remaining card budget:** one audit/pilot card, completed in meaningful
  audit, pilot-review, and closeout chunks
- **Dispatch topology:** serial. g15.012 follows only after this audit defines
  the remaining catalogue work and the orchestrator reconciles the runway
- **Parallel safety:** no second worker owns catalogue specimen files,
  g15 curation cards, or the audit records in this handoff
- **Canonical refs:** `AGENTS.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/architecture/003-component-docs-ia-and-implementation-substrates.md`,
  `docs/roadmaps/g14/026-human-centred-specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/release-baseline-roster.md`
- **Audit output:** `docs/roadmaps/g15/specimen-catalogue-audit.md`
- **Shared outline output:**
  `docs/roadmaps/g15/specimen-plan-outline.md`
- **Pilot components:** Button, RangeSlider, Tabs in Svelte, React, and GPUI
- **Teaching authority:** Svelte is the reference implementation, component
  contracts define behaviour, and the approved page should answer what the
  component is for, what is normally available, and how it is composed
- **Fixed grade vocabulary:** A ready, B usable, C curate, D missing/broken;
  dispositions are keep, pilot-fix, curation-tranche, or
  contract/runtime-blocker
- **Operator review:** required for the three live pilot pages. Do not call
  them approved merely because checks pass
- **Model capability profile:** capable coding model, medium reasoning; pause
  for frontier/orchestrator review on architecture or public API questions
- **Tool/runtime restrictions:** never run a `*-windowed` selector,
  `test:native-visual`, `qa:jetstream`, or any Jetstream selector. Live Svelte
  and React previews are required for operator review; native validation stays
  headless
- **Required validation:** focused pilot tests, `effigy catalogue:check`,
  `effigy check:svelte`, `effigy react:build`, `effigy check:gpui`,
  `effigy docs:check`, one final headless `effigy qa`, and
  `git diff --check origin/main...HEAD`
- **PR base/head:** `main` <- selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker delivery and operator pilot review
- **Merge authorisation:** none. Push a PR and stop for orchestrator review

## Boundaries

- Keep the run inside g15.011. Do not begin g15.012 visual conformance or
  g15.013 release certification.
- Audit every one of the 175 generated catalogue entries. Record separate
  Svelte, React, and GPUI grades, named drift, and one disposition. Missing or
  broken pages receive a D; they are not omitted.
- Change only the Button, RangeSlider, and Tabs pilot pages. The remaining 172
  components are inventory and future-card inputs, not implementation scope.
- Keep `Examples` curated: default use, meaningful variants, important states,
  and composition. Sizes and densities belong only in their dedicated tabs
  where those axes apply.
- Do not add a `Conformance` tab or project focused-test fixtures into the
  catalogue. Confirm the rejected projection wiring is absent.
- The renderer-neutral specimen plan is an outline-level planning document.
  It may name ordered tabs, sections, captions, example IDs, fixture
  references, and axis eligibility. It must not become a schema, codegen
  target, generated adapter, render tree, callback format, or runtime API.
- Runtime specimens continue to render real local components. A complex page
  may keep a bounded renderer-owned adapter rather than forcing detail into the
  shared outline.
- Do not change component public APIs, component semantics, contracts,
  dependencies, workflows, task definitions, release machinery, or
  Jetstream runtime code. If the audit exposes a real semantic defect, grade
  and route it as a blocker instead of fixing it here.
- Draft future curation cards from evidence, grouped into meaningful reviewable
  families. Use the next unused g15 card IDs, mark them planned and
  orchestrator-review-required, and do not edit generation status or dispatch.
- Work only in the selected worker worktree. Never edit, clean, reset, stash,
  or remove the orchestrator checkout or another worker's checkout.
- Do not merge the PR.

## Important Context

- g14.008 rejected executable conformance as cross-runtime component
  authority. g14.021 removed its projection from the catalogue. This work must
  not rebuild that mechanism with softer names.
- g14.026 preserved the useful part: a human-centred audit rubric and the
  possibility of sharing only page outline metadata. g15.011 intentionally
  tightens that possibility to a documentation artifact. No codegen is in
  scope.
- The operator's key preference is simple: specimen pages teach humans. They
  should not show every possible variant in the main view, especially when
  Sizes and Densities tabs already exist. An exhaustive reference view may be
  considered later, but it is not this card and never replaces Examples.
- Svelte and React should use the same structure and copy. GPUI should teach
  the same component and representative states using native composition; it
  need not copy web layout mechanics.
- Audit narrow-layout behaviour and dead specimen controls explicitly. A page
  that looks plausible but contains inert buttons is not ready.
- The audit table is a decision surface, not a source dump. Keep each row
  compact enough that totals and family patterns remain visible.
- Report after three meaningful chunks:
  1. the complete 175-row inventory, grade totals, and proposed curation
     tranche boundaries;
  2. the first Button/RangeSlider/Tabs pilot pass running in live Svelte and
     React previews, with GPUI teaching structure aligned, then pause for
     operator feedback;
  3. applied feedback, final outline/cards/log, validation, pushed SHA, and PR.
- Report to the operator, who will relay material planning questions and the
  PR to the orchestrator.

## Suggested Next Move

Read this handoff from the top, then run the worker startup safety probe before
any broad repository reads. Once the worktree is accepted, read the g15.011
card, the carried g14.026 rubric, the working rules' Catalogue Specimens
section, and the frozen roster.

Build the audit table before editing pilot pages. Derive the 175 names from the
frozen roster/generated catalogue rather than a hand-written list, inspect all
three active preview implementations, and record compact evidence for each
grade. Use the resulting defect patterns to propose a small number of curation
tranches.

Then rework the three pilots as one coherent teaching-system pass. Start the
Svelte and React previews and tell the operator exactly which routes to open.
Keep the worker thread alive while the operator reviews them. Apply that
feedback before final closeout; if review is explicitly deferred, name it as a
pending PR acceptance item rather than silently checking the box.

## Completion Protocol

### Before starting

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` fields activate worker mode. Run one
   read-only probe before broad reads:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record
   its actual path and branch. Do not compare them with the branch placeholder
   or create another worktree because the names differ.
3. If the launcher supplied `main`, a dirty checkout, an unregistered path, or
   another unusable context, stop and report it. Do not create a hidden second
   worktree, use `/tmp`, guess a path, or clean/reset/stash the supplied state.
4. From the selected worktree, fetch origin. Confirm
   `git rev-parse HEAD` equals `git rev-parse origin/main`, confirm
   `git merge-base --is-ancestor 10c53abce26275b4eb79899e7f9b0daf7b924990 HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, the repo-local Effigy skill, g15 README, g15.011, the
   canonical refs above, then the pilot contracts and all three runtime
   specimen implementations.
6. Run `effigy tasks` for selector routing. Record the doctor baseline but do
   not turn its known generated-file/god-file/stale-suppression scan findings
   into this card.

### While working

- Work in three coherent chunks: audit, reviewed pilots, closeout. Do not run
  the full QA board after each individual page.
- Keep the audit mechanically complete and the judgments human-readable.
- Add focused tests only for pilot interaction or structure changed by this
  card. Do not use specimen screenshots as parity tests.
- After the first complete pilot pass, start the live Svelte and React
  previews, report their routes, and pause for operator review.
- Record small execution friction in `PAPERCUTS.md`. Stop on public API,
  contract, architecture, security, release, or cross-lane scope changes.

### When the assigned runway is complete

1. Apply the operator's pilot feedback or record explicit deferred approval.
2. Finish the audit, outline, planned curation cards, and one August batch log.
   The log must record grade totals, named pilot changes, source-cost evidence,
   operator-review state, and exact selectors run.
3. Run the required final validation:
   - focused pilot component/preview tests
   - `effigy catalogue:check`
   - `effigy check:svelte`
   - `effigy react:build`
   - `effigy check:gpui`
   - `effigy docs:check`
   - one final headless `effigy qa`
   - `git diff --check origin/main...HEAD`
4. Fetch and rebase onto current `origin/main`, then rerun the affected final
   checks on the rebased head.
5. Push the selected worker branch and open a reviewable PR against `main`.
6. In the PR body, link this handoff, g15.011, the audit, outline, proposed
   cards, log, changed pilot surfaces, operator-review state, validation, and
   unresolved items.
7. Report the PR URL and pushed SHA. Do not merge.

### Review and merge path

The orchestrator will independently review audit completeness, grading
consistency, pilot page quality, runtime alignment, proposed tranche scope,
diff, and checks. Because worker and orchestrator share a GitHub identity, the
canonical verdict will be a PR comment rather than a formal self-approval.

If changes are requested, make only those changes on this branch, push again,
and report through the operator. Merge remains operator-authorised and is not
granted by this handoff.

Closeout authority remains with the orchestrator:

- `docs/roadmaps/g15/011-specimen-catalogue-audit.md`
- `docs/roadmaps/g15/README.md`
- `docs/roadmaps/generation-index.md`
- `docs/roadmaps/README.md`
- `docs/roadmaps/dispatch.md`

### Handoff closeout

Do not call g15.011 complete until every roster entry has a grade and
disposition, the three pilots have an explicit operator-review state, and the
remaining work is represented by bounded planned cards. If any of those
conditions cannot be met, leave the lane honest and report the blocker.
