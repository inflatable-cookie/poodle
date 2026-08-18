---
title: g15.017 web specimen axis placement worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-18
updated: 2026-08-18
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260818-162914-g15-017-web-specimen-axis-placement.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, specimens, svelte, react]
---

## What This Thread Is Doing

Poodle's catalogue still advertises axis tabs that are empty or belong to a
child component, while 20 pages omit size or density evidence their public
component props support. This worker owns g15.017 only: correct the exact 24
paired web routes and make the preview helper unable to advertise empty axis
tabs.

Start from this file without a copied transcript or second prompt.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `6103ddc2202ebbc2d758049c2b1ac64efb460581`
- **Pushed-main verification:** local `HEAD` and `origin/main` matched that
  commit before this handoff was created
- **Planning checkout:** clean `main`; implementation edits are forbidden there
- **Worker branch placeholder:** `t3code/g15-017-specimen-axis-placement`
- **Worker worktree:** launcher-managed. If the launcher did not supply a clean
  registered non-`main` worktree, stop and ask the operator. Do not create one
  beside the repository or under a guessed temporary path.
- **Ready card:** `docs/roadmaps/g15/017-specimen-axis-placement.md`
- **Allowed runway:** g15.017 only
- **Remaining card budget:** one card
- **Dispatch topology:** serial against g15.019 and every other specimen edit
- **Canonical refs:** `docs/roadmaps/g15/specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/specimen-plan-outline.md` (Axis Eligibility), and
  `docs/contracts/001-working-rules.md`
- **Known doctor baseline:** existing generated-in-src, god-file,
  stale-suppression, stale-graph, and comment-ratio findings. Record them; do
  not absorb them.
- **Tool restrictions:** never run a `*-windowed` selector. Do not run GPUI,
  Jetstream, native visual, release, or deployment tasks.
- **PR base/head:** `main` <- selected worker branch
- **Merge authorisation:** none. Push one PR and stop.

## Merged-State Correction

The audit predates PR #38. Do not follow its old `12 audio + ~22 others`
estimate mechanically.

PR #38 already moved the twelve React audio matrices into `SpecimenLayout`,
kept the Svelte layouts aligned, and gave `ListCardCounter` a dedicated page.
Those twelve audio pages are validation-only here, and `ListCardCounter` is no
longer a spurious-axis target. Native `audio_specimens` remains combined, but
g15.019 now owns its split and consumer migration in one change.

## Exact Route Scope

Add both eligible axes on these 15 paired pages:

`ConfirmAction`, `SplitView`, `ToastHost`, `AlertDialog`, `Dialog`, `Drawer`,
`FormDialog`, `BlockEditor`, `LogList`, `VideoPlayer`, `LicenceActivation`,
`LicenceSeats`, `LicenceStatus`, `UpdateCenter`, `UpdateStatus`.

Add size only on `Eyebrow` and `Text`.

Add density only on `IconButton`, `Icon`, and `UiPresentationProvider`.
`IconButton` and `Icon` already have size evidence; preserve it.

Remove ineligible axes from these four paired pages:

- `Avatar`: size stays, density goes. Change the authored specimen model and
  regenerate all outputs. Make both web `SceneSpecimen` renderers obey the
  scene's declared `tabs`.
- `Tooltip`: remove both matrices; they currently vary the child Button.
- `PickerShell`: remove density; provider density is not a PickerShell prop.
- `MeterSurface`: remove both empty tabs.

## Required Implementation Shape

- Each eligible pane shows one representative per step: `xs`, `sm`, `md`,
  `lg`, `xl`; `compact`, `default`, `comfortable`.
- Do not copy the Examples matrix. Pick one stable, ordinary state and vary
  only the relevant axis.
- Harden both preview-local `SpecimenLayout` helpers so an axis renderer must
  exist before its tab can appear. Existing `showSizes` / `showDensities`
  inputs may hide a supplied renderer; they must not force an empty pane.
- Keep Svelte and React tab set, order, representative state, labels, and copy
  paired.
- For Avatar, edit the authored source under `packages/codegen`, regenerate,
  and check drift. Never hand-edit generated files.
- Preserve all existing Examples content. Do not shorten long pages, reword
  teaching copy, or fix unrelated interactions.
- Do not touch `poodle-render`, GPUI, or Jetstream. Do not add a temporary
  native return type or compatibility path.

## Evidence and Validation

Add focused evidence for:

- callback-less `SpecimenLayout` showing only Examples
- one supplied renderer exposing only its matching tab
- explicit `show*={false}` hiding an otherwise supplied renderer
- the 24 corrected route decisions and Svelte/React agreement
- authored-scene tab projection and Avatar's size-only result
- the twelve PR #38 audio pages retaining paired, populated axis tabs outside
  Examples

Then run, through the repo-local Effigy skill:

- focused helper, scene, and paired-axis tests
- the full 175-route web axis census
- `effigy ir:build` and `effigy ir:check`
- `effigy check:svelte`
- `effigy react:build`
- `effigy catalogue:check`
- `effigy ci:web`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Record exact commands, counts, and outcomes in one batch log under
`docs/logs/2026-08/`. The PR must leave live review of the 24 changed routes as
an explicit operator acceptance item.

## Stop and Return

Stop and report instead of expanding scope if:

- a component's public props disagree with the card's axis classification
- representative evidence requires changing component behaviour or contract
  semantics
- an authored-scene change cannot be made through its source and generator
- completing the work requires any native or Jetstream edit
- another active worker overlaps these web specimens or preview helpers

## Finish

Commit intentional batches, push the worker branch, open one PR against
`main`, and return the PR URL, head SHA, validation evidence, live-review list,
and any deviations. Do not merge and do not edit roadmap status or
`docs/roadmaps/dispatch.md`.
