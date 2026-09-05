---
title: g15.045 GPUI offscreen capture adoption worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-21
updated: 2026-08-21
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260821-215028-g15-045-gpui-offscreen-capture-adoption.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, gpui, offscreen, capture]
---

## What This Thread Is Doing

Execute `g15.045`: adopt the exact GPUI revision proved by `g15.044`, migrate
the bounded GPUI surfaces, and retain a deterministic offscreen Button capture
command with a typed receipt. This is production adoption of the capture seam,
not the fixture inventory or comparator.

The accepted revision is
`zed-industries/zed@1ea16c1ab9dd6d36649e002dc60995634da04daf`.
The proof rendered a real Poodle Button through `poodle-render` and the GPUI
node backend with no `NSWindow`, screen capture, focus, or desktop permission.

This is one bounded implementation handoff. You do not need the originating
transcript or another prompt.

## Why It Matters

The current retained GPUI pixel gate opens a real macOS window and takes focus.
Poodle cannot build a useful cross-runtime visual comparison lane on that
foundation. `g15.044` proved the safe replacement and measured its migration;
this card makes that seam real while keeping named fixtures, tolerances, and
the optional Longhorn lab out of scope.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `7f76d0d1a234b17a40e17157ff0bd77324237d82`
- **Pushed-main verification:** local `HEAD` and `origin/main` matched that SHA
  before this handoff was written; the planning checkout was clean.
- **Posture:** `strict-ready` implementation lane.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Worker branch:** `t3code/gpui-offscreen-capture-adoption`
- **Worker worktree:** use the clean, registered, dedicated non-`main`
  worktree supplied by the launcher, regardless of generated path or branch
  name.
- **Worktree creation command:** none. The launcher owns normal creation. A
  manual fallback may only use the operator-selected
  `AGENTS_WORKTREE_CONTAINER_DIR` from ignored `.agents.local.env`.
- **Ready card:**
  `docs/roadmaps/g15/045-gpui-offscreen-capture-adoption.md`
- **Parent:** `docs/roadmaps/g15/012-visual-conformance-lane.md`
- **Accepted proof:**
  `docs/research/gpui-offscreen-capture-feasibility.md` and
  `docs/logs/2026-08/assets/g15-044/reproduce.sh`
- **Allowed runway:** `g15.045` only. Do not start `g15.046` or `g15.047`.
- **Remaining budget:** exact dependency migration, one bounded capture target
  and selector, focused evidence, one August batch log, and one PR; then stop.
- **Parallel safety:** this lane owns GPUI manifests, the tracked preview lock,
  the exact migration files, capture tooling, and one Effigy selector. Do not
  overlap release workflows, the remaining specimen audit, web packages, or
  another worker's closeout.
- **Canonical refs:** `AGENTS.md`, `.agents/skills/effigy/SKILL.md`,
  `docs/architecture/001-poodle-system-shape.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/005-agent-local-paths.md`,
  `docs/specs/022-packaging-versioning-and-release-channel-rules.md`, and the
  parent/card/proof above.
- **Model capability profile:** frontier coding model, high reasoning.
- **Tool/runtime restrictions:** all capture and validation is headless. Never
  run `*-windowed`, `test:native-visual`, `native-visual:*`, Jetstream,
  release, or workflow selectors. Do not open a window to test that none opens.
- **Required validation:** the new offscreen smoke selector,
  `effigy regressions:native`, `effigy probe:gpui-specimens`,
  `effigy check:gpui`, `effigy ci:native`, `effigy docs:check`, the Rust 1.95
  checks below, and `git diff --check origin/main...HEAD`.
- **PR base/head:** `main` <- worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation and orchestrator review.
- **Merge authorisation:** absent. Push the PR and stop for review.

The motion-learning and Longhorn-lab triage notes remain open and are not this
card's work. Do not run `effigy doctor`; the execution route is explicit.

## Boundaries

Adopt exactly what the proof measured:

- Replace the crates.io `gpui = "0.2.2"` dependencies in the GPUI node backend
  and preview with the immutable Zed git revision above.
- Add `gpui_platform` as the preview's one new normal dependency, pinned to the
  same revision and with `font-kit` enabled. Upstream application construction
  now comes from `gpui_platform::application()`; omitting `font-kit` silently
  leaves the macOS platform without real text rendering.
- Keep GPUI `test-support` non-default. It may be enabled by dev tests and the
  dedicated capture feature/target, never by an ordinary
  `cargo build --bin poodle-preview`.
- Apply the measured 8 + 6 + 3 mechanical migration only. The retained script
  is the executable patch inventory; do not reinterpret it as permission for a
  broader refactor.
- Use the new `BoxShadow.inset` capability truthfully: project
  `shadow_layers[*].inset` instead of filtering inset layers out. Ordinary
  fallback shadows stay `inset: false`. Remove the obsolete approximation and
  add focused evidence for both inset and non-inset projection.
- Add one dedicated offscreen capture target and one Effigy selector. It must
  render a real Button through `ButtonSpec` → `poodle_render::button` →
  `poodle_gpui_node_backend::to_gpui` → GPUI screenshot readback.
- Keep the capture smoke internal. It is not a public component API, fixture
  namespace, baseline, or portable scene representation.

Writable scope:

- `packages/gpui/node-backend/Cargo.toml` and the exact measured migration
  files under `packages/gpui/node-backend/src/`;
- `packages/gpui/preview/Cargo.toml`, its tracked `Cargo.lock`, the exact
  measured migration files, and a small owner-local offscreen capture target;
- focused GPUI/node-backend tests for the migration and capture contract;
- `tasks/effigy.tasks.toml` for one clearly named headless selector;
- `docs/roadmaps/g15/release-gap-register.md` only to replace the adoption gap
  with landed evidence;
- one August g15.045 batch log;
- `PAPERCUTS.md` only for new small execution friction.

Out of scope:

- named component fixtures, fixture inventories, visual baselines, image
  comparison, tolerances, reports, or cross-runtime completion claims;
- Svelte, React, Jetstream, Longhorn, a Tauri lab, IPC, process pooling, or a
  long-running sidecar;
- deleting or running the old windowed scripts and selectors;
- `.github/workflows/`, release files, package versions, or release mutation;
- changing the public component contracts or introducing a compatibility shim;
- changing roadmap/card status, the generation front door, dispatch ledger, or
  merging the PR.

Stop on any card stop condition. Also stop if the exact pin requires a renderer
or component redesign, if normal preview builds need `test-support`, if the
capture path creates an `NSWindow` or subprocess, or if Rust 1.95 no longer
works. Do not raise public `rust-version` metadata without a new operator
decision.

## Capture Command Contract

Use the smallest CLI that can support later fixture work without inventing it
now. Require explicit output PNG and receipt paths, logical viewport width and
height, theme, control size, and scale. For this card:

- accepted scale is exactly `2.0`; any other value fails with a clear error;
- accepted themes and control sizes use the existing Poodle domains and reject
  unknown values rather than silently defaulting;
- output device dimensions equal logical dimensions × 2;
- missing PNG or receipt is failure;
- the receipt is typed Rust data serialized as JSON and includes a versioned
  schema, component smoke identity, immutable GPUI revision, renderer/platform,
  theme, control size, logical viewport, scale, device dimensions, and PNG
  SHA-256;
- do not include timestamps or machine-specific paths in the stable identity;
- the selector captures identical input repeatedly in one ordinary worktree,
  asserts one hash, checks the receipt against the PNG, and uses a temporary
  output directory rather than writing a baseline into the repository;
- unsupported OS or missing Metal is an explicit failure, never a green skip.

A one-shot command is enough. Later lab work owns process lifecycle and IPC.

## Important Context

- `g15.044` independently reproduced 17 mechanical errors across 9 files:
  node backend 8, preview bin 6, headless tests 3. The migrated copy passed
  56/56 headless regressions.
- The orchestrator then reran the complete proof from a clean detached checkout
  with `RUSTUP_TOOLCHAIN=1.95.0`. Production baseline, migration, preview,
  tests, 56/56 regressions, Metal capture, 10 identical hashes, dimensions, and
  viewport checks all passed. Preserve Poodle's declared Rust 1.95 floor.
- The upstream revision itself pins Rust 1.97.1, but that is not Poodle's
  measured minimum. Do not copy that number into public package metadata.
- `gpui_platform` has no default features. Normal preview application
  construction needs its `font-kit` feature; capture additionally needs
  `test-support`.
- `gpui/test-support` pulls test-only dependencies. A required feature on the
  capture binary is appropriate; enabling it by default is not.
- Scale is hardcoded to 2.0 in upstream `TestWindow`. This is accepted for the
  first lane. Do not add a local platform shim.
- Cross-machine byte identity is not claimed. Core Text and Metal can vary by
  machine; renderer-aware tolerance belongs to `g15.047`.
- The existing windowed capture remains local-only historical tooling until a
  later cleanup card. Adoption does not need to touch or run it.
- The rejected g13/g14 executable corpus remains rejected. The capture target
  renders one smoke primitive and carries no shared behavior authority.

Work in three meaningful chunks:

1. exact dependency/signature migration plus inset-shadow correction;
2. bounded capture target, typed receipt, selector, and focused tests;
3. full headless validation, release-note classification, gap/log closeout,
   rebase, and PR.

Report after each chunk with changed files, actual validation, remaining work,
and blockers.

## Suggested Next Move

Read this handoff from the top, then run the four-command worktree preflight
below before broad reads. Read the card, proof note, reproduction script, and
package-intent spec. Use `effigy tasks` to confirm current selector names.

Start by applying the exact manifest and source migration from
`reproduce.sh`. Keep it mechanical, but correct the proof patch's placeholder
`BoxShadow.inset: false` treatment to preserve real inset layers. Prove the
ordinary preview binary still builds without `test-support` before adding the
capture target.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before any
   broad read, run only:
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
   `git merge-base --is-ancestor 7f76d0d1a234b17a40e17157ff0bd77324237d82 HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `.agents/skills/effigy/SKILL.md`, the g15 milestone,
   parent, card, g15.044 research/log/script, system shape, working/local-path
   contracts, package-intent spec, and the headless-pixels gap row.
6. Do not run `effigy doctor`, broad visual selectors, or any windowed,
   Jetstream, release, or workflow path.

### While you work

- Keep the adopted revision identical in every manifest and receipt constant.
- Keep the normal preview feature graph free of `test-support`; verify rather
  than infer it.
- Preserve Rust 1.95. Run the focused build, regression, and capture route with
  `RUSTUP_TOOLCHAIN=1.95.0` before handoff.
- Use the existing Poodle theme and control-size definitions. Do not duplicate
  their domains in a second authority without drift evidence.
- Make invalid CLI input fail before renderer construction or output writes.
- Write PNG and receipt atomically enough that an interrupted or failed capture
  cannot leave a matching-looking success pair.
- Drive real offscreen readback. A blank image, synthetic PNG, or node-tree
  serialization is not acceptance evidence.
- Do not retain generated captures or temp directories in Git.
- Append a PAPERCUTS entry only for new small execution friction.
- Stop and report any scope expansion, public break, MSRV drift, or validation
  failure that changes the plan.

### When the assigned runway is complete

1. Run the new offscreen smoke selector twice from an ordinary worktree and
   retain the hashes/results in the batch log, not the generated images.
2. Run the selector's negative cases: unsupported scale, invalid theme/control
   size, missing output, and receipt/PNG mismatch detection.
3. Run `effigy regressions:native`, `effigy probe:gpui-specimens`,
   `effigy check:gpui`, `effigy ci:native`, and `effigy docs:check`.
4. Run the focused migrated build, headless regressions, and capture selector
   under `RUSTUP_TOOLCHAIN=1.95.0`.
5. Finish with `git diff --check origin/main...HEAD`. Confirm the ordinary
   preview bin builds without the capture feature and no generated output,
   baseline, workflow, or sibling file entered the diff.
6. In the batch log, satisfy spec 022 explicitly: name changed packages,
   public-entry-point impact, additive/behavioral/breaking class, dependency and
   MSRV result, and downstream checks.
7. Update only the release-gap evidence row. Workers do not mark cards or the
   milestone complete.
8. Rebase onto current `main`, rerun affected checks, push one branch, and open
   one PR.
9. In the PR body, link g15.045, g15.044 proof, the batch log, exact dependency
   revision, receipt schema, changed packages, all validations, and remaining
   environmental constraints.
10. Report the PR URL and stop. Do not merge.

### Review and merge path

The orchestrator will inspect the dependency graph, ordinary preview feature
set, inset-shadow semantics, capture/receipt failure modes, Rust 1.95 evidence,
and headless validation independently. Because worker and orchestrator may
share a GitHub identity, the verdict may be a PR comment rather than a formal
approval.

If changes are requested, make only those changes on this branch, push again,
and report back. Merge requires explicit operator authorisation.

- **Requested changes:** none yet.
- **Closeout refs:** `docs/roadmaps/g15/045-gpui-offscreen-capture-adoption.md`,
  `docs/roadmaps/g15/012-visual-conformance-lane.md`,
  `docs/roadmaps/g15/release-gap-register.md`, the g15.045 August log,
  `docs/roadmaps/g15/README.md`, `docs/roadmaps/generation-index.md`, and
  `docs/roadmaps/dispatch.md`.

### Handoff closeout

The worker owns the exact GPUI migration, capture target/selector, focused
evidence, release-gap row, and batch log. The orchestrator owns review, merge,
roadmap status, and promotion of `g15.046`. Leave the lane open if any capture,
receipt, feature-isolation, MSRV, or required validation evidence remains.
