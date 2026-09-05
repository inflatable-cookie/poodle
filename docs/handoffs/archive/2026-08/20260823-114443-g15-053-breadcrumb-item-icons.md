---
title: g15.053 breadcrumb item icons worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: queued-after-g15-043
owner: Poodle core
created: 2026-08-23
updated: 2026-08-23
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260823-114443-g15-053-breadcrumb-item-icons.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, breadcrumbs, icons]
---

## What This Thread Is Doing

Execute `g15.053` only. Add optional icons to every authored Breadcrumbs item
and support a visually icon-only, semantically named root crumb across Svelte,
React, shared Rust composition, and the GPUI specimen.

This handoff is queued, not parallel with `g15.043`. Do not begin until PR #70
has been accepted and `g15.043` is complete on `origin/main`.

## Why It Matters

An active consumer needs a home glyph with no visible root label. The current
text-only item model cannot express it, and applications should not replace the
whole Breadcrumbs component to add familiar hierarchy icons.

The fixed design keeps `label` as semantic identity, adds `icon` to every item,
and adds an explicit `iconOnly` presentation. It therefore supports the compact
home case without creating an unnamed link or a root-specific prop.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base before this handoff commit:**
  `030d04c263f7a0b61d01337c67c282da01a298e3`
- **Planning checkout at preparation:** clean; local `HEAD` matched
  `origin/main`.
- **Posture:** `strict-ready`, dependency-queued.
- **Worker branch:** `t3code/g15-053-breadcrumb-item-icons`.
- **Worker worktree:** use the clean registered non-`main` worktree supplied by
  the launcher, regardless of its generated path or branch name.
- **Ready card:**
  `docs/roadmaps/g15/053-breadcrumb-item-icons.md`.
- **Allowed runway:** `g15.053` only.
- **Dispatch topology:** serial after `g15.043`; it overlaps
  `packages/contracts/components/src/breadcrumbs.rs` and
  `packages/render/src/breadcrumbs.rs` with PR #70.
- **Canonical refs:** `AGENTS.md`, `.agents/skills/effigy/SKILL.md`, working
  rules 001, architecture 001, the Breadcrumbs contract, `g15.053`, the active
  g15 README, and the release gap register.
- **Model profile:** capable coding model, medium reasoning; escalate if the
  accessibility or icon substrate requires a new architecture.
- **Runtime restriction:** headless only. No windowed/native-visual selector,
  Jetstream preview/QA, workflow, release, tag, or publication command.
- **PR base/head:** current pushed `main` <- worker branch.
- **Review state:** awaiting implementation after the dependency closes.
- **Merge authority:** absent. Push a PR and stop.

## Boundaries

Implement the exact card, including contract, paired web types/components/CSS,
renderer-neutral Rust item data, shared Rust composition through the
post-`g15.043` `RenderContext`, curated Svelte/React/GPUI examples, focused
evidence, one default `home` icon manifest entry plus generated artifacts, and
one August execution log.

Do not add arbitrary item slots, a root-only icon prop, a second Breadcrumbs
renderer, a new icon authority, tooltips, overflow menus, editing, tokens, node
vocabulary, backend paint behavior, unrelated specimen changes, package or
workflow changes, release mutations, Longhorn edits, or Jetstream parity work.

Do not edit the dispatch ledger, active-runway status, or another card. Record
implementation evidence in the `g15.053` log; the orchestrator owns planning
closeout after review.

## Important Context

- The web shape is a discriminated item union: `icon` is optional normally and
  required when `iconOnly: true`; `label` always remains required.
- Rust mirrors named icons with `Option<String>` and constructs icon-only state
  atomically with `with_icon_only(icon)`. A malformed direct native item must
  render its label rather than a blank crumb.
- Icon and label live inside the same anchor, button, or current span. Icons
  are decorative; the containing item owns its accessible name.
- Item icons use the Breadcrumbs resolved size directly and
  `space.inline.xs` inside the item. Existing separator size/opacity/gap and
  breadcrumb navigation/truncation behavior do not change.
- The synthetic ellipsis never inherits icon fields.
- Add only `home` to the default icon manifest and regenerate with the existing
  build. `folder` and `package` already exist.
- The specimen adds one compact Icons group: icon-only Home, labelled Folder,
  labelled current Poodle. Keep the existing teaching groups.
- Report after the contract/data-model batch and after the coherent
  implementation/evidence batch. Stop on a planning or substrate gap.

## Suggested Next Move

After the orchestrator confirms PR #70 merged, start by reading this file. Run
the four-command worktree preflight below before broad repository reads. Reuse
the launcher-provided worktree when it is clean, registered, and non-`main`;
do not create another because its generated name differs from this handoff.

Then confirm the dependency and current API shape, read the card and contract,
and implement the public item model before touching specimens. Keep the change
one bounded cross-runtime batch rather than landing web and native variants
independently.

## Completion Protocol

### Before editing

1. This file's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Run only:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. Reuse a clean registered non-`main` launcher worktree. If the launcher
   supplied `main`, a dirty checkout, or an unregistered checkout, stop and
   report it. Do not clean, reset, stash, or silently create a second worktree.
3. Fetch `origin`; require `HEAD == origin/main`; require the planning base
   above to be an ancestor; confirm this handoff exists in `HEAD`.
4. Confirm `g15.043` is marked complete on current `origin/main` and the shared
   Breadcrumbs renderer uses `RenderContext`. If not, stop: the lane is still
   dependency-blocked.
5. Read `AGENTS.md`, the Effigy skill, the ready card, Breadcrumbs contract,
   active g15 README, and the exact current source files named by the card.
6. Run `effigy tasks` and the smallest clean Breadcrumbs/icon starting checks.

If no launcher worktree exists and the operator explicitly asks for manual
fallback, read `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`,
and create a unique worktree/branch under that container from `origin/main`.
Ask if the key is absent. Never use `/tmp`, `TMPDIR`, or a guessed path.

### While working

- Execute only `g15.053`; keep commits aligned with the contract/model batch
  and implementation/evidence batch.
- Report meaningful chunks through the operator with changed files, validation
  actually run, remaining work, and blockers.
- Stop on any card stop condition. Do not invent a new accessibility, icon,
  scene, or backend architecture.
- Generate icon artifacts through `effigy icons:build`; never hand-edit them.

### Completion

1. Run the card's final validation: focused paired and Rust tests,
   `effigy icons:build`, `effigy audit:icons`, `effigy test:components`,
   `effigy check:svelte`, `effigy react:build`,
   `effigy probe:gpui-specimens`, `effigy check:gpui`, `effigy ci:rust`,
   `effigy docs:check`, and `git diff --check origin/main...HEAD`.
2. Never run a windowed/native-visual/Jetstream preview or QA selector, release
   mutation, workflow, tag, or publication command.
3. Write one `docs/logs/2026-08/20260823-g15-053-*.md` execution record with
   API shape, accessibility behavior, runtime evidence, generated icon change,
   validation, and any honest limitation.
4. Inspect the diff against the writable and forbidden scopes, commit, push,
   and open one PR against current `main`.
5. Link the card, contract, log, validation, and operator-review requirement in
   the PR body. Report the PR URL and stop. Never merge.

### Review and closeout

The orchestrator independently reviews the contract, item types, accessible
icon-only shape, shared rendering, generated assets, focused tests, specimens,
and headless checks. The operator reviews the Svelte/React and GPUI teaching
examples before merge. Formal self-approval may be unavailable; the
orchestrator's PR comment is the canonical review record.

After explicit operator merge authority and a green review gate, the
orchestrator merges, closes `g15.053`, updates the runway and release gap, and
returns to `g15.050`.
