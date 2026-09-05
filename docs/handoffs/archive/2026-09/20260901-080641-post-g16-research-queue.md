---
title: Post-g16 research queue planning delegate handoff
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
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260901-080641-post-g16-research-queue.md
base_required: pushed-main
tags: [coordination, handoff, planning, conversation, research, queue, pr]
---

## What This Thread Was Doing

Turn four completed research dossiers into one operator-reviewed post-g16 queue:
block Slider/RangeSlider appearance, Poodle-owned icon morphing, the exhaustive
transitions.dev catalogue audit, and GPU-conscious text shimmer. The delegate
should expose the shared motion decisions, independent work, ordering choices,
and promotion gates without implementing or promoting any track.

This dispatches one operator-facing planning conversation. The delegate owns
discovery and evidence synthesis for this queue, not canonical promotion or
implementation.

## Why It Matters

All four investigations are complete, but each deliberately stops at operator
choices. Three intersect with motion policy; the block slider direction is more
independent but still spans the active runtime cohort. A single synthesis avoids
four competing cards and makes the first useful post-runway bet explicit.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Base commit:** `697d6fc579a3eedf3406c8863a7518cb83f51d5e`
- **Pushed main verification:** local `HEAD` equals `origin/main` at the base commit
- **Planning-delegate branch:** `planning/post-g16-research-queue`
- **Planning-delegate worktree:** `/Users/tom/.paseo/worktrees/1ugbsx1t/post-g16-research-queue`
- **Required sibling worktree links:** none
- **Topic boundary:** operator decisions, dependency synthesis, and recommended promotion order for the four named research dossiers
- **Canonical context:** `docs/roadmaps/README.md`, `docs/roadmaps/g16/README.md`, `docs/contracts/001-working-rules.md`, `docs/architecture/001-poodle-system-shape.md`, `docs/architecture/product-guardrails.md`
- **Named triage packet:** `docs/triage/20260901-080641-post-g16-research-queue.md`
- **Named research evidence:** `docs/research/value-tracks/block-slider-visual-direction.md`, `docs/research/value-tracks/icon-morphing.md`, `docs/research/value-tracks/transitions-dev-catalogue.md`, `docs/research/value-tracks/text-shimmer-effect.md`
- **Allowed write paths:** `docs/triage/20260901-080641-post-g16-research-queue.md` only
- **Concurrent orchestrator work:** `g16.028` implementation owns drag/component/certification surfaces; the HistoryCenter delegate owns only its separate decision packet; neither depends on this queue
- **Frontier planning profile:** user-selected high-effort documentation and audit profile
- **PR base/head:** `main` ← `planning/post-g16-research-queue`
- **PR URL:** pending
- **Promotion owner:** orchestrator after accepted review and merge

## Boundaries

- Stay inside the four named dossiers, their shared dependencies, and the one
  new queue packet.
- Talk directly with the operator. Ask focused questions and distinguish
  operator-confirmed priorities, recommendations, evidence, alternatives, and
  unresolved choices.
- Treat the research dossiers as read-only evidence. Do not repair, rewrite, or
  append to them in this lane.
- Do not edit product code, architecture, contracts, specs, roadmaps, cards,
  logs, front doors, existing triage notes, or component specimens.
- Do not promote a dossier, mark a card ready, choose implementation scope, or
  launch a worker. The operator asked to queue these after the current runway.
- Do not merge. The orchestrator reviews and merges the planning PR, then owns
  any later promotion.

## Important Context

- **Known decisions:** research is complete for all four tracks; block sliders
  should be additive rather than replace the current default; icon morphing's
  recommended public direction is a curated pair registry rather than arbitrary
  raw paths; transitions.dev is evidence for semantic lifecycle policy, not a
  Poodle recipe catalogue; shimmer should not become a generic `TextShimmer`
  API and needs a semantic consumer plus benchmark before admission; Jetstream
  remains deferred.
- **Questions worth exploring:** Which user-visible outcome matters first after
  the current runway? Should one cross-runtime motion-policy architecture lane
  precede transition, morph, and shimmer implementation? Is the block slider
  independent enough to promote first? Which exact slider appearance/content/
  fit decisions is the operator ready to make? Does icon morphing warrant a
  bounded native-feasibility spike before public API work? Which transition
  pilot components justify lifecycle investment? Does shimmer have a confirmed
  semantic consumer, and must it reach GPUI or remain an explicit web recipe?
  What work is rejected, deferred, or grouped rather than merely ranked?
- **Research needs:** Reconcile the four existing dossiers and current
  architecture/contracts. External browsing is out of scope unless one narrow,
  mutable fact blocks a recommendation; cite any such source and stop before
  expanding the research programme.
- **Non-goals:** implementation, pixel matching, importing third-party code,
  copying Pro transitions, generic raw SVG APIs, a generic animated Text prop,
  default Slider replacement, Jetstream admission, release work, or changes to
  the current g16 runway.
- **Mainline drift risk:** `g16.028` may close the drag programme and update
  front doors while this packet is in flight. Rebase before PR and describe the
  queue relative to current `main`; do not resolve drift by editing canonical
  surfaces.
- **Stop conditions:** operator priorities remain genuinely undecided; a
  recommendation needs new material research; the dossiers contradict current
  authority; one topic expands beyond the four-track queue; an unlisted write
  path is required; or promotion would overlap active `g16.028`.

## Suggested Next Move

Read the four recommendations and unresolved-decision sections first. Open with
a compact dependency model: block slider as the independent visual/component
lane, transitions as the likely shared motion-policy foundation, icon morphing
as curated geometry plus that policy, and shimmer as a semantic-consumer and
benchmark decision. Ask the operator which outcome they most want to see, then
test that priority against dependencies and active-cohort cost.

## Completion Protocol

### Before the conversation

1. Confirm the current checkout is a clean, dedicated, non-`main` registered
   worktree for `planning/post-g16-research-queue`. Start with
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`. If the launcher
   did not supply one, use the repository's operator-selected worktree
   container; never guess `/tmp`, clean another checkout, or overwrite dirty
   state.
2. Fetch `origin`, confirm this handoff exists in the selected `HEAD`, and
   confirm `697d6fc579a3eedf3406c8863a7518cb83f51d5e` is an ancestor. The tracked
   handoff is canonical.
3. Required sibling links are `none`; skip sibling-link setup.
4. Read `AGENTS.md`, the named canonical refs, all four dossiers, and any
   current triage index guidance. Do not treat this handoff or the queue packet
   as execution authority.

### During the conversation

- Keep the operator in the loop directly; the orchestrator is not a message
  proxy and may continue unrelated work.
- Preserve exact operator priorities and decisions. Label recommendations,
  evidence, alternatives, rejected options, and open questions separately.
- Keep any research delegation read-only and bounded. Reconcile its output
  before writing the packet.
- Stop on topic expansion, conflicting decisions, an unlisted write path,
  required implementation, or a canonical change that cannot wait for
  promotion.

### When the planning packet is ready

1. Re-read the packet against the conversation. Include a dependency-aware
   recommended order, explicit promotion gates per track, shared versus
   independent decisions, rejected/deferred work, unresolved questions,
   non-goals, and suggested canonical destinations.
2. Run `effigy docs:lint` and `git diff --check origin/main...HEAD`. Inspect the
   full diff; it may contain only the named triage file.
3. Commit and push `planning/post-g16-research-queue`, then open a PR against
   current `main`. The PR body lists base/head, changed files,
   operator-confirmed priorities, recommendations, unresolved questions,
   validation, and the proposed promotion map.
4. Report the PR URL. Do not edit canonical surfaces or merge.

### Review, merge, and promotion

The orchestrator reviews the exact PR head for fidelity to this handoff and the
operator confirmations recorded in the packet, evidence quality, dependency
reasoning, scope, and separation between decisions and recommendations.
Requested changes stay on this branch. Merge is intake, not promotion. After
merge, the orchestrator reconciles the packet with current `main`, chooses the
first canonical promotion, and keeps implementation behind the current runway
and normal readiness gates.

