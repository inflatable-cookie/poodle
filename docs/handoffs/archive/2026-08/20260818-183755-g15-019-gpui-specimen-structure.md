---
title: g15.019 GPUI specimen structure worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-18
updated: 2026-08-18
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260818-183755-g15-019-gpui-specimen-structure.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, specimens, gpui, rust]
---

## What This Thread Is Doing

Bring the GPUI specimen catalogue onto the merged web axis contract. This is
one native structure migration: explicit axis admission, 74 exact axis
corrections, six caption repairs, and separation of the shared Rust audio
specimens into Examples / Sizes / Densities inputs.

Start from this file without a copied transcript or second prompt.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `121bb5712b7b7f58d65eae6c9d33f13bfafc4dd5`
- **Pushed-main verification:** local `HEAD` and `origin/main` matched that
  commit before this handoff was created
- **Planning checkout:** clean `main`; implementation edits are forbidden there
- **Worker branch placeholder:** `t3code/g15-019-gpui-specimen-structure`
- **Worker worktree:** launcher-managed. Reuse it only if it is a clean,
  registered, dedicated non-`main` worktree. Otherwise stop and ask the
  operator. Do not create one beside the repository or under `/tmp`.
- **Fallback worktree rule:** manual creation is allowed only from the
  operator-selected `AGENTS_WORKTREE_CONTAINER_DIR` in ignored
  `.agents.local.env`; never guess the path
- **Ready card:** `docs/roadmaps/g15/019-gpui-specimen-structure.md`
- **Allowed runway:** g15.019 only
- **Remaining card budget:** one card
- **Dispatch topology:** serial against other GPUI specimen or
  `poodle_render::audio_specimens` work
- **Canonical refs:** `docs/roadmaps/g15/specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/specimen-plan-outline.md` (Cross-Runtime Agreement),
  `docs/contracts/001-working-rules.md`, and
  `test/parity/specimen-axis-census.test.tsx`
- **Known doctor baseline:** existing generated-in-src, god-file,
  stale-suppression, stale-graph, and comment-ratio findings. Record them; do
  not absorb them.
- **PR base/head:** `main` <- selected worker branch
- **Merge authorisation:** none. Push one PR and stop.

## Worktree Preflight

Before editing:

1. Read repository `AGENTS.md`, the Northstar skill in Orchestrator/worker PR
   mode, and `.agents/skills/effigy/SKILL.md` in full.
2. Confirm the current checkout is a registered, clean, dedicated non-`main`
   worktree. Do not edit the orchestrator's `main` checkout.
3. Fetch `origin` and verify the worktree base contains planning commit
   `121bb5712b7b7f58d65eae6c9d33f13bfafc4dd5` and the handoff file exists.
4. Verify no active worker overlaps GPUI specimen modules,
   `specimen_layout.rs`, or `packages/render/src/audio_specimens.rs`.
5. Run `effigy doctor` only if selector routing or repository health is
   ambiguous. Do not turn known baseline findings into card work.

Stop and report if any preflight condition fails.

## Exact Delivery

The roadmap contains the authoritative component lists. Treat the denominator
as fixed:

- 59 pages need both axes
- four pages need size only
- eight pages need density only
- `Avatar` and `Progress` keep size only; `Tooltip` keeps Examples only
- six named pages need captions; some overlap the axis set

That is 74 axis corrections plus six caption corrections. Do not fall back to
the audit's old `59 + 6` shorthand.

### 1. Make native axis admission explicit

Break the preview-local `specimen_layout` API once so a caller explicitly
declares which axis panes exist. A missing or unsupported renderer must not
produce a tab or an empty pane. Migrate every caller in the same change,
including the bounded scene adapter. Do not keep a default-both path, alias,
wrapper, or compatibility twin.

The merged web census is the eligibility authority. Do not infer eligibility
from what a current native page happens to render, and do not import web data
into production runtime code.

Each eligible pane contains one ordinary representative per step: `xs`, `sm`,
`md`, `lg`, `xl`; `compact`, `default`, `comfortable`. Vary only that axis.
Do not copy the Examples matrix or introduce size × density products.

### 2. Repair the named GPUI pages

Adopt the axis-aware layout on every page named by the card. Preserve the
curated Examples content except where an existing axis matrix moves into its
pane. Add honest, short captions to the six named captionless pages. GPUI may
shorten web copy, but it must teach the same representative states.

Keep renderer-owned layout, spacing, node construction, and interaction
wiring native. This is not a DOM imitation and not a shared scene/tree
project.

### 3. Split the shared audio specimen shape once

`packages/render/src/audio_specimens.rs` currently returns a combined page for
12 controls: `DragNumberField`, `AudioMeter`, `AudioSwitch`, `EnvelopeEditor`,
`Fader`, `GainReductionMeter`, `Keyboard`, `Knob`, `ModMatrixGrid`,
`ValueReadout`, `WaveformDisplay`, and `XYPad`.

Replace that internal shape with the smallest one-way API that exposes
Examples, a single requested size representative, and a single requested
density representative. GPUI consumes those parts through its axis-aware
layout. Update direct Jetstream compile consumers only as required by the
breaking shape; keep their present Examples behaviour and add no Jetstream
parity work. Do not leave the combined return API beside the new one.

## Evidence

Add the narrowest durable headless evidence available for:

- axis admission: both, size-only, density-only, and Examples-only
- invalid retained tab state normalising to Examples when the available set
  shrinks, if the helper owns retained tab state
- the shared audio split returning one representative for the requested step
- captions on the six named pages where current test seams allow it

Record an exact 74-row checked list in the batch log, grouped as the roadmap
groups it. This is structural proof only. Do not claim that native pages were
rendered live: `g15.026` owns the library seam and 174-page headless probe.

## Validation

Use `effigy tasks` to confirm selector names, then run one solid validation
round after the implementation batch:

- focused tests added or changed by this card
- `effigy check:gpui`
- `effigy regressions:native`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Record exact commands, counts, outcomes, and any baseline warnings in one
batch log under `docs/logs/2026-08/`.

Hard restrictions:

- never run any `*-windowed` selector
- never run `test:native-visual`
- never run a Jetstream selector or create a sibling Jetstream symlink
- do not build a page probe, take native screenshots, or run conformance;
  `g15.026` and `g15.012` own those later lanes
- do not edit `.github/workflows/` or perform release mutations

## Writable Scope

- `packages/gpui/preview/src/specimens/*`
- `packages/render/src/audio_specimens.rs`
- direct GPUI/Jetstream compile consumers required by the one breaking audio
  specimen shape migration; no Jetstream feature or parity work
- focused tests for the preview-local axis decision and shared audio split
- one batch log under `docs/logs/2026-08/`
- append-only `PAPERCUTS.md` only for small execution friction encountered

Do not edit roadmap status, `docs/roadmaps/dispatch.md`, this handoff, or
unrelated component implementation/contracts.

## Stop and Return

Stop and report instead of expanding scope if:

- a component's native spec cannot express the merged web axis contract
- an axis example exposes a real component API or renderer defect
- the audio split requires a public package API or a compatibility bridge
- a caption or representative state requires changing component behaviour
- proof requires the future page-construction seam or any focused window
- another active worker overlaps the writable surfaces

## Finish

Commit meaningful batches, push the worker branch, open one PR against
`main`, and return the PR URL, head SHA, exact validation evidence, the
74-axis/six-caption checklist result, and any deviations or stop findings.
Do not merge.
