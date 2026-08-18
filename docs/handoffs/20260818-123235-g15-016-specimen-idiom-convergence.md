---
title: g15.016 specimen idiom convergence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-18
updated: 2026-08-18
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260818-123235-g15-016-specimen-idiom-convergence.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, specimens, svelte, react]
---

## What This Thread Was Doing

Poodle's catalogue audit found that the web previews still use several
different caption shells. Thirteen Svelte pages hand-roll captions with
`Eyebrow`, thirteen audio pages fork the caption idiom in both web runtimes,
and SettingsShell has no example captions. Two more catalogue routes borrow a
different component's specimen entirely.

This worker owns g15.016 only. Converge those 27 divergent or missing-caption
routes on the preview-local `SpecimenGroup`, then give `ListCardCounter` and
`MetaItem` dedicated paired pages. Start from this file without a copied
transcript or second prompt.

## Why It Matters

Specimen pages are Poodle's human-facing component documentation. A shared
caption shell gives related pages consistent hierarchy and makes blank or
missing captions mechanically visible. Dedicated pages also stop the
catalogue title from naming one component while the body teaches another.

This is presentation convergence, not content curation. Axis placement is
g15.017 and overloaded Examples are g15.018 plus its children. Keeping those
boundaries intact is as important as making the 29 scoped routes consistent.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `d9f9b74bb34b76ff4d093d78234516bc92d48dd3`
- **Pushed-main verification:** local `HEAD` and `origin/main` both equalled
  that planning base before this handoff was created
- **Planning checkout:** clean `main`; implementation edits are forbidden there
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** exact scope and acceptance in
  `docs/roadmaps/g15/016-specimen-idiom-convergence.md`; corrected 29-page
  audit accounting and current runway front doors
- **Worker branch placeholder:** `t3code/g15-016-specimen-idiom-convergence`
- **Worker worktree:** launcher-managed. No manual path is authorised; use the
  clean registered non-`main` worktree supplied by T3 Code
- **Worktree creation command:** none. If the launcher did not supply a usable
  worktree, stop and ask the operator; never guess a path or use `/tmp`
- **Active spec lane:** `docs/roadmaps/g15/specimen-plan-outline.md`
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready card:** `docs/roadmaps/g15/016-specimen-idiom-convergence.md`
- **Allowed runway:** g15.016 only
- **Remaining card budget:** one card
- **Dispatch topology:** serial against g15.017 and web specimen curation.
  A separately authorised native-probe lane may run independently, but it is
  not part of this handoff
- **Parallel safety check:** do not share mutable web specimen/helper files
  with another worker; stop if an overlapping web lane is already active
- **Canonical refs:**
  `docs/architecture/003-component-docs-ia-and-implementation-substrates.md`;
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/components/list-card-counter.md`, and
  `docs/contracts/components/meta-item.md`
- **Model capability profile:** capable coding model, medium reasoning; stop
  for orchestrator review on public API, contract semantics, or architecture
- **Known doctor baseline:** pre-existing generated-in-src, god-file,
  stale-suppression, stale-graph, and comment-ratio findings. Record them; do
  not absorb them
- **Tool/runtime restrictions:** never run a `*-windowed` selector. Do not run
  GPUI, Jetstream, native visual, or release tasks; this is a paired web-preview
  batch
- **Required validation:** focused preview evidence and source census,
  `effigy check:svelte`, `effigy react:build`, `effigy catalogue:check`,
  `effigy ci:web`, `effigy docs:check`, and
  `git diff --check origin/main...HEAD`
- **PR base/head:** `main` <- selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker delivery, orchestrator review, and paired
  live operator review
- **Merge authorisation:** none. Push a PR and stop

## Boundaries

- Converge exactly these 13 bare-Eyebrow Svelte routes:
  `SplitButton`, `TriStateSwitch`, `Select`, `TextInput`, `TokenInput`,
  `TimeInput`, `TimeZoneSelect`, `Eyebrow`, `AlertDialog`, `Dialog`, `Drawer`,
  `Menu`, and `MarkdownEditor`. Replace only the caption chrome. In
  `EyebrowSpecimen`, keep `Eyebrow` instances that demonstrate the component.
- Add meaningful `SpecimenGroup` captions around the existing SettingsShell
  examples in both web runtimes. Do not add or remove examples.
- Converge exactly these 13 audio-helper routes:
  `DragNumberField`, `AudioMeter`, `AudioSwitch`, `EnvelopeEditor`, `Fader`,
  `GainReductionMeter`, `Keyboard`, `Knob`, `ModMatrixGrid`, `ValueReadout`,
  `WaveformDisplay`, `XYPad`, and `MeterSurface`. Svelte's direct
  `<section><h3>` caption chrome and React's `AudioSpecimenGroup title=` must
  use the ordinary preview-local `SpecimenGroup label=` helper.
- Delete the forked React caption helper when no caller remains. Keep
  `AudioSpecimenPage`, row, and axis helpers if still useful; g15.017 owns axis
  placement and tab structure.
- Add paired dedicated `ListCardCounterSpecimen` and `MetaItemSpecimen` files
  and point both registries at them. Update only the specimen-documentation
  sections of their component contracts.
- Teach `ListCardCounter` in its intended ListCard footer context: a compact
  static/tooltip example and a linked counter with a visibly wired callback.
  Teach `MetaItem` with labelled, unlabelled/rich, and interactive child
  content; wire the interactive example.
- Preserve existing captions, example content, order, interactions, and
  size/density placement on the other 27 routes. Copy alignment may fix a
  literal Svelte/React wording mismatch, but do not rewrite the page.
- Do not move audio axes into tabs, change axis eligibility, shorten overloaded
  Examples, add a new shared schema, or create executable specimen authority.
- Do not change package exports, component APIs, component behaviour,
  dependencies, release machinery, `.github/workflows/`, Rust, GPUI, or
  Jetstream.
- Work only in the selected worker worktree. Never edit, clean, reset, stash,
  or remove the orchestrator checkout or another worker's checkout.
- Do not edit `docs/roadmaps/dispatch.md`, card status, generation status, or
  merge the PR.

## Important Context

- The exact defect accounting is in finding 3 and finding 8 of
  `docs/roadmaps/g15/specimen-catalogue-audit.md`. The intended page vocabulary
  is in `docs/roadmaps/g15/specimen-plan-outline.md`.
- `SpecimenGroup` is preview-local, not package API. It already carries
  `label` and optional `description` after g15.015. Reuse it rather than adding
  another compatibility prop or layout mode.
- Svelte is the reference. React should match its caption order and copy, but
  renderer-owned dense rows may remain different in mechanism.
- `ListCardCounter` currently maps to `ListCardSpecimen`; `MetaItem` maps to
  `MetaBarSpecimen`. Their dedicated pages should stay small and teach the
  child in its real composition context, not duplicate the host's full page.
- `docs/contracts/components/list-card-counter.md` currently says there is no
  standalone specimen. Reconcile that section. `MetaItem` needs equivalent
  specimen documentation. Do not change semantic contract sections.
- Add focused evidence that prevents the fork from returning: census the
  scoped caption idioms, ensure the React `AudioSpecimenGroup` is gone, and
  lock the two dedicated registry mappings. Do not write a brittle ban on all
  `<Eyebrow>` usage because the Eyebrow page legitimately demonstrates it.
- The required live sweep is read-only: do not click specimen controls while
  crawling all 175 routes. The known FileUpload chooser papercut applies to
  interaction sweeps, not a caption-only read.
- Live operator review belongs to the orchestrator review loop. Deliver the PR
  with all routes ready and list the 29 hashes; do not block the worker thread
  waiting for the operator to review them there.
- Use the repo-local Effigy skill for selector routing. Validate coherent
  batches, not each individual page.

## Suggested Next Move

Run the worker startup probe before broad reads. Then read `AGENTS.md`, the
repo-local Effigy skill, g15 README, g15.016, the two audit findings, the
specimen-plan outline, both `SpecimenGroup` helpers, the React audio helper,
and the two borrowed registry entries.

Implement three coherent batches: the 14 non-audio caption routes, the 13
audio-helper routes, then the two dedicated pages plus contract specimen docs
and focused evidence. Keep captions and ordering stable. After the batch is
coherent, run the scoped source census and paired live caption sweep, then the
aggregate web/docs gates once.

Write one batch log under `docs/logs/2026-08/`. Push the worker branch, open a
PR, and report the PR URL, pushed SHA, exact validation, caption-count sweep,
29 live-review routes, and any deviation. Leave operator visual review open in
the PR and stop for orchestrator review.

## Completion Protocol

### Before starting

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before
   broad reads run: `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. Accept the current context only if it is a registered, clean, non-`main`
   worktree. Its generated path and branch may differ from the placeholders;
   record the actual values and do not create a second worktree.
3. If the launcher supplied `main`, a dirty checkout, an unregistered checkout,
   or another unusable context, stop and report it. Do not clean it or silently
   create a fallback. A manual worktree requires the operator-selected
   `AGENTS_WORKTREE_CONTAINER_DIR` from `.agents.local.env`; never use `/tmp`,
   `TMPDIR`, or a guessed path.
4. Fetch origin. Confirm `HEAD == origin/main`, confirm
   `d9f9b74bb34b76ff4d093d78234516bc92d48dd3` is an ancestor of `HEAD`, and
   confirm this handoff file exists in `HEAD`.
5. Read the sources named in Suggested Next Move and the canonical refs above.
6. Run `effigy tasks`; record `effigy doctor` as a baseline only.

### While working

- Keep commits aligned with the three coherent batches, not model turns.
- Use `apply_patch` for hand edits. Preserve unrelated worktree state.
- Add one batch log with exact files/routes, before/after idiom counts, paired
  caption counts, live-review state, validation, and deviations.
- Record small solvable friction in `PAPERCUTS.md`; do not absorb it.
- Stop on any required package API, component semantic, dependency, workflow,
  release, native-runtime, or architecture change.

### Validate and hand off

1. Run focused preview tests and the source census covering all scoped idioms
   and the two dedicated registry mappings.
2. Run a read-only live caption sweep over all 175 Svelte and React routes.
   Record zero blank captions globally and exact matching caption counts/copy
   for the 29 scoped routes.
3. Run `effigy check:svelte`, `effigy react:build`,
   `effigy catalogue:check`, `effigy ci:web`, and `effigy docs:check`.
4. Run `git diff --check origin/main...HEAD` and confirm the worktree is clean
   after committing.
5. Leave the paired operator visual checkpoint explicitly open and provide the
   29 `#components/<slug>` routes in the PR body or log.
6. Push the worker branch and open a reviewable PR against current `main`.
   Link the card, audit, changed surfaces, batch log, evidence, validation, and
   open operator review. Report the PR URL and pushed SHA. Do not merge.

### Review and merge path

The orchestrator will inspect the PR independently and record its verdict in
the GitHub review surface. Formal self-approval may be unavailable when worker
and orchestrator share an identity; a top-level PR review comment is then the
canonical record. Requested changes are currently none. The operator must
explicitly authorise any merge.

- **Closeout refs:** g15.016 card, g15 README, generation index, dispatch
  ledger, worker batch log, and the next-task state for g15.017

### Handoff closeout

Before calling the runway complete, leave the batch log and PR evidence honest.
If the work is blocked, record the blocker and stop rather than widening into
axis placement or content curation.
