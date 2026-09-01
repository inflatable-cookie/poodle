---
title: Design-guidance pilot planning delegate handoff
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
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260901-121256-design-guidance-pilot-planning.md
base_required: pushed-main
tags: [coordination, handoff, planning, design-guidance, evaluation, conversation, pr]
---

## What This Thread Was Doing

PR #119 merged the research dossier on Vercel's agent-facing design method.
The evidence recommends adapting a small contributor routing layer and testing
it with a finite manual first-attempt pilot, while holding a public `design.md`
and any permanent evaluation harness. This delegate owns the operator
conversation needed to decide whether and how that pilot should proceed.

## Why It Matters

Poodle already has component contracts, tokens, specimens, deterministic
checks, and bounded human review. The open question is whether a small
agent-readable routing layer improves composition choices without duplicating
those authorities. A finite matched pilot can answer that before Poodle adopts
another maintained guidance or evaluation surface.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Base commit:** `240f39c866807210daa96dcec81d999c55982e48`
- **Pushed main verification:** local `main` and `origin/main` both resolve to
  `240f39c866807210daa96dcec81d999c55982e48`
- **Planning-delegate branch:** `planning/design-guidance-pilot-decision`
- **Planning-delegate worktree:** `/Users/tom/.paseo/worktrees/1ugbsx1t/design-guidance-pilot-planning`
- **Required sibling worktree links:** none
- **Topic boundary:** contributor-local design guidance and its finite manual
  composition-quality pilot
- **Canonical context:** `docs/contracts/001-working-rules.md`,
  `docs/architecture/001-poodle-system-shape.md`,
  `docs/architecture/003-component-docs-ia-and-implementation-substrates.md`,
  `docs/roadmaps/g15/specimen-catalogue-audit.md`
- **Named triage packet:**
  `docs/triage/20260901-121256-design-guidance-pilot-decision.md`
- **Named research evidence:**
  `docs/research/value-tracks/agent-facing-design-guidance-and-evaluation.md`
- **Allowed write paths:**
  `docs/triage/20260901-121256-design-guidance-pilot-decision.md` only
- **Concurrent orchestrator work:** interactive drag-and-drop bug fixes and the
  separate motion-policy planning delegate; neither owns this packet
- **Frontier planning profile:** frontier conversational planning selected from
  live adapter profile notes at launch
- **PR base/head:** `main` ← `planning/design-guidance-pilot-decision`
- **PR URL:** pending
- **Promotion owner:** orchestrator after accepted review and merge

## Boundaries

- Keep the conversation to the contributor-local routing candidate, finite
  manual evaluation, ownership, rubric, and promotion/stop decisions.
- Talk directly with the operator and distinguish confirmed choices from
  recommendations, evidence, alternatives, and open questions.
- Write only the named triage packet. Do not create the skill, prompts,
  fixtures, generated pages, evaluation harness, selectors, public
  `design.md`, canonical architecture, contracts, roadmaps, or implementation.
- Keep semantic parity, renderer parity, accessibility, mechanical failures,
  and composition quality as separate evidence tracks.
- Do not copy Vercel brand guidance, reproduce its public file, import its
  thresholds, or generalize a Svelte pilot to React/GPUI.

## Important Context

- **Known decisions:** adapt a small contributor routing surface only if it
  links to canonical Poodle authority; hold a consumer-facing `design.md`;
  adapt a finite manual matched evaluation; hold permanent automation; use
  first attempts, fixed conditions, blind human review, and a hidden holdout;
  reject pixel difference or one aggregate quality score as the verdict.
- **Questions worth exploring:** confirm the contributor-local candidate as the
  only condition under test; name guidance and review owners; approve or change
  the proposed three in-set scenarios plus one holdout, two trials per
  condition, and Svelte-only target; settle rubric aggregation and N/A rules;
  define predeclared blocker/known-failure handling; set promotion, revise, and
  reject thresholds before results exist; choose retention and model/version
  recording boundaries.
- **Research needs:** none expected. The merged dossier contains the external
  and local evidence. Any new source must answer a concrete unresolved claim.
- **Non-goals:** no public guide, broad design prompt, model judge, Slack/PR
  corpus agent, permanent harness, catalogue replacement, token/class system,
  runtime implementation, or release coupling.
- **Mainline drift risk:** motion planning and drag bug work do not own this
  packet. Stop if another lane creates or reserves the same guidance/pilot
  authority.
- **Stop conditions:** the operator does not want a pilot; candidate, owner,
  audience, scoring, or threshold remains materially unresolved; the work needs
  generated outputs or implementation; or the topic expands to public package
  documentation.

## Suggested Next Move

Read the merged dossier, then ask whether the pilot should test only a small
contributor-local routing draft—the dossier's recommendation—or another
candidate. From there settle ownership, frozen scenario/trial shape, blind
grading, and the decision rule before any outputs are generated.

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
4. Read `AGENTS.md`, the named canonical refs, and the merged research dossier.
   Treat the packet as planning evidence, never execution authority.

### During the conversation

- Keep the operator in the loop directly and work in small question groups.
- Preserve exact operator decisions. Label recommendations and unresolved
  questions rather than silently resolving them.
- Update the named triage packet at meaningful topic shifts so the branch is
  useful without the private transcript.
- Stop on topic expansion, conflicting decisions, implementation, generated
  pilot outputs, or an unlisted write path.

### When the planning packet is ready

1. Re-read it against the conversation. Separate settled decisions,
   recommendations, evidence, alternatives, unresolved questions, non-goals,
   decision thresholds, and proposed canonical destinations.
2. Run `effigy docs:lint` and `git diff --check origin/main...HEAD`; confirm the
   diff contains only the named triage packet.
3. Commit and push `planning/design-guidance-pilot-decision`, then open a PR
   against current `main`. The PR body records base/head, decisions,
   recommendations, unresolved questions, validation, and promotion map.
4. Report the PR URL. Do not edit canonical surfaces or merge.

### Review, merge, and promotion

The Poodle orchestrator reviews the exact planning PR head. Accepted merge is
intake, not promotion: the orchestrator then reconciles the packet with current
`main`, chooses canonical destinations, promotes settled meaning, removes or
splits resolved triage, and only then decides whether a pilot implementation
lane is ready.
