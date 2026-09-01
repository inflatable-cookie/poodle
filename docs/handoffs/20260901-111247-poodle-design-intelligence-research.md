---
title: Poodle agent-facing design guidance and evaluation research handoff
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
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260901-111247-poodle-design-intelligence-research.md
base_required: pushed-main
tags: [coordination, handoff, planning, research, design-guidance, evaluation, pr]
---

## What This Thread Was Doing

Research whether the method in Vercel's “How our agents build on-brand pages
with design.md” can improve how agents compose, review, and teach Poodle
interfaces. Produce one evidence-backed value-track dossier. Do not create the
guidance, skill, eval harness, component changes, or an implementation card.

The operator approved this bounded research lane. It may run beside `g16.033`.
The delegate owns source collection and synthesis, not canonical promotion or
implementation.

## Why It Matters

Poodle already owns strong semantic contracts, shared tokens, human-centred
specimen guidance, and cross-framework visual comparison. Those surfaces
largely prove behavior and renderer agreement. They do not yet give an agent a
small, explicit layer for composition judgment or measure whether generated
interfaces are useful, coherent, and recognizably Poodle on the first attempt.

Vercel's method separates prose judgment, mechanical styling constraints,
deterministic checks, and an iterative evaluation harness. Poodle needs a
careful translation of that structure, not a copy of Vercel's brand rules.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning base:** `8cccdc65cb14fb6d1aa3fbc67bd9c3296977076d`
- **Pushed-main verification:** local `HEAD` equals `origin/main` at the planning base before this handoff commit
- **Planning-delegate branch:** `research/poodle-design-intelligence`
- **Planning-delegate worktree:** Paseo-managed `research-poodle-design-intelligence`
- **Required sibling worktree links:** none
- **Topic boundary:** agent-facing Poodle design judgment, distribution, governance, and composition-quality evaluation
- **Canonical context:** `docs/architecture/product-guardrails.md`, `docs/contracts/001-working-rules.md`, `docs/roadmaps/g14/026-human-centred-specimen-catalogue-audit.md`, `docs/roadmaps/g15/011-specimen-catalogue-audit.md`, `docs/roadmaps/g15/specimen-catalogue-audit.md`, `docs/roadmaps/g15/specimen-plan-outline.md`, `test/visual/README.md`, `docs/policy/internal-writing-style.md`
- **Research process:** `docs/research/README.md`, `docs/research/master-index.md`, `docs/research/research-to-implementation-playbook.md`, `docs/research/templates/template-value-track.md`
- **Named research output:** `docs/research/value-tracks/agent-facing-design-guidance-and-evaluation.md`
- **Allowed write paths:** the named research output only
- **Concurrent orchestrator work:** `g16.033` runs separately and owns HistoryCenter contracts, packages, tests, roadmap/log/front-door closeout, and its `Papercuts` workspace; this lane must not touch any of those surfaces
- **Research profile:** operator-selected Luna at maximum reasoning
- **PR base/head:** `main` <- `research/poodle-design-intelligence`
- **PR URL:** pending
- **Promotion owner:** orchestrator after accepted review and merge

## Boundaries

- Write one point-in-time research dossier. Do not edit existing research,
  triage, architecture, contracts, specs, roadmaps, logs, front doors, skills,
  source, tests, specimens, or configuration.
- Treat Poodle contracts as authority and the dossier as evidence only.
- Separate these candidate outcomes rather than collapsing them:
  1. a repository-local contributor skill that routes to canonical sources;
  2. a compact consumer-facing `design.md` or equivalent public guidance;
  3. a matched composition-quality evaluation harness.
- Do not duplicate contracts or tokens into a second source of truth. Assess
  reference routing, ownership, update cadence, and drift detection for every
  proposed guidance surface.
- Do not treat Svelte/React pixel agreement as proof of design quality. Do not
  turn specimens into an exhaustive conformance corpus.
- Do not design a Slack corpus agent, copy Vercel's stylesheet or brand rules,
  open an implementation card, or recommend a broad agent platform.
- Keep Jetstream deferred. The active Poodle cohort and public package boundary
  remain governed by current repository rules.
- Do not merge. The orchestrator reviews and merges the research PR, then owns
  any translation memo, architecture decision, or later planning.

## Important Context

- **Primary seed:** [Vercel, “How our agents build on-brand pages with design.md”](https://vercel.com/blog/how-our-agents-build-on-brand-pages-with-design-md), published 2026-08-31. Inspect the exact article, its linked artifacts, method, sample size, stated limitations, and update loop.
- **Named adjacent primary sources:** [Vercel's public `design.md`](https://vercel.com/design.md), [“Teaching agents product design at Vercel”](https://vercel.com/blog/teaching-agents-product-design-at-vercel), and the [Eve design template](https://github.com/vercel-labs/eve-design-template). Follow only primary links that materially answer the research questions.
- **Known local assets:** Poodle has behavioral contracts, shared CSS/tokens,
  a fixed human-centred specimen rubric, cross-runtime specimen plans, and a
  Svelte/React pixel-diff gate. `.agents/skills/` currently contains only the
  repository's Effigy routing skill. Verify current state rather than relying
  on this summary.
- **Opening research questions:**
  - What problem does each of the three candidate outcomes solve, and who is
    its audience: Poodle contributors, package consumers, or evaluators?
  - Which existing Poodle documents already supply judgment, mechanics,
    examples, and deterministic enforcement? Where are the material gaps?
  - Which recurring Poodle specimen/review corrections are stable enough to
    become guidance, and which remain contextual judgment? Use a bounded,
    named evidence sample rather than an unbounded history scrape.
  - What is the smallest credible evaluation: realistic composition prompts,
    matched with/without guidance runs, model/version recording, first-attempt
    outputs, blind human rubric, holdout scenarios, and falsifiable success
    thresholds?
  - How should Poodle distinguish semantic/behavioral parity, visual renderer
    parity, accessibility, and composition quality so one metric cannot stand
    in for another?
  - Which recurring failures belong in prose, shared package mechanics,
    deterministic checks, or the harness? When should a correction stay out?
  - What governance prevents a skill or `design.md` from drifting from
    contracts, tokens, and specimen authority?
  - Which claims from Vercel's small evaluation transfer to Poodle, which are
    source-author claims only, and which require a Poodle pilot before use?
- **Required recommendations:** compare `adopt`, `adapt`, `hold`, and `reject`
  for each candidate outcome; propose a dependency-aware pilot if warranted;
  name promotion gates, operator decisions, non-goals, risks, and the smallest
  useful next artifact. A recommendation is not permission to implement.
- **Evidence rules:** prefer official documentation, source repositories, and
  standards. Label live pages mutable. Pin repository citations to an exact
  revision when line-sensitive or used as durable evidence. Separate verified
  facts, source-author claims, and worker inferences. Do not quote sources at
  length or copy proprietary prompt/style content into Poodle.
- **Stop conditions:** the work requires implementation or a code spike; a
  source cannot be accessed or reproduced; a proposed public surface requires
  an operator-owned product decision; the dossier would need an unlisted path;
  current authority contradicts the premise; or the topic expands into generic
  agent infrastructure.

## Suggested Next Move

Read the article and its linked public artifacts first, then map the method's
four enforcement layers against Poodle's current contracts, token/style
packages, specimens, visual gates, and review history. Use that gap map to test
whether Poodle needs one, two, or all three candidate outcomes. Prefer a small
pilot recommendation with a clear kill condition over a broad programme.

## Completion Protocol

### Before research

1. Confirm the current checkout is a clean, dedicated, non-`main` registered
   worktree. Start with `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`. Use the launcher-provided worktree even if
   its generated path or branch differs from this handoff. Do not create a
   second worktree or clean another checkout.
2. Fetch `origin`, confirm this handoff exists in the selected `HEAD`, and
   confirm the planning base is an ancestor. The tracked handoff is canonical.
3. Required sibling links are `none`; skip sibling setup.
4. Read `AGENTS.md`, the named research process, and the canonical context.
   Read the repository-local Effigy skill before validation. Do not treat this
   handoff as product authority.

### During research

- Browse because the task concerns a named, current article and external
  artifacts. Use primary/official sources for technical claims.
- Keep source acquisition reproducible. Record checked dates, exact revisions
  where available, and acknowledged limitations.
- Inspect local source and history read-only. Stay bounded to evidence needed
  for the gap map and proposed pilot.
- Do not ask the operator to decide the eventual public shape during this lane.
  Record genuine choices as explicit unresolved promotion gates.
- Stop on expansion, contradiction, missing evidence, implementation, or an
  unlisted write path.

### When the dossier is ready

1. Ensure the dossier includes an executive summary, method/source inventory,
   current Poodle audit, Vercel-method analysis, gap map, candidate comparison,
   proposed bounded pilot and eval rubric, governance/source-of-truth model,
   risks and rejected work, exact recommendation, operator decisions, and
   follow-up disposition.
2. Make the disposition explicit: research complete and awaiting orchestrator
   review; no implementation card or canonical change is authorized.
3. Run `effigy docs:lint` and `git diff --check origin/main...HEAD`. Inspect the
   full diff; it may contain only
   `docs/research/value-tracks/agent-facing-design-guidance-and-evaluation.md`.
4. Commit and push `research/poodle-design-intelligence`, then open a PR against
   current `main`. The PR body lists sources, findings, recommendations,
   unresolved decisions, changed files, validation, and proposed triage
   disposition.
5. Report the exact head and PR URL. Do not merge or promote.

### Review, merge, and promotion

The orchestrator reviews the exact PR head for source quality, reproducibility,
transfer reasoning, local evidence, scope, and separation of facts, claims,
inferences, recommendations, and unresolved decisions. Requested changes stay
on the same branch and return to this delegate. Merge is research intake only.
Any public guidance, skill, eval harness, translation memo, architecture change,
or roadmap work requires a separate orchestrator-owned promotion decision.
