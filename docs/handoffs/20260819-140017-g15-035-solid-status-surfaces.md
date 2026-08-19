---
title: g15.035 solid status surfaces worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-19
updated: 2026-08-19
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260819-140017-g15-035-solid-status-surfaces.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, status-surfaces]
---

## What This Thread Was Doing

The operator asked for solid-color variants of Callout and
RemediationBanner while the separate application-shell specimen PR was in
review. The orchestrator settled one shared additive API —
`fill: "tint" | "solid"`, default tint — and compiled the complete
cross-runtime implementation card as `g15.035`.

The planning pass also checked the current token palette rather than assuming
raw status colors would carry inverse text. A shared 45/55 sRGB mix is now part
of the ruling because a tone-heavier mix misses normal-text contrast in Clay.
This is one bounded implementation lane; you should not need the originating
conversation.

## Why It Matters

Callout and RemediationBanner are public status surfaces used across Poodle's
consumers. A solid treatment is useful only if it remains one dependable API
across Svelte, React, shared Rust composition, and GPUI. The card protects that
parity, keeps tint behavior stable, and lands before the primitive visual lane
and v0.2.0 certification.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `eacffd6432b926c48029133aac1e39a81f746b50`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `eacffd6432b926c48029133aac1e39a81f746b50` before this handoff was created
- **Planning checkout:** clean at the planning base
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:**
  `docs/roadmaps/g15/035-solid-status-surfaces.md` is ready, and the g15/front-
  door/dispatch surfaces name the independent lane
- **Worker branch:** `t3code/g15-035-solid-status-surfaces`
- **Worker worktree:** harness-managed; expected placeholder
  `/Users/tom/.t3/worktrees/poodle/g15-035-solid-status-surfaces`
- **Worktree creation command:** the launcher should supply the worktree. Only
  if fallback is required, read `.agents.local.env` as data, validate
  `AGENTS_WORKTREE_CONTAINER_DIR`, then run `git worktree add
  <validated-container>/poodle-g15-035-solid-status-surfaces -b
  t3code/g15-035-solid-status-surfaces origin/main`
- **Worker worktree policy:** use a clean, dedicated, non-`main` registered
  worktree supplied by the launcher even when its path or branch differs from
  these placeholders. Record the actual values and do not create another
  worktree. If the current context is unusable, use the named worktree when it
  matches; only then use the validated manual fallback. Never use `/tmp`,
  `TMPDIR`, or a guessed path.
- **Active spec lane:** component contracts plus
  `docs/contracts/004-shared-control-types.md`; no provisional architecture
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g15/035-solid-status-surfaces.md`
- **Allowed runway:** `g15.035` only
- **Remaining card budget:** one card, one PR, then stop
- **Dispatch topology:** parallel with the existing `g15.021` review loop
- **Parallel safety check:** `g15.021` owns seven application-shell specimen
  pages and its unique parity test. This lane owns Callout/RemediationBanner
  contracts, components, styles, shared types, renderers, their own specimens,
  generated Callout artifacts, focused evidence, and one unique log.
- **Canonical refs:** `AGENTS.md`, `docs/contracts/001-working-rules.md`,
  `docs/contracts/004-shared-control-types.md`,
  `docs/contracts/components/callout.md`,
  `docs/contracts/components/remediation-banner.md`,
  `docs/specs/022-packaging-versioning-and-release-channel-rules.md`,
  `docs/architecture/001-poodle-system-shape.md`, and
  `docs/architecture/product-guardrails.md`
- **Model capability profile:** frontier coding model, high reasoning; this is
  public API, cross-runtime color math, generated evidence, and accessibility
- **Tool/runtime restrictions:** headless only. Do not run windowed,
  native-visual, conformance, Jetstream, or release selectors.
- **Required validation:** focused paired component, Rust render/spec,
  contrast, and specimen evidence; `effigy ir:check`;
  `effigy test:components`; `effigy check:svelte`; `effigy react:build`;
  `effigy test:parity`; `effigy check:gpui`; `effigy regressions:native`;
  `effigy test:web-pack-install`; `effigy docs:check`;
  `git diff --check origin/main...HEAD`
- **PR base/head:** `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting orchestrator review after the pushed PR
- **Merge authorisation:** absent; the worker must not merge

## Boundaries

Please keep this run inside `g15.035`.

- **In scope:** the shared two-value fill type; Callout and RemediationBanner
  contracts, shells, shared CSS, Rust specs/renderers, focused tests,
  representative Svelte/React/GPUI specimens, Callout's presentation-only
  specimen model and regenerated artifacts, contrast evidence, and one August
  batch log.
- **Out of scope:** other component APIs, global status-tone values, token
  schema, shared catalogue navigation/shells, another specimen family,
  Jetstream parity, conformance architecture, release work, and
  `.github/workflows/`.
- Keep tint behavior unchanged. The only adjacent correction is the card's
  explicit Rust Callout default change from Info to the contracted Neutral.
- Use the approved 45% tone / 55% text-primary sRGB solid formula. Do not swap
  in a raw status fill, per-component ratios, or an unproved renderer-specific
  approximation.
- Keep Callout's generated specimen path presentation-only. If callbacks or
  conditional behavior seem necessary, stop and report the boundary.
- Do not add compatibility aliases or a third fill value.
- Work only in the selected worker worktree. Never edit the orchestrator's
  `main` checkout or clean/reset somebody else's dirty state.
- Do not merge the PR.

## Important Context

- **Planning lineage:** g13 rejected behavior-heavy shared component IR; g14
  rejected executable conformance authority. The small display-specimen model
  survived only for static presentation. This card may add the `fill` prop and
  representative instances there, but it must not grow that model into
  behavior.
- **Why this card is ready:** the operator approved the prop name, two values,
  tint default, and inclusion of solid neutral. The card fixes the token rule,
  runtime denominator, public release class, specimens, evidence, validation,
  and stop conditions.
- **Contrast ruling:** `color.text.inverse` is approved foreground. Raw warning
  and success fills do not meet normal-text contrast in every light theme. The
  45/55 sRGB mix keeps one portable formula and measured margin; preserve it
  and make the evidence durable.
- **Composition ruling:** Poodle Buttons inside either action area must remain
  readable without rewriting their supplied variants. Solve this locally to
  the status surface.
- **Existing parity defect:** `CallOutSpec::default()` is Info; web and contract
  default neutral. Correct it and record the behavioral release note.
- **Human-facing rule:** add a compact representative solid group. Tests own
  the exhaustive 6 × 2 matrix; Examples does not.
- **Known baseline:** `effigy doctor` reports the recorded generated-in-src,
  god-file, stale-suppression, and comment-ratio findings. Do not absorb them.
- **Report after:** first, contracts/shared types plus paired web
  implementation and focused tests; second, Rust/GPUI, generated specimens,
  and the final headless gate.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Read this handoff from the top, then run the four-command worktree preflight
below before broad repository reads. Once the worktree is accepted, read the
card and both component contracts. Start with the shared type and contract
edits, then add a small contrast test that pins the approved formula against
all current themes. That gives the web and Rust implementations the same
observable target before either renderer is changed.

Build the paired web shells and shared CSS together, then carry the same type
and color resolver through the Rust specs/renderers and GPUI specimens. Use the
generator for Callout artifacts; never hand-edit them.

## Completion Protocol

### Before you start

1. This file's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad reads,
   run only: `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and the
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root and branch. Do not compare them with the placeholders above or
   create another worktree merely because they differ.
3. Only if the current context is `main`, dirty, unregistered, or unusable
   should you inspect the named worktree. If that also cannot be used, read
   `.agents.local.env` as data and require an absolute, outside-repository
   `AGENTS_WORKTREE_CONTAINER_DIR`. Ask the operator if it is missing. Create a
   unique worktree and branch under that container from `origin/main`. Never
   use `/tmp`, `TMPDIR`, or a guessed path. Never clean, reset, stash over, or
   discard the original checkout. If the launcher itself supplied a dirty or
   `main` worktree, stop and report it rather than hiding the problem.
4. From the selected worktree, run `git fetch origin`; confirm `HEAD` equals
   `origin/main`; confirm `git merge-base --is-ancestor
   eacffd6432b926c48029133aac1e39a81f746b50 HEAD`; and confirm this handoff
   exists in `HEAD`.
5. Read `AGENTS.md`, `docs/roadmaps/g15/README.md`, the `g15.035` card, both
   component contracts, shared control types, and spec 022.
6. Run `effigy tasks` and `effigy doctor`. Record the known doctor baseline;
   stop only if a new finding changes this card's plan.

### While you work

- Update the contracts first, then keep Svelte/React and Rust/GPUI changes in
  coherent paired chunks.
- Add durable full-matrix and contrast evidence before relying on specimens.
- Regenerate Callout artifacts with the repository generator and inspect the
  resulting diff. Do not hand-edit generated output.
- Keep representative specimen copy/order aligned across all three active
  previews. Do not turn Examples into a tone matrix.
- Update one August batch log with package change class, the Rust default
  correction, exact contrast results, generated files, commands actually run,
  and unresolved findings.
- Report each meaningful chunk through the operator with changed files,
  validation run, remaining work, and blockers.
- Stop on any card stop condition or if implementation needs token-schema,
  Button public-API, generated behavior, or another component change.

### When the assigned runway is complete

1. Run the focused paired component, Rust spec/render, contrast, and specimen
   evidence, then `effigy ir:check`, `effigy test:components`,
   `effigy check:svelte`, `effigy react:build`, `effigy test:parity`,
   `effigy check:gpui`, `effigy regressions:native`,
   `effigy test:web-pack-install`, `effigy docs:check`, and
   `git diff --check origin/main...HEAD`.
2. Do not run windowed, native-visual, conformance, Jetstream, or release
   selectors.
3. Finish the batch log. Leave live Svelte/React review of Callout and
   RemediationBanner as an explicit open PR checkpoint; do not claim it passed.
4. Push the selected worker branch and open a PR against current `main`.
5. The PR body must link this handoff, `g15.035`, both component contracts,
   shared control types, spec 022, changed surfaces, contrast evidence,
   generated artifacts, validation, live review checkpoint, and unresolved
   items.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect the PR independently against the card,
contracts, diff, checks, and actual rendered surfaces. Because the worker and
orchestrator may share a GitHub identity, the orchestrator's PR comment is the
canonical review record when formal self-approval is unavailable. Make only
requested changes on this branch and report the updated head through the
operator.

Current review state: awaiting review. Requested changes: none yet. The
operator must explicitly authorise any merge after the code/check gate and the
live paired-preview checkpoint are satisfied.

- **Closeout refs:** `docs/roadmaps/g15/035-solid-status-surfaces.md`, the
  August batch log, `docs/roadmaps/g15/README.md`,
  `docs/roadmaps/generation-index.md`, `docs/roadmaps/README.md`, and
  `docs/roadmaps/dispatch.md`.

### Handoff closeout

Stop after one reviewable PR. Do not mark the card complete or advance the
runway; the orchestrator owns review, merge, roadmap currentness, and the next
dispatch decision.
