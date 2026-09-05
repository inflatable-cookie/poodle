---
title: Shared motion policy planning delegate handoff
kind: northstar-handoff
handoff_mode: planning-delegate
planning_mode: conversational-discovery
dispatch_authority: orchestrator
promotion_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260901-121255-motion-policy-planning.md
base_required: pushed-main
tags: [coordination, handoff, planning, motion, conversation, pr]
---

## What This Thread Was Doing

Poodle has closed `g16.028` and `g16.033`. The operator-reviewed research
queue names a shared host-level motion policy and a five-family pilot as the
first post-g16 outcome. This delegate owns the remaining product conversation
and one decision packet; it does not promote or implement the policy.

## Why It Matters

Disclosure, notifications, Tabs, discrete state changes, and loading effects
currently use different lifecycle and reduced-motion assumptions. Icon
morphing and shimmer also need this boundary before their evidence gates can
be interpreted honestly. A canonical implementation card would be premature
until the host default, lifecycle, property budget, and pilot oracle are
operator-settled.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Base commit:** `240f39c866807210daa96dcec81d999c55982e48`
- **Pushed main verification:** local `main` and `origin/main` both resolve to
  `240f39c866807210daa96dcec81d999c55982e48`
- **Planning-delegate branch:** `planning/motion-policy-decision`
- **Planning-delegate worktree:** `/Users/tom/.paseo/worktrees/1ugbsx1t/motion-policy-planning`
- **Required sibling worktree links:** none
- **Topic boundary:** host-level motion policy and the bounded five-family pilot
- **Canonical context:** `docs/contracts/001-working-rules.md`,
  `docs/architecture/001-poodle-system-shape.md`,
  `docs/architecture/010-native-presentation-construction-context.md`,
  `docs/roadmaps/g16/README.md`
- **Named triage packet:** `docs/triage/20260901-121255-motion-policy-decision.md`
- **Named research evidence:**
  `docs/triage/20260901-080641-post-g16-research-queue.md`,
  `docs/research/value-tracks/transitions-dev-catalogue.md`
- **Allowed write paths:**
  `docs/triage/20260901-121255-motion-policy-decision.md` only
- **Concurrent orchestrator work:** interactive drag-and-drop bug fixes and the
  independent design-guidance pilot planning delegate; neither owns this packet
- **Frontier planning profile:** frontier conversational planning selected from
  live adapter profile notes at launch
- **PR base/head:** `main` ← `planning/motion-policy-decision`
- **PR URL:** pending
- **Promotion owner:** orchestrator after accepted review and merge

## Boundaries

- Stay inside motion-policy semantics, lifecycle, propagation, property budget,
  and pilot evidence. Do not design icon morphing, shimmer, block sliders, or a
  transition catalogue here.
- Talk directly with the operator. Keep operator-confirmed decisions distinct
  from recommendations, evidence, alternatives, and open questions.
- Write only the named triage packet. Do not edit product code, tokens,
  architecture, contracts, specs, roadmaps, logs, front doors, or the existing
  research dossiers.
- Do not make a card ready, choose implementation details for a worker, admit
  Jetstream, or claim active-cohort parity from a planning conversation.
- You may use bounded read-only research subagents if a question genuinely
  needs evidence. They cannot edit, contact the operator, or start workers.

## Important Context

- **Known decisions:** policy is host-level across web core and
  `RenderContext`; modes are full, reduced, and deterministic frozen capture;
  semantic/ARIA state updates immediately; stable identity, reversal/retarget,
  abort/unmount cleanup, and final visual state are policy concerns; the
  default property budget is opacity, translation, scale, and rotation; the
  pilot covers Accordion/Collapsible disclosure, ToastStack, Tabs indicator,
  Checkbox plus same-slot IconButton/state swap, and Skeleton/Spinner;
  Dialog presence is out; Jetstream remains deferred.
- **Questions worth exploring:** the default when preference is unavailable;
  exact host/public policy shape and inheritance; reduced and frozen outcomes
  for each pilot role; identity and interruption rules; native approximation
  limits; loop capture state; focus/live-region boundaries; and the smallest
  falsifiable evidence matrix that proves the five families without becoming a
  general transition framework.
- **Research needs:** existing dossier and repository evidence should be enough.
  Research externally only if a concrete unresolved claim requires it.
- **Non-goals:** no named effect catalogue, arbitrary easing/layout/blur/path
  system, Dialog intermediate states, public icon/shimmer API, implementation,
  release, downstream adoption, or permanent conformance authority.
- **Mainline drift risk:** the drag bug lane may touch component implementations
  but not this packet. If it changes a pilot component contract or motion
  behavior materially, record the drift and stop before claiming closure.
- **Stop conditions:** operator intent remains unresolved; the policy requires a
  new authority boundary not represented here; the conversation expands into a
  downstream track; or a canonical edit is needed before the packet can be
  honest.

## Suggested Next Move

Read the queue and transitions dossier, then begin with the smallest decisions
that unlock the rest: what normal hosts do when preference is unavailable, and
what reduced versus frozen means for semantic state and continuous loops. Use
those answers to walk the five pilot families and record a bounded promotion
map.

## Completion Protocol

### Before the conversation

1. Confirm a clean, dedicated, non-`main` registered worktree with the four
   standard Git preflight commands. If the launcher did not supply one, use the
   operator-selected worktree container; never guess a path or clean another
   checkout.
2. Fetch `origin`, confirm this handoff exists in the selected `HEAD`, and
   confirm `240f39c866807210daa96dcec81d999c55982e48` is an ancestor. The
   tracked handoff is canonical.
3. Required sibling links are `none`.
4. Read `AGENTS.md`, the named canonical refs, and the existing queue/research
   evidence. Treat the packet as planning evidence, never execution authority.

### During the conversation

- Keep the operator in the loop directly and work in small question groups.
- Preserve exact operator decisions. Label recommendations and unresolved
  questions rather than silently resolving them.
- Update the named triage packet at meaningful topic shifts so the branch is
  useful without the private transcript.
- Stop on topic expansion, conflicting decisions, implementation, or an
  unlisted write path.

### When the planning packet is ready

1. Re-read it against the conversation. Separate settled decisions,
   recommendations, evidence, alternatives, unresolved questions, non-goals,
   pilot oracle, and proposed canonical destinations.
2. Run `effigy docs:lint` and `git diff --check origin/main...HEAD`; confirm the
   diff contains only the named triage packet.
3. Commit and push `planning/motion-policy-decision`, then open a PR against
   current `main`. The PR body records base/head, decisions, recommendations,
   unresolved questions, validation, and promotion map.
4. Report the PR URL. Do not edit canonical surfaces or merge.

### Review, merge, and promotion

The Poodle orchestrator reviews the exact planning PR head. Accepted merge is
intake, not promotion: the orchestrator then reconciles the packet with current
`main`, promotes settled meaning into canonical architecture/contracts/cards,
removes or splits promoted triage, runs readiness, and only then may dispatch
implementation.
