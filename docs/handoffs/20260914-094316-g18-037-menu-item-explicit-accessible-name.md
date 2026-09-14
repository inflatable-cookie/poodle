---
title: g18.037 Menu item explicit accessible names
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar Chatterbox
created: 2026-09-14
updated: 2026-09-14
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom said Continue on 2026-09-14 after Chatterbox proposed promoting the assessed Menu accessible-name fix as g18.037."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: false
tags: [coordination, handoff, worker, g18, g18.037, menu, accessibility]
---

## What This Thread Was Doing

Execute [`g18.037`](../roadmaps/g18/037-menu-item-explicit-accessible-name.md)
as the next bounded Poodle-owned finding from the consumer papercut sweep.

## Why It Matters

MenuSurface's visible required label gives conforming browsers a computed
accessible name, but the semantic label is not projected explicitly in either
web runtime. Bounded accessibility serializers can therefore lose names for
the menuitem role family. Poodle already owns the label and shared Rust already
projects it; web parity should be explicit too.

## Current State

Main is clean and synchronized at the planning commit carrying this handoff.
g18.036 is merged and terminal, and Queue inspection found no unfinished
Poodle task. Both web MenuSurface adapters render every non-separator row as a
button with visible `item.label`, role/state attributes, and hidden metadata,
but no item `aria-label`. Menu and ContextMenu share these adapters.

## Boundaries

Change only the Svelte and React MenuSurface item attributes, paired Menu and
ContextMenu tests as needed, and the Menu/ContextMenu accessibility contracts.
Use exactly the existing required `item.label`; add no prop or fallback. Cover
action, checkbox, radio, disabled, separator, ContextMenu inheritance, and the
Svelte submenu parent. Do not edit Longhorn, Figmatic, serializers, consumer
pins, other menuitem renderers, core DOM, native/GPUI code, workflows, or
release state.

## Important Context

### UI Design Brief

This is an accessibility refinement with no visible or workflow change. Each
non-separator row exposes the same text users already see as its exact explicit
name. Separators remain unnamed; shortcut, check, and submenu metadata remains
hidden. Preserve roles, checked/disabled state, focus movement, activation,
dismissal, placement, and submenu behavior.

The browser's current computed name is already correct. This task stabilizes
the semantic projection for bounded serializers; it must not invent new copy
or imply a general role-policy sweep. Shared Rust already satisfies the rule.

Validation budget: run the paired focused Svelte/React Menu and ContextMenu
tests during implementation, then one final `effigy ci:web`, one Impeccable
detector pass over the changed UI targets, and `git diff --check`. Do not add
overlapping `test:components`, `docs:check`, or `qa` boards after a green
`ci:web`.

## Suggested Next Move

Plant the role-family matrix and separator counterexample first, then add the
one exact label attribute to both shared adapters and prove ContextMenu
inherits it.

## Completion Protocol

Push one clean non-draft PR from the Queue workspace. Report exact focused
tests, the final web board, detector result, and diff check as
`ready_for_review`, then stop. Queue owns independent exact-head review, CI
observation, merge, and hook closeout. Leave consumer adoption and npm release
state untouched.
